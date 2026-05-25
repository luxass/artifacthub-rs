# artifacthub-client

Rust client library for the [Artifact Hub](https://artifacthub.io) API.

## Install

```sh
cargo add artifacthub-client
```

## Usage

```rust
use artifacthub_client::{ArtifactHub, PackageGetParams, PackageSearchParams, RepoSearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ArtifactHub::new();

    // Search for packages
    let results = client.packages.search(&PackageSearchParams {
        q: Some("nginx".to_string()),
        ..Default::default()
    }).await?;

    println!("Found {} packages", results.packages.len());

    // Get package details
    let package = client.packages.get(&PackageGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
        version: None,
    }).await?;

    println!("Package: {}", package.name);

    // Search repositories
    let repos = client.repositories.search(&RepoSearchParams {
        name: Some("bitnami".to_string()),
        ..Default::default()
    }).await?;

    println!("Found {} repositories", repos.repositories.len());

    // Get Helm values.yaml
    let values = client.helm.values(&artifacthub_client::HelmGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
        version: None,
    }).await?;

    println!("Values:\n{}", values.values);

    // Get star history
    let stats = client.stats.star_stats(&artifacthub_client::StatsGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
    }).await?;

    println!("Star entries: {}", stats.stars.len());

    // Get security report
    let report = client.security.report(&artifacthub_client::SecurityGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
    }).await?;

    println!("Security report: {:?}", report.summary);

    Ok(())
}
```

## License

Published under [MIT License](../../LICENSE).
