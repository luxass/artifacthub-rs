mod get_changelog_md;
mod get_package;
mod get_package_changelog;
mod get_package_readme;
mod get_package_security_report;
mod get_package_star_stats;
mod get_package_templates;
mod get_package_values;
mod get_package_values_schema;
mod get_package_versions;
mod get_server_info;
mod search_packages;
mod search_repositories;

use rmcp::handler::server::wrapper::Json;
use rmcp::{handler::server::wrapper::Parameters, tool, tool_router};

use crate::client::ArtifactHubClient;

#[derive(Clone)]
pub struct ArtifactHubServer {
    pub client: ArtifactHubClient,
}

#[tool_router(server_handler)]
impl ArtifactHubServer {
    #[tool(description = "Get basic information about this MCP server")]
    async fn get_server_info(
        &self,
        Parameters(p): Parameters<get_server_info::GetServerInfoParams>,
    ) -> Result<Json<get_server_info::ServerInfo>, String> {
        get_server_info::handle_get_server_info(self, p).await
    }

    #[tool(
        description = "Search for packages in Artifact Hub. Results are ranked by popularity/stars."
    )]
    async fn search_packages(
        &self,
        Parameters(p): Parameters<search_packages::SearchParams>,
    ) -> Result<Json<search_packages::SearchResponse>, String> {
        search_packages::handle_search_packages(self, p).await
    }

    #[tool(
        description = "Get metadata summary for a package (name, version, description, repository, stats, keywords, links, containers, security). Does NOT include readme or available_versions."
    )]
    async fn get_package(
        &self,
        Parameters(p): Parameters<get_package::GetPackageParams>,
    ) -> Result<Json<get_package::PackageSummary>, String> {
        get_package::handle_get_package(self, p).await
    }

    #[tool(
        description = "Get the README for a package (can be very large, 100KB+). Use sparingly - prefer get_package for metadata."
    )]
    async fn get_package_readme(
        &self,
        Parameters(p): Parameters<get_package_readme::GetPackageReadmeParams>,
    ) -> Result<Json<get_package_readme::PackageReadme>, String> {
        get_package_readme::handle_get_package_readme(self, p).await
    }

    #[tool(description = "List all available versions for a package")]
    async fn get_package_versions(
        &self,
        Parameters(p): Parameters<get_package_versions::GetPackageVersionsParams>,
    ) -> Result<Json<get_package_versions::PackageVersions>, String> {
        get_package_versions::handle_get_package_versions(self, p).await
    }

    #[tool(description = "Get changelog for a package between versions")]
    async fn get_package_changelog(
        &self,
        Parameters(p): Parameters<get_package_changelog::GetChangelogParams>,
    ) -> Result<Json<get_package_changelog::Changelog>, String> {
        get_package_changelog::handle_get_package_changelog(self, p).await
    }

    #[tool(description = "Get star history for a package")]
    async fn get_package_star_stats(
        &self,
        Parameters(p): Parameters<get_package_star_stats::GetStarStatsParams>,
    ) -> Result<Json<Vec<get_package_star_stats::StarHistoryEntry>>, String> {
        get_package_star_stats::handle_get_package_star_stats(self, p).await
    }

    #[tool(description = "Get the default values.yaml for a Helm chart")]
    async fn get_package_values(
        &self,
        Parameters(p): Parameters<get_package_values::GetPackageValuesParams>,
    ) -> Result<Json<get_package_values::PackageValues>, String> {
        get_package_values::handle_get_package_values(self, p).await
    }

    #[tool(
        description = "Search for repositories by name, kind, user, or organization"
    )]
    async fn search_repositories(
        &self,
        Parameters(p): Parameters<search_repositories::SearchRepositoriesParams>,
    ) -> Result<Json<search_repositories::SearchRepositoriesResponse>, String> {
        search_repositories::handle_search_repositories(self, p).await
    }

    #[tool(
        description = "Get changelog for a package as pre-formatted markdown (no JSON parsing needed)"
    )]
    async fn get_changelog_md(
        &self,
        Parameters(p): Parameters<get_changelog_md::GetChangelogMdParams>,
    ) -> Result<Json<get_changelog_md::ChangelogMarkdown>, String> {
        get_changelog_md::handle_get_changelog_md(self, p).await
    }

    #[tool(
        description = "Get detailed security report with CVEs for a package. Requires package_id (UUID) from get_package."
    )]
    async fn get_package_security_report(
        &self,
        Parameters(p): Parameters<get_package_security_report::GetSecurityReportParams>,
    ) -> Result<Json<get_package_security_report::SecurityReport>, String> {
        get_package_security_report::handle_get_security_report(self, p).await
    }

    #[tool(
        description = "Get JSON schema for a Helm chart's values.yaml. Requires package_id (UUID) from get_package."
    )]
    async fn get_package_values_schema(
        &self,
        Parameters(p): Parameters<get_package_values_schema::GetValuesSchemaParams>,
    ) -> Result<Json<get_package_values_schema::ValuesSchema>, String> {
        get_package_values_schema::handle_get_values_schema(self, p).await
    }

    #[tool(
        description = "Get list of Kubernetes resources (templates) a Helm chart creates. Requires package_id (UUID) from get_package."
    )]
    async fn get_package_templates(
        &self,
        Parameters(p): Parameters<get_package_templates::GetTemplatesParams>,
    ) -> Result<Json<get_package_templates::ChartTemplates>, String> {
        get_package_templates::handle_get_templates(self, p).await
    }
}
