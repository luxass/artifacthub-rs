use std::sync::Arc;

use crate::client::{ClientInner, encode_path_segment, package_url};
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

    /// Get values.yaml for a Helm chart.
    pub async fn values(&self, params: &GetParams) -> Result<PackageValues, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;

        let package_id = json["package_id"]
            .as_str()
            .ok_or("No package_id found for this package")?;
        let version = json["version"]
            .as_str()
            .ok_or("No version found for this package")?
            .to_string();
        let values_path = format!(
            "/packages/{}/{}/values",
            encode_path_segment(package_id),
            encode_path_segment(&version)
        );
        let values = self.inner.get(&values_path, &[]).await?;

        Ok(PackageValues {
            package: params.name.clone(),
            version,
            values,
        })
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
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/templates");
        let json = self.inner.get_json(&path, &query_params).await?;
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
