use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::StarStats;

#[derive(Clone, Copy)]
pub struct StatsHandler<'client> {
    client: &'client ArtifactHubClient,
}

impl<'client> StatsHandler<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self { client }
    }

    pub fn star_stats(
        self,
        kind: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
    ) -> StatsStarStatsBuilder<'client> {
        StatsStarStatsBuilder {
            client: self.client,
            kind: kind.into(),
            repo: repo.into(),
            name: name.into(),
        }
    }
}

pub struct StatsStarStatsBuilder<'client> {
    client: &'client ArtifactHubClient,
    kind: String,
    repo: String,
    name: String,
}

impl<'client> StatsStarStatsBuilder<'client> {
    pub async fn send(self) -> Result<StarStats> {
        self.client
            .packages()
            .star_stats(self.kind, self.repo, self.name)
            .send()
            .await
    }
}
