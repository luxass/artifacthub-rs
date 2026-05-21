use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::package_url;
use crate::kind::KIND_DESCRIPTION;
use crate::tools::ArtifactHubServer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StarHistoryEntry {
    pub total: i32,
    pub dates: Vec<StarDateEntry>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StarDateEntry {
    pub date: String,
    pub stars: i32,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetStarStatsParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
}

pub async fn handle_get_package_star_stats(
    server: &ArtifactHubServer,
    params: GetStarStatsParams,
) -> Result<Json<Vec<StarHistoryEntry>>, String> {
    let url = server
        .client
        .build_url(&package_url(&params.kind, &params.repo, &params.name, "/stars"), &[]);
    let json = server.client.get_json(&url).await?;

    let stars: Vec<StarHistoryEntry> =
        serde_json::from_value(json).map_err(|e| format!("Failed to parse star stats: {}", e))?;

    Ok(Json(stars))
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
    async fn test_get_package_star_stats_returns_history() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/stars"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "total": 150,
                    "dates": [
                        { "date": "2024-01-01", "stars": 100 },
                        { "date": "2024-02-01", "stars": 125 },
                        { "date": "2024-03-01", "stars": 150 }
                    ]
                }
            ])))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_star_stats(
            &server,
            GetStarStatsParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].total, 150);
        assert_eq!(result.0[0].dates.len(), 3);
    }
}
