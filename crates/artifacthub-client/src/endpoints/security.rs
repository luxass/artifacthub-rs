use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::SecurityReport;

/// Security report endpoints.
///
/// Access via `client.security.*`.
#[derive(Clone)]
pub struct Security {
    inner: Arc<ClientInner>,
}

impl Security {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Get detailed security report with CVEs for a package.
    pub async fn report(&self, params: &GetParams) -> Result<SecurityReport, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "/security-report");
        let json = self.inner.get_json(&path, &[]).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }
}

/// Parameters for security endpoints.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
}
