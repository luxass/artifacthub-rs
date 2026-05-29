use crate::api::packages::{PackageRef, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::{PackageVersion, PackageVersions};

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
        let json = self.client.get_json(&self.package.path(""), &[]).await?;
        let versions: Vec<PackageVersion> =
            serde_json::from_value(json["available_versions"].clone())
                .map_err(|e| ArtifactHubError::json("Failed to parse versions", e))?;

        Ok(PackageVersions {
            count: versions.len(),
            versions,
        })
    }
}
