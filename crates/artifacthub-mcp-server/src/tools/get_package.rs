use artifacthub_client::models::PackageSummary;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetPackageParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Specific version (defaults to latest)")]
    pub version: Option<String>,
}

pub async fn handle_get_package(
    server: &ArtifactHubServer,
    params: GetPackageParams,
) -> Result<Json<PackageSummary>, String> {
    let mut package = server
        .client
        .packages()
        .get(params.kind, params.repo, params.name);

    if let Some(version) = params.version {
        package = package.version(version);
    }

    let summary = package.send().await?;

    Ok(Json(summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ALL_TOOL_NAMES;
    use artifacthub_client::client::ArtifactHubClient;
    use artifacthub_server_mock::{HubMockServer, package_snapshot};
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
    async fn test_get_package_returns_summary() {
        let hub = HubMockServer::start().await;
        let server = test_server(&hub.uri());
        let result = handle_get_package(
            &server,
            GetPackageParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.name, "nginx");
        assert_eq!(result.0.version, "1.3.0");
        assert_eq!(result.0.repository.name, "bitnami");
    }

    #[tokio::test]
    async fn test_get_package_with_version() {
        let hub = HubMockServer::start().await;
        let server = test_server(&hub.uri());
        let result = handle_get_package(
            &server,
            GetPackageParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.name, "nginx");
        assert_eq!(result.0.version, "1.2.0");
    }

    #[tokio::test]
    async fn test_get_package_with_version_mismatch_errors() {
        let mock_server = MockServer::start().await;

        // Simulates a proxy returning latest instead of the requested version.
        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/1.2.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(package_snapshot("1.3.0")))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package(
            &server,
            GetPackageParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.0".to_string()),
            },
        )
        .await;

        let Err(err) = result else {
            panic!("expected version mismatch error");
        };
        assert!(err.contains("1.2.0"));
    }

    #[tokio::test]
    async fn test_get_package_defaults_missing_keywords() {
        let mock_server = MockServer::start().await;
        let mut body = package_snapshot("1.3.0");
        body.as_object_mut().unwrap().remove("keywords");

        Mock::given(method("GET"))
            .and(path("/packages/helm/kvalitetsit/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package(
            &server,
            GetPackageParams {
                kind: "helm".to_string(),
                repo: "kvalitetsit".to_string(),
                name: "templates".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert!(result.0.keywords.is_empty());
    }
}
