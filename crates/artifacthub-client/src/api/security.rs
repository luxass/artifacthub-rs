use crate::api::packages::PackageReference;
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::SecurityReport;

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
            package: PackageReference::new(kind, repo, name),
            version: None,
        }
    }
}

pub struct SecurityReportBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    version: Option<String>,
}

impl<'client> SecurityReportBuilder<'client> {
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<Option<SecurityReport>> {
        let identity = self
            .package
            .resolve_identity(self.client, self.version.as_deref())
            .await?;

        self.client
            .packages()
            .security_report(&identity.package_id, &identity.version)
            .await
    }
}
