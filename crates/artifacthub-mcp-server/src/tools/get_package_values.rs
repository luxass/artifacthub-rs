use artifacthub_client::models::PackageValues;
use rmcp::handler::server::wrapper::Json;
use schemars::JsonSchema;
use std::io::Read;

use crate::tools::ArtifactHubServer;
use artifacthub_client::client::package_url;
use artifacthub_client::kind::KIND_DESCRIPTION;

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct GetPackageValuesParams {
    #[schemars(description = KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Specific version (defaults to latest)")]
    pub version: Option<String>,
}

pub async fn handle_get_package_values(
    server: &ArtifactHubServer,
    params: GetPackageValuesParams,
) -> Result<Json<PackageValues>, String> {
    let mut query_params: Vec<(String, String)> = vec![];
    if let Some(ref version) = params.version {
        query_params.push(("version".to_string(), version.clone()));
    }

    let path = package_url(&params.kind, &params.repo, &params.name, "");
    let json = server.client.get_json(&path, &query_params).await?;

    let content_url = json["content_url"].as_str().ok_or(
        "No content_url found for this package. Values are only available for Helm charts.",
    )?;

    let version = json["version"].as_str().unwrap_or("unknown").to_string();

    let tarball = server.client.get_bytes(content_url).await?;

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

            return Ok(Json(PackageValues {
                package: params.name,
                version,
                values: contents,
            }));
        }
    }

    Err(format!(
        "values.yaml not found in {}@{}",
        params.name, version
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ALL_TOOL_NAMES;
    use artifacthub_client::client::ArtifactHubClient;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::collections::HashSet;
    use tar::Builder;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_server(base_url: &str) -> ArtifactHubServer {
        ArtifactHubServer {
            client: ArtifactHubClient {
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            },
            enabled_tools: ALL_TOOL_NAMES
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>(),
        }
    }

    fn create_test_tarball(values_content: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let encoder = GzEncoder::new(&mut buf, Compression::default());
        let mut builder = Builder::new(encoder);

        let mut header = tar::Header::new_gnu();
        header.set_path("test-chart/values.yaml").unwrap();
        header.set_size(values_content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append(&header, values_content.as_bytes()).unwrap();

        builder.finish().unwrap();
        drop(builder);
        buf
    }

    #[tokio::test]
    async fn test_get_package_values_returns_values_yaml() {
        let mock_server = MockServer::start().await;
        let tarball = create_test_tarball("replicaCount: 3\nimage:\n  repository: nginx\n");

        Mock::given(method("GET"))
            .and(path("/packages/helm/bitnami/nginx"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "nginx",
                "version": "1.0.0",
                "content_url": format!("{}/chart.tgz", mock_server.uri())
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/chart.tgz"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(tarball))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_values(
            &server,
            GetPackageValuesParams {
                kind: "helm".to_string(),
                repo: "bitnami".to_string(),
                name: "nginx".to_string(),
                version: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.0.package, "nginx");
        assert_eq!(result.0.version, "1.0.0");
        assert!(result.0.values.contains("replicaCount: 3"));
    }

    #[tokio::test]
    async fn test_get_package_values_no_content_url() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/packages/falco/falcosecurity/falco"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "falco",
                "version": "1.0.0"
            })))
            .mount(&mock_server)
            .await;

        let server = test_server(&mock_server.uri());
        let result = handle_get_package_values(
            &server,
            GetPackageValuesParams {
                kind: "falco".to_string(),
                repo: "falcosecurity".to_string(),
                name: "falco".to_string(),
                version: None,
            },
        )
        .await;

        assert!(result.is_err());
        let Err(err) = result else {
            panic!("expected error")
        };
        assert!(err.contains("No content_url"));
    }
}
