use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;

use crate::tools::ArtifactHubServer;
use artifacthub_client::{kind::KIND_DESCRIPTION, models::ChangelogMarkdown};

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetChangelogMdParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
}

pub async fn handle_get_changelog_md(
    server: &ArtifactHubServer,
    params: GetChangelogMdParams,
) -> Result<Json<ChangelogMarkdown>, String> {
    let changelog = server
        .client
        .packages()
        .changelog_markdown(params.kind, params.repo, params.name)
        .send()
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
            },
        )
        .await
        .unwrap();

        assert!(result.0.changelog.contains("Changelog"));
        assert!(result.0.changelog.contains("1.0.0"));
    }
}
