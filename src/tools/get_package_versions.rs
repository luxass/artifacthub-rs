use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::package_url;
use crate::kind::KIND_DESCRIPTION;
use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageVersion {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub contains_security_updates: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub prerelease: bool,
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageVersions {
    pub versions: Vec<PackageVersion>,
    pub count: usize,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetPackageVersionsParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Max versions to return (defaults to all)")]
    pub limit: Option<usize>,
}

pub async fn handle_get_package_versions(
    server: &ArtifactHubServer,
    params: GetPackageVersionsParams,
) -> Result<Json<PackageVersions>, String> {
    let url = server.client.build_url(
        &package_url(&params.kind, &params.repo, &params.name, ""),
        &[],
    );
    let json = server.client.get_json(&url).await?;

    let mut versions: Vec<PackageVersion> =
        serde_json::from_value(json["available_versions"].clone())
            .map_err(|e| format!("Failed to parse versions: {}", e))?;

    let count = versions.len();
    if let Some(limit) = params.limit {
        versions.truncate(limit);
    }

    Ok(Json(PackageVersions { versions, count }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ArtifactHubClient;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_server(base_url: &str) -> ArtifactHubServer {
        ArtifactHubServer {
            client: ArtifactHubClient {
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            },
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
