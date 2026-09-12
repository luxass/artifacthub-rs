//! Wire routes mirroring the hub route table.
//!
//! Hub truth: `internal/handlers/handlers.go:268-297` (packages) and
//! `internal/handlers/handlers.go:243` (repositories search).

use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

use crate::fixtures;

/// `GET /packages/helm/bitnami/nginx[/{version}]`.
///
/// Hub: `handlers.go:273-284` routes `/{version}` and `/` both to
/// `pkg/handlers.go:182` (`Get`), which reads `chi.URLParam("version")`.
/// So version is a path param only: no `query_param` matchers here, and a
/// regressed `?version=` client gets latest. Unknown versions fall through
/// to wiremock 404, like hub's `get_package.sql` no-row 404.
pub(crate) async fn mount_package_routes(server: &MockServer) {
    for version in ["1.3.0", "1.2.0", "1.1.0"] {
        Mock::given(method("GET"))
            .and(path(format!("/packages/helm/bitnami/nginx/{version}")))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(fixtures::package_snapshot(version)),
            )
            .mount(server)
            .await;
    }

    // Serves latest on unversioned path; ignores `?version=` on purpose.
    Mock::given(method("GET"))
        .and(path("/packages/helm/bitnami/nginx"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::package_snapshot("1.3.0")))
        .mount(server)
        .await;
}

/// `GET /packages/{id}/changelog` + `.../changelog.md`.
///
/// Hub: `handlers.go:296` (`GetChangelog`, `pkg/handlers.go:198`) and
/// `handlers.go:277` (`GenerateChangelogMD`, `pkg/handlers.go:145`).
/// Both take no query params and return the full list; range filtering is
/// client-side.
pub(crate) async fn mount_changelog_routes(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/changelog"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::changelog_entries()))
        .mount(server)
        .await;

    Mock::given(method("GET"))
        .and(path("/packages/helm/bitnami/nginx/changelog.md"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixtures::changelog_markdown()))
        .mount(server)
        .await;
}

/// Snapshot endpoints with hub content-types.
///
/// Hub: `handlers.go:290-293` mapping
/// `/{id}/{version}/values|values-schema|templates|security-report` to
/// `pkg/handlers.go:236,404,210,330`. Empty body means None upstream.
pub(crate) async fn mount_snapshot_routes(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/1.2.0/values"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixtures::chart_values())
                .insert_header("Content-Type", "application/yaml"),
        )
        .mount(server)
        .await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/1.2.0/values-schema"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::values_schema()))
        .mount(server)
        .await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/1.2.0/templates"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::chart_templates()))
        .mount(server)
        .await;

    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/1.2.0/security-report"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::security_report()))
        .mount(server)
        .await;

    // Hub `GetStars` strips `starred_by_user` for anonymous callers.
    Mock::given(method("GET"))
        .and(path("/packages/pkg-123/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixtures::package_stars()))
        .mount(server)
        .await;
}

/// Search endpoints with `Pagination-Total-Count`.
///
/// Hub: `handlers.go:271` (`Packages.Search`, `pkg/handlers.go:533`) and
/// `handlers.go:243` (`Repositories.Search`, `repo/handlers.go:129`);
/// header name in `helpers/helpers.go:30`.
pub(crate) async fn mount_search_routes(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/packages/search"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(fixtures::search_packages_body())
                .insert_header("Pagination-Total-Count", "1"),
        )
        .mount(server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repositories/search"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(fixtures::search_repositories_body())
                .insert_header("Pagination-Total-Count", "1"),
        )
        .mount(server)
        .await;
}
