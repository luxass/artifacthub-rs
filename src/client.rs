const DEFAULT_API_BASE: &str = "https://artifacthub.io/api/v1";

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

pub fn package_url(kind: &str, repo: &str, name: &str, suffix: &str) -> String {
    format!("/packages/{}/{}/{}{}", kind, repo, name, suffix)
}

impl ArtifactHubClient {
    pub fn build_url(&self, path: &str, params: &[(String, String)]) -> String {
        let base = self.base_url.strip_suffix('/').unwrap_or(&self.base_url);
        if params.is_empty() {
            return format!("{}{}", base, path);
        }
        let encoded: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();
        format!("{}{}?{}", base, path, encoded.join("&"))
    }

    pub async fn get(&self, url: &str) -> Result<String, String> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| e.to_string())?;

        if !status.is_success() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                    return Err(format!("API error {}: {}", status, msg));
                }
            }
            return Err(format!("API error {}: {}", status, body));
        }

        Ok(body)
    }

    pub async fn get_json(&self, url: &str) -> Result<serde_json::Value, String> {
        let body = self.get(url).await?;
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse response: {}", e))
    }

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

        let bytes = resp.bytes().await.map_err(|e| format!("Failed to read response: {}", e))?;
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
    fn test_build_url_no_trailing_slash() {
        let client = client_with_base("https://example.com/api/v1");
        let url = client.build_url("/packages/helm/repo/pkg", &[]);
        assert_eq!(url, "https://example.com/api/v1/packages/helm/repo/pkg");
    }

    #[test]
    fn test_build_url_trailing_slash_stripped() {
        let client = client_with_base("https://example.com/api/v1/");
        let url = client.build_url("/packages/helm/repo/pkg", &[]);
        assert_eq!(url, "https://example.com/api/v1/packages/helm/repo/pkg");
        assert!(!url.contains("//packages"));
    }

    #[test]
    fn test_build_url_with_params() {
        let client = client_with_base("https://example.com/api/v1/");
        let url = client.build_url("/packages/search", &[("q".to_string(), "nginx".to_string())]);
        assert_eq!(url, "https://example.com/api/v1/packages/search?q=nginx");
        assert!(!url.contains("//packages"));
    }
}
