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
    GetPackageBuilder, GetPackageVersionBuilder, PackageStarStatsBuilder, PackageSummaryBuilder,
    PackageVersionsBuilder, ProductionUsageBuilder, ReadmeBuilder,
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

pub(crate) fn version_suffix(version: &str) -> String {
    format!("/{}", encode_path_segment(version))
}

pub(crate) fn optional_query_params<const N: usize>(
    pairs: [(&str, Option<&str>); N],
) -> Vec<(String, String)> {
    pairs
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key.to_string(), value.to_string())))
        .collect()
}

pub(crate) fn optional_usize_query_params<const N: usize>(
    pairs: [(&str, Option<usize>); N],
) -> Vec<(String, String)> {
    pairs
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key.to_string(), value.to_string())))
        .collect()
}
