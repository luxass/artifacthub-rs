pub mod get_changelog_md;
pub mod get_package;
pub mod get_package_changelog;
pub mod get_package_readme;
pub mod get_package_security_report;
pub mod get_package_star_stats;
pub mod get_package_template;
pub mod get_package_template_data;
pub mod get_package_templates;
pub mod get_package_values;
pub mod get_package_values_schema;
pub mod get_package_versions;
pub mod get_server_info;
pub mod search_packages;
pub mod search_repositories;

use std::collections::HashSet;

use rmcp::handler::server::wrapper::Json;
use rmcp::{ServerHandler, handler::server::wrapper::Parameters, tool, tool_handler, tool_router};

use artifacthub_client::client::ArtifactHubClient;
use artifacthub_client::models::{
    Changelog, ChangelogMarkdown, PackageReadme, PackageSummary, PackageValues, PackageVersions,
    SearchRepositoriesResponse, SearchResponse, SecurityReport, StarStats, ValuesSchema,
};

/// Names of all available MCP tools exposed by this server.
pub const ALL_TOOL_NAMES: &[&str] = &[
    "get_server_info",
    "search_packages",
    "get_package",
    "get_package_readme",
    "get_package_versions",
    "get_package_changelog",
    "get_package_star_stats",
    "get_package_values",
    "search_repositories",
    "get_changelog_md",
    "get_package_security_report",
    "get_package_values_schema",
    "get_package_templates",
    "get_package_template",
    "get_package_template_data",
];

/// The MCP server that holds the HTTP client and tracks which tools are enabled.
#[derive(Clone)]
pub struct ArtifactHubServer {
    pub client: ArtifactHubClient,
    pub enabled_tools: HashSet<String>,
}

impl ArtifactHubServer {
    /// Checks if a tool is enabled by name.
    pub fn is_tool_enabled(&self, name: &str) -> bool {
        self.enabled_tools.contains(name)
    }
}

fn tool_disabled_error<T>(name: &str) -> Result<Json<T>, String> {
    Err(format!(
        "Tool '{}' is disabled. Start the server with --tools to enable it.",
        name
    ))
}

#[tool_router(router = tool_router, vis = "pub")]
impl ArtifactHubServer {
    #[tool(description = "Get basic information about this MCP server")]
    async fn get_server_info(
        &self,
        Parameters(p): Parameters<get_server_info::GetServerInfoParams>,
    ) -> Result<Json<get_server_info::ServerInfo>, String> {
        if !self.is_tool_enabled("get_server_info") {
            return tool_disabled_error("get_server_info");
        }
        get_server_info::handle_get_server_info(self, p).await
    }

