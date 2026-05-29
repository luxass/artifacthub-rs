use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetTemplateDataParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (from get_package; required by Artifact Hub API)")]
    pub version: String,
    #[schemars(
        description = "Exact template file name from get_package_templates, for example templates/deployment.yaml"
    )]
    pub name: String,
}

pub async fn handle_get_template_data(
    server: &ArtifactHubServer,
    params: GetTemplateDataParams,
) -> Result<String, String> {
    let templates = server
        .client
        .packages()
        .templates(&params.package_id, &params.version)
        .await?;

    let template = templates
        .templates
        .into_iter()
        .find(|template| template.name.as_deref() == Some(params.name.as_str()))
        .ok_or_else(|| format!("Template '{}' not found", params.name))?;

    template
        .data
        .ok_or_else(|| format!("Template '{}' has no data", params.name))
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
    async fn test_get_template_data_returns_decoded_data() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "templates/deployment.yaml",
                        "data": "YXBpVmVyc2lvbjogYXBwcy92MQpraW5kOiBEZXBsb3ltZW50Cg=="
                    },
                    {
                        "name": "templates/service.yaml",
                        "data": "YXBpVmVyc2lvbjogdjEKa2luZDogU2VydmljZQo="
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_template_data(
            &server,
            GetTemplateDataParams {
                package_id: "pkg-123".to_string(),
                version: "1.0.0".to_string(),
                name: "templates/service.yaml".to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(result, "apiVersion: v1\nkind: Service\n");
    }

    #[tokio::test]
    async fn test_get_template_data_returns_error_when_missing() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "templates/service.yaml",
                        "data": "YXBpVmVyc2lvbjogdjEKa2luZDogU2VydmljZQo="
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let err = handle_get_template_data(
            &server,
            GetTemplateDataParams {
                package_id: "pkg-123".to_string(),
                version: "1.0.0".to_string(),
                name: "templates/deployment.yaml".to_string(),
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err, "Template 'templates/deployment.yaml' not found");
    }
}
