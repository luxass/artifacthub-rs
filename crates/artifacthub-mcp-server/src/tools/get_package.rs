use artifacthub_client::models::PackageSummary;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::endpoints::PackageGetParams;
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
    let summary = server
        .client
        .packages
        .get_with(&PackageGetParams {
            kind: params.kind,
            repo: params.repo,
            name: params.name,
            version: params.version,
        })
        .await?;

    Ok(Json(summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ALL_TOOL_NAMES;
    use artifacthub_client::client::ArtifactHubClient;
    use std::collections::HashSet;
    use wiremock::matchers::{method, path, query_param};
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

    #[tokio::test]
    async fn test_get_package_defaults_missing_keywords() {
        let mock_server = MockServer::start().await;
        let mut body = sample_package_json();
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
