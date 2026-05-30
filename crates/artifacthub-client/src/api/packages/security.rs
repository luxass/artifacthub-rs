use crate::api::packages::{PackagesHandler, package_version_url};
use crate::error::Result;
use crate::models::SecurityReport;

impl<'client> PackagesHandler<'client> {
    pub async fn security_report(
        self,
        package_id: &str,
        version: &str,
    ) -> Result<Option<SecurityReport>> {
        let path = package_version_url(package_id, version, "/security-report");
        self.client.get_optional_json(&path, &[]).await
    }
}
