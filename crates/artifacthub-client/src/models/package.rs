use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageSummary {
    pub package_id: String,
    pub name: String,
    pub normalized_name: String,
    pub version: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub prerelease: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub signed: bool,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub ts: i64,
    pub repository: RepositoryInfo,
    pub stats: PackageStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::models::json_value_schema")
    )]
    pub data: Option<serde_json::Value>,
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_images: Option<Vec<ContainerImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_summary: Option<SecurityReportSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_created_at: Option<i64>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub contains_security_updates: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct RepositoryInfo {
    pub name: String,
    pub display_name: String,
    pub url: String,
    pub kind: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub verified_publisher: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub official: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageStats {
    pub subscriptions: i32,
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
    pub name: Option<String>,
    pub image: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
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
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub contains_security_updates: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub prerelease: bool,
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageVersions {
    pub versions: Vec<PackageVersion>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageReadme {
    pub readme: String,
}
