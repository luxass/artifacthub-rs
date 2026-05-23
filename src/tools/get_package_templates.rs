use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChartTemplates {
    pub templates: Vec<ChartTemplate>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChartTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetTemplatesParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (defaults to latest)")]
    pub version: Option<String>,
}

pub async fn handle_get_templates(
    server: &ArtifactHubServer,
    params: GetTemplatesParams,
) -> Result<Json<ChartTemplates>, String> {
    let mut path = format!("/packages/{}/{}", params.package_id, params.version.as_deref().unwrap_or(""));
    path.push_str("/templates");

    let url = server.client.build_url(&path, &[]);
    let json = server.client.get_json(&url).await?;
    let templates: ChartTemplates =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(templates))
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
            enabled_tools: ALL_TOOL_NAMES.iter().map(|s| s.to_string()).collect::<HashSet<_>>(),
        }
    }

    #[tokio::test]
    async fn test_get_templates_returns_templates() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "deployment",
                        "kind": "Deployment",
                        "api_version": "apps/v1"
                    },
                    {
                        "name": "service",
                        "kind": "Service",
                        "api_version": "v1"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_templates(
            &server,
            GetTemplatesParams {
                package_id: "pkg-123".to_string(),
                version: Some("1.0.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.templates.len(), 2);
        assert_eq!(result.0.templates[0].kind.as_deref(), Some("Deployment"));
        assert_eq!(result.0.templates[1].kind.as_deref(), Some("Service"));
    }

    #[tokio::test]
    async fn test_get_templates_no_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123//templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": []
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_templates(
            &server,
            GetTemplatesParams {
                package_id: "pkg-123".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert!(result.0.templates.is_empty());
    }
}
