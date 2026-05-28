use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::{StarHistoryEntry, StarStats};

/// Package statistics endpoints (star history).
///
/// Access via `client.stats.*`.
#[derive(Clone)]
pub struct Stats {
    inner: Arc<ClientInner>,
}

impl Stats {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Get star history for a package.
    pub async fn star_stats(&self, params: &GetParams) -> Result<StarStats, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "/stars");
        let json = self.inner.get_json(&path, &[]).await?;
        let stars: Vec<StarHistoryEntry> =
            serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(StarStats { stars })
    }
}

/// Parameters for stats endpoints.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArtifactHubClient;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn star_stats_uses_stars_endpoint_and_wraps_history() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/stars"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "total": 150,
                    "dates": [
                        { "date": "2024-01-01", "stars": 100 },
                        { "date": "2024-02-01", "stars": 150 }
                    ]
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let stats = client
            .stats
            .star_stats(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(stats.stars.len(), 1);
        assert_eq!(stats.stars[0].total, 150);
        assert_eq!(stats.stars[0].dates.len(), 2);
    }
}
