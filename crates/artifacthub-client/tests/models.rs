use artifacthub_client::models::{
    ArtifactHubValue, ChartTemplate, ChartTemplates, SearchRepositoryResult, SearchResult,
};

#[test]
fn artifact_hub_value_preserves_large_integer_precision() {
    let value: ArtifactHubValue =
        serde_json::from_str(r#"9007199254740993"#).expect("valid JSON number");

    assert_eq!(value.0.to_string(), "9007199254740993");
}

#[test]
fn chart_without_templates_returns_an_empty_collection() {
    // GetChartTemplates marshals Helm's nil slice as null for dependency-only charts.
    let response: ChartTemplates = serde_json::from_value(serde_json::json!({
        "templates": null,
        "values": {"dependency": {"enabled": true}}
    }))
    .unwrap();

    assert!(response.templates.is_empty());
    assert_eq!(response.values.unwrap().0["dependency"]["enabled"], true);
}

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

#[test]
fn search_repository_tolerates_missing_bools_and_always_serializes_them() {
    // Regression test: Artifact Hub omits `official` / `verified_publisher`
    // for some repos, and rmcp rejects output where a required bool is
    // missing (e.g. org=kvalitetsit). Missing must default to false, and
    // false must still serialize so the output schema stays valid.
    let repo: SearchRepositoryResult = serde_json::from_value(serde_json::json!({
        "repository_id": "repo-123",
        "name": "kvalitetsit",
        "url": "https://example.com",
        "kind": 0
    }))
    .expect("missing bools should default to false");

    assert!(!repo.official);
    assert!(!repo.verified_publisher);

    let value = serde_json::to_value(&repo).expect("serializable");
    assert_eq!(value.get("official"), Some(&serde_json::Value::Bool(false)));
    assert_eq!(
        value.get("verified_publisher"),
        Some(&serde_json::Value::Bool(false))
    );
}

#[test]
fn search_result_tolerates_missing_description() {
    // Regression test: real /packages/search omits `description` for some
    // packages (seen with org=kvalitetsit). Missing must default to "".
    let result: SearchResult = serde_json::from_value(serde_json::json!({
        "package_id": "pkg-123",
        "name": "some-chart",
        "normalized_name": "some-chart",
        "version": "1.0.0",
        "repository": {"name": "kvalitetsit", "url": "https://example.com"},
        "stars": 3,
        "ts": 1700000000
    }))
    .expect("missing description should default to empty string");

    assert_eq!(result.description, "");
    let value = serde_json::to_value(&result).expect("serializable");
    assert_eq!(
        value.get("description"),
        Some(&serde_json::Value::String(String::new()))
    );
}
