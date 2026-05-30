use crate::client::{ArtifactHubClient, package_url};
use crate::error::{ArtifactHubError, Result};
use crate::models::{ChartTemplates, PackageValues, ValuesSchema};
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageIdentityResponse {
    package_id: Option<String>,
    version: Option<String>,
}

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
    package: HelmPackageRef,
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
            package: HelmPackageRef::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<PackageValues> {
        let (package_id, version) = self
            .package
            .resolve(self.client, self.version.as_deref())
            .await?;
        let values = self.client.packages().values(&package_id, &version).await?;

        Ok(PackageValues {
            package: self.package.name,
            version,
            values,
        })
    }
}

pub struct HelmValuesSchemaBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: HelmPackageRef,
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
            package: HelmPackageRef::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<ValuesSchema> {
        let (package_id, version) = self
            .package
            .resolve(self.client, self.version.as_deref())
            .await?;
        let schema = self
            .client
            .packages()
            .values_schema(&package_id, &version)
            .await?;

        Ok(ValuesSchema { schema })
    }
}

pub struct HelmTemplatesBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: HelmPackageRef,
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
            package: HelmPackageRef::new(kind, repo, name),
            version: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub async fn send(self) -> Result<ChartTemplates> {
        let (package_id, version) = self
            .package
            .resolve(self.client, self.version.as_deref())
            .await?;
        self.client
            .packages()
            .templates(&package_id, &version)
            .await
    }
}

struct HelmPackageRef {
    kind: String,
    repo: String,
    name: String,
}

impl HelmPackageRef {
    fn new(kind: impl Into<String>, repo: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            repo: repo.into(),
            name: name.into(),
        }
    }

    async fn resolve(
        &self,
        client: &ArtifactHubClient,
        version: Option<&str>,
    ) -> Result<(String, String)> {
        let query_params = version
            .map(|version| vec![("version".to_string(), version.to_string())])
            .unwrap_or_default();
        let path = package_url(&self.kind, &self.repo, &self.name, "");
        let response: PackageIdentityResponse = client.get_json(&path, &query_params).await?;

        let package_id = response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;
        let version = response
            .version
            .ok_or_else(|| ArtifactHubError::missing_field("version", "this package"))?;

        Ok((package_id, version))
    }
}
