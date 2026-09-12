use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::models::ArtifactHubValue;
use crate::models::SearchResult;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageCounts {
    pub packages: i64,
    pub releases: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageList {
    pub packages: Vec<SearchResult>,
    #[cfg_attr(feature = "schemars", schemars(transform = remove_format))]
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ProductionUsageOrganization {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    #[serde(default)]
    pub used_in_production: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageViews(pub BTreeMap<String, BTreeMap<String, i64>>);

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_operator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crds: Option<Vec<ArtifactHubValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crds_examples: Option<Vec<ArtifactHubValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_containers_images_whitelisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_values_schema: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_changelog: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<crate::models::ChangelogChange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<Maintainer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Recommendation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<Screenshot>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<SignKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_organizations_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[serde(default)]
    pub package_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub normalized_name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub signed: bool,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub ts: i64,
    #[serde(default)]
    pub repository: RepositoryInfo,
    #[serde(default)]
    pub stats: PackageStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ArtifactHubValue>,
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_images: Option<Vec<ContainerImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_summary: Option<SecurityReportSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_created_at: Option<i64>,
    #[serde(default)]
    pub contains_security_updates: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct RepositoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_alias: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub kind: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
    #[serde(default)]
    pub verified_publisher: bool,
    #[serde(default)]
    pub official: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_disabled: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageStats {
    #[serde(default)]
    pub subscriptions: i32,
    #[serde(default)]
    pub webhooks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ContainerImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub image: String,
    #[serde(default)]
    pub whitelisted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SecurityReportSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageVersion {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(default)]
    pub contains_security_updates: bool,
    #[serde(default)]
    pub prerelease: bool,
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageVersions {
    pub versions: Vec<PackageVersion>,
    #[cfg_attr(feature = "schemars", schemars(transform = remove_format))]
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageReadme {
    pub readme: String,
}

#[cfg(feature = "schemars")]
fn remove_format(schema: &mut schemars::Schema) {
    schema.remove("format");
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Channel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Maintainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Recommendation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Screenshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SignKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
