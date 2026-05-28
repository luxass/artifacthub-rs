use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize};

use crate::models::{ArtifactHubValue, ValuesSchemaDocument};

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
    pub schema: Option<ValuesSchemaDocument>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ChartTemplates {
    pub templates: Vec<ChartTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<ArtifactHubValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ChartTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_base64_string"
    )]
    #[cfg_attr(
        feature = "schemars",
        schemars(description = "Decoded Helm template source")
    )]
    pub data: Option<String>,
}

fn deserialize_base64_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Some(encoded) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    let decoded = String::from_utf8(decoded).map_err(serde::de::Error::custom)?;

    Ok(Some(decoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_template_data_is_decoded_from_base64() {
        let template: ChartTemplate = serde_json::from_value(serde_json::json!({
            "name": "templates/service.yaml",
            "data": "YXBpVmVyc2lvbjogdjEKa2luZDogU2VydmljZQo=",
        }))
        .unwrap();

        assert_eq!(
            template.data.as_deref(),
            Some("apiVersion: v1\nkind: Service\n")
        );
    }
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
