use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::StarStats;

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
        let path = package_url(&params.kind, &params.repo, &params.name, "/stats");
        let json = self.inner.get_json(&path, &[]).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }
}

/// Parameters for stats endpoints.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
}
