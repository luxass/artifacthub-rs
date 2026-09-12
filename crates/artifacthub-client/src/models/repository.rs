use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchRepositoriesResponse {
    pub repositories: Vec<SearchRepositoryResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SearchRepositoryResult {
    #[serde(default)]
    pub repository_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub kind: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_alias: Option<String>,
    #[serde(default)]
    pub verified_publisher: bool,
    #[serde(default)]
    pub official: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cncf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanning_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanning_errors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tracking_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tracking_errors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_deletion_protection: Option<bool>,
}
