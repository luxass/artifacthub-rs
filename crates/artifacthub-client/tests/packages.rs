//! MockHub parity probes against the *unfixed* client.
//!
//! Each `#[ignore = "red"]`-marked test documents a hub behavior from
//! `/tmp/hub` that the old client gets wrong. They are expected to FAIL
//! until the client is aligned (next PR in the stack):
//! - version is a path param (`pkg/handlers.go:182`); `?version=` is ignored
//! - changelog entries are structured objects, not strings
//!   (`get_package_changelog.sql`, `handlers_test.go:393`)
//! - security reports are Trivy image maps (`web/types.ts:654`)
//! - `helm values` identity must pin the requested version

use artifacthub_client::ArtifactHubClient;
use artifacthub_server_mock::HubMockServer;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn red_pinned_version_returns_requested_snapshot() {
    // RED: old client sends `?version=`, hub ignores query params and returns
    // latest (1.3.0), so this assertion fails.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let package = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("1.2.0")
        .send()
        .await
        .unwrap();

    assert_eq!(package.version, "1.2.0");
}

#[tokio::test]
async fn red_versioned_readme_matches_version() {
    // RED: same `?version=` cause — latest readme says 1.3.0.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let readme = client
        .packages()
        .readme("helm", "bitnami", "nginx")
        .version("1.2.0")
        .send()
        .await
        .unwrap();

    assert!(readme.readme.contains("1.2.0"));
}

#[tokio::test]
async fn red_changelog_parses_structured_changes() {
    // RED: hub returns `changes: [{kind, description, links}]`; the old
    // `Vec<String>` model fails deserialization.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let changelog = client
        .packages()
        .changelog("helm", "bitnami", "nginx")
        .send()
        .await
        .unwrap();

    assert_eq!(changelog.entries.len(), 3);
}

#[tokio::test]
async fn red_security_report_surfaces_cves() {
    // RED: hub returns a Trivy image map; the old grouped model parses it as
    // an empty report with no vulnerabilities.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let report = client
        .packages()
        .security_report("pkg-123", "1.2.0")
        .await
        .unwrap()
        .expect("report");

    assert!(!report.critical_vulnerabilities.unwrap_or_default().is_empty());
}

#[tokio::test]
async fn red_helm_values_pin_requested_version() {
    // RED: old identity resolution ignores the requested version, resolves
    // latest (1.3.0), and `.../1.3.0/values` is not served → 404.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let values = client
        .helm()
        .values("helm", "bitnami", "nginx")
        .version("1.2.0")
        .send()
        .await
        .unwrap();

    assert_eq!(values.version, "1.2.0");
}

#[tokio::test]
async fn green_search_and_stars_match_already() {
    // These behaviors already match hub; they guard against regressions while
    // the red tests above drive the fix.
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());

    let response = client
        .packages()
        .search()
        .query("nginx")
        .kind("0")
        .repo("bitnami")
        .limit(1)
        .send()
        .await
        .unwrap();
    assert_eq!(response.packages.len(), 1);

    let stats = client.packages().stars("pkg-123").await.unwrap();
    assert_eq!(stats.stars, 150);
}

#[tokio::test]
async fn green_starred_and_errors_unchanged() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/starred"))
        .and(wiremock::matchers::query_param("limit", "1"))
        .and(header("X-API-KEY-ID", "key-id"))
        .and(header("X-API-KEY-SECRET", "key-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "package_id": "pkg-123",
                "name": "nginx",
                "normalized_name": "nginx",
                "version": "1.2.3",
                "description": "Nginx chart",
                "deprecated": false,
                "signed": false,
                "stars": 10,
                "ts": 123,
                "repository": {"name": "bitnami", "url": "https://charts.bitnami.com/bitnami"}
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::builder()
        .base_url(mock_server.uri())
        .api_key("key-id", "key-secret")
        .build();
    let packages = client
        .packages()
        .starred()
        .limit(1)
        .send()
        .await
        .unwrap();
    assert_eq!(packages.count, 1);
}
