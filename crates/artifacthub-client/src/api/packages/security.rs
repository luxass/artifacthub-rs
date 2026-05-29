use crate::api::packages::{PackagesHandler, package_version_url};
use crate::error::{ArtifactHubError, Result};
use crate::models::SecurityReport;

impl<'client> PackagesHandler<'client> {
    pub async fn security_report(
        self,
        package_id: &str,
        version: &str,
    ) -> Result<Option<SecurityReport>> {
        let path = package_version_url(package_id, version, "/security-report");
        let body = self.client.get(&path, &[]).await?;
        if body.trim().is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&body)
            .map(Some)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }
}
