use crate::api::packages::{PackageRef, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::{PackageVersion, PackageVersions};
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageVersionsResponse {
    available_versions: Vec<PackageVersion>,
}

impl<'client> PackagesHandler<'client> {
    pub fn versions(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> PackageVersionsBuilder<'client> {
        PackageVersionsBuilder::new(self.client, kind, repo, name)
    }
}

pub struct PackageVersionsBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageRef,
}

impl<'client> PackageVersionsBuilder<'client> {
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

    pub async fn send(self) -> Result<PackageVersions> {
        let response: PackageVersionsResponse = self
            .client
            .get_json_with_context(&self.package.path(""), &[], "Failed to parse versions")
            .await?;
        let versions = response.available_versions;

        Ok(PackageVersions {
            count: versions.len(),
            versions,
        })
    }
}
