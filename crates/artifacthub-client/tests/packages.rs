//! Contract tests against [`artifacthub_server_mock::HubMockServer`], a faithful
//! mock of `/tmp/hub` behavior — not hand-rolled subsets.
//!
//! HubMockServer serves full `get_package` payloads, structured changelogs, Trivy
//! security maps, and hub content-types/headers. If the client drifts from
//! hub wire format, these fail.

use artifacthub_client::{ArtifactHubClient, ArtifactHubError};
use artifacthub_server_mock::HubMockServer;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_uses_hub_params_and_captures_total_count() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let response = client
        .packages()
        .search()
        .query("nginx")
        .kind("0")
        .repo("bitnami")
        .org("vmware")
        .limit(1)
        .offset(2)
        .send()
        .await
        .unwrap();

    assert_eq!(response.packages.len(), 1);
    assert_eq!(response.packages[0].package_id, "pkg-123");
    assert_eq!(response.total_count, Some(1));
}

#[tokio::test]
async fn starred_builder_sends_pagination_and_auth_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/starred"))
        .and(wiremock::matchers::query_param("limit", "1"))
        .and(wiremock::matchers::query_param("offset", "2"))
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
        .offset(2)
        .send()
        .await
        .unwrap();

    assert_eq!(packages.count, 1);
}

#[tokio::test]
async fn get_pinned_version_returns_exact_snapshot() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let package = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("1.2.0")
        .send()
        .await
        .unwrap();

    assert_eq!(package.package_id, "pkg-123");
    assert_eq!(package.version, "1.2.0");
    // Full payload parses (readme/available_versions siblings present).
    assert!(!package.description.is_empty());
}

#[tokio::test]
async fn get_latest_without_version() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let package = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .send()
        .await
        .unwrap();

    assert_eq!(package.version, "1.3.0");
}

#[tokio::test]
async fn get_rejects_version_mismatch() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/helm/bitnami/nginx/1.2.3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "package_id": "pkg-123",
            "name": "nginx",
            "normalized_name": "nginx",
            "version": "9.9.9",
            "description": "Nginx chart",
            "deprecated": false,
            "prerelease": false,
            "signed": false,
            "keywords": [],
            "ts": 123,
            "repository": {
                "name": "bitnami",
                "display_name": "Bitnami",
                "url": "https://charts.bitnami.com/bitnami",
                "kind": 0,
                "verified_publisher": true,
                "official": false
            },
            "stats": {"subscriptions": 0, "webhooks": 0},
            "links": [],
            "contains_security_updates": false
        })))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let error = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("1.2.3")
        .send()
        .await
        .unwrap_err();

    assert!(
        matches!(error, ArtifactHubError::VersionMismatch { .. }),
        "expected VersionMismatch, got {error:?}"
    );
}

#[tokio::test]
async fn get_unknown_version_is_404_like_hub() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let error = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("0.0.0-bogus")
        .send()
        .await
        .unwrap_err();

    assert!(
        matches!(error, ArtifactHubError::Api { .. }),
        "expected Api 404, got {error:?}"
    );
}

#[tokio::test]
async fn readme_pinned_version_returns_versioned_readme() {
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
async fn helm_values_with_version_uses_version_path_for_identity() {
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
    assert!(values.values.contains("replicaCount: 2"));
}

#[tokio::test]
async fn package_id_version_endpoints_use_encoded_paths() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg%2F123/1.0.0%2Bbuild/values"))
        .respond_with(ResponseTemplate::new(200).set_body_string("replicaCount: 2\n"))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let values = client
        .packages()
        .values("pkg/123", "1.0.0+build")
        .await
        .unwrap();

    assert_eq!(values, "replicaCount: 2\n");
}

#[tokio::test]
async fn stars_anonymous_shape_has_no_starred_by_user() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let stats = client.packages().stars("pkg-123").await.unwrap();

    assert_eq!(stats.stars, 150);
    assert_eq!(stats.starred_by_user, None);
}

#[tokio::test]
async fn api_errors_use_json_message_when_present() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/stats"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "package not found"
        })))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let error = client.packages().stats().await.unwrap_err();

    match error {
        ArtifactHubError::Api { status, message } => {
            assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
            assert_eq!(message, "package not found");
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[tokio::test]
async fn changelog_fetches_full_list_and_filters_client_side() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());

    // No range: full structured list.
    let full = client
        .packages()
        .changelog("helm", "bitnami", "nginx")
        .send()
        .await
        .unwrap();
    assert_eq!(full.entries.len(), 3);
    assert_eq!(full.entries[0].changes[0].description, "Added new feature");

    // Range is client-side (hub ignores ?from=&to=).
    let ranged = client
        .packages()
        .changelog("helm", "bitnami", "nginx")
        .from("1.1.0")
        .to("1.2.0")
        .send()
        .await
        .unwrap();
    assert_eq!(ranged.entries.len(), 1);
    assert_eq!(ranged.entries[0].version, "1.2.0");
}

#[tokio::test]
async fn security_report_parses_trivy_map_shape() {
    let hub = HubMockServer::start().await;
    let client = ArtifactHubClient::with_base_url(hub.uri());
    let report = client
        .packages()
        .security_report("pkg-123", "1.2.0")
        .await
        .unwrap()
        .expect("report");

    let entry = report.0.get("quay.io/org/pkg1:1.2.0").expect("image entry");
    assert_eq!(
        entry.results[0].vulnerabilities[0]
            .vulnerability_id
            .as_deref(),
        Some("CVE-2024-1234")
    );
}
