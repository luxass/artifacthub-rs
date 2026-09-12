use artifacthub_client::kind::{self as pkg_kind};

/// Shared `limit` validation for search tools (upstream max 60).
pub fn validate_limit(limit: Option<usize>) -> Result<(), String> {
    if let Some(limit) = limit
        && (limit == 0 || limit > 60)
    {
        return Err("limit must be between 1 and 60".to_string());
    }
    Ok(())
}

/// Resolve kind names to upstream ids, supporting repeats.
/// Uses canonical aliases (`tinkerbell`→`tbaction`, etc.) via `to_id`.
pub fn resolve_kind_ids(kinds: Option<Vec<String>>) -> Result<Vec<String>, String> {
    let mut ids = Vec::new();
    for kind in kinds.unwrap_or_default() {
        if let Some(id) = pkg_kind::to_id(&kind) {
            ids.push(id.to_string());
        } else {
            return Err(format!(
                "Unknown kind: '{}'. Valid kinds: {}",
                kind,
                pkg_kind::valid_kinds().join(", ")
            ));
        }
    }
    Ok(ids)
}
