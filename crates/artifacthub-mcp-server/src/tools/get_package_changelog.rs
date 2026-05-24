use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::ArtifactHubServer;
use artifacthub_client::client::package_url;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChangelogEntry {
    pub version: String,
    pub ts: i64,
    pub changes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Changelog {
    pub entries: Vec<ChangelogEntry>,
}

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
    let mut query_params: Vec<(String, String)> = vec![];
    if let Some(ref to) = params.to {
        query_params.push(("to".to_string(), to.clone()));
    }
    if let Some(ref from) = params.from {
        query_params.push(("from".to_string(), from.clone()));
    }

    let path = package_url(&params.kind, &params.repo, &params.name, "/changelog");
    let json = server.client.get_json(&path, &query_params).await?;

    let entries: Vec<ChangelogEntry> =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse changelog: {}", e))?;

    Ok(Json(Changelog { entries }))
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
            client: ArtifactHubClient {
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            },
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
            .and(path("/packages/helm/bitnami/nginx/changelog"))
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
