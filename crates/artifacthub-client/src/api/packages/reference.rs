use serde::Deserialize;

use crate::client::{ArtifactHubClient, encode_path_segment, package_url};
use crate::error::{ArtifactHubError, Result};
use crate::kind::canonicalize;

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
            kind: canonicalize(&kind.into()).to_string(),
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

    /// Path for fetching a package, pinning an exact version when given.
    ///
    /// Artifact Hub ignores `?version=` and always returns latest, so the
    /// version must be part of the path (`/.../name/{version}`).
    pub(crate) fn versioned_path(&self, version: Option<&str>) -> String {
        match version {
            Some(version) => self.path(&format!("/{}", encode_path_segment(version))),
            None => self.path(""),
        }
    }

    /// Fail instead of silently returning a different version than requested.
    pub(crate) fn ensure_version(requested: Option<&str>, actual: &str) -> Result<()> {
        if let Some(requested) = requested
            && requested != actual
        {
            return Err(ArtifactHubError::version_mismatch(requested, actual));
        }
        Ok(())
    }

    pub(crate) async fn resolve_identity(
        &self,
        client: &ArtifactHubClient,
        version: Option<&str>,
    ) -> Result<PackageIdentity> {
        let response: PackageIdentityResponse =
            client.get_json(&self.versioned_path(version), &[]).await?;

        let package_id = response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;
        let resolved_version = response
            .version
            .ok_or_else(|| ArtifactHubError::missing_field("version", "this package"))?;

        Self::ensure_version(version, &resolved_version)?;

        Ok(PackageIdentity {
            package_id,
            version: resolved_version,
        })
    }

    pub(crate) async fn resolve_package_id(&self, client: &ArtifactHubClient) -> Result<String> {
        let response: PackageIdResponse = client.get_json(&self.path(""), &[]).await?;

        response
            .package_id
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))
    }
}
