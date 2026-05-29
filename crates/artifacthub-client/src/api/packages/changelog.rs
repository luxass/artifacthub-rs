use serde::Serialize;

use crate::api::packages::{PackageRef, PackagesHandler, optional_query_params, package_id_url};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::{Changelog, ChangelogEntry, ChangelogMarkdown};

impl<'client> PackagesHandler<'client> {
    pub fn changelog(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> ChangelogBuilder<'client> {
        ChangelogBuilder::new(self.client, kind, repo, name)
    }

    pub fn changelog_by_package_id(
        self,
        package_id: impl Into<String>,
    ) -> ChangelogByPackageIdBuilder<'client> {
        ChangelogByPackageIdBuilder::new(self.client, package_id)
    }

    pub fn changelog_markdown(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> ChangelogMarkdownBuilder<'client> {
        ChangelogMarkdownBuilder::new(self.client, kind, repo, name)
    }
}

#[derive(Serialize)]
pub struct ChangelogBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(skip)]
    package: PackageRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

impl<'client> ChangelogBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageRef::new(kind, repo, name),
            from: None,
            to: None,
        }
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub async fn send(self) -> Result<Changelog> {
        let package = self.client.get_json(&self.package.path(""), &[]).await?;
        let package_id = package["package_id"]
            .as_str()
            .ok_or_else(|| ArtifactHubError::missing_field("package_id", "this package"))?;

        ChangelogByPackageIdBuilder {
            client: self.client,
            package_id: package_id.to_string(),
            from: self.from,
            to: self.to,
        }
        .send()
        .await
    }
}

#[derive(Serialize)]
pub struct ChangelogByPackageIdBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(skip)]
    package_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

impl<'client> ChangelogByPackageIdBuilder<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient, package_id: impl Into<String>) -> Self {
        Self {
            client,
            package_id: package_id.into(),
            from: None,
            to: None,
        }
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub async fn send(self) -> Result<Changelog> {
        let path = package_id_url(&self.package_id, "/changelog");
        let query =
            optional_query_params([("from", self.from.as_deref()), ("to", self.to.as_deref())]);
        let json = self.client.get_json(&path, &query).await?;
        let entries: Vec<ChangelogEntry> = serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))?;

        Ok(Changelog { entries })
    }
}

#[derive(Serialize)]
pub struct ChangelogMarkdownBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(skip)]
    package: PackageRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

impl<'client> ChangelogMarkdownBuilder<'client> {
    pub(crate) fn new(
        client: &'client ArtifactHubClient,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            package: PackageRef::new(kind, repo, name),
            from: None,
            to: None,
        }
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub async fn send(self) -> Result<ChangelogMarkdown> {
        let query =
            optional_query_params([("from", self.from.as_deref()), ("to", self.to.as_deref())]);
        let body = self
            .client
            .get(&self.package.path("/changelog.md"), &query)
            .await?;
        Ok(ChangelogMarkdown { changelog: body })
    }
}
