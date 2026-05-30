use crate::api::packages::{PackagesHandler, package_id_url};
use crate::error::Result;
use crate::models::PackageViews;

impl<'client> PackagesHandler<'client> {
    pub async fn views(self, package_id: &str) -> Result<PackageViews> {
        let path = package_id_url(package_id, "/views");
        self.client.get_json(&path, &[]).await
    }
}
