use serde::Deserialize;

use crate::client::{ArtifactHubClient, package_url};
use crate::error::{ArtifactHubError, Result};

pub(crate) struct PackageReference {
    kind: String,
    repo: String,
    name: String,
}

pub(crate) struct PackageIdentity {
    pub(crate) package_id: String,
    pub(crate) version: String,
}

#[derive(Deserialize)]
struct PackageIdentityResponse {
    package_id: Option<String>,
    version: Option<String>,
}

#[derive(Deserialize)]
struct PackageIdResponse {
    package_id: Option<String>,
}

impl PackageReference {
    pub(crate) fn new(
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            kind: kind.into(),
            repo: repo.into(),
            name: name.into(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self, suffix: &str) -> String {
        package_url(&self.kind, &self.repo, &self.name, suffix)
    }

    pub(crate) async fn resolve_identity(
        &self,
        client: &ArtifactHubClient,
        version: Option<&str>,
    ) -> Result<PackageIdentity> {
        let query_params = version
            .map(|version| vec![("version".to_string(), version.to_string())])
            .unwrap_or_default();
        let response: PackageIdentityResponse =
            client.get_json(&self.path(""), &query_params).await?;

        let package_id = response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;
        let version = response
            .version
            .ok_or_else(|| ArtifactHubError::missing_field("version", "this package"))?;

        Ok(PackageIdentity {
            package_id,
            version,
        })
    }

    pub(crate) async fn resolve_package_id(&self, client: &ArtifactHubClient) -> Result<String> {
        let response: PackageIdResponse = client.get_json(&self.path(""), &[]).await?;

        response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))
    }
}
