use crate::api::packages::{PackageRef, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::StarStats;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageIdResponse {
    package_id: Option<String>,
}

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
    package: PackageRef,
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
            package: PackageRef::new(kind, repo, name),
        }
    }

    pub async fn send(self) -> Result<StarStats> {
        let package: PackageIdResponse = self.client.get_json(&self.package.path(""), &[]).await?;
        let package_id = package
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;

        self.client.packages().stars(&package_id).await
    }
}
