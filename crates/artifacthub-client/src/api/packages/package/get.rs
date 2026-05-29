use serde::Serialize;

use crate::api::packages::{PackageRef, PackagesHandler, optional_query_params};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
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

#[derive(Serialize)]
pub struct GetPackageBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(skip)]
    package: PackageRef,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            package: PackageRef::new(kind, repo, name),
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
        let json = self.client.get_json(&path, &query).await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }
}
