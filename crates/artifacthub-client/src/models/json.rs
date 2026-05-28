use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum ArtifactHubValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<ArtifactHubValue>),
    Object(BTreeMap<String, ArtifactHubValue>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ValuesSchemaDocument {
    #[serde(flatten)]
    pub fields: BTreeMap<String, ArtifactHubValue>,
}
