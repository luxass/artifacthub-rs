use crate::api::packages::PackageReference;
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::{ChartTemplates, PackageValues, ValuesSchema};

#[derive(Clone, Copy)]
pub struct HelmHandler<'client> {
    client: &'client ArtifactHubClient,
}

impl<'client> HelmHandler<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self { client }
    }

    pub fn values(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> HelmValuesBuilder<'client> {
        HelmValuesBuilder::new(self.client, kind, repo, name)
    }

    pub fn values_schema(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> HelmValuesSchemaBuilder<'client> {
        HelmValuesSchemaBuilder::new(self.client, kind, repo, name)
    }

    pub fn templates(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> HelmTemplatesBuilder<'client> {
        HelmTemplatesBuilder::new(self.client, kind, repo, name)
    }
}

pub struct HelmValuesBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    version: Option<String>,
}

impl<'client> HelmValuesBuilder<'client> {
    fn new(
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

    pub async fn send(self) -> Result<PackageValues> {
        let identity = self
            .package
            .resolve_identity(self.client, self.version.as_deref())
            .await?;
        let values = self
            .client
            .packages()
            .values(&identity.package_id, &identity.version)
            .await?;

        Ok(PackageValues {
            package: self.package.name().to_string(),
            version: identity.version,
            values,
        })
    }
}

pub struct HelmValuesSchemaBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    version: Option<String>,
}

impl<'client> HelmValuesSchemaBuilder<'client> {
    fn new(
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

    pub async fn send(self) -> Result<ValuesSchema> {
        let identity = self
            .package
            .resolve_identity(self.client, self.version.as_deref())
            .await?;
        let schema = self
            .client
            .packages()
            .values_schema(&identity.package_id, &identity.version)
            .await?;

        Ok(ValuesSchema { schema })
    }
}

pub struct HelmTemplatesBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    version: Option<String>,
}

impl<'client> HelmTemplatesBuilder<'client> {
    fn new(
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

    pub async fn send(self) -> Result<ChartTemplates> {
        let identity = self
            .package
            .resolve_identity(self.client, self.version.as_deref())
            .await?;
        self.client
            .packages()
            .templates(&identity.package_id, &identity.version)
            .await
    }
}
