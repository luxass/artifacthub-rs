use crate::api::packages::{PackageReference, PackagesHandler, version_suffix};
use crate::client::ArtifactHubClient;
use crate::error::Result;
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
    package: PackageReference,
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
            package: PackageReference::new(kind, repo, name),
            version: version.into(),
        }
    }

    pub async fn send(self) -> Result<PackageSummary> {
        let path = self.package.path(&version_suffix(&self.version));
        self.client.get_json(&path, &[]).await
    }
}
