use artifacthub_client::models::PackageValues;
use artifacthub_client::params::HelmGetParams;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetPackageValuesParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Specific version (defaults to latest)")]
    pub version: Option<String>,
}

pub async fn handle_get_package_values(
    server: &ArtifactHubServer,
    params: GetPackageValuesParams,
) -> Result<Json<PackageValues>, String> {
    let values = server
        .client
        .helm
        .values(&HelmGetParams {
            kind: params.kind,
            repo: params.repo,
            name: params.name,
            version: params.version,
        })
        .await?;

    Ok(Json(values))
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
    async fn test_get_package_values_returns_values_yaml() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "name": "nginx",
                "version": "1.0.0"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/values"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("replicaCount: 3\nimage:\n  repository: nginx\n"),
            )
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_values(
            &server,
            GetPackageValuesParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.package, "nginx");
        assert_eq!(result.0.version, "1.0.0");
        assert!(result.0.values.contains("replicaCount: 3"));
    }

    #[tokio::test]
    async fn test_get_package_values_no_package_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/falco/falcosecurity/falco"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "falco",
                "version": "1.0.0"
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_values(
            &server,
            GetPackageValuesParams {
                kind: "falco".to_string(),
                repo: "falcosecurity".to_string(),
                name: "falco".to_string(),
                version: None,
            },
        )
        .await;

        assert!(result.is_err());
        let Err(err) = result else {
            panic!("expected error")
        };
        assert!(err.contains("No package_id"));
    }
}
