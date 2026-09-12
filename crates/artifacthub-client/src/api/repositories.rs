use crate::api::packages::QueryParams;
use crate::client::ArtifactHubClient;
use crate::error::Result;
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
            url: None,
            kinds: Vec::new(),
            users: Vec::new(),
            orgs: Vec::new(),
            limit: None,
            offset: None,
        }
    }
}

pub struct SearchRepositoriesBuilder<'client> {
    client: &'client ArtifactHubClient,
    name: Option<String>,
    url: Option<String>,
    kinds: Vec<String>,
    users: Vec<String>,
    orgs: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'client> SearchRepositoriesBuilder<'client> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Append one kind id (repeatable for `?kind=0&kind=3`).
    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kinds.push(kind.into());
        self
    }

    pub fn kinds(mut self, kinds: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.kinds.extend(kinds.into_iter().map(Into::into));
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.users.push(user.into());
        self
    }

    pub fn users(mut self, users: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.users.extend(users.into_iter().map(Into::into));
        self
    }

    pub fn org(mut self, org: impl Into<String>) -> Self {
        self.orgs.push(org.into());
        self
    }

    pub fn orgs(mut self, orgs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.orgs.extend(orgs.into_iter().map(Into::into));
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
        let (repositories, total_count) = self
            .client
            .get_json_with_pagination::<Vec<SearchRepositoryResult>>(
                "/repositories/search",
                &self.query_params(),
            )
            .await?;

        Ok(SearchRepositoriesResponse {
            repositories,
            total_count,
        })
    }

    fn query_params(&self) -> Vec<(String, String)> {
        let mut q = QueryParams::new();
        q.opt("name", self.name.as_deref());
        q.opt("url", self.url.as_deref());
        q.multi("kind", &self.kinds);
        q.multi("user", &self.users);
        q.multi("org", &self.orgs);
        q.opt_usize("limit", self.limit);
        q.opt_usize("offset", self.offset);
        q.finish()
    }
}
