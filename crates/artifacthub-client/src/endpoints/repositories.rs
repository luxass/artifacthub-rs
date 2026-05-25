use std::sync::Arc;

use crate::client::ClientInner;
use crate::models::{SearchRepositoriesResponse, SearchRepositoryResult};

/// Repository search endpoints.
///
/// Access via `client.repositories.*`.
#[derive(Clone)]
pub struct Repositories {
    inner: Arc<ClientInner>,
}

impl Repositories {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Search repositories by name, kind, user, or org.
    pub async fn search(
        &self,
        params: &SearchParams,
    ) -> Result<SearchRepositoriesResponse, String> {
        let mut query_params: Vec<(String, String)> = vec![];

        if let Some(name) = &params.name {
            query_params.push(("name".to_string(), name.clone()));
        }
        if let Some(kind) = &params.kind {
            query_params.push(("kind".to_string(), kind.clone()));
        }
        if let Some(user) = &params.user {
            query_params.push(("user".to_string(), user.clone()));
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
            .get_json("/repositories/search", &query_params)
            .await?;
        let repositories: Vec<SearchRepositoryResult> =
            serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(SearchRepositoriesResponse { repositories })
    }
}

/// Parameters for searching repositories.
#[derive(Debug, Default)]
pub struct SearchParams {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub user: Option<String>,
    pub org: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
