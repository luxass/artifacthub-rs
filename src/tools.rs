use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use std::io::Read;

use crate::client::{ArtifactHubClient, build_url, package_url};
use crate::kind::{self as pkg_kind};

#[derive(Clone)]
pub struct ArtifactHubServer {
    pub client: ArtifactHubClient,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    #[schemars(description = "Search query string")]
    pub q: Option<String>,
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: Option<String>,
    #[schemars(description = "Filter by repository name")]
    pub repo: Option<String>,
    #[schemars(description = "Filter by organization name")]
    pub org: Option<String>,
    #[schemars(description = "Number of results (max 60)")]
    pub limit: Option<usize>,
    #[schemars(description = "Offset for pagination")]
    pub offset: Option<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPackageParams {
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Specific version (defaults to latest)")]
    pub version: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPackageVersionsParams {
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetChangelogParams {
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Target version (defaults to latest)")]
    pub to: Option<String>,
    #[schemars(description = "Source version")]
    pub from: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetStarStatsParams {
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPackageValuesParams {
    #[schemars(description = pkg_kind::KIND_DESCRIPTION)]
    pub kind: String,
    #[schemars(description = "Repository name")]
    pub repo: String,
    #[schemars(description = "Package name")]
    pub name: String,
    #[schemars(description = "Specific version (defaults to latest)")]
    pub version: Option<String>,
}

#[tool_router(server_handler)]
impl ArtifactHubServer {
    #[tool(description = "Search for packages in Artifact Hub. Results are ranked by popularity/stars.")]
    async fn search_packages(&self, Parameters(p): Parameters<SearchParams>) -> Result<String, String> {
        let mut params = vec![];

        if let Some(q) = &p.q {
            params.push(("q".to_string(), q.clone()));
        }
        if let Some(kind) = &p.kind {
            if let Some(id) = pkg_kind::to_id(kind) {
                params.push(("kind".to_string(), id.to_string()));
            } else {
                return Err(format!(
                    "Unknown kind: '{}'. Valid kinds: {}",
                    kind,
                    pkg_kind::valid_kinds().join(", ")
                ));
            }
        }
        if let Some(repo) = &p.repo {
            params.push(("repo".to_string(), repo.clone()));
        }
        if let Some(org) = &p.org {
            params.push(("org".to_string(), org.clone()));
        }
        if let Some(limit) = p.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = p.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        let url = build_url("/packages/search", &params);
        let json = self.client.get_json(&url).await?;

        serde_json::to_string(&json).map_err(|e| format!("Failed to serialize response: {}", e))
    }

    #[tool(description = "Get full details for a specific package including readme, versions, maintainers, and metadata")]
    async fn get_package(&self, Parameters(p): Parameters<GetPackageParams>) -> Result<String, String> {
        let mut params = vec![];
        if let Some(ref version) = p.version {
            params.push(("version".to_string(), version.clone()));
        }

        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, ""), &params);
        let json = self.client.get_json(&url).await?;

        serde_json::to_string(&json).map_err(|e| format!("Failed to serialize response: {}", e))
    }

    #[tool(description = "List all available versions for a package")]
    async fn get_package_versions(
        &self,
        Parameters(p): Parameters<GetPackageVersionsParams>,
    ) -> Result<String, String> {
        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, ""), &[]);
        let json = self.client.get_json(&url).await?;

        serde_json::to_string(&json).map_err(|e| format!("Failed to serialize response: {}", e))
    }

    #[tool(description = "Get changelog for a package between versions")]
    async fn get_package_changelog(
        &self,
        Parameters(p): Parameters<GetChangelogParams>,
    ) -> Result<String, String> {
        let mut params = vec![];
        if let Some(ref to) = p.to {
            params.push(("to".to_string(), to.clone()));
        }
        if let Some(ref from) = p.from {
            params.push(("from".to_string(), from.clone()));
        }

        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, "/changelog"), &params);
        let json = self.client.get_json(&url).await?;

        serde_json::to_string(&json).map_err(|e| format!("Failed to serialize response: {}", e))
    }

    #[tool(description = "Get star history for a package")]
    async fn get_package_star_stats(
        &self,
        Parameters(p): Parameters<GetStarStatsParams>,
    ) -> Result<String, String> {
        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, "/stars"), &[]);
        let json = self.client.get_json(&url).await?;

        serde_json::to_string(&json).map_err(|e| format!("Failed to serialize response: {}", e))
    }

    #[tool(description = "Get the default values.yaml for a Helm chart")]
    async fn get_package_values(
        &self,
        Parameters(p): Parameters<GetPackageValuesParams>,
    ) -> Result<String, String> {
        let mut params = vec![];
        if let Some(ref version) = p.version {
            params.push(("version".to_string(), version.clone()));
        }

        let pkg_url = build_url(&package_url(&p.kind, &p.repo, &p.name, ""), &params);
        let json = self.client.get_json(&pkg_url).await?;

        let content_url = json["content_url"]
            .as_str()
            .ok_or("No content_url found for this package. Values are only available for Helm charts.")?;

        let version = json["version"].as_str().unwrap_or("unknown");

        let tarball = self.client.get_bytes(content_url).await?;

        let decoder = flate2::read::GzDecoder::new(&tarball[..]);
        let mut archive = tar::Archive::new(decoder);

        for entry in archive.entries().map_err(|e| format!("Failed to read tarball: {}", e))? {
            let mut entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry
                .path()
                .map_err(|e| format!("Failed to get entry path: {}", e))?;

            if path.ends_with("values.yaml") {
                let mut contents = String::new();
                entry
                    .read_to_string(&mut contents)
                    .map_err(|e| format!("Failed to read values.yaml: {}", e))?;

                let result = serde_json::json!({
                    "package": p.name,
                    "version": version,
                    "values": contents
                });
                return serde_json::to_string(&result).map_err(|e| format!("Failed to serialize response: {}", e));
            }
        }

        Err(format!("values.yaml not found in {}@{}", p.name, version))
    }
}
