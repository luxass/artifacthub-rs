use crate::client::{ArtifactHubClient, encode_path_segment};

pub mod changelog;
pub mod helm;
pub mod list;
pub mod package;
pub(crate) mod reference;
pub mod search;
pub mod security;
pub mod stats;
pub mod views;

pub use changelog::{ChangelogBuilder, ChangelogByPackageIdBuilder, ChangelogMarkdownBuilder};
pub use list::StarredPackagesBuilder;
pub use package::{
    GetPackageBuilder, PackageStarStatsBuilder, PackageSummaryBuilder, PackageVersionsBuilder,
    ProductionUsageBuilder, ReadmeBuilder,
};
pub use search::SearchPackagesBuilder;

pub(crate) use reference::PackageReference;

#[derive(Clone, Copy)]
pub struct PackagesHandler<'client> {
    pub(crate) client: &'client ArtifactHubClient,
}

impl<'client> PackagesHandler<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self { client }
    }
}

pub(crate) fn package_id_url(package_id: &str, suffix: &str) -> String {
    format!("/packages/{}{}", encode_path_segment(package_id), suffix)
}

pub(crate) fn package_version_url(package_id: &str, version: &str, suffix: &str) -> String {
    format!(
        "/packages/{}/{}{}",
        encode_path_segment(package_id),
        encode_path_segment(version),
        suffix
    )
}

/// Shared query-string collector. Handles single, repeated, bool, and usize
/// params so search builders don't each reimplement the same ~30 lines.
/// `Vec<(String, String)>` preserves duplicates for `?kind=0&kind=3`.
#[derive(Default)]
pub(crate) struct QueryParams(Vec<(String, String)>);

impl QueryParams {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn opt(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        if let Some(value) = value {
            self.0.push((key.to_string(), value.to_string()));
        }
        self
    }

    pub(crate) fn opt_bool(&mut self, key: &str, value: Option<bool>) -> &mut Self {
        if let Some(value) = value {
            self.0.push((key.to_string(), value.to_string()));
        }
        self
    }

    pub(crate) fn opt_usize(&mut self, key: &str, value: Option<usize>) -> &mut Self {
        if let Some(value) = value {
            self.0.push((key.to_string(), value.to_string()));
        }
        self
    }

    pub(crate) fn multi(&mut self, key: &str, values: &[String]) -> &mut Self {
        for value in values {
            self.0.push((key.to_string(), value.clone()));
        }
        self
    }

    pub(crate) fn finish(self) -> Vec<(String, String)> {
        self.0
    }
}
