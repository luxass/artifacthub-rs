use std::sync::Arc;

use crate::client::{ClientInner, encode_path_segment, package_url};
use crate::models::{
    Changelog, ChangelogEntry, ChangelogMarkdown, ChartTemplates, PackageReadme, PackageSummary,
    PackageVersion, PackageVersions, SearchResponse, SecurityReport, StarHistoryEntry, StarStats,
    ValuesSchemaDocument,
};

/// Package search and lookup endpoints.
///
/// Access via `client.packages.*`.
#[derive(Clone)]
pub struct Packages {
    inner: Arc<ClientInner>,
}

impl Packages {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Search for packages using a parameter struct.
    pub async fn search_with(&self, params: &SearchParams) -> Result<SearchResponse, String> {
        let mut query_params: Vec<(String, String)> = vec![];

        if let Some(q) = &params.q {
            query_params.push(("ts_query_web".to_string(), q.clone()));
        }
        if let Some(kind) = &params.kind {
            query_params.push(("kind".to_string(), kind.clone()));
        }
        if let Some(repo) = &params.repo {
            query_params.push(("repo".to_string(), repo.clone()));
        }
        if let Some(org) = &params.org {
            query_params.push(("org".to_string(), org.clone()));
        }
        if let Some(limit) = params.limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = params.offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }

        let json = self
            .inner
            .get_json("/packages/search", &query_params)
            .await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get metadata summary for a package using a parameter struct.
    pub async fn get_with(&self, params: &GetParams) -> Result<PackageSummary, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get the README content for a package.
    pub async fn readme(&self, params: &GetParams) -> Result<PackageReadme, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref version) = params.version {
            query_params.push(("version".to_string(), version.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &query_params).await?;
        Ok(PackageReadme {
            readme: json["readme"].as_str().unwrap_or("").to_string(),
        })
    }

    /// List all available versions for a package.
    pub async fn versions(&self, params: &GetParams) -> Result<PackageVersions, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "");
        let json = self.inner.get_json(&path, &[]).await?;

        let versions: Vec<PackageVersion> =
            serde_json::from_value(json["available_versions"].clone())
                .map_err(|e| format!("Failed to parse versions: {}", e))?;

        Ok(PackageVersions {
            count: versions.len(),
            versions,
        })
    }

    /// Get changelog between versions (JSON).
    pub async fn changelog(&self, params: &ChangelogParams) -> Result<Changelog, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref from) = params.from {
            query_params.push(("from".to_string(), from.clone()));
        }
        if let Some(ref to) = params.to {
            query_params.push(("to".to_string(), to.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/changelog");
        let json = self.inner.get_json(&path, &query_params).await?;
        let entries: Vec<ChangelogEntry> =
            serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(Changelog { entries })
    }

    /// Get changelog between versions as markdown.
    pub async fn changelog_markdown(
        &self,
        params: &ChangelogParams,
    ) -> Result<ChangelogMarkdown, String> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(ref from) = params.from {
            query_params.push(("from".to_string(), from.clone()));
        }
        if let Some(ref to) = params.to {
            query_params.push(("to".to_string(), to.clone()));
        }

        let path = package_url(&params.kind, &params.repo, &params.name, "/changelog.md");
        let body = self.inner.get(&path, &query_params).await?;

        Ok(ChangelogMarkdown { changelog: body })
    }

    /// Get package star history using the package kind, repository, and name.
    pub async fn star_stats(&self, params: &GetParams) -> Result<StarStats, String> {
        let path = package_url(&params.kind, &params.repo, &params.name, "/stars");
        let json = self.inner.get_json(&path, &[]).await?;

        let stars: Vec<StarHistoryEntry> = serde_json::from_value(json)
            .map_err(|e| format!("Failed to parse star stats: {}", e))?;

        Ok(StarStats { stars })
    }

    /// Get chart values using the official package ID and version endpoint.
    pub async fn values(&self, package_id: &str, version: &str) -> Result<String, String> {
        let path = package_version_url(package_id, version, "/values");
        self.inner.get(&path, &[]).await
    }

    /// Get values schema using the official package ID and version endpoint.
    ///
    /// Artifact Hub returns the schema object directly. Some packages return a 200
    /// response with an empty body when no schema is available, so this method
    /// returns `None` for an empty body.
    pub async fn values_schema(
        &self,
        package_id: &str,
        version: &str,
    ) -> Result<Option<ValuesSchemaDocument>, String> {
        let path = package_version_url(package_id, version, "/values-schema");
        let body = self.inner.get(&path, &[]).await?;
        if body.trim().is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&body)
            .map(Some)
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get Helm chart templates using the official package ID and version endpoint.
    pub async fn templates(
        &self,
        package_id: &str,
        version: &str,
    ) -> Result<ChartTemplates, String> {
        let path = package_version_url(package_id, version, "/templates");
        let json = self.inner.get_json(&path, &[]).await?;
        serde_json::from_value(json).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get security report using the official package ID and version endpoint.
    ///
    /// Some packages return a 200 response with an empty body when no report is
    /// available, so this method returns `None` for an empty body.
    pub async fn security_report(
        &self,
        package_id: &str,
        version: &str,
    ) -> Result<Option<SecurityReport>, String> {
        let path = package_version_url(package_id, version, "/security-report");
        let body = self.inner.get(&path, &[]).await?;
        if body.trim().is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&body)
            .map(Some)
            .map_err(|e| format!("Failed to parse response: {}", e))
    }
}

