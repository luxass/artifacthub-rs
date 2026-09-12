# artifacthub-client

Rust client library for the [Artifact Hub](https://artifacthub.io) API.

## Install

```sh
cargo add artifacthub-client
```

## Usage

```rust
use artifacthub_client::ArtifactHubClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ArtifactHubClient::builder()
        .api_key("key-id", "key-secret")
        .build();

    // Search for packages
    let results = client
        .packages()
        .search()
        .query("nginx")
        .repo("bitnami")
        .limit(10)
        .send()
        .await?;

    println!("Found {} packages", results.packages.len());

    // Get package details
    let package = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("15.0.0")
        .send()
        .await?;

    println!("Package: {}", package.name);

    // Search repositories
    let repos = client
        .repositories()
        .search()
        .name("bitnami")
        .send()
        .await?;

    println!("Found {} repositories", repos.repositories.len());

    // Get Helm values.yaml
    let values = client
        .helm()
        .values("helm", "bitnami", "nginx")
        .send()
        .await?;

    println!("Values:\n{}", values.values);

    // Get star count
    let stats = client
        .stats()
        .star_stats("helm", "bitnami", "nginx")
        .send()
        .await?;

    println!("Stars: {}", stats.stars);

    // Get security report
    let report = client
        .security()
        .report("helm", "bitnami", "nginx")
        .send()
        .await?;

    if let Some(report) = report {
        println!("Security report: {:?}", report.0);
    }

    Ok(())
}
```

Package metadata includes license, maintainers, signing details, repository
identity, and container platforms. Fetch README text and version lists with
`.packages().readme(...)` and `.packages().versions(...)`.

Use `.packages().search().facets(true)` to request Hub's facet groups and counts.
Security reports retain Trivy's additional fields, including CVSS scores and
references, in the `extra` maps on reports, scan results, and vulnerabilities.

## License

Published under [MIT License](../../LICENSE).
