use artifacthub_client::ArtifactHubClient;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_builder_uses_repository_query_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repositories/search"))
        .and(query_param("name", "bitnami"))
        .and(query_param("kind", "0"))
        .and(query_param("user", "alice"))
        .and(query_param("org", "vmware"))
        .and(query_param("limit", "1"))
        .and(query_param("offset", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "repository_id": "repo-123",
                "name": "bitnami",
                "display_name": "Bitnami",
                "url": "https://charts.bitnami.com/bitnami",
                "kind": 0,
                "verified_publisher": true,
                "official": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = ArtifactHubClient::with_base_url(mock_server.uri());
    let response = client
        .repositories()
        .search()
        .name("bitnami")
        .kind("0")
        .user("alice")
        .org("vmware")
        .limit(1)
        .offset(2)
        .send()
        .await
        .unwrap();

    assert_eq!(response.repositories.len(), 1);
    assert_eq!(response.repositories[0].name, "bitnami");
}
