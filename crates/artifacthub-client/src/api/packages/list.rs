use crate::api::packages::{PackagesHandler, optional_usize_query_params};
use crate::client::ArtifactHubClient;
use crate::error::{ArtifactHubError, Result};
use crate::models::{PackageList, SearchResult};

impl<'client> PackagesHandler<'client> {
    pub fn starred(self) -> StarredPackagesBuilder<'client> {
        StarredPackagesBuilder::new(self.client)
    }

    pub async fn random(self) -> Result<PackageList> {
        let json = self.client.get_json("/packages/random", &[]).await?;
        let packages: Vec<SearchResult> = serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))?;

        Ok(PackageList {
            count: packages.len(),
            packages,
        })
    }
}

pub struct StarredPackagesBuilder<'client> {
    client: &'client ArtifactHubClient,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'client> StarredPackagesBuilder<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self {
            client,
            limit: None,
            offset: None,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PackageList> {
        let json = self
            .client
            .get_json(
                "/packages/starred",
                &optional_usize_query_params([("limit", self.limit), ("offset", self.offset)]),
            )
            .await?;
        let packages: Vec<SearchResult> = serde_json::from_value(json)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))?;

        Ok(PackageList {
            count: packages.len(),
            packages,
        })
    }
}
