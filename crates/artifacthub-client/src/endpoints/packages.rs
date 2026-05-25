use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::{Changelog, PackageReadme, PackageSummary, PackageVersions, SearchResponse};

/// Package search and lookup endpoints.
///
/// Access via `client.packages.*`.
#[derive(Clone)]
pub struct Packages {
    inner: Arc<ClientInner>,
}

impl Packages {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Search for packages by query, kind, repo, or org.
    pub async fn search(&self, params: &SearchParams) -> Result<SearchResponse, String> {
        let mut query_params: Vec<(String, String)> = vec![];

        if let Some(q) = &params.q {
            query_params.push(("q".to_string(), q.clone()));
        }
        if let Some(kind) = &params.kind {
            query_params.push(("kind".to_string(), kind.clone()));
        }
        if let Some(repo) = &params.repo {
            query_params.push(("repo".to_string(), repo.clone()));
        }
        if let Some(org) = &params.org {
            query_params.push(("org".to_string(), org.clone()));
        }
        if let Some(limit) = params.limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = params.offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }

        let json = self
            .inner
            .get_json("/packages/search", &query_params)
            .await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get metadata summary for a package.
    pub async fn get(&self, params: &GetParams) -> Result<PackageSummary, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get the README content for a package.
    pub async fn readme(&self, params: &GetParams) -> Result<PackageReadme, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/readme");
        let json = self.inner.get_json(&path, &query_params).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// List all available versions for a package.
    pub async fn versions(&self, params: &GetParams) -> Result<PackageVersions, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "/changelog");
        let json = self.inner.get_json(&path, &[]).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get changelog between versions (JSON).
    pub async fn changelog(&self, params: &ChangelogParams) -> Result<Changelog, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref from) = params.from {
            query_params.push(("from".to_string(), from.clone()));
        }
        if let Some(ref to) = params.to {
            query_params.push(("to".to_string(), to.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/changelog");
        let json = self.inner.get_json(&path, &query_params).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }
}

/// Parameters for searching packages.
#[derive(Debug, Default)]
pub struct SearchParams {
    pub q: Option<String>,
    pub kind: Option<String>,
    pub repo: Option<String>,
    pub org: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Parameters for getting package details.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub version: Option<String>,
}

/// Parameters for getting changelog between versions.
#[derive(Debug)]
pub struct ChangelogParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub from: Option<String>,
    pub to: Option<String>,
}
