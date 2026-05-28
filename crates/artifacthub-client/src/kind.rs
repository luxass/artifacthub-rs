pub const KIND_DESCRIPTION: &str = "Package kind: helm, falco, opa, olm, tinkerbell, krew, helm-plugin, tekton-task, keda-scaler, etc.";

pub fn to_id(kind: &str) -> Option<i32> {
    match kind {
        "helm" => Some(0),
        "falco" => Some(1),
        "opa" => Some(2),
        "olm" => Some(3),
        "tinkerbell" => Some(4),
        "krew" => Some(5),
        "helm-plugin" => Some(6),
        "tekton-task" | "tekton" => Some(7),
        "keda-scaler" | "keda" => Some(8),
        "coredns-plugin" | "coredns" => Some(9),
        "keptn" => Some(10),
        "tekton-pipeline" => Some(11),
        "container" => Some(12),
        "kubewarden" => Some(13),
        "gatekeeper" => Some(14),
        "kyverno" => Some(15),
        "knative-client-plugin" => Some(16),
        "backstage" => Some(17),
        "argo-template" => Some(18),
        "kubearmor" => Some(19),
        "kcl" => Some(20),
        "headlamp" => Some(21),
        "inspektor-gadget" => Some(22),
        "tekton-stepaction" => Some(23),
        "meshery-design" => Some(24),
        "opencost" => Some(25),
        "radius" => Some(26),
        "bootc" | "bootable-container" => Some(27),
        "kagent" => Some(28),
        _ => None,
    }
}

pub fn valid_kinds() -> &'static [&'static str] {
    &[
        "helm",
        "falco",
        "opa",
        "olm",
        "tinkerbell",
        "krew",
        "helm-plugin",
        "tekton-task",
        "tekton",
        "keda-scaler",
        "keda",
        "coredns-plugin",
        "coredns",
        "keptn",
        "tekton-pipeline",
        "container",
        "kubewarden",
        "gatekeeper",
        "kyverno",
        "knative-client-plugin",
        "backstage",
        "argo-template",
        "kubearmor",
        "kcl",
        "headlamp",
        "inspektor-gadget",
        "tekton-stepaction",
        "meshery-design",
        "opencost",
        "radius",
        "bootc",
        "bootable-container",
        "kagent",
    ]
}

#[cfg(test)]
mod tests {
    use super::to_id;

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
}
