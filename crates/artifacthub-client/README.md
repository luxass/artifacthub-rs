# artifacthub-client

Rust client library for the [Artifact Hub](https://artifacthub.io) API.

## Install

```sh
cargo add artifacthub-client
```

## Usage

```rust
use artifacthub_client::client::ArtifactHubClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ArtifactHubClient::new();

    // Search for packages
    let results = client.search_packages(&artifacthub_client::models::SearchPackagesParams {
        query: Some("nginx".to_string()),
        ..Default::default()
    }).await?;

    println!("Found {} packages", results.len());

    // Get package details
    let package = client.get_package(&artifacthub_client::models::GetPackageParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
        ..Default::default()
    }).await?;

    println!("Package: {}", package.name);

    Ok(())
}
```

## License

Published under [MIT License](../../LICENSE).
