use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::package_url;
use crate::kind::KIND_DESCRIPTION;
use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageSummary {
    pub package_id: String,
    pub name: String,
    pub normalized_name: String,
    pub version: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub prerelease: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub signed: bool,
    pub keywords: Vec<String>,
    pub ts: i64,
    pub repository: RepositoryInfo,
    pub stats: PackageStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_images: Option<Vec<ContainerImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_summary: Option<SecurityReportSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_created_at: Option<i64>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub contains_security_updates: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RepositoryInfo {
    pub name: String,
    pub display_name: String,
    pub url: String,
    pub kind: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub verified_publisher: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub official: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub cncf: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageStats {
    pub subscriptions: i32,
    pub webhooks: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContainerImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub image: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub whitelisted: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SecurityReportSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
}

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
    let mut query_params = vec![];
    if let Some(ref version) = params.version {
        query_params.push(("version".to_string(), version.clone()));
    }

    let url = server
        .client
        .build_url(&package_url(&params.kind, &params.repo, &params.name, ""), &query_params);
    let json = server.client.get_json(&url).await?;
    let summary: PackageSummary =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ArtifactHubClient;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_server(base_url: &str) -> ArtifactHubServer {
        ArtifactHubServer {
            client: ArtifactHubClient {
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            },
        }
    }

    fn sample_package_json() -> serde_json::Value {
        serde_json::json!({
            "package_id": "pkg-123",
            "name": "nginx",
            "normalized_name": "nginx",
            "version": "15.0.0",
            "description": "A Helm chart for nginx",
            "deprecated": false,
            "prerelease": false,
            "signed": true,
            "keywords": ["nginx", "http", "web"],
            "ts": 1700000000,
            "repository": {
                "name": "bitnami",
                "display_name": "Bitnami",
                "url": "https://charts.bitnami.com/bitnami",
                "kind": 0,
                "verified_publisher": true,
                "official": true,
                "cncf": false
            },
            "stats": {
                "subscriptions": 100,
                "webhooks": 5
            },
            "links": [],
            "contains_security_updates": false
        })
    }

    #[tokio::test]
    async fn test_get_package_returns_summary() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_package_json()))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
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
        assert_eq!(result.0.version, "15.0.0");
        assert_eq!(result.0.repository.name, "bitnami");
    }

    #[tokio::test]
    async fn test_get_package_with_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .and(query_param("version", "14.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_package_json()))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package(
            &server,
            GetPackageParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("14.0.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.name, "nginx");
    }
}
