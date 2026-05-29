use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchResponse {
    pub packages: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchResult {
    pub package_id: String,
    pub name: String,
    pub normalized_name: String,
    pub version: String,
    pub description: String,
    pub repository: SearchRepositoryInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub signed: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_containers_images_whitelisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_organizations_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_summary: Option<crate::models::SecurityReportSummary>,
    pub stars: i32,
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchRepositoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_publisher: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_disabled: Option<bool>,
}