fn package_version_url(package_id: &str, version: &str, suffix: &str) -> String {
    format!(
        "/packages/{}/{}{}",
        encode_path_segment(package_id),
        encode_path_segment(version),
        suffix
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArtifactHubClient;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn package_version_url_encodes_dynamic_segments() {
        assert_eq!(
            package_version_url("pkg/id", "1.0.0+build", "/templates"),
            "/packages/pkg%2Fid/1.0.0%2Bbuild/templates"
        );
    }

    #[tokio::test]
    async fn search_with_uses_artifact_hub_query_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/search"))
            .and(query_param("ts_query_web", "nginx"))
            .and(query_param("kind", "0"))
            .and(query_param("repo", "bitnami"))
            .and(query_param("org", "vmware"))
            .and(query_param("limit", "1"))
            .and(query_param("offset", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "packages": [
                    {
                        "package_id": "pkg-123",
                        "name": "nginx",
                        "normalized_name": "nginx",
                        "version": "1.0.0",
                        "description": "Nginx chart",
                        "repository": {
                            "name": "bitnami",
                            "url": "https://charts.bitnami.com/bitnami"
                        },
                        "deprecated": false,
                        "signed": false,
                        "stars": 10,
                        "ts": 123
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let response = client
            .packages
            .search_with(&SearchParams {
                q: Some("nginx".to_string()),
                kind: Some("0".to_string()),
                repo: Some("bitnami".to_string()),
                org: Some("vmware".to_string()),
                limit: Some(1),
                offset: Some(2),
            })
            .await
            .unwrap();

        assert_eq!(response.packages.len(), 1);
        assert_eq!(response.packages[0].package_id, "pkg-123");
    }

    #[tokio::test]
    async fn get_with_sends_version_query_param() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .and(query_param("version", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_package_json()))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let package = client
            .packages
            .get_with(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.3".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(package.package_id, "pkg-123");
        assert_eq!(package.version, "1.2.3");
    }

    #[tokio::test]
    async fn readme_extracts_readme_from_package_detail() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .and(query_param("version", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "package_id": "pkg-123",
                "name": "nginx",
                "readme": "# Nginx"
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let readme = client
            .packages
            .readme(&GetParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: Some("1.2.3".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(readme.readme, "# Nginx");
    }

    #[tokio::test]
    async fn changelog_wraps_artifact_hub_array_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/changelog"))
            .and(query_param("from", "1.0.0"))
            .and(query_param("to", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "version": "1.2.3",
                    "ts": 1700000000,
                    "changes": ["Fixed service ports"],
                    "prerelease": false
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let changelog = client
            .packages
            .changelog(&ChangelogParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                from: Some("1.0.0".to_string()),
                to: Some("1.2.3".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(changelog.entries.len(), 1);
        assert_eq!(changelog.entries[0].version, "1.2.3");
    }

    #[tokio::test]
    async fn changelog_markdown_returns_markdown_body() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx/changelog.md"))
            .and(query_param("from", "1.0.0"))
            .and(query_param("to", "1.2.3"))
            .respond_with(ResponseTemplate::new(200).set_body_string("# Changelog"))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let changelog = client
            .packages
            .changelog_markdown(&ChangelogParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                from: Some("1.0.0".to_string()),
                to: Some("1.2.3".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(changelog.changelog, "# Changelog");
    }

    #[tokio::test]
    async fn values_uses_package_id_version_endpoint() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/values"))
            .respond_with(ResponseTemplate::new(200).set_body_string("replicaCount: 2\n"))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let values = client.packages.values("pkg-123", "1.0.0").await.unwrap();

        assert_eq!(values, "replicaCount: 2\n");
    }

    #[tokio::test]
    async fn values_schema_returns_raw_schema_object() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/values-schema"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "replicaCount": { "type": "integer" }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let schema = client
            .packages
            .values_schema("pkg-123", "1.0.0")
            .await
            .unwrap()
            .unwrap();
        let schema = serde_json::to_value(schema).unwrap();

        assert_eq!(schema["type"], "object");
        assert!(schema.get("schema").is_none());
    }

    #[tokio::test]
    async fn values_schema_empty_body_returns_none() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/values-schema"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let schema = client
            .packages
            .values_schema("pkg-123", "1.0.0")
            .await
            .unwrap();

        assert!(schema.is_none());
    }

    #[tokio::test]
    async fn templates_preserves_values_and_decodes_template_data() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "templates": [
                    {
                        "name": "templates/service.yaml",
                        "data": "YXBpVmVyc2lvbjogdjEKa2luZDogU2VydmljZQo="
                    }
                ],
                "values": {
                    "service": {
                        "type": "ClusterIP"
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let templates = client.packages.templates("pkg-123", "1.0.0").await.unwrap();

        assert_eq!(templates.templates.len(), 1);
        assert_eq!(
            templates.templates[0].data.as_deref(),
            Some("apiVersion: v1\nkind: Service\n")
        );
        let values = serde_json::to_value(templates.values.unwrap()).unwrap();
        assert_eq!(values["service"]["type"].as_str(), Some("ClusterIP"));
    }

    #[tokio::test]
    async fn security_report_empty_body_returns_none() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let report = client
            .packages
            .security_report("pkg-123", "1.0.0")
            .await
            .unwrap();

        assert!(report.is_none());
    }

    #[tokio::test]
    async fn security_report_parses_non_empty_body() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/pkg-123/1.0.0/security-report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "summary": {
                    "critical": 1,
                    "high": 0,
                    "medium": 0,
                    "low": 0,
                    "unknown": 0
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ArtifactHubClient::with_base_url(mock_server.uri());
        let report = client
            .packages
            .security_report("pkg-123", "1.0.0")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(report.summary.unwrap().critical, Some(1));
    }

    fn sample_package_json() -> serde_json::Value {
        serde_json::json!({
            "package_id": "pkg-123",
            "name": "nginx",
            "normalized_name": "nginx",
            "version": "1.2.3",
            "description": "Nginx chart",
            "deprecated": false,
            "prerelease": false,
            "signed": false,
            "keywords": [],
            "ts": 123,
            "repository": {
                "name": "bitnami",
                "display_name": "Bitnami",
                "url": "https://charts.bitnami.com/bitnami",
                "kind": 0,
                "verified_publisher": true,
                "official": false
            },
            "stats": {
                "subscriptions": 0,
                "webhooks": 0
            },
            "links": [],
            "contains_security_updates": false
        })
    }
}

/// Parameters for searching packages.
#[derive(Debug, Default)]
pub struct SearchParams {
    pub q: Option<String>,
    pub kind: Option<String>,
    pub repo: Option<String>,
    pub org: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Parameters for getting package details.
#[derive(Debug)]
pub struct GetParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub version: Option<String>,
}

/// Parameters for getting changelog between versions.
#[derive(Debug)]
pub struct ChangelogParams {
    pub kind: String,
    pub repo: String,
    pub name: String,
    pub from: Option<String>,
    pub to: Option<String>,
}
