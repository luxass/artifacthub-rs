use crate::api::packages::{PackageReference, PackagesHandler};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::PackageReadme;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageReadmeResponse {
    readme: Option<String>,
    version: Option<String>,
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
    package: PackageReference,
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
            package: PackageReference::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<PackageReadme> {
        let response: PackageReadmeResponse = self
            .client
            .get_json(&self.package.versioned_path(self.version.as_deref()), &[])
            .await?;
        // Same strictness as get.rs: a requested version must match, even if
        // the server omits `version` (stripped null).
        if self.version.is_some() {
            PackageReference::ensure_version(
                self.version.as_deref(),
                response.version.as_deref().unwrap_or_default(),
            )?;
        }
        let readme = response
            .readme
            .ok_or_else(|| ArtifactHubError::missing_field("readme", "this package"))?;
        Ok(PackageReadme { readme })
    }
}
