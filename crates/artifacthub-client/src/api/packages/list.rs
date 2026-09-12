use crate::api::packages::{PackagesHandler, QueryParams};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::{PackageList, SearchResult};

impl<'client> PackagesHandler<'client> {
    pub fn starred(self) -> StarredPackagesBuilder<'client> {
        StarredPackagesBuilder::new(self.client)
    }

    pub async fn random(self) -> Result<PackageList> {
        let packages: Vec<SearchResult> = self.client.get_json("/packages/random", &[]).await?;

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
        let mut q = QueryParams::new();
        q.opt_usize("limit", self.limit);
        q.opt_usize("offset", self.offset);
        let packages: Vec<SearchResult> = self
            .client
            .get_json("/packages/starred", &q.finish())
            .await?;

        Ok(PackageList {
            count: packages.len(),
            packages,
        })
    }
}
