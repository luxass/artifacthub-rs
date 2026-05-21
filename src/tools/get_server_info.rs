use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::Serialize;

use crate::tools::ArtifactHubServer;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetServerInfoParams {}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub async fn handle_get_server_info(
    _server: &ArtifactHubServer,
    _params: GetServerInfoParams,
) -> Result<Json<ServerInfo>, String> {
    Ok(Json(ServerInfo {
        name: "artifacthub-mcp".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "MCP server for interacting with Artifact Hub".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ArtifactHubClient;

    #[tokio::test]
    async fn test_server_info_returns_expected_fields() {
        let server = ArtifactHubServer {
            client: ArtifactHubClient::default(),
        };
        let result = handle_get_server_info(&server, GetServerInfoParams {})
            .await
            .unwrap();

        assert_eq!(result.0.name, "artifacthub-mcp");
        assert_eq!(result.0.version, env!("CARGO_PKG_VERSION"));
        assert!(result.0.description.contains("Artifact Hub"));
    }
}