    #[tool(
        description = "Search for packages in Artifact Hub. Results are ranked by popularity/stars."
    )]
    async fn search_packages(
        &self,
        Parameters(p): Parameters<search_packages::SearchParams>,
    ) -> Result<Json<SearchResponse>, String> {
        if !self.is_tool_enabled("search_packages") {
            return tool_disabled_error("search_packages");
        }
        search_packages::handle_search_packages(self, p).await
    }

    #[tool(
        description = "Get metadata summary for a package (name, version, description, repository, stats, keywords, links, containers, security). Does NOT include readme or available_versions."
    )]
    async fn get_package(
        &self,
        Parameters(p): Parameters<get_package::GetPackageParams>,
    ) -> Result<Json<PackageSummary>, String> {
        if !self.is_tool_enabled("get_package") {
            return tool_disabled_error("get_package");
        }
        get_package::handle_get_package(self, p).await
    }

    #[tool(
        description = "Get the README for a package (can be very large, 100KB+). Use sparingly - prefer get_package for metadata."
    )]
    async fn get_package_readme(
        &self,
        Parameters(p): Parameters<get_package_readme::GetPackageReadmeParams>,
    ) -> Result<Json<PackageReadme>, String> {
        if !self.is_tool_enabled("get_package_readme") {
            return tool_disabled_error("get_package_readme");
        }
        get_package_readme::handle_get_package_readme(self, p).await
    }

    #[tool(description = "List all available versions for a package")]
    async fn get_package_versions(
        &self,
        Parameters(p): Parameters<get_package_versions::GetPackageVersionsParams>,
    ) -> Result<Json<PackageVersions>, String> {
        if !self.is_tool_enabled("get_package_versions") {
            return tool_disabled_error("get_package_versions");
        }
        get_package_versions::handle_get_package_versions(self, p).await
    }

    #[tool(description = "Get changelog for a package between versions")]
    async fn get_package_changelog(
        &self,
        Parameters(p): Parameters<get_package_changelog::GetChangelogParams>,
    ) -> Result<Json<Changelog>, String> {
        if !self.is_tool_enabled("get_package_changelog") {
            return tool_disabled_error("get_package_changelog");
        }
        get_package_changelog::handle_get_package_changelog(self, p).await
    }

    #[tool(description = "Get star history for a package")]
    async fn get_package_star_stats(
        &self,
        Parameters(p): Parameters<get_package_star_stats::GetStarStatsParams>,
    ) -> Result<Json<StarStats>, String> {
        if !self.is_tool_enabled("get_package_star_stats") {
            return tool_disabled_error("get_package_star_stats");
        }
        get_package_star_stats::handle_get_package_star_stats(self, p).await
    }

    #[tool(description = "Get the default values.yaml for a Helm chart")]
    async fn get_package_values(
        &self,
        Parameters(p): Parameters<get_package_values::GetPackageValuesParams>,
    ) -> Result<Json<PackageValues>, String> {
        if !self.is_tool_enabled("get_package_values") {
            return tool_disabled_error("get_package_values");
        }
        get_package_values::handle_get_package_values(self, p).await
    }

    #[tool(description = "Search for repositories by name, kind, user, or organization")]
    async fn search_repositories(
        &self,
        Parameters(p): Parameters<search_repositories::SearchRepositoriesParams>,
    ) -> Result<Json<SearchRepositoriesResponse>, String> {
        if !self.is_tool_enabled("search_repositories") {
            return tool_disabled_error("search_repositories");
        }
        search_repositories::handle_search_repositories(self, p).await
    }

    #[tool(
        description = "Get changelog for a package as pre-formatted markdown (no JSON parsing needed)"
    )]
    async fn get_changelog_md(
        &self,
        Parameters(p): Parameters<get_changelog_md::GetChangelogMdParams>,
    ) -> Result<Json<ChangelogMarkdown>, String> {
        if !self.is_tool_enabled("get_changelog_md") {
            return tool_disabled_error("get_changelog_md");
        }
        get_changelog_md::handle_get_changelog_md(self, p).await
    }

    #[tool(
        description = "Get detailed security report with CVEs for a package. Requires package_id and version from get_package."
    )]
    async fn get_package_security_report(
        &self,
        Parameters(p): Parameters<get_package_security_report::GetSecurityReportParams>,
    ) -> Result<Json<SecurityReport>, String> {
        if !self.is_tool_enabled("get_package_security_report") {
            return tool_disabled_error("get_package_security_report");
        }
        get_package_security_report::handle_get_security_report(self, p).await
    }

    #[tool(
        description = "Get JSON schema for a Helm chart's values.yaml. Requires package_id and version from get_package."
    )]
    async fn get_package_values_schema(
        &self,
        Parameters(p): Parameters<get_package_values_schema::GetValuesSchemaParams>,
    ) -> Result<Json<ValuesSchema>, String> {
        if !self.is_tool_enabled("get_package_values_schema") {
            return tool_disabled_error("get_package_values_schema");
        }
        get_package_values_schema::handle_get_values_schema(self, p).await
    }

    #[tool(
        description = "List Helm chart template names and metadata without template source. Use get_package_template to fetch one decoded template. Requires package_id and version from get_package."
    )]
    async fn get_package_templates(
        &self,
        Parameters(p): Parameters<get_package_templates::GetTemplatesParams>,
    ) -> Result<Json<get_package_templates::TemplateList>, String> {
        if !self.is_tool_enabled("get_package_templates") {
            return tool_disabled_error("get_package_templates");
        }
        get_package_templates::handle_get_templates(self, p).await
    }

    #[tool(
        description = "Get one decoded Helm chart template by exact name. Use get_package_templates first to list template names. Requires package_id and version from get_package."
    )]
    async fn get_package_template(
        &self,
        Parameters(p): Parameters<get_package_template::GetTemplateParams>,
    ) -> Result<Json<get_package_template::Template>, String> {
        if !self.is_tool_enabled("get_package_template") {
            return tool_disabled_error("get_package_template");
        }
        get_package_template::handle_get_template(self, p).await
    }

    #[tool(
        description = "Get only the decoded Helm chart template source text by exact name. Use get_package_templates first to list template names. Requires package_id and version from get_package."
    )]
    async fn get_package_template_data(
        &self,
        Parameters(p): Parameters<get_package_template_data::GetTemplateDataParams>,
    ) -> Result<String, String> {
        if !self.is_tool_enabled("get_package_template_data") {
            return Err(
                "Tool 'get_package_template_data' is disabled. Start the server with --tools to enable it."
                    .to_string(),
            );
        }
        get_package_template_data::handle_get_template_data(self, p).await
    }
}

#[tool_handler]
impl ServerHandler for ArtifactHubServer {}
