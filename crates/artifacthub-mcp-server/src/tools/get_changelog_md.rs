use artifacthub_client::endpoints::ChangelogParams;
use artifacthub_client::models::ChangelogMarkdown;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetChangelogMdParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Source version (optional)")]
    pub from: Option<String>,
    #[schemars(description = "Target version (defaults to latest)")]
    pub to: Option<String>,
}

pub async fn handle_get_changelog_md(
    server: &ArtifactHubServer,
    params: GetChangelogMdParams,
) -> Result<Json<ChangelogMarkdown>, String> {
    let changelog = server
        .client
        .packages
        .changelog_markdown(&ChangelogParams {
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

    #[tokio::test]
    async fn test_get_changelog_md_returns_markdown() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/changelog.md"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("# Changelog\n\n## 1.0.0\n- Initial release"),
            )
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_changelog_md(
            &server,
            GetChangelogMdParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                from: None,
                to: None,
            },
        )
        .await
        .unwrap();

        assert!(result.0.changelog.contains("Changelog"));
        assert!(result.0.changelog.contains("1.0.0"));
    }

    #[tokio::test]
    async fn test_get_changelog_md_with_version_range() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/changelog.md"))
            .and(query_param("from", "1.0.0"))
            .and(query_param("to", "2.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_string("# Changelog"))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_changelog_md(
            &server,
            GetChangelogMdParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                from: Some("1.0.0".to_string()),
                to: Some("2.0.0".to_string()),
            },
        )
        .await
        .unwrap();

        assert!(!result.0.changelog.is_empty());
    }
}
