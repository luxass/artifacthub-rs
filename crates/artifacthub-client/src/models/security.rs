use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::json::deserialize_null_default;

/// Security report as stored by Artifact Hub: map of image reference to its
/// Trivy scan report. Upstream column holds only `images_reports`
/// (`update_snapshot_security_report.sql`), never a grouped
/// `{critical_vulnerabilities,...}` object.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SecurityReport(pub HashMap<String, ImageReport>);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ImageReport {
    #[serde(
        default,
        rename = "Results",
        deserialize_with = "deserialize_null_default"
    )]
    pub results: Vec<ScanResult>,
    /// Trivy metadata is returned unchanged by Artifact Hub.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScanResult {
    #[serde(default, rename = "Target")]
    pub target: String,
    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(
        default,
        rename = "Vulnerabilities",
        deserialize_with = "deserialize_null_default"
    )]
    pub vulnerabilities: Vec<Vulnerability>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Vulnerability {
    #[serde(
        default,
        rename = "VulnerabilityID",
        skip_serializing_if = "Option::is_none"
    )]
    pub vulnerability_id: Option<String>,
    #[serde(default, rename = "PkgName", skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    #[serde(
        default,
        rename = "InstalledVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub installed_version: Option<String>,
    #[serde(
        default,
        rename = "FixedVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_version: Option<String>,
    #[serde(default, rename = "Severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
