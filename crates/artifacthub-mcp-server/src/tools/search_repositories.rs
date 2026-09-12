use artifacthub_client::models::SearchRepositoriesResponse;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::ArtifactHubServer;
use crate::tools::validation::{resolve_kind_ids, validate_limit};
use artifacthub_client::kind::{self as pkg_kind};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoriesParams {
    #[schemars(description = "Search query string for repository name (regex upstream)")]
    pub name: Option<String>,
    #[schemars(description = "Exact repository URL match")]
    pub url: Option<String>,
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: Option<Vec<String>>,
    #[schemars(description = "Filter by user aliases (repeatable)")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by organization names (repeatable)")]
    pub org: Option<Vec<String>>,
    #[schemars(
        description = "Number of results (max 60)",
        transform = crate::tools::schema::remove_format
    )]
    pub limit: Option<usize>,
    #[schemars(
        description = "Offset for pagination",
        transform = crate::tools::schema::remove_format
    )]
    pub offset: Option<usize>,
}

pub async fn handle_search_repositories(
    server: &ArtifactHubServer,
    params: SearchRepositoriesParams,
) -> Result<Json<SearchRepositoriesResponse>, String> {
    validate_limit(params.limit)?;

    let kind_ids = resolve_kind_ids(params.kind)?;
    let mut search = server.client.repositories().search();

    if let Some(name) = params.name {
        search = search.name(name);
    }
    if let Some(url) = params.url {
        search = search.url(url);
    }
    if !kind_ids.is_empty() {
        search = search.kinds(kind_ids);
    }
    if let Some(users) = params.user {
        search = search.users(users);
    }
    if let Some(orgs) = params.org {
        search = search.orgs(orgs);
    }
    if let Some(limit) = params.limit {
        search = search.limit(limit);
    }
    if let Some(offset) = params.offset {
        search = search.offset(offset);
    }

    let repositories = search.send().await?;

    Ok(Json(repositories))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ALL_TOOL_NAMES;
    use artifacthub_client::client::ArtifactHubClient;
    use std::collections::HashSet;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_server(base_url: &str) -> ArtifactHubServer {
        ArtifactHubServer {
            client: ArtifactHubClient::with_base_url(base_url),
            enabled_tools: ALL_TOOL_NAMES
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>(),
        }
    }

    #[tokio::test]
    async fn test_search_repositories_returns_results() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repositories/search"))
            .and(query_param("name", "bitnami"))
            .and(query_param("kind", "0"))
            .and(query_param("limit", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "repository_id": "repo-123",
                    "name": "bitnami",
                    "display_name": "Bitnami",
                    "url": "https://charts.bitnami.com/bitnami",
                    "kind": 0,
                    "verified_publisher": true,
                    "official": true,
                    "cncf": false
                }
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: Some("bitnami".to_string()),
                url: None,
                kind: Some(vec!["helm".to_string()]),
                user: None,
                org: None,
                limit: Some(10),
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.repositories.len(), 1);
        assert_eq!(result.0.repositories[0].name, "bitnami");
        assert!(result.0.repositories[0].verified_publisher);
    }

    #[tokio::test]
    async fn test_search_repositories_false_bools_still_serialize() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repositories/search"))
            .and(query_param("org", "kvalitetsit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "repository_id": "repo-456",
                    "name": "kvalitetsit",
                    "url": "https://example.com",
                    "kind": 0,
                    "verified_publisher": false,
                    "official": false
                }
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: None,
                url: None,
                kind: None,
                user: None,
                org: Some(vec!["kvalitetsit".to_string()]),
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.repositories.len(), 1);
        let value = serde_json::to_value(&result.0).unwrap();
        let repo = &value["repositories"][0];
        assert_eq!(repo["official"], serde_json::Value::Bool(false));
        assert_eq!(repo["verified_publisher"], serde_json::Value::Bool(false));
    }

    #[tokio::test]
    async fn test_search_repositories_captures_total_count_header() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repositories/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([]))
                    .insert_header("Pagination-Total-Count", "7"),
            )
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: None,
                url: None,
                kind: None,
                user: None,
                org: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.total_count, Some(7));
    }

    #[tokio::test]
    async fn test_search_repositories_invalid_kind() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: None,
                url: None,
                kind: Some(vec!["invalid".to_string()]),
                user: None,
                org: None,
                limit: None,
                offset: None,
            },
        )
        .await;

        assert!(result.is_err());
        let Err(err) = result else {
            panic!("expected error")
        };
        assert!(err.contains("Unknown kind"));
    }

    #[tokio::test]
    async fn test_search_repositories_limit_too_high() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: None,
                url: None,
                kind: None,
                user: None,
                org: None,
                limit: Some(61),
                offset: None,
            },
        )
        .await;

        assert!(result.is_err());
        let Err(err) = result else {
            panic!("expected error")
        };
        assert!(err.contains("limit must be between 1 and 60"));
    }
}
