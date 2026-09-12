use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchResponse {
    pub packages: Vec<SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<SearchFacet>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchFacet {
    pub title: String,
    pub filter_key: String,
    pub options: Vec<SearchFacetOption>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchFacetOption {
    /// Kind/category IDs are numbers; license/capability IDs are strings.
    pub id: crate::models::ArtifactHubValue,
    pub name: String,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_values_schema: Option<bool>,
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
    #[serde(default)]
    pub repository: SearchRepositoryInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub signed: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_containers_images_whitelisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_organizations_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_summary: Option<crate::models::SecurityReportSummary>,
    #[serde(default)]
    pub stars: i32,
    #[serde(default)]
    pub ts: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchRepositoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
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
