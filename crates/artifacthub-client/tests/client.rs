use artifacthub_client::ArtifactHubClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn custom_base_url_with_trailing_slash_is_normalized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "packages": 10,
            "releases": 20
        })))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(format!("{}/", mock_server.uri()));
    let stats = client.packages().stats().await.unwrap();

    assert_eq!(stats.packages, 10);
    assert_eq!(stats.releases, 20);
}

#[tokio::test]
async fn package_reference_segments_are_encoded() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/helm/repo%2Fname/pkg%3Fname"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_package_json()))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let package = client
        .packages()
        .get("helm", "repo/name", "pkg?name")
        .send()
        .await
        .unwrap();

    assert_eq!(package.package_id, "pkg-123");
}

fn sample_package_json() -> serde_json::Value {
    serde_json::json!({
        "package_id": "pkg-123",
        "name": "pkg?name",
        "normalized_name": "pkg-name",
        "version": "1.2.3",
        "description": "Package",
        "deprecated": false,
        "prerelease": false,
        "signed": false,
        "keywords": [],
        "ts": 123,
        "repository": {
            "name": "repo/name",
            "display_name": "Repo",
            "url": "https://example.com",
            "kind": 0,
            "verified_publisher": true,
            "official": false
        },
        "stats": {
            "subscriptions": 0,
            "webhooks": 0
        },
        "links": [],
        "contains_security_updates": false
    })
}
