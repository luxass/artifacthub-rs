use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SecurityReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SecuritySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Vulnerability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SecuritySummary {
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
