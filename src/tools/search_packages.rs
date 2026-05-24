use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::kind::{self as pkg_kind};
use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResponse {
    pub packages: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResult {
    pub package_id: String,
    pub name: String,
    pub normalized_name: String,
    pub version: String,
    pub description: String,
    pub repository: SearchRepositoryInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub signed: bool,
    pub stars: i32,
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoryInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct SearchParams {
    #[schemars(description = "Search query string")]
    pub q: Option<String>,
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: Option<String>,
    #[schemars(description = "Filter by repository name")]
    pub repo: Option<String>,
    #[schemars(description = "Filter by organization name")]
    pub org: Option<String>,
    #[schemars(description = "Number of results (max 60)")]
    pub limit: Option<usize>,
    #[schemars(description = "Offset for pagination")]
    pub offset: Option<usize>,
}

fn resolve_kind(kind: &str) -> Result<String, String> {
    if let Some(id) = pkg_kind::to_id(kind) {
        Ok(id.to_string())
    } else {
        Err(format!(
            "Unknown kind: '{}'. Valid kinds: {}",
            kind,
            pkg_kind::valid_kinds().join(", ")
        ))
    }
}

pub async fn handle_search_packages(
    server: &ArtifactHubServer,
    params: SearchParams,
) -> Result<Json<SearchResponse>, String> {
    if let Some(limit) = params.limit
        && (limit == 0 || limit > 60)
    {
        return Err("limit must be between 1 and 60".to_string());
    }

    let mut query_params: Vec<(String, String)> = vec![];

    if let Some(q) = &params.q {
        query_params.push(("q".to_string(), q.clone()));
    }
    if let Some(kind) = &params.kind {
        let id = resolve_kind(kind)?;
        query_params.push(("kind".to_string(), id));
    }
    if let Some(repo) = &params.repo {
        query_params.push(("repo".to_string(), repo.clone()));
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
        .get_json("/packages/search", &query_params)
        .await?;
    let response: SearchResponse =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ArtifactHubClient;
    use crate::tools::ALL_TOOL_NAMES;
    use std::collections::HashSet;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_server(base_url: &str) -> ArtifactHubServer {
        ArtifactHubServer {
            client: ArtifactHubClient {
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            },
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
                kind: Some("helm".to_string()),
                repo: None,
                org: None,
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
    async fn test_search_packages_invalid_kind() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                kind: Some("invalid-kind".to_string()),
                repo: None,
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
        assert!(err.contains("Valid kinds"));
    }

    #[tokio::test]
    async fn test_search_packages_limit_too_high() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                kind: None,
                repo: None,
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

    #[tokio::test]
    async fn test_search_packages_limit_zero() {
        let server = test_server("http://localhost:12345");
        let result = handle_search_packages(
            &server,
            SearchParams {
                q: None,
                kind: None,
                repo: None,
                org: None,
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
