use crate::api::packages::{PackageRef, PackagesHandler, optional_query_params};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::PackageReadme;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageReadmeResponse {
    readme: Option<String>,
}

impl<'client> PackagesHandler<'client> {
    pub fn readme(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> ReadmeBuilder<'client> {
        ReadmeBuilder::new(self.client, kind, repo, name)
    }
}

pub struct ReadmeBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageRef,
    version: Option<String>,
}

impl<'client> ReadmeBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageRef::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<PackageReadme> {
        let path = self.package.path("");
        let query = optional_query_params([("version", self.version.as_deref())]);
        let response: PackageReadmeResponse = self.client.get_json(&path, &query).await?;
        let readme = response
            .readme
            .ok_or_else(|| ArtifactHubError::missing_field("readme", "this package"))?;
        Ok(PackageReadme { readme })
    }
}
