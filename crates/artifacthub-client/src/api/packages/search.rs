use crate::api::packages::{PackagesHandler, QueryParams};
use crate::client::ArtifactHubClient;
use crate::error::Result;
use crate::models::SearchResponse;

impl<'client> PackagesHandler<'client> {
    pub fn search(self) -> SearchPackagesBuilder<'client> {
        SearchPackagesBuilder::new(self.client)
    }
}

pub struct SearchPackagesBuilder<'client> {
    client: &'client ArtifactHubClient,
    query: Option<String>,
    ts_query: Option<String>,
    kinds: Vec<String>,
    repos: Vec<String>,
    orgs: Vec<String>,
    users: Vec<String>,
    categories: Vec<String>,
    verified_publisher: Option<bool>,
    official: Option<bool>,
    cncf: Option<bool>,
    operators: Option<bool>,
    deprecated: Option<bool>,
    licenses: Vec<String>,
    capabilities: Vec<String>,
    sort: Option<String>,
    facets: Option<bool>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'client> SearchPackagesBuilder<'client> {
    pub(crate) fn new(client: &'client ArtifactHubClient) -> Self {
        Self {
            client,
            query: None,
            ts_query: None,
            kinds: Vec::new(),
            repos: Vec::new(),
            orgs: Vec::new(),
            users: Vec::new(),
            categories: Vec::new(),
            verified_publisher: None,
            official: None,
            cncf: None,
            operators: None,
            deprecated: None,
            licenses: Vec::new(),
            capabilities: Vec::new(),
            sort: None,
            facets: None,
            limit: None,
            offset: None,
        }
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn ts_query(mut self, query: impl Into<String>) -> Self {
        self.ts_query = Some(query.into());
        self
    }

    /// Append one kind id (may be called repeatedly for `?kind=0&kind=3`).
    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kinds.push(kind.into());
        self
    }

    pub fn kinds(mut self, kinds: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.kinds.extend(kinds.into_iter().map(Into::into));
        self
    }

    pub fn repo(mut self, repo: impl Into<String>) -> Self {
        self.repos.push(repo.into());
        self
    }

    pub fn repos(mut self, repos: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.repos.extend(repos.into_iter().map(Into::into));
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

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.users.push(user.into());
        self
    }

    pub fn users(mut self, users: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.users.extend(users.into_iter().map(Into::into));
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.categories.push(category.into());
        self
    }

    pub fn categories(mut self, categories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.categories
            .extend(categories.into_iter().map(Into::into));
        self
    }

    pub fn verified_publisher(mut self, value: bool) -> Self {
        self.verified_publisher = Some(value);
        self
    }

    pub fn official(mut self, value: bool) -> Self {
        self.official = Some(value);
        self
    }

    pub fn cncf(mut self, value: bool) -> Self {
        self.cncf = Some(value);
        self
    }

    pub fn operators(mut self, value: bool) -> Self {
        self.operators = Some(value);
        self
    }

    pub fn deprecated(mut self, value: bool) -> Self {
        self.deprecated = Some(value);
        self
    }

    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.licenses.push(license.into());
        self
    }

    pub fn licenses(mut self, licenses: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.licenses.extend(licenses.into_iter().map(Into::into));
        self
    }

    pub fn capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    pub fn capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.capabilities
            .extend(capabilities.into_iter().map(Into::into));
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn facets(mut self, facets: bool) -> Self {
        self.facets = Some(facets);
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
        let (mut response, total_count) = self
            .client
            .get_json_with_pagination::<SearchResponse>("/packages/search", &self.query_params())
            .await?;
        response.total_count = total_count;
        Ok(response)
    }

    fn query_params(&self) -> Vec<(String, String)> {
        let mut q = QueryParams::new();
        q.opt("ts_query_web", self.query.as_deref());
        q.opt("ts_query", self.ts_query.as_deref());
        q.multi("kind", &self.kinds);
        q.multi("repo", &self.repos);
        q.multi("org", &self.orgs);
        q.multi("user", &self.users);
        q.multi("category", &self.categories);
        q.opt_bool("verified_publisher", self.verified_publisher);
        q.opt_bool("official", self.official);
        q.opt_bool("cncf", self.cncf);
        q.opt_bool("operators", self.operators);
        q.opt_bool("deprecated", self.deprecated);
        q.multi("license", &self.licenses);
        q.multi("capabilities", &self.capabilities);
        q.opt("sort", self.sort.as_deref());
        q.opt_bool("facets", self.facets);
        q.opt_usize("limit", self.limit);
        q.opt_usize("offset", self.offset);
        q.finish()
    }
}
