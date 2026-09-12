use artifacthub_client::models::Changelog;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetChangelogParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Only include versions <= this (client-side filter, semver)")]
    pub to: Option<String>,
    #[schemars(description = "Only include versions > this (client-side filter, semver)")]
    pub from: Option<String>,
}

pub async fn handle_get_package_changelog(
    server: &ArtifactHubServer,
    params: GetChangelogParams,
) -> Result<Json<Changelog>, String> {
    let mut changelog_request =
        server
            .client
            .packages()
            .changelog(params.kind, params.repo, params.name);

    if let Some(from) = params.from {
        changelog_request = changelog_request.from(from);
    }

    if let Some(to) = params.to {
        changelog_request = changelog_request.to(to);
    }

    let changelog = changelog_request.send().await?;

    Ok(Json(changelog))
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
    async fn test_get_package_changelog_returns_entries() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "version": "1.3.0"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/changelog"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "version": "1.3.0",
                    "ts": 1700000000,
                    "changes": [
                        {"kind": "added", "description": "Added new feature"},
                        {"kind": "fixed", "description": "Fixed bug"}
                    ],
                    "contains_security_updates": false,
                    "prerelease": false
                },
                {
                    "version": "1.2.0",
                    "ts": 1699000000,
                    "changes": [{"description": "Initial release"}],
                    "contains_security_updates": false,
                    "prerelease": false
                }
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_changelog(
            &server,
            GetChangelogParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                to: None,
                from: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.entries.len(), 2);
        assert_eq!(result.0.entries[0].version, "1.3.0");
        assert_eq!(result.0.entries[0].changes.len(), 2);
        assert_eq!(
            result.0.entries[0].changes[0].description,
            "Added new feature"
        );
    }

    #[tokio::test]
    async fn test_get_package_changelog_filters_range_client_side() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "version": "1.3.0"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/changelog"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"version": "1.3.0", "ts": 1700000000, "changes": [], "contains_security_updates": false, "prerelease": false},
                {"version": "1.2.0", "ts": 1699000000, "changes": [], "contains_security_updates": false, "prerelease": false},
                {"version": "1.1.0", "ts": 1698000000, "changes": [], "contains_security_updates": false, "prerelease": false}
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_changelog(
            &server,
            GetChangelogParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                to: Some("1.2.0".to_string()),
                from: Some("1.1.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.entries.len(), 1);
        assert_eq!(result.0.entries[0].version, "1.2.0");
    }
}
