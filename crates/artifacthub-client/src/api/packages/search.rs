use serde::Serialize;

use crate::api::packages::{PackagesHandler, optional_query_params, optional_usize_query_params};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::SearchResponse;

impl<'client> PackagesHandler<'client> {
    pub fn search(self) -> SearchPackagesBuilder<'client> {
        SearchPackagesBuilder::new(self.client)
    }
}

#[derive(Serialize)]
pub struct SearchPackagesBuilder<'client> {
    #[serde(skip)]
    client: &'client ArtifactHubClient,
    #[serde(rename = "ts_query_web", skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
}

impl<'client> SearchPackagesBuilder<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self {
            client,
            query: None,
            kind: None,
            repo: None,
            org: None,
            limit: None,
            offset: None,
        }
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn repo(mut self, repo: impl Into<String>) -> Self {
        self.repo = Some(repo.into());
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

    pub async fn send(self) -> Result<SearchResponse> {
        let json = self
            .client
            .get_json("/packages/search", &self.query_params())
            .await?;
        serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }

    fn query_params(&self) -> Vec<(String, String)> {
        optional_query_params([
            ("ts_query_web", self.query.as_deref()),
            ("kind", self.kind.as_deref()),
            ("repo", self.repo.as_deref()),
            ("org", self.org.as_deref()),
        ])
        .into_iter()
        .chain(optional_usize_query_params([
            ("limit", self.limit),
            ("offset", self.offset),
        ]))
        .collect()
    }
}
