//! Response models for the Artifact Hub API.

pub mod changelog;
pub mod package;
pub mod repository;
pub mod search;
pub mod security;
pub mod values;

pub use changelog::*;
pub use package::*;
pub use repository::*;
pub use search::*;
pub use security::*;
pub use values::*;

#[cfg(feature = "schemars")]
pub(crate) fn json_value_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Arbitrary JSON value returned by Artifact Hub",
        "type": ["object", "array", "string", "number", "integer", "boolean", "null"]
    })
}

#[cfg(all(test, feature = "schemars"))]
mod tests {
    use super::{PackageSummary, ValuesSchema};

    #[test]
    fn arbitrary_json_fields_emit_object_schemas() {
        let package_schema = serde_json::to_value(schemars::schema_for!(PackageSummary)).unwrap();
        let data_schema = package_schema.pointer("/properties/data").unwrap();
        assert!(
            data_schema.is_object(),
            "PackageSummary.data schema must not be a boolean schema: {data_schema:?}"
        );

        let values_schema = serde_json::to_value(schemars::schema_for!(ValuesSchema)).unwrap();
        let schema_schema = values_schema.pointer("/properties/schema").unwrap();
        assert!(
            schema_schema.is_object(),
            "ValuesSchema.schema schema must not be a boolean schema: {schema_schema:?}"
        );
    }
}
