use crate::api::packages::{PackagesHandler, package_version_url};
use crate::error::{ArtifactHubError, Result};
use crate::models::{ChartTemplates, ValuesSchemaDocument};

impl<'client> PackagesHandler<'client> {
    pub async fn values(self, package_id: &str, version: &str) -> Result<String> {
        let path = package_version_url(package_id, version, "/values");
        self.client.get(&path, &[]).await
    }

    pub async fn values_schema(
        self,
        package_id: &str,
        version: &str,
    ) -> Result<Option<ValuesSchemaDocument>> {
        let path = package_version_url(package_id, version, "/values-schema");
        let body = self.client.get(&path, &[]).await?;
        if body.trim().is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&body)
            .map(Some)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }

    pub async fn templates(self, package_id: &str, version: &str) -> Result<ChartTemplates> {
        let path = package_version_url(package_id, version, "/templates");
        let json = self.client.get_json(&path, &[]).await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }
}
