use crate::api::packages::{PackageReference, PackagesHandler, package_id_url};
use crate::client::ArtifactHubClient;
use crate::error::Result;
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

pub struct ChangelogBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
    from: Option<String>,
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
            package: PackageReference::new(kind, repo, name),
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
        let package_id = self.package.resolve_package_id(self.client).await?;

        ChangelogByPackageIdBuilder {
            client: self.client,
            package_id,
            from: self.from,
            to: self.to,
        }
        .send()
        .await
    }
}

pub struct ChangelogByPackageIdBuilder<'client> {
    client: &'client ArtifactHubClient,
    package_id: String,
    from: Option<String>,
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
        // Upstream `GET /packages/{id}/changelog` takes no query params and
        // always returns the full list (handlers.go GetChangelog, manager
        // GetChangelog, get_package_changelog.sql). Filter locally.
        let path = package_id_url(&self.package_id, "/changelog");
        let entries: Vec<ChangelogEntry> = self.client.get_json(&path, &[]).await?;

        Ok(Changelog {
            entries: filter_entries(entries, self.from.as_deref(), self.to.as_deref()),
        })
    }
}

/// Keep entries with `from < version <= to` (both inclusive of `to`,
/// exclusive of `from`). Unparseable versions fall back to lexical compare;
/// unparseable bounds disable that side of the filter.
fn filter_entries(
    entries: Vec<ChangelogEntry>,
    from: Option<&str>,
    to: Option<&str>,
) -> Vec<ChangelogEntry> {
    entries
        .into_iter()
        .filter(|e| version_in_range(&e.version, from, to))
        .collect()
}

fn version_in_range(version: &str, from: Option<&str>, to: Option<&str>) -> bool {
    if let Some(from) = from
        && compare_versions(version, from) != std::cmp::Ordering::Greater
    {
        return false;
    }
    if let Some(to) = to
        && compare_versions(version, to) == std::cmp::Ordering::Greater
    {
        return false;
    }
    true
}

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (
        semver::Version::parse(a.trim_start_matches('v')),
        semver::Version::parse(b.trim_start_matches('v')),
    ) {
        (Ok(a), Ok(b)) => a.cmp(&b),
        _ => a.cmp(b),
    }
}

pub struct ChangelogMarkdownBuilder<'client> {
    client: &'client ArtifactHubClient,
    package: PackageReference,
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
            package: PackageReference::new(kind, repo, name),
        }
    }

    pub async fn send(self) -> Result<ChangelogMarkdown> {
        // Upstream `.../changelog.md` takes no query params (OpenAPI
        // generateChangelogMD, web client getChangelogMD). Always full text.
        let body = self
            .client
            .get(&self.package.path("/changelog.md"), &[])
            .await?;
        Ok(ChangelogMarkdown { changelog: body })
    }
}
