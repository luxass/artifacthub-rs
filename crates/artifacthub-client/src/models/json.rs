use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ArtifactHubValue(
    #[cfg_attr(feature = "schemars", schemars(schema_with = "json_value_schema"))]
    pub serde_json::Value,
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ValuesSchemaDocument {
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

#[cfg(feature = "schemars")]
fn json_value_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Arbitrary JSON value returned by Artifact Hub",
        "type": ["object", "array", "string", "number", "integer", "boolean", "null"]
    })
}

#[cfg(test)]
mod tests {
    use super::ArtifactHubValue;

    #[test]
    fn artifact_hub_value_preserves_large_integer_precision() {
        let value: ArtifactHubValue =
            serde_json::from_str(r#"9007199254740993"#).expect("valid JSON number");

        assert_eq!(value.0.to_string(), "9007199254740993");
    }
}
