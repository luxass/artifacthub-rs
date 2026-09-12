//! Mock server lifecycle.

use wiremock::MockServer;

use crate::routes;

/// Faithful in-memory Artifact Hub.
pub struct HubMockServer {
    server: MockServer,
}

impl HubMockServer {
    /// Start a mock serving every route group.
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        routes::mount_package_routes(&server).await;
        routes::mount_changelog_routes(&server).await;
        routes::mount_snapshot_routes(&server).await;
        routes::mount_search_routes(&server).await;
        Self { server }
    }

    /// Base URL for a client under test.
    pub fn uri(&self) -> String {
        self.server.uri()
    }

    /// Requests received by the mock, for assertions about the wire contract.
    pub async fn received_requests(&self) -> Vec<wiremock::Request> {
        self.server
            .received_requests()
            .await
            .expect("request recording is enabled")
    }
}
