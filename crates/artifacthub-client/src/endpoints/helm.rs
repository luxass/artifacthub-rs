use std::io::Read;
use std::sync::Arc;

use crate::client::{ClientInner, package_url};
use crate::models::{ChartTemplates, PackageValues, ValuesSchema};

/// Helm chart specific endpoints (values, schema, templates).
///
/// Access via `client.helm.*`.
#[derive(Clone)]
pub struct Helm {
    inner: Arc<ClientInner>,
}

impl Helm {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Extract values.yaml from a Helm chart.
    pub async fn values(&self, params: &GetParams) -> Result<PackageValues, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;

        let content_url = json["content_url"].as_str().ok_or(
            "No content_url found for this package. Values are only available for Helm charts.",
        )?;

        let version = json["version"].as_str().unwrap_or("unknown").to_string();

        let tarball = self.inner.get_bytes(content_url).await?;

        let decoder = flate2::read::GzDecoder::new(&tarball[..]);
        let mut archive = tar::Archive::new(decoder);

        for entry in archive
            .entries()
            .map_err(|e| format!("Failed to read tarball: {}", e))?
        {
            let mut entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry
                .path()
                .map_err(|e| format!("Failed to get entry path: {}", e))?;

            if path.ends_with("values.yaml") && path.components().count() == 2 {
                let mut contents = String::new();
                entry
                    .read_to_string(&mut contents)
                    .map_err(|e| format!("Failed to read values.yaml: {}", e))?;

                return Ok(PackageValues {
                    package: params.name.clone(),
                    version,
                    values: contents,
                });
            }
        }

        Err(format!(
            "values.yaml not found in {}@{}",
            params.name, version
        ))
    }

    /// Get JSON schema for Helm chart values.
    pub async fn values_schema(&self, params: &GetParams) -> Result<ValuesSchema, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/values-schema");
        let json = self.inner.get_json(&path, &query_params).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// List Kubernetes resources a chart creates.
    pub async fn templates(&self, params: &GetParams) -> Result<ChartTemplates, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "/templates");
        let json = self.inner.get_json(&path, &[]).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }
}

/// Parameters for Helm chart operations.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub version: Option<String>,
}
