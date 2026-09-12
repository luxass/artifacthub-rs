//! Fixtures shaped like hub rows and handler outputs.
//!
//! Every builder cites its hub source. No hand-rolled subsets.

/// Full `get_package` row.
///
/// Hub source: `database/migrations/functions/packages/get_package.sql:28-108`
/// served by `internal/handlers/pkg/handlers.go:182` (`Get`).
pub fn package_snapshot(version: &str) -> serde_json::Value {
    serde_json::json!({
        "package_id": "pkg-123",
        "name": "nginx",
        "normalized_name": "nginx",
        "category": 5,
        "is_operator": false,
        "official": true,
        "cncf": false,
        "display_name": "NGINX",
        "channels": [{"name": "stable", "version": version}],
        "default_channel": "stable",
        "description": "NGINX chart",
        "logo_image_id": "logo-123",
        "keywords": ["nginx", "http"],
        "home_url": "https://example.com",
        "readme": format!("# Nginx {version}\n\nReadme."),
        "install": "helm install",
        "links": [{"name": "docs", "url": "https://example.com/docs"}],
        "crds": [],
        "crds_examples": [],
        "capabilities": "basic install",
        "security_report_summary": {"high": 1},
        "security_report_created_at": 1700000000,
        "all_containers_images_whitelisted": true,
        "data": {"key": "value"},
        "version": version,
        "available_versions": [
            {"version": "1.3.0", "app_version": "1.31.0", "contains_security_updates": false, "prerelease": false, "ts": 1700000000},
            {"version": "1.2.0", "app_version": "1.30.0", "contains_security_updates": false, "prerelease": false, "ts": 1699000000},
            {"version": "1.1.0", "app_version": "1.29.0", "contains_security_updates": false, "prerelease": false, "ts": 1698000000}
        ],
        "app_version": "1.31.0",
        "digest": "digest-123",
        "deprecated": false,
        "contains_security_updates": false,
        "prerelease": false,
        "license": "Apache-2.0",
        "signed": false,
        "signatures": [],
        "content_url": "https://example.com/pkg.tgz",
        "containers_images": [{"name": "nginx", "image": "nginx:1.31.0", "whitelisted": true, "platforms": ["linux/amd64", "linux/arm64"]}],
        "provider": "Bitnami",
        "has_values_schema": true,
        "has_changelog": true,
        "changes": [],
        "ts": 1700000000,
        "maintainers": [{"name": "Bitnami", "email": "bitnami@example.com"}],
        "recommendations": [{"url": "https://artifacthub.io/packages/helm/bitnami/apache"}],
        "screenshots": [{"title": "Overview", "url": "https://example.com/screenshot.png"}],
        "sign_key": {"fingerprint": "fixture-fingerprint", "url": "https://example.com/key.asc"},
        "repository": {
            "repository_id": "repo-123",
            "kind": 0,
            "name": "bitnami",
            "display_name": "Bitnami",
            "url": "https://charts.bitnami.com/bitnami",
            "branch": "main",
            "private": false,
            "user_alias": "publisher",
            "verified_publisher": true,
            "official": true,
            "organization_name": "bitnami",
            "organization_display_name": "Bitnami",
            "scanner_disabled": false
        },
        "stats": {"subscriptions": 10, "webhooks": 2},
        "production_organizations_count": 0,
        "relative_path": ""
    })
}

/// Structured changelog array.
///
/// Hub source: `database/migrations/functions/packages/get_package_changelog.sql:3-21`
/// shaped like `internal/handlers/pkg/handlers_test.go:393` (`GetChangelog`).
pub(crate) fn changelog_entries() -> serde_json::Value {
    serde_json::json!([
        {
            "version": "1.3.0",
            "ts": 1700000000,
            "changes": [
                {"kind": "added", "description": "Added new feature", "links": [{"name": "issue", "url": "https://example.com/1"}]},
                {"kind": "fixed", "description": "Fixed bug"}
            ],
            "contains_security_updates": false,
            "prerelease": false
        },
        {
            "version": "1.2.0",
            "ts": 1699000000,
            "changes": [{"description": "Older release"}],
            "contains_security_updates": false,
            "prerelease": false
        },
        {
            "version": "1.1.0",
            "ts": 1698000000,
            "changes": [],
            "contains_security_updates": false,
            "prerelease": false
        }
    ])
}

