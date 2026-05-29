use crate::api::packages::{PackageRef, PackagesHandler, version_suffix};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::PackageSummary;

impl<'client> PackagesHandler<'client> {
    pub fn version(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> GetPackageVersionBuilder<'client> {
        GetPackageVersionBuilder::new(self.client, kind, repo, name, version)
    }
}

pub struct GetPackageVersionBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageRef,
    version: String,
}

impl<'client> GetPackageVersionBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageRef::new(kind, repo, name),
            version: version.into(),
        }
    }

    pub async fn send(self) -> Result<PackageSummary> {
        let path = self.package.path(&version_suffix(&self.version));
        let json = self.client.get_json(&path, &[]).await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }
}
