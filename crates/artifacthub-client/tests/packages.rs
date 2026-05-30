use artifacthub_client::{ArtifactHubClient, ArtifactHubError};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_builder_uses_artifact_hub_query_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/search"))
        .and(query_param("ts_query_web", "nginx"))
        .and(query_param("kind", "0"))
        .and(query_param("repo", "bitnami"))
        .and(query_param("org", "vmware"))
        .and(query_param("limit", "1"))
        .and(query_param("offset", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "packages": [sample_package_summary_json()]
        })))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
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
}

#[tokio::test]
async fn starred_builder_sends_pagination_and_auth_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/starred"))
        .and(query_param("limit", "1"))
        .and(query_param("offset", "2"))
        .and(header("X-API-KEY-ID", "key-id"))
        .and(header("X-API-KEY-SECRET", "key-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([sample_package_summary_json()])),
        )
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
async fn get_builder_sends_version_query_param() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/helm/bitnami/nginx"))
        .and(query_param("version", "1.2.3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_package_json()))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let package = client
        .packages()
        .get("helm", "bitnami", "nginx")
        .version("1.2.3")
        .send()
        .await
        .unwrap();

    assert_eq!(package.package_id, "pkg-123");
    assert_eq!(package.version, "1.2.3");
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
async fn stars_returns_count_shape() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "stars": 150,
            "starred_by_user": true
        })))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let stats = client.packages().stars("pkg-123").await.unwrap();

    assert_eq!(stats.stars, 150);
    assert_eq!(stats.starred_by_user, Some(true));
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
async fn changelog_builder_resolves_package_then_uses_package_id_endpoint() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/packages/helm/bitnami/nginx"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "package_id": "pkg-123",
            "version": "1.2.3"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/changelog"))
        .and(query_param("from", "1.0.0"))
        .and(query_param("to", "1.2.3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "version": "1.2.3",
                "ts": 1700000000,
                "changes": ["Fixed service ports"],
                "prerelease": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let changelog = client
        .packages()
        .changelog("helm", "bitnami", "nginx")
        .from("1.0.0")
        .to("1.2.3")
        .send()
        .await
        .unwrap();

    assert_eq!(changelog.entries.len(), 1);
    assert_eq!(changelog.entries[0].version, "1.2.3");
}

fn sample_package_json() -> serde_json::Value {
    serde_json::json!({
        "package_id": "pkg-123",
        "name": "nginx",
        "normalized_name": "nginx",
        "version": "1.2.3",
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
        "stats": {
            "subscriptions": 0,
            "webhooks": 0
        },
        "links": [],
        "contains_security_updates": false
    })
}

fn sample_package_summary_json() -> serde_json::Value {
    serde_json::json!({
        "package_id": "pkg-123",
        "name": "nginx",
        "normalized_name": "nginx",
        "version": "1.2.3",
        "description": "Nginx chart",
        "deprecated": false,
        "signed": false,
        "stars": 10,
        "ts": 123,
        "repository": {
            "name": "bitnami",
            "url": "https://charts.bitnami.com/bitnami"
        }
    })
}
