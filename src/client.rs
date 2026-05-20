const API_BASE: &str = "https://artifacthub.io/api/v1";

#[derive(Clone)]
pub struct ArtifactHubClient {
    pub client: reqwest::Client,
}

pub fn build_url(path: &str, params: &[(String, String)]) -> String {
    if params.is_empty() {
        return format!("{}{}", API_BASE, path);
    }
    let encoded: Vec<String> = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect();
    format!("{}{}?{}", API_BASE, path, encoded.join("&"))
}

pub fn package_url(kind: &str, repo: &str, name: &str, suffix: &str) -> String {
    format!("/packages/{}/{}/{}{}", kind, repo, name, suffix)
}

impl ArtifactHubClient {
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
