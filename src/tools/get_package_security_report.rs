use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SecurityReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SecuritySummary>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Vulnerability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SecuritySummary {
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
pub struct GetSecurityReportParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (defaults to latest)")]
    pub version: Option<String>,
}

pub async fn handle_get_security_report(
    server: &ArtifactHubServer,
    params: GetSecurityReportParams,
) -> Result<Json<SecurityReport>, String> {
    let mut path = format!("/packages/{}/{}", params.package_id, params.version.as_deref().unwrap_or(""));
    path.push_str("/security-report");

    let url = server.client.build_url(&path, &[]);
    let json = server.client.get_json(&url).await?;
    let report: SecurityReport =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(Json(report))
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

    #[tokio::test]
    async fn test_get_security_report_returns_report() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "summary": {
                    "critical": 1,
                    "high": 2,
                    "medium": 0,
                    "low": 3,
                    "unknown": 0
                },
                "critical_vulnerabilities": [
                    {
                        "vulnerability_id": "CVE-2024-1234",
                        "package_name": "openssl",
                        "package_version": "1.1.1",
                        "severity": "critical",
                        "fixed_version": "1.1.2",
                        "title": "Critical vulnerability in OpenSSL"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_security_report(
            &server,
            GetSecurityReportParams {
                package_id: "pkg-123".to_string(),
                version: Some("1.0.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert!(result.0.summary.is_some());
        let summary = result.0.summary.unwrap();
        assert_eq!(summary.critical, Some(1));
        assert!(result.0.critical_vulnerabilities.is_some());
        let vulns = result.0.critical_vulnerabilities.unwrap();
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].vulnerability_id.as_deref(), Some("CVE-2024-1234"));
    }

    #[tokio::test]
    async fn test_get_security_report_no_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123//security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "summary": {
                    "critical": 0,
                    "high": 0,
                    "medium": 0,
                    "low": 0,
                    "unknown": 0
                }
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_security_report(
            &server,
            GetSecurityReportParams {
                package_id: "pkg-123".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert!(result.0.summary.is_some());
    }
}
