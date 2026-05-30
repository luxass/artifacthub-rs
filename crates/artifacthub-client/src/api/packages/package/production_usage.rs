use crate::api::packages::{PackageRef, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::ProductionUsageOrganization;

impl<'client> PackagesHandler<'client> {
    pub fn production_usage(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> ProductionUsageBuilder<'client> {
        ProductionUsageBuilder::new(self.client, kind, repo, name)
    }
}

pub struct ProductionUsageBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageRef,
}

impl<'client> ProductionUsageBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageRef::new(kind, repo, name),
        }
    }

    pub async fn send(self) -> Result<Vec<ProductionUsageOrganization>> {
        self.client
            .get_json(&self.package.path("/production-usage"), &[])
            .await
    }
}
