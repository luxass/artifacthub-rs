mod tools;

use std::collections::HashSet;
use std::time::Duration;

use artifacthub_client::client::ArtifactHubClient;
use clap::Parser;
use rmcp::ServiceExt;
use rmcp::handler::server::router::Router;
use rmcp::transport::stdio;
use tools::{ALL_TOOL_NAMES, ArtifactHubServer};

const USER_AGENT: &str = concat!(
    "artifacthub-mcp/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/luxass/artifacthub-mcp)"
);

#[derive(Parser, Debug)]
#[command(
    name = "artifacthub-mcp",
    version,
    about = "MCP server for Artifact Hub"
)]
struct Args {
    /// Comma-separated list of tools to enable. If set, only these tools are available.
    /// Cannot be used together with --exclude-tools.
    #[arg(long, value_delimiter = ',')]
    tools: Option<Vec<String>>,

    /// Comma-separated list of tools to exclude from the default set.
    /// Cannot be used together with --tools.
    #[arg(long, value_delimiter = ',')]
    exclude_tools: Option<Vec<String>>,
}

fn resolve_enabled_tools(args: &Args) -> Result<HashSet<String>, String> {
    if args.tools.is_some() && args.exclude_tools.is_some() {
        return Err("--tools and --exclude-tools cannot be used together".to_string());
    }

    if let Some(ref tool_list) = args.tools {
        let set: HashSet<String> = tool_list.iter().map(|s| s.trim().to_string()).collect();
        let all: HashSet<&str> = ALL_TOOL_NAMES.iter().copied().collect();
        for t in &set {
            if !all.contains(t.as_str()) {
                return Err(format!(
                    "Unknown tool '{}'. Available tools: {}",
                    t,
                    ALL_TOOL_NAMES.join(", ")
                ));
            }
        }
        return Ok(set);
    }

    if let Some(ref exclude_list) = args.exclude_tools {
        let exclude: HashSet<String> = exclude_list.iter().map(|s| s.trim().to_string()).collect();
        let all: HashSet<&str> = ALL_TOOL_NAMES.iter().copied().collect();
        for t in &exclude {
            if !all.contains(t.as_str()) {
                return Err(format!(
                    "Unknown tool '{}'. Available tools: {}",
                    t,
                    ALL_TOOL_NAMES.join(", ")
                ));
            }
        }
        return Ok(all
            .into_iter()
            .filter(|t| !exclude.contains(*t))
            .map(String::from)
            .collect());
    }

    Ok(ALL_TOOL_NAMES.iter().map(|s| s.to_string()).collect())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let enabled_tools = resolve_enabled_tools(&args).map_err(|e| e.to_string())?;

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let server = ArtifactHubServer {
        client: ArtifactHubClient {
            client,
            ..Default::default()
        },
        enabled_tools,
    };

    // Router::new() creates an empty ToolRouter; merge in the generated one
    let mut router = Router::new(server).with_tools(ArtifactHubServer::tool_router());
    for name in ALL_TOOL_NAMES {
        if !router.service.enabled_tools.contains(*name) {
            router.tool_router.disable_route(*name);
        }
    }

    let running = router.serve(stdio()).await?;
    running.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_all_tools_enabled() {
        let args = Args {
            tools: None,
            exclude_tools: None,
        };
        let result = resolve_enabled_tools(&args).unwrap();
        assert_eq!(result.len(), ALL_TOOL_NAMES.len());
        for name in ALL_TOOL_NAMES {
            assert!(result.contains(*name));
        }
    }

    #[test]
    fn test_tools_whitelist() {
        let args = Args {
            tools: Some(vec![
                "search_packages".to_string(),
                "get_package".to_string(),
            ]),
            exclude_tools: None,
        };
        let result = resolve_enabled_tools(&args).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains("search_packages"));
        assert!(result.contains("get_package"));
    }

    #[test]
    fn test_exclude_tools() {
        let args = Args {
            tools: None,
            exclude_tools: Some(vec!["get_server_info".to_string()]),
        };
        let result = resolve_enabled_tools(&args).unwrap();
        assert_eq!(result.len(), ALL_TOOL_NAMES.len() - 1);
        assert!(!result.contains("get_server_info"));
        assert!(result.contains("search_packages"));
    }

    #[test]
    fn test_tools_and_exclude_tools_mutually_exclusive() {
        let args = Args {
            tools: Some(vec!["search_packages".to_string()]),
            exclude_tools: Some(vec!["get_package".to_string()]),
        };
        let result = resolve_enabled_tools(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be used together"));
    }

    #[test]
    fn test_unknown_tool_in_tools_list() {
        let args = Args {
            tools: Some(vec!["nonexistent_tool".to_string()]),
            exclude_tools: None,
        };
        let result = resolve_enabled_tools(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown tool"));
    }

    #[test]
    fn test_unknown_tool_in_exclude_list() {
        let args = Args {
            tools: None,
            exclude_tools: Some(vec!["nonexistent_tool".to_string()]),
        };
        let result = resolve_enabled_tools(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown tool"));
    }
}
