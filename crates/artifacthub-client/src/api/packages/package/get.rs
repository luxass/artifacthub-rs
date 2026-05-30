use crate::api::packages::{PackageReference, PackagesHandler, optional_query_params};
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
        let path = self.package.path("");
        let query = optional_query_params([("version", self.version.as_deref())]);
        self.client.get_json(&path, &query).await
    }
}
