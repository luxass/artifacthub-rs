use artifacthub_client::models::SearchResponse;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::ArtifactHubServer;
use crate::tools::validation::{resolve_kind_ids, validate_limit};
use artifacthub_client::kind::{self as pkg_kind};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchParams {
    #[schemars(description = "Search query string (web search syntax)")]
    pub q: Option<String>,
    #[schemars(description = "Raw tsquery (advanced)")]
    pub ts_query: Option<String>,
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: Option<Vec<String>>,
    #[schemars(description = "Filter by repository names (repeatable)")]
    pub repo: Option<Vec<String>>,
    #[schemars(description = "Filter by organization names (repeatable)")]
    pub org: Option<Vec<String>>,
    #[schemars(description = "Filter by user aliases (repeatable)")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by category ids (repeatable)")]
    pub category: Option<Vec<i32>>,
    #[schemars(description = "Only verified publishers")]
    pub verified_publisher: Option<bool>,
    #[schemars(description = "Only official packages")]
    pub official: Option<bool>,
    #[schemars(description = "Only CNCF projects")]
    pub cncf: Option<bool>,
    #[schemars(description = "Only operators")]
    pub operators: Option<bool>,
    #[schemars(description = "Include deprecated (default excludes)")]
    pub deprecated: Option<bool>,
    #[schemars(description = "Filter by licenses (repeatable)")]
    pub license: Option<Vec<String>>,
    #[schemars(description = "Filter by capabilities (repeatable)")]
    pub capabilities: Option<Vec<String>>,
    #[schemars(description = "Sort: relevance|stars|last_updated")]
    pub sort: Option<String>,
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

pub async fn handle_search_packages(
    server: &ArtifactHubServer,
    params: SearchParams,
) -> Result<Json<SearchResponse>, String> {
    validate_limit(params.limit)?;

    let kind_ids = resolve_kind_ids(params.kind)?;
    let mut search = server.client.packages().search();

    if let Some(q) = params.q {
        search = search.query(q);
    }
    if let Some(q) = params.ts_query {
        search = search.ts_query(q);
    }
    if !kind_ids.is_empty() {
        search = search.kinds(kind_ids);
    }
    if let Some(repos) = params.repo {
        search = search.repos(repos);
    }
    if let Some(orgs) = params.org {
        search = search.orgs(orgs);
    }
    if let Some(users) = params.user {
        search = search.users(users);
    }
    if let Some(categories) = params.category {
        search = search.categories(categories.iter().map(|c| c.to_string()));
    }
    if let Some(v) = params.verified_publisher {
        search = search.verified_publisher(v);
    }
    if let Some(v) = params.official {
        search = search.official(v);
    }
    if let Some(v) = params.cncf {
        search = search.cncf(v);
    }
    if let Some(v) = params.operators {
        search = search.operators(v);
    }
    if let Some(v) = params.deprecated {
        search = search.deprecated(v);
    }
    if let Some(licenses) = params.license {
        search = search.licenses(licenses);
    }
    if let Some(caps) = params.capabilities {
        search = search.capabilities(caps);
    }
    if let Some(sort) = params.sort {
        search = search.sort(sort);
    }
    if let Some(limit) = params.limit {
        search = search.limit(limit);
    }
    if let Some(offset) = params.offset {
        search = search.offset(offset);
    }

    let response = search.send().await?;

    Ok(Json(response))
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
    async fn test_search_packages_returns_results() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/search"))
            .and(query_param("ts_query_web", "test"))
            .and(query_param("kind", "0"))
            .and(query_param("limit", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "packages": [
                    {
                        "package_id": "abc-123",
                        "name": "test-chart",
                        "normalized_name": "test-chart",
                        "version": "1.0.0",
                        "description": "A test chart",
                        "repository": {
                            "name": "test-repo",
                            "display_name": "Test Repo",
                            "url": "https://example.com"
                        },
                        "deprecated": false,
                        "signed": false,
                        "stars": 42,
                        "ts": 1700000000
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: Some("test".to_string()),
                ts_query: None,
                kind: Some(vec!["helm".to_string()]),
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: None,
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: None,
                limit: Some(10),
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.packages.len(), 1);
        assert_eq!(result.0.packages[0].name, "test-chart");
        assert_eq!(result.0.packages[0].stars, 42);
    }

    #[tokio::test]
    async fn test_search_packages_sends_multi_value_and_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/search"))
            .and(query_param("kind", "0"))
            .and(query_param("verified_publisher", "true"))
            .and(query_param("sort", "stars"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "packages": []
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                ts_query: None,
                kind: Some(vec!["helm".to_string()]),
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: Some(true),
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: Some("stars".to_string()),
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        assert!(result.0.packages.is_empty());
    }

    #[tokio::test]
    async fn test_search_packages_captures_total_count_header() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"packages": []}))
                    .insert_header("Pagination-Total-Count", "42"),
            )
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                ts_query: None,
                kind: None,
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: None,
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.total_count, Some(42));
    }

    #[tokio::test]
    async fn test_search_packages_invalid_kind() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                ts_query: None,
                kind: Some(vec!["invalid-kind".to_string()]),
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: None,
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: None,
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
        assert!(err.contains("Valid kinds"));
    }

    #[tokio::test]
    async fn test_search_packages_limit_too_high() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                ts_query: None,
                kind: None,
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: None,
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: None,
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

    #[tokio::test]
    async fn test_search_packages_limit_zero() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                ts_query: None,
                kind: None,
                repo: None,
                org: None,
                user: None,
                category: None,
                verified_publisher: None,
                official: None,
                cncf: None,
                operators: None,
                deprecated: None,
                license: None,
                capabilities: None,
                sort: None,
                limit: Some(0),
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
