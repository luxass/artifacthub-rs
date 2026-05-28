use artifacthub_client::models::ValuesSchema;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetValuesSchemaParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (from get_package; required by Artifact Hub API)")]
    pub version: String,
}

pub async fn handle_get_values_schema(
    server: &ArtifactHubServer,
    params: GetValuesSchemaParams,
) -> Result<Json<ValuesSchema>, String> {
    let path = format!(
        "/packages/{}/{}/values-schema",
        params.package_id, params.version
    );

    let json = server.client.get_json(&path, &[]).await?;
    let schema: ValuesSchema =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(schema))
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
    async fn test_get_values_schema_returns_schema() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/values-schema"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "schema": {
                    "type": "object",
                    "properties": {
                        "replicaCount": {
                            "type": "integer",
                            "default": 1
                        },
                        "image": {
                            "type": "object",
                            "properties": {
                                "repository": { "type": "string" },
                                "tag": { "type": "string" }
                            }
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_values_schema(
            &server,
            GetValuesSchemaParams {
                package_id: "pkg-123".to_string(),
                version: "1.0.0".to_string(),
            },
        )
        .await
        .unwrap();

        assert!(result.0.schema.is_some());
        let schema = result.0.schema.unwrap();
        assert_eq!(schema["type"].as_str(), Some("object"));
    }
}
