use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::Serialize;

use crate::tools::ArtifactHubServer;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetTemplatesParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (from get_package; required by Artifact Hub API)")]
    pub version: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TemplateList {
    pub templates: Vec<TemplateListItem>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TemplateListItem {
    pub name: String,
}

pub async fn handle_get_templates(
    server: &ArtifactHubServer,
    params: GetTemplatesParams,
) -> Result<Json<TemplateList>, String> {
    let templates = server
        .client
        .packages()
        .templates(&params.package_id, &params.version)
        .await?;

    let templates = templates
        .templates
        .into_iter()
        .filter_map(|template| template.name.map(|name| TemplateListItem { name }))
        .collect();

    Ok(Json(TemplateList { templates }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ALL_TOOL_NAMES;
    use artifacthub_client::client::ArtifactHubClient;
    use std::collections::HashSet;
    use wiremock::matchers::{method, path};
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
    async fn test_get_templates_returns_templates() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "deployment",
                        "data": "YXBpVmVyc2lvbjogYXBwcy92MQpraW5kOiBEZXBsb3ltZW50Cg=="
                    },
                    {
                        "name": "service"
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
                version: "1.0.0".to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.templates.len(), 2);
        assert_eq!(result.0.templates[0].name, "deployment");
        assert_eq!(result.0.templates[1].name, "service");
    }
}
