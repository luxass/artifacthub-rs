use crate::api::packages::{PackagesHandler, package_id_url};
use crate::error::Result;
use crate::models::{PackageCounts, StarStats};

impl<'client> PackagesHandler<'client> {
    pub async fn stats(self) -> Result<PackageCounts> {
        self.client.get_json("/packages/stats", &[]).await
    }

    pub async fn stars(self, package_id: &str) -> Result<StarStats> {
        let path = package_id_url(package_id, "/stars");
        self.client
            .get_json_with_context(&path, &[], "Failed to parse star stats")
            .await
    }
}
