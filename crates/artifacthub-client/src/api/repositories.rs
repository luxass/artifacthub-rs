use serde::Serialize;

use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::{SearchRepositoriesResponse, SearchRepositoryResult};

#[derive(Clone, Copy)]
pub struct RepositoriesHandler<'client> {
    client: &'client ArtifactHubClient,
}

impl<'client> RepositoriesHandler<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self { client }
    }

    pub fn search(self) -> SearchRepositoriesBuilder<'client> {
        SearchRepositoriesBuilder {
            client: self.client,
            name: None,
            kind: None,
            user: None,
            org: None,
            limit: None,
            offset: None,
        }
    }
}

#[derive(Serialize)]
pub struct SearchRepositoriesBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
}

impl<'client> SearchRepositoriesBuilder<'client> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn org(mut self, org: impl Into<String>) -> Self {
        self.org = Some(org.into());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<SearchRepositoriesResponse> {
        let json = self
            .client
            .get_json("/repositories/search", &self.query_params())
            .await?;
        let repositories: Vec<SearchRepositoryResult> = serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))?;

        Ok(SearchRepositoriesResponse { repositories })
    }

    fn query_params(&self) -> Vec<(String, String)> {
        let mut query_params = Vec::new();
        if let Some(name) = &self.name {
            query_params.push(("name".to_string(), name.clone()));
        }
        if let Some(kind) = &self.kind {
            query_params.push(("kind".to_string(), kind.clone()));
        }
        if let Some(user) = &self.user {
            query_params.push(("user".to_string(), user.clone()));
        }
        if let Some(org) = &self.org {
            query_params.push(("org".to_string(), org.clone()));
        }
        if let Some(limit) = self.limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = self.offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }
        query_params
    }
}
