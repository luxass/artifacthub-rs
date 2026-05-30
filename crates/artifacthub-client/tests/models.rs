use artifacthub_client::models::{ArtifactHubValue, ChartTemplate};

#[test]
fn artifact_hub_value_preserves_large_integer_precision() {
    let value: ArtifactHubValue =
        serde_json::from_str(r#"9007199254740993"#).expect("valid JSON number");

    assert_eq!(value.0.to_string(), "9007199254740993");
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
