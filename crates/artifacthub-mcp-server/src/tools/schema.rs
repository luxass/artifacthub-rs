pub fn remove_format(schema: &mut schemars::Schema) {
    schema.remove("format");
}

#[cfg(test)]
mod tests {
    use crate::tools::{
        get_package_versions::GetPackageVersionsParams, search_packages::SearchParams,
        search_repositories::SearchRepositoriesParams,
    };
    use artifacthub_client::models::{PackageList, PackageVersions};

    fn contains_uint_format(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Object(fields) => {
                fields.get("format").and_then(|format| format.as_str()) == Some("uint")
                    || fields.values().any(contains_uint_format)
            }
            serde_json::Value::Array(values) => values.iter().any(contains_uint_format),
            _ => false,
        }
    }

    #[test]
    fn exposed_usize_schemas_do_not_emit_rust_uint_format() {
        let schemas = [
            serde_json::to_value(schemars::schema_for!(GetPackageVersionsParams)).unwrap(),
            serde_json::to_value(schemars::schema_for!(SearchParams)).unwrap(),
            serde_json::to_value(schemars::schema_for!(SearchRepositoriesParams)).unwrap(),
            serde_json::to_value(schemars::schema_for!(PackageList)).unwrap(),
            serde_json::to_value(schemars::schema_for!(PackageVersions)).unwrap(),
        ];

        for schema in schemas {
            assert!(
                !contains_uint_format(&schema),
                "schema should not contain Rust-specific uint format: {schema}"
            );
        }
    }
}
