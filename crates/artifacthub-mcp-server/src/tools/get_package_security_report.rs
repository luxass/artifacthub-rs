use artifacthub_client::models::SecurityReport;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetSecurityReportParams {
    #[schemars(description = "Package ID (UUID, get this from get_package)")]
    pub package_id: String,
    #[schemars(description = "Package version (from get_package; required by Artifact Hub API)")]
    pub version: String,
}

pub async fn handle_get_security_report(
    server: &ArtifactHubServer,
    params: GetSecurityReportParams,
) -> Result<Json<SecurityReport>, String> {
    let report = server
        .client
        .packages()
        .security_report(&params.package_id, &params.version)
        .await?
        .unwrap_or_default();

    Ok(Json(report))
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
    async fn test_get_security_report_returns_report() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "quay.io/org/pkg1:1.0.0": {
                    "Results": [
                        {
                            "Target": "quay.io/org/pkg1:1.0.0",
                            "Type": "alpine",
                            "Vulnerabilities": [
                                {
                                    "VulnerabilityID": "CVE-2024-1234",
                                    "PkgName": "openssl",
                                    "InstalledVersion": "1.1.1",
                                    "FixedVersion": "1.1.2",
                                    "Severity": "CRITICAL",
                                    "Title": "Critical vulnerability in OpenSSL"
                                }
                            ]
                        }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_security_report(
            &server,
            GetSecurityReportParams {
                package_id: "pkg-123".to_string(),
                version: "1.0.0".to_string(),
            },
        )
        .await
        .unwrap();

        let entry = result
            .0
            .0
            .get("quay.io/org/pkg1:1.0.0")
            .expect("image entry");
        assert_eq!(entry.results.len(), 1);
        assert_eq!(entry.results[0].vulnerabilities.len(), 1);
        assert_eq!(
            entry.results[0].vulnerabilities[0]
                .vulnerability_id
                .as_deref(),
            Some("CVE-2024-1234")
        );
    }

    #[tokio::test]
    async fn test_get_security_report_empty_body_returns_empty_report() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_security_report(
            &server,
            GetSecurityReportParams {
                package_id: "pkg-123".to_string(),
                version: "1.0.0".to_string(),
            },
        )
        .await
        .unwrap();

        assert!(result.0.0.is_empty());
    }
}
