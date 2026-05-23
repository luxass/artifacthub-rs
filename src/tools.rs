mod get_package;
mod get_package_changelog;
mod get_package_readme;
mod get_package_star_stats;
mod get_package_values;
mod get_package_versions;
mod get_server_info;
mod search_packages;

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
}