/// Rendered changelog markdown.
///
/// Hub source: `internal/handlers/pkg/handlers.go:68-114`
/// (`setupChangelogMDTmpl`) served as `text/markdown` by
/// `internal/handlers/pkg/handlers.go:145` (`GenerateChangelogMD`).
pub(crate) fn changelog_markdown() -> &'static str {
    "# Changelog\n\n## 1.3.0 - 2024-01-01\n\n### Added\n\n- Added new feature\n\n## 1.2.0 - 2023-12-01\n\n### Fixed\n\n- Fixed bug\n"
}

/// Raw `values.yaml` bytes.
///
/// Hub source: `internal/handlers/pkg/handlers.go:236`
/// (`GetChartValues`, `Content-Type: application/yaml`, 404 when missing).
pub(crate) fn chart_values() -> &'static str {
    "replicaCount: 2\n"
}

/// Values schema JSON.
///
/// Hub source: `internal/handlers/pkg/handlers.go:404`
/// (`GetValuesSchema`).
pub(crate) fn values_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "replicaCount": {"type": "integer", "default": 1}
        }
    })
}

/// Chart templates + values map.
///
/// Hub source: `internal/handlers/pkg/handlers.go:210`
/// (`GetChartTemplates` returns `{"templates": ..., "values": ...}`).
pub(crate) fn chart_templates() -> serde_json::Value {
    serde_json::json!({
        "templates": [
            {
                "name": "templates/deployment.yaml",
                "data": "YXBpVmVyc2lvbjogYXBwcy92MQpraW5kOiBEZXBsb3ltZW50Cg=="
            }
        ],
        "values": {"replicaCount": 2}
    })
}

/// Trivy security-report map keyed by image.
///
/// Hub source: `internal/handlers/pkg/handlers.go:330`
/// (`GetSnapshotSecurityReport`).
pub(crate) fn security_report() -> serde_json::Value {
    serde_json::json!({
        "quay.io/org/pkg1:1.2.0": {
            "Results": [
                {
                    "Target": "quay.io/org/pkg1:1.2.0",
                    "Type": "alpine",
                    "Vulnerabilities": [
                        {
                            "VulnerabilityID": "CVE-2024-1234",
                            "PkgName": "openssl",
                            "InstalledVersion": "1.1.1",
                            "FixedVersion": "1.1.2",
                            "Severity": "CRITICAL",
                            "Title": "Critical vulnerability in OpenSSL"
                        }
                    ]
                }
            ]
        }
    })
}

/// Anonymous stars shape (`starred_by_user` stripped).
///
/// Hub source: `database/migrations/functions/packages/get_package_stars.sql:3-17`
/// served by `internal/handlers/pkg/handlers.go:364` (`GetStars`).
pub(crate) fn package_stars() -> serde_json::Value {
    serde_json::json!({
        "stars": 150
    })
}

/// Package search body (hub `search_packages` shape).
///
/// Hub source: `internal/handlers/pkg/handlers.go:533` (`Search`).
pub(crate) fn search_packages_body() -> serde_json::Value {
    serde_json::json!({
        "packages": [
            {
                "package_id": "pkg-123",
                "name": "nginx",
                "normalized_name": "nginx",
                "category": 5,
                "version": "1.3.0",
                "app_version": "1.31.0",
                "description": "NGINX chart",
                "license": "Apache-2.0",
                "deprecated": false,
                "signed": false,
                "stars": 150,
                "ts": 1700000000,
                "repository": {
                    "repository_id": "repo-123",
                    "kind": 0,
                    "name": "bitnami",
                    "display_name": "Bitnami",
                    "url": "https://charts.bitnami.com/bitnami",
                    "verified_publisher": true,
                    "official": true
                }
            }
        ]
    })
}

/// Repository search body.
///
/// Hub source: `internal/handlers/repo/handlers.go:129` (`Search`).
pub(crate) fn search_repositories_body() -> serde_json::Value {
    serde_json::json!([
        {
            "repository_id": "repo-123",
            "name": "bitnami",
            "display_name": "Bitnami",
            "url": "https://charts.bitnami.com/bitnami",
            "branch": "main",
            "kind": 0,
            "verified_publisher": true,
            "official": true,
            "disabled": false
        }
    ])
}
