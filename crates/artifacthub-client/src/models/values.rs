use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PackageValues {
    pub package: String,
    pub version: String,
    pub values: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ValuesSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::models::json_value_schema")
    )]
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ChartTemplates {
    pub templates: Vec<ChartTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ChartTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StarStats {
    pub stars: Vec<StarHistoryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StarHistoryEntry {
    pub total: i32,
    pub dates: Vec<StarDateEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StarDateEntry {
    pub date: String,
    pub stars: i32,
}
