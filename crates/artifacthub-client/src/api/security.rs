use crate::client::{ArtifactHubClient, package_url};
use crate::error::{ArtifactHubError, Result};
use crate::models::SecurityReport;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageIdentityResponse {
    package_id: Option<String>,
    version: Option<String>,
}

#[derive(Clone, Copy)]
pub struct SecurityHandler<'client> {
    client: &'client ArtifactHubClient,
}

impl<'client> SecurityHandler<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self { client }
    }

    pub fn report(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> SecurityReportBuilder<'client> {
        SecurityReportBuilder {
            client: self.client,
            kind: kind.into(),
            repo: repo.into(),
            name: name.into(),
            version: None,
        }
    }
}

pub struct SecurityReportBuilder<'client> {
    client: &'client ArtifactHubClient,
    kind: String,
    repo: String,
    name: String,
    version: Option<String>,
}

impl<'client> SecurityReportBuilder<'client> {
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<Option<SecurityReport>> {
        let query_params = self
            .version
            .as_deref()
            .map(|version| vec![("version".to_string(), version.to_string())])
            .unwrap_or_default();
        let path = package_url(&self.kind, &self.repo, &self.name, "");
        let response: PackageIdentityResponse = self.client.get_json(&path, &query_params).await?;
        let package_id = response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;
        let version = response
            .version
            .ok_or_else(|| ArtifactHubError::missing_field("version", "this package"))?;

        self.client
            .packages()
            .security_report(&package_id, &version)
            .await
    }
}
