use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use std::io::Read;

use crate::client::{ArtifactHubClient, build_url, package_url};
use crate::format::ts_to_date;
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

        let packages = json["packages"].as_array();
        let empty = vec![];
        let packages = packages.unwrap_or(&empty);
        if packages.is_empty() {
            return Ok("No packages found.".to_string());
        }

        let mut out = String::new();
        out.push_str(&format!("Found {} package(s):\n\n", packages.len()));

        for (i, pkg) in packages.iter().enumerate() {
            let name = pkg["name"].as_str().unwrap_or("?");
            let version = pkg["version"].as_str().unwrap_or("?");
            let desc = pkg["description"].as_str().unwrap_or("");
            let stars = pkg["stars"].as_i64().unwrap_or(0);
            let repo_name = pkg["repository"]["name"].as_str().unwrap_or("?");
            let repo_org = pkg["repository"]["organization_display_name"]
                .as_str()
                .unwrap_or("");
            let official = if pkg["official"].as_bool() == Some(true) {
                " [official]"
            } else {
                ""
            };

            out.push_str(&format!(
                "{}. **{}** ({}){}\n",
                i + 1,
                name,
                version,
                official
            ));
            if !desc.is_empty() {
                out.push_str(&format!("   {}\n", desc));
            }
            out.push_str(&format!(
                "   Repo: {} ({}) | Stars: {}\n\n",
                repo_name, repo_org, stars
            ));
        }

        Ok(out)
    }

    #[tool(description = "Get full details for a specific package including readme, versions, maintainers, and metadata")]
    async fn get_package(&self, Parameters(p): Parameters<GetPackageParams>) -> Result<String, String> {
        let mut params = vec![];
        if let Some(ref version) = p.version {
            params.push(("version".to_string(), version.clone()));
        }

        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, ""), &params);
        let json = self.client.get_json(&url).await?;

        let mut out = String::new();

        let name = json["name"].as_str().unwrap_or("?");
        let version = json["version"].as_str().unwrap_or("?");
        let app_version = json["app_version"].as_str().unwrap_or("");
        let desc = json["description"].as_str().unwrap_or("");
        let license = json["license"].as_str().unwrap_or("");
        let deprecated = json["deprecated"].as_bool() == Some(true);
        let ts = json["ts"].as_i64().unwrap_or(0);

        out.push_str(&format!("# {}@{}\n\n", name, version));
        if !app_version.is_empty() {
            out.push_str(&format!("App Version: {}\n", app_version));
        }
        if !license.is_empty() {
            out.push_str(&format!("License: {}\n", license));
        }
        if deprecated {
            out.push_str("**DEPRECATED**\n");
        }
        out.push_str(&format!("Updated: {}\n\n", ts_to_date(ts)));

        if !desc.is_empty() {
            out.push_str(&format!("{}\n\n", desc));
        }

        let repo_name = json["repository"]["display_name"]
            .as_str()
            .unwrap_or(&p.repo);
        let repo_url = json["repository"]["url"].as_str().unwrap_or("");
        let verified = json["repository"]["verified_publisher"].as_bool() == Some(true);
        out.push_str(&format!(
            "**Repository:** {}{}\n",
            repo_name,
            if verified { " (verified)" } else { "" }
        ));
        if !repo_url.is_empty() {
            out.push_str(&format!("URL: {}\n", repo_url));
        }
        out.push('\n');

        if let Some(maintainers) = json["maintainers"].as_array() {
            if !maintainers.is_empty() {
                out.push_str("**Maintainers:**\n");
                for m in maintainers {
                    let m_name = m["name"].as_str().unwrap_or("?");
                    let m_email = m["email"].as_str().unwrap_or("");
                    if !m_email.is_empty() {
                        out.push_str(&format!("- {} <{}>\n", m_name, m_email));
                    } else {
                        out.push_str(&format!("- {}\n", m_name));
                    }
                }
                out.push('\n');
            }
        }

        if let Some(versions) = json["available_versions"].as_array() {
            out.push_str(&format!(
                "**Available versions** ({} total):\n\n",
                versions.len()
            ));
            let show = versions.iter().take(10);
            for v in show {
                let ver = v["version"].as_str().unwrap_or("?");
                let v_ts = v["ts"].as_i64().unwrap_or(0);
                let pre = if v["prerelease"].as_bool() == Some(true) {
                    " (pre-release)"
                } else {
                    ""
                };
                out.push_str(&format!("- {} ({}){}\n", ver, ts_to_date(v_ts), pre));
            }
            if versions.len() > 10 {
                out.push_str(&format!("\n... and {} more", versions.len() - 10));
            }
            out.push('\n');
        }

        if let Some(readme) = json["readme"].as_str() {
            if !readme.is_empty() {
                out.push_str("---\n\n## Readme\n\n");
                out.push_str(readme);
            }
        }

        Ok(out)
    }

    #[tool(description = "List all available versions for a package")]
    async fn get_package_versions(
        &self,
        Parameters(p): Parameters<GetPackageVersionsParams>,
    ) -> Result<String, String> {
        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, ""), &[]);
        let json = self.client.get_json(&url).await?;

        let versions = json["available_versions"].as_array();
        let empty = vec![];
        let versions = versions.unwrap_or(&empty);

        if versions.is_empty() {
            let current = json["version"].as_str().unwrap_or("?");
            let current_ts = json["ts"].as_i64().unwrap_or(0);
            return Ok(format!(
                "**{}**\n\nOnly current version: {} ({})\n",
                json["name"].as_str().unwrap_or(&p.name),
                current,
                ts_to_date(current_ts)
            ));
        }

        let mut out = String::new();
        out.push_str(&format!(
            "# {} — {} versions\n\n",
            json["name"].as_str().unwrap_or(&p.name),
            versions.len()
        ));
        out.push_str("| Version | App Version | Date | Notes |\n");
        out.push_str("|---------|-------------|------|-------|\n");

        for v in versions {
            let ver = v["version"].as_str().unwrap_or("?");
            let app = v["app_version"].as_str().unwrap_or("");
            let v_ts = v["ts"].as_i64().unwrap_or(0);
            let pre = if v["prerelease"].as_bool() == Some(true) {
                " pre-release"
            } else {
                ""
            };
            let sec = if v["contains_security_updates"].as_bool() == Some(true) {
                " security-update"
            } else {
                ""
            };
            let notes = format!("{}{}", pre, sec);
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                ver,
                app,
                ts_to_date(v_ts),
                notes
            ));
        }

        Ok(out)
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

        if let Some(items) = json.as_array() {
            if items.is_empty() {
                return Ok(format!(
                    "No changelog available for {} ({} → {})",
                    p.name,
                    p.from.as_deref().unwrap_or("earliest"),
                    p.to.as_deref().unwrap_or("latest")
                ));
            }

            let mut out = String::new();
            out.push_str(&format!(
                "# Changelog: {} ({} → {})\n\n",
                p.name,
                p.from.as_deref().unwrap_or("earliest"),
                p.to.as_deref().unwrap_or("latest")
            ));

            let empty_arr = vec![];
            for item in items {
                let ver = item["version"].as_str().unwrap_or("?");
                let ts = item["ts"].as_i64().unwrap_or(0);
                let changes = item["changes"].as_array().unwrap_or(&empty_arr);

                out.push_str(&format!("## {} ({})\n\n", ver, ts_to_date(ts)));

                if changes.is_empty() {
                    out.push_str("No detailed changes recorded.\n\n");
                } else {
                    for change in changes {
                        let kind = change["kind"].as_str().unwrap_or("");
                        let desc = change["description"].as_str().unwrap_or("");
                        let links = change["links"].as_array().unwrap_or(&empty_arr);

                        let kind_label = match kind {
                            "added" => "Added",
                            "changed" => "Changed",
                            "deprecated" => "Deprecated",
                            "removed" => "Removed",
                            "fixed" => "Fixed",
                            "security" => "Security",
                            _ => kind,
                        };

                        out.push_str(&format!("- **{}**: {}", kind_label, desc));

                        if !links.is_empty() {
                            for link in links {
                                if let (Some(name), Some(url)) =
                                    (link["name"].as_str(), link["url"].as_str())
                                {
                                    out.push_str(&format!(" ([{}]({}))", name, url));
                                }
                            }
                        }
                        out.push('\n');
                    }
                    out.push('\n');
                }
            }

            return Ok(out);
        }

        Ok(json.to_string())
    }

    #[tool(description = "Get star history for a package")]
    async fn get_package_star_stats(
        &self,
        Parameters(p): Parameters<GetStarStatsParams>,
    ) -> Result<String, String> {
        let url = build_url(&package_url(&p.kind, &p.repo, &p.name, "/stars"), &[]);
        let json = self.client.get_json(&url).await?;

        if let Some(points) = json.as_array() {
            if points.is_empty() {
                return Ok(format!("No star history data for {}.", p.name));
            }

            let current = points.last().unwrap();
            let stars = current[1].as_i64().unwrap_or(0);
            let first = points.first().unwrap();
            let first_stars = first[1].as_i64().unwrap_or(0);
            let first_ts = first[0].as_i64().unwrap_or(0);
            let last_ts = current[0].as_i64().unwrap_or(0);

            let mut out = String::new();
            out.push_str(&format!("# Star History: {}\n\n", p.name));
            out.push_str(&format!(
                "- Current stars: {}\n",
                stars
            ));
            out.push_str(&format!(
                "- Growth: {} → {} (+{})\n",
                first_stars,
                stars,
                stars - first_stars
            ));
            out.push_str(&format!(
                "- Period: {} → {}\n\n",
                ts_to_date(first_ts),
                ts_to_date(last_ts)
            ));

            let milestones: Vec<_> = points
                .iter()
                .filter(|p| {
                    let s = p[1].as_i64().unwrap_or(0);
                    s % 100 == 0 || s == first_stars || s == stars
                })
                .collect();

            if !milestones.is_empty() {
                out.push_str("| Date | Stars |\n|------|-------|\n");
                for p in milestones {
                    let ts = p[0].as_i64().unwrap_or(0);
                    let s = p[1].as_i64().unwrap_or(0);
                    out.push_str(&format!("| {} | {} |\n", ts_to_date(ts), s));
                }
            }

            return Ok(out);
        }

        Ok(json.to_string())
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

                return Ok(format!(
                    "# values.yaml for {}@{}\n\n{}",
                    p.name, version, contents
                ));
            }
        }

        Err(format!("values.yaml not found in {}@{}", p.name, version))
    }
}
