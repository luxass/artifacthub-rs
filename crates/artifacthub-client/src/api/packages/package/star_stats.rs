use crate::api::packages::{PackageReference, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::StarStats;

impl<'client> PackagesHandler<'client> {
    pub fn star_stats(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> PackageStarStatsBuilder<'client> {
        PackageStarStatsBuilder::new(self.client, kind, repo, name)
    }
}

pub struct PackageStarStatsBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
}

impl<'client> PackageStarStatsBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageReference::new(kind, repo, name),
        }
    }

    pub async fn send(self) -> Result<StarStats> {
        let package_id = self.package.resolve_package_id(self.client).await?;

        self.client.packages().stars(&package_id).await
    }
}
