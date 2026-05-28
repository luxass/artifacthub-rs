use artifacthub_client::endpoints::ChangelogParams;
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
    #[schemars(description = "Target version (defaults to latest)")]
    pub to: Option<String>,
    #[schemars(description = "Source version")]
    pub from: Option<String>,
}

pub async fn handle_get_package_changelog(
    server: &ArtifactHubServer,
    params: GetChangelogParams,
) -> Result<Json<Changelog>, String> {
    let changelog = server
        .client
        .packages
        .changelog(&ChangelogParams {
            kind: params.kind,
            repo: params.repo,
            name: params.name,
            from: params.from,
            to: params.to,
        })
        .await?;

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
                    "changes": ["Added new feature", "Fixed bug"],
                    "prerelease": false
                },
                {
                    "version": "1.2.0",
                    "ts": 1699000000,
                    "changes": ["Initial release"]
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
    }
}
