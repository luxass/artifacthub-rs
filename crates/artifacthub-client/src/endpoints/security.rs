use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::SecurityReport;

/// Security report endpoints.
///
/// Access via `client.security.*`.
#[derive(Clone)]
pub struct Security {
    inner: Arc<ClientInner>,
}

impl Security {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Get detailed security report with CVEs for a package.
    pub async fn report(&self, params: &GetParams) -> Result<Option<SecurityReport>, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;
        let package_id = json["package_id"]
            .as_str()
            .ok_or("No package_id found for this package")?;
        let version = json["version"]
            .as_str()
            .ok_or("No version found for this package")?;

        crate::endpoints::Packages::new(self.inner.clone())
            .security_report(package_id, version)
            .await
    }
}

/// Parameters for security endpoints.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArtifactHubClient;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn report_resolves_package_before_package_id_endpoint() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .and(wiremock::matchers::query_param("version", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "version": "1.2.3"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.2.3/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "summary": {
                    "critical": 0,
                    "high": 1,
                    "medium": 0,
                    "low": 0,
                    "unknown": 0
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let report = client
            .security
            .report(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.3".to_string()),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(report.summary.unwrap().high, Some(1));
    }
}
