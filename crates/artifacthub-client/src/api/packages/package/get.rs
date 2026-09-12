use crate::api::packages::{PackageReference, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::PackageSummary;

impl<'client> PackagesHandler<'client> {
    pub fn get(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> GetPackageBuilder<'client> {
        GetPackageBuilder::new(self.client, kind, repo, name)
    }
}

pub struct GetPackageBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    version: Option<String>,
}

impl<'client> GetPackageBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageReference::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<PackageSummary> {
        let summary: PackageSummary = self
            .client
            .get_json(&self.package.versioned_path(self.version.as_deref()), &[])
            .await?;
        PackageReference::ensure_version(self.version.as_deref(), &summary.version)?;
        Ok(summary)
    }
}
