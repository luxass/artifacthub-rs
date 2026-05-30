use crate::api::{
    HelmHandler, PackagesHandler, RepositoriesHandler, SecurityHandler, StatsHandler,
};
use crate::error::{ArtifactHubError, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::Arc;

const DEFAULT_API_BASE: &str = "https://artifacthub.io/api/v1";

/// HTTP client for making requests to the Artifact Hub API.
#[derive(Clone)]
pub struct ArtifactHubClient {
    pub(crate) inner: Arc<ClientInner>,
}

pub(crate) struct ClientInner {
    client: Client,
    base_url: String,
    api_key_id: Option<String>,
    api_key_secret: Option<String>,
}

/// Builder for configuring an [`ArtifactHubClient`].
pub struct ArtifactHubClientBuilder {
    client: Option<Client>,
    base_url: String,
    api_key_id: Option<String>,
    api_key_secret: Option<String>,
}

impl Default for ArtifactHubClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl ArtifactHubClient {
    /// Create a new client with the default API base URL.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configurable client builder.
    pub fn builder() -> ArtifactHubClientBuilder {
        ArtifactHubClientBuilder::default()
    }

    /// Create a client with a custom API base URL.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self::builder().base_url(base_url).build()
    }

    /// Access package search and lookup operations.
    pub fn packages(&self) -> PackagesHandler<'_> {
        PackagesHandler::new(self)
    }

    /// Access repository search operations.
    pub fn repositories(&self) -> RepositoriesHandler<'_> {
        RepositoriesHandler::new(self)
    }

    /// Access Helm compatibility operations.
    pub fn helm(&self) -> HelmHandler<'_> {
        HelmHandler::new(self)
    }

    /// Access security compatibility operations.
    pub fn security(&self) -> SecurityHandler<'_> {
        SecurityHandler::new(self)
    }

    /// Access statistics compatibility operations.
    pub fn stats(&self) -> StatsHandler<'_> {
        StatsHandler::new(self)
    }

    /// Sends a GET request and returns the response body as a string.
    pub(crate) async fn get(&self, path: &str, params: &[(String, String)]) -> Result<String> {
        self.inner.get(path, params).await
    }

    /// Sends a GET request and parses the response as JSON.
    pub(crate) async fn get_json<T>(&self, path: &str, params: &[(String, String)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_json_with_context(path, params, "Failed to parse response")
            .await
    }

    pub(crate) async fn get_json_with_context<T>(
        &self,
        path: &str,
        params: &[(String, String)],
        context: &'static str,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.inner
            .get_json_with_context(path, params, context)
            .await
    }

    pub(crate) async fn get_optional_json<T>(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let body = self.get(path, params).await?;
        if body.trim().is_empty() {
            return Ok(None);
        }

        serde_json::from_str(&body)
            .map(Some)
            .map_err(|e| ArtifactHubError::json("Failed to parse response", e))
    }
}

impl Default for ArtifactHubClientBuilder {
    fn default() -> Self {
        Self {
            client: None,
            base_url: DEFAULT_API_BASE.to_string(),
            api_key_id: None,
            api_key_secret: None,
        }
    }
}

impl ArtifactHubClientBuilder {
    /// Set a custom API base URL.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Use a preconfigured reqwest client.
    pub fn reqwest_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Set Artifact Hub API key credentials for authenticated endpoints.
    pub fn api_key(mut self, id: impl Into<String>, secret: impl Into<String>) -> Self {
        self.api_key_id = Some(id.into());
        self.api_key_secret = Some(secret.into());
        self
    }

    /// Build the client and all resource namespaces from one shared inner state.
    pub fn build(self) -> ArtifactHubClient {
        let inner = Arc::new(ClientInner {
            client: self.client.unwrap_or_default(),
            base_url: self.base_url,
            api_key_id: self.api_key_id,
            api_key_secret: self.api_key_secret,
        });

        ArtifactHubClient { inner }
    }
}

impl ClientInner {
    pub(crate) fn full_url(&self, path: &str) -> String {
        let base = self.base_url.strip_suffix('/').unwrap_or(&self.base_url);
        let path = format!("/{}", path.trim_start_matches('/'));
        format!("{}{}", base, path)
    }

    pub(crate) async fn get(&self, path: &str, params: &[(String, String)]) -> Result<String> {
        let mut req = self.client.get(self.full_url(path));
        if let (Some(id), Some(secret)) = (&self.api_key_id, &self.api_key_secret) {
            req = req
                .header("X-API-KEY-ID", id)
                .header("X-API-KEY-SECRET", secret);
        }
        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req.send().await.map_err(ArtifactHubError::Request)?;

        let status = resp.status();
        let body = resp.text().await.map_err(ArtifactHubError::Body)?;

        if !status.is_success() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body)
                && let Some(msg) = json.get("message").and_then(|m| m.as_str())
            {
                return Err(ArtifactHubError::Api {
                    status,
                    message: msg.to_string(),
                });
            }
            return Err(ArtifactHubError::Api {
                status,
                message: body,
            });
        }

        Ok(body)
    }

    pub(crate) async fn get_json_with_context<T>(
        &self,
        path: &str,
        params: &[(String, String)],
        context: &'static str,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.get(path, params).await?;
        serde_json::from_str(&body).map_err(|e| ArtifactHubError::json(context, e))
    }
}

/// Builds an API path for a package given its kind, repository, and name.
pub fn package_url(kind: &str, repo: &str, name: &str, suffix: &str) -> String {
    format!(
        "/packages/{}/{}/{}{}",
        encode_path_segment(kind),
        encode_path_segment(repo),
        encode_path_segment(name),
        suffix
    )
}

pub(crate) fn encode_path_segment(segment: &str) -> String {
    let mut encoded = String::new();

    for byte in segment.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }

    encoded
}
