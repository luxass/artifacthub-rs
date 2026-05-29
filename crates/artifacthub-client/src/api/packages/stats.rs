use crate::api::packages::{PackagesHandler, package_id_url};
use crate::error::{ArtifactHubError, Result};
use crate::models::{PackageCounts, StarStats};

impl<'client> PackagesHandler<'client> {
    pub async fn stats(self) -> Result<PackageCounts> {
        let json = self.client.get_json("/packages/stats", &[]).await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }

    pub async fn stars(self, package_id: &str) -> Result<StarStats> {
        let path = package_id_url(package_id, "/stars");
        let json = self.client.get_json(&path, &[]).await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse star stats", e))
    }
}
