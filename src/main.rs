mod client;
mod format;
mod kind;
mod tools;

use std::time::Duration;

use rmcp::ServiceExt;
use rmcp::transport::stdio;

use crate::client::ArtifactHubClient;
use crate::tools::ArtifactHubServer;

const USER_AGENT: &str = "artifacthub-mcp/0.1.0";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

    let server = ArtifactHubServer {
        client: ArtifactHubClient { client },
    }
    .serve(stdio())
    .await?;
    server.waiting().await?;
    Ok(())
}
