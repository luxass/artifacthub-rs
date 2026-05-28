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
        let values = crate::endpoints::Packages::new(self.inner.clone())
            .values(package_id, &version)
            .await?;

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

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;
        let package_id = json["package_id"]
            .as_str()
            .ok_or("No package_id found for this package")?;
        let version = json["version"]
            .as_str()
            .ok_or("No version found for this package")?;
        let schema = crate::endpoints::Packages::new(self.inner.clone())
            .values_schema(package_id, version)
            .await?;

        Ok(ValuesSchema { schema })
    }

    /// List Kubernetes resources a chart creates.
    pub async fn templates(&self, params: &GetParams) -> Result<ChartTemplates, String> {
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
            .ok_or("No version found for this package")?;

        crate::endpoints::Packages::new(self.inner.clone())
            .templates(package_id, version)
            .await
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArtifactHubClient;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn values_schema_resolves_package_before_package_id_endpoint() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .and(query_param("version", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "version": "1.2.3"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.2.3/values-schema"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "type": "object"
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let schema = client
            .helm
            .values_schema(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.3".to_string()),
            })
            .await
            .unwrap();

        let schema = serde_json::to_value(schema.schema.unwrap()).unwrap();
        assert_eq!(schema["type"], "object");
    }

    #[tokio::test]
    async fn templates_resolves_package_before_package_id_endpoint() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "version": "1.2.3"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.2.3/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "templates/service.yaml",
                        "data": "YXBpVmVyc2lvbjogdjEKa2luZDogU2VydmljZQo="
                    }
                ],
                "values": {}
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let templates = client
            .helm
            .templates(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: None,
            })
            .await
            .unwrap();

        assert_eq!(templates.templates.len(), 1);
        assert_eq!(
            templates.templates[0].name.as_deref(),
            Some("templates/service.yaml")
        );
    }
}
