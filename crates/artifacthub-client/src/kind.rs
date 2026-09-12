pub const KIND_DESCRIPTION: &str = "Package kind: helm, falco, opa, olm, tbaction, krew, helm-plugin, tekton-task, keda-scaler, coredns, keptn, tekton-pipeline, container, kubewarden, gatekeeper, kyverno, knative-client-plugin, backstage, argo-template, kubearmor, kcl, headlamp, inspektor-gadget, tekton-stepaction, meshery, opencost, radius, bootc, kagent, etc.";

pub fn canonicalize(kind: &str) -> &str {
    match kind {
        "tinkerbell" | "tbaction" => "tbaction",
        "meshery-design" | "meshery" => "meshery",
        "coredns-plugin" | "coredns" => "coredns",
        "tekton" => "tekton-task",
        "keda" => "keda-scaler",
        "bootable-container" | "bootc" => "bootc",
        _ => kind,
    }
}

pub fn to_id(kind: &str) -> Option<i32> {
    match canonicalize(kind) {
        "helm" => Some(0),
        "falco" => Some(1),
        "opa" => Some(2),
        "olm" => Some(3),
        "tbaction" => Some(4),
        "krew" => Some(5),
        "helm-plugin" => Some(6),
        "tekton-task" => Some(7),
        "keda-scaler" => Some(8),
        "coredns" => Some(9),
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
        "meshery" => Some(24),
        "opencost" => Some(25),
        "radius" => Some(26),
        "bootc" => Some(27),
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
        "tbaction",
        "krew",
        "helm-plugin",
        "tekton-task",
        "keda-scaler",
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
        "meshery",
        "opencost",
        "radius",
        "bootc",
        "kagent",
    ]
}
