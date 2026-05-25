use crate::endpoints::{Helm, Packages, Repositories, Security, Stats};
use reqwest::Client;
use std::sync::Arc;

const DEFAULT_API_BASE: &str = "https://artifacthub.io/api/v1";

/// HTTP client for making requests to the Artifact Hub API.
#[derive(Clone)]
pub struct ArtifactHubClient {
    inner: Arc<ClientInner>,
    pub packages: Packages,
    pub repositories: Repositories,
    pub helm: Helm,
    pub stats: Stats,
    pub security: Security,
}

pub(crate) struct ClientInner {
    client: Client,
    base_url: String,
}

/// Builder for configuring an [`ArtifactHubClient`].
pub struct ArtifactHubClientBuilder {
    client: Option<Client>,
    base_url: String,
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

    /// Sends a GET request and returns the response body as a string.
    pub async fn get(&self, path: &str, params: &[(String, String)]) -> Result<String, String> {
        self.inner.get(path, params).await
    }

    /// Sends a GET request and parses the response as JSON.
    pub async fn get_json(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value, String> {
        self.inner.get_json(path, params).await
    }

    /// Sends a GET request and returns the raw response bytes (for tarball downloads).
    pub async fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        self.inner.get_bytes(url).await
    }
}

impl Default for ArtifactHubClientBuilder {
    fn default() -> Self {
        Self {
            client: None,
            base_url: DEFAULT_API_BASE.to_string(),
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

    /// Build the client and all resource namespaces from one shared inner state.
    pub fn build(self) -> ArtifactHubClient {
        let inner = Arc::new(ClientInner {
            client: self.client.unwrap_or_default(),
            base_url: self.base_url,
        });

        ArtifactHubClient {
            inner: inner.clone(),
            packages: Packages::new(inner.clone()),
            repositories: Repositories::new(inner.clone()),
            helm: Helm::new(inner.clone()),
            stats: Stats::new(inner.clone()),
            security: Security::new(inner),
        }
    }
}

impl ClientInner {
    pub(crate) fn full_url(&self, path: &str) -> String {
        let base = self.base_url.strip_suffix('/').unwrap_or(&self.base_url);
        let path = format!("/{}", path.trim_start_matches('/'));
        format!("{}{}", base, path)
    }

    pub(crate) async fn get(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<String, String> {
        let mut req = self.client.get(self.full_url(path));
        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| e.to_string())?;

        if !status.is_success() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body)
                && let Some(msg) = json.get("message").and_then(|m| m.as_str())
            {
                return Err(format!("API error {}: {}", status, msg));
            }
            return Err(format!("API error {}: {}", status, body));
        }

        Ok(body)
    }

    pub(crate) async fn get_json(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value, String> {
        let body = self.get(path, params).await?;
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse response: {}", e))
    }

    pub(crate) async fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(format!("Download error {}: {}", status, status));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(bytes.to_vec())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn client_with_base(base: &str) -> ArtifactHubClient {
        ArtifactHubClient::with_base_url(base)
    }

    #[test]
    fn test_full_url_no_trailing_slash() {
        let client = client_with_base("https://example.com/api/v1");
        assert_eq!(
            client.inner.full_url("/packages/helm/repo/pkg"),
            "https://example.com/api/v1/packages/helm/repo/pkg"
        );
    }

    #[test]
    fn test_full_url_trailing_slash_stripped() {
        let client = client_with_base("https://example.com/api/v1/");
        let url = client.inner.full_url("/packages/helm/repo/pkg");
        assert_eq!(url, "https://example.com/api/v1/packages/helm/repo/pkg");
        assert!(!url.contains("//packages"));
    }

    #[test]
    fn test_full_url_adds_missing_leading_slash() {
        let client = client_with_base("https://example.com/api/v1");
        assert_eq!(
            client.inner.full_url("packages/search"),
            "https://example.com/api/v1/packages/search"
        );
    }

    #[test]
    fn test_full_url_collapses_extra_leading_slashes() {
        let client = client_with_base("https://example.com/api/v1/");
        assert_eq!(
            client.inner.full_url("///packages/search"),
            "https://example.com/api/v1/packages/search"
        );
    }

    #[test]
    fn test_package_url_encodes_dynamic_segments() {
        assert_eq!(
            package_url("helm", "repo/name", "pkg?name", "/readme"),
            "/packages/helm/repo%2Fname/pkg%3Fname/readme"
        );
    }
}
