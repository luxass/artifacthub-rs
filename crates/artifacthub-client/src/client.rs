const DEFAULT_API_BASE: &str = "https://artifacthub.io/api/v1";

/// HTTP client for making requests to the Artifact Hub API.
#[derive(Clone)]
pub struct ArtifactHubClient {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl Default for ArtifactHubClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: DEFAULT_API_BASE.to_string(),
        }
    }
}

/// Builds an API path for a package given its kind, repository, and name.
pub fn package_url(kind: &str, repo: &str, name: &str, suffix: &str) -> String {
    format!("/packages/{}/{}/{}{}", kind, repo, name, suffix)
}

impl ArtifactHubClient {
    fn full_url(&self, path: &str) -> String {
        let base = self.base_url.strip_suffix('/').unwrap_or(&self.base_url);
        format!("{}{}", base, path)
    }

    /// Sends a GET request and returns the response body as a string.
    pub async fn get(&self, path: &str, params: &[(String, String)]) -> Result<String, String> {
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

    /// Sends a GET request and parses the response as JSON.
    pub async fn get_json(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value, String> {
        let body = self.get(path, params).await?;
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Sends a GET request and returns the raw response bytes (for tarball downloads).
    pub async fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn client_with_base(base: &str) -> ArtifactHubClient {
        ArtifactHubClient {
            client: reqwest::Client::new(),
            base_url: base.to_string(),
        }
    }

    #[test]
    fn test_full_url_no_trailing_slash() {
        let client = client_with_base("https://example.com/api/v1");
        assert_eq!(
            client.full_url("/packages/helm/repo/pkg"),
            "https://example.com/api/v1/packages/helm/repo/pkg"
        );
    }

    #[test]
    fn test_full_url_trailing_slash_stripped() {
        let client = client_with_base("https://example.com/api/v1/");
        let url = client.full_url("/packages/helm/repo/pkg");
        assert_eq!(url, "https://example.com/api/v1/packages/helm/repo/pkg");
        assert!(!url.contains("//packages"));
    }
}
