pub const KIND_DESCRIPTION: &str = "Package kind: helm, falco, opa, olm, tekton, krew, etc.";

pub fn to_id(kind: &str) -> Option<i32> {
    match kind {
        "helm" => Some(0),
        "falco" => Some(1),
        "opa" => Some(2),
        "olm" => Some(3),
        "tekton" => Some(4),
        "krew" => Some(5),
        "helm-plugin" => Some(6),
        "gatekeeper" => Some(7),
        "keptn" => Some(8),
        "tinkerbell" => Some(9),
        "cni" => Some(10),
        "contour" => Some(11),
        "keda" => Some(12),
        "coredns" => Some(13),
        "operator" => Some(14),
        "kubewarden" => Some(15),
        "inspektor-gadget" => Some(16),
        "kubearmor" => Some(17),
        "backstage" => Some(18),
        "headlamp" => Some(19),
        "kpt" => Some(20),
        "kubeescape" => Some(21),
        "argo-template" => Some(22),
        "helm-oci" => Some(23),
        _ => None,
    }
}

pub fn valid_kinds() -> &'static [&'static str] {
    &[
        "helm", "falco", "opa", "olm", "tekton", "krew", "helm-plugin", "gatekeeper",
        "keptn", "tinkerbell", "cni", "contour", "keda", "coredns", "operator",
        "kubewarden", "inspektor-gadget", "kubearmor", "backstage", "headlamp",
        "kpt", "kubeescape", "argo-template", "helm-oci",
    ]
}
