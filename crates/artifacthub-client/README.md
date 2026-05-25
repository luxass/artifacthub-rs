# artifacthub-client

Rust client library for the [Artifact Hub](https://artifacthub.io) API.

## Install

```sh
cargo add artifacthub-client
```

## Usage

```rust
use artifacthub_client::ArtifactHubClient;
use artifacthub_client::params::RepoSearchParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ArtifactHubClient::new();

    // Search for packages
    let results = client
        .packages
        .search()
        .query("nginx")
        .send()
        .await?;

    println!("Found {} packages", results.packages.len());

    // Get package details
    let package = client
        .packages
        .get("helm", "bitnami", "nginx")
        .send()
        .await?;

    println!("Package: {}", package.name);

    // Search repositories
    let repos = client.repositories.search(&RepoSearchParams {
        name: Some("bitnami".to_string()),
        ..Default::default()
    }).await?;

    println!("Found {} repositories", repos.repositories.len());

    // Get Helm values.yaml
    let values = client.helm.values(&artifacthub_client::params::HelmGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
        version: None,
    }).await?;

    println!("Values:\n{}", values.values);

    // Get star history
    let stats = client.stats.star_stats(&artifacthub_client::params::StatsGetParams {
        kind: "helm".to_string(),
        repo: "bitnami".to_string(),
        name: "nginx".to_string(),
    }).await?;

    println!("Star entries: {}", stats.stars.len());

    // Get security report
    let report = client.security.report(&artifacthub_client::params::SecurityGetParams {
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
