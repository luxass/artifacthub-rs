use crate::api::packages::{PackagesHandler, package_version_url};
use crate::error::Result;
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
        self.client.get_optional_json(&path, &[]).await
    }

    pub async fn templates(self, package_id: &str, version: &str) -> Result<ChartTemplates> {
        let path = package_version_url(package_id, version, "/templates");
        self.client.get_json(&path, &[]).await
    }
}
