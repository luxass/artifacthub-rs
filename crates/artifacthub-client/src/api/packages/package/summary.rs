use crate::api::packages::{PackageReference, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::SearchResult;

impl<'client> PackagesHandler<'client> {
    pub fn summary(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> PackageSummaryBuilder<'client> {
        PackageSummaryBuilder::new(self.client, kind, repo, name)
    }
}

pub struct PackageSummaryBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
}

impl<'client> PackageSummaryBuilder<'client> {
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

    pub async fn send(self) -> Result<SearchResult> {
        self.client
            .get_json(&self.package.path("/summary"), &[])
            .await
    }
}
