use artifacthub_client::models::{SearchRepositoriesResponse, SearchRepositoryResult};
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::{self as pkg_kind};

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct SearchRepositoriesParams {
    #[schemars(description = "Search query string for repository name")]
    pub name: Option<String>,
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: Option<String>,
    #[schemars(description = "Filter by user alias (repositories owned by user)")]
    pub user: Option<String>,
    #[schemars(description = "Filter by organization name")]
    pub org: Option<String>,
    #[schemars(description = "Number of results (max 60)")]
    pub limit: Option<usize>,
    #[schemars(description = "Offset for pagination")]
    pub offset: Option<usize>,
}

pub async fn handle_search_repositories(
    server: &ArtifactHubServer,
    params: SearchRepositoriesParams,
) -> Result<Json<SearchRepositoriesResponse>, String> {
    if let Some(limit) = params.limit
        && (limit == 0 || limit > 60)
    {
        return Err("limit must be between 1 and 60".to_string());
    }

    let mut query_params: Vec<(String, String)> = vec![];

    if let Some(name) = &params.name {
        query_params.push(("name".to_string(), name.clone()));
    }
    if let Some(kind) = &params.kind {
        let id = if let Some(id) = pkg_kind::to_id(kind) {
            id.to_string()
        } else {
            return Err(format!(
                "Unknown kind: '{}'. Valid kinds: {}",
                kind,
                pkg_kind::valid_kinds().join(", ")
            ));
        };
        query_params.push(("kind".to_string(), id));
    }
    if let Some(user) = &params.user {
        query_params.push(("user".to_string(), user.clone()));
    }
    if let Some(org) = &params.org {
        query_params.push(("org".to_string(), org.clone()));
    }
    if let Some(limit) = params.limit {
        query_params.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(offset) = params.offset {
        query_params.push(("offset".to_string(), offset.to_string()));
    }

    let json = server
        .client
        .get_json("/repositories/search", &query_params)
        .await?;
    let repositories: Vec<SearchRepositoryResult> =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(SearchRepositoriesResponse { repositories }))
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
                    "cncf": false,
                    "package_count": 500
                }
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: Some("bitnami".to_string()),
                kind: Some("helm".to_string()),
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
    async fn test_search_repositories_invalid_kind() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_repositories(
            &server,
            SearchRepositoriesParams {
                name: None,
                kind: Some("invalid".to_string()),
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
