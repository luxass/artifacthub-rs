use artifacthub_client::kind::to_id;

#[test]
fn maps_current_artifact_hub_kinds() {
    assert_eq!(to_id("helm"), Some(0));
    assert_eq!(to_id("tekton-task"), Some(7));
    assert_eq!(to_id("keda-scaler"), Some(8));
    assert_eq!(to_id("gatekeeper"), Some(14));
    assert_eq!(to_id("kagent"), Some(28));
}

#[test]
fn maps_legacy_aliases_to_current_kinds() {
    assert_eq!(to_id("tekton"), Some(7));
    assert_eq!(to_id("keda"), Some(8));
    assert_eq!(to_id("coredns"), Some(9));
    assert_eq!(to_id("bootable-container"), Some(27));
}
