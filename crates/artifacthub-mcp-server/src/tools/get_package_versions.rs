use artifacthub_client::models::PackageVersions;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetPackageVersionsParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(
        description = "Max versions to return (defaults to all)",
        transform = crate::tools::schema::remove_format
    )]
    pub limit: Option<usize>,
}

pub async fn handle_get_package_versions(
    server: &ArtifactHubServer,
    params: GetPackageVersionsParams,
) -> Result<Json<PackageVersions>, String> {
    let mut package_versions = server
        .client
        .packages()
        .versions(params.kind, params.repo, params.name)
        .send()
        .await?;

    if let Some(limit) = params.limit {
        package_versions.versions.truncate(limit);
    }

    Ok(Json(package_versions))
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

    fn sample_versions_json() -> serde_json::Value {
        serde_json::json!({
            "available_versions": [
                { "version": "1.3.0", "app_version": "1.25.0", "contains_security_updates": false, "prerelease": false, "ts": 1700000000 },
                { "version": "1.2.0", "app_version": "1.24.0", "contains_security_updates": false, "prerelease": false, "ts": 1699000000 },
                { "version": "1.1.0", "app_version": "1.23.0", "contains_security_updates": true, "prerelease": false, "ts": 1698000000 },
                { "version": "1.0.0", "app_version": "1.22.0", "contains_security_updates": false, "prerelease": true, "ts": 1697000000 }
            ]
        })
    }

    #[tokio::test]
    async fn test_get_package_versions_returns_all() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_versions_json()))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_versions(
            &server,
            GetPackageVersionsParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                limit: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.count, 4);
        assert_eq!(result.0.versions.len(), 4);
        assert_eq!(result.0.versions[0].version, "1.3.0");
    }

    #[tokio::test]
    async fn test_get_package_versions_with_limit() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_versions_json()))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_versions(
            &server,
            GetPackageVersionsParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                limit: Some(2),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.count, 4);
        assert_eq!(result.0.versions.len(), 2);
        assert_eq!(result.0.versions[0].version, "1.3.0");
        assert_eq!(result.0.versions[1].version, "1.2.0");
    }
}
