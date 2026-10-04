---
title: Tool reference
navTitle: Tools
order: 3
description: All 15 tools exposed by artifacthub-mcp, with required inputs and optional filters.
---

All tools are read-only. They query package data or describe the server. They do not install, render, or deploy charts.

## Input conventions

Package-coordinate tools use these required fields:

| Field | Meaning |
| --- | --- |
| `kind` | Package kind, such as `helm` |
| `repo` | Artifact Hub repository name |
| `name` | Package name in that repository |

Use search results to get valid coordinates. An optional `version` selects a specific release. Omit it to request the latest release for tools that support it.

ID-based tools require `package_id` and `version` from `get_package`. Template lookups also require an exact template `name` from `get_package_templates`.

## Discovery

### `search_packages`

Search packages. All inputs are optional.

| Inputs | Purpose |
| --- | --- |
| `q`, `ts_query` | Web search query or advanced raw tsquery |
| `kind`, `repo`, `org`, `user` | Arrays of kinds, repository names, organizations, or user aliases |
| `category` | Array of category IDs |
| `verified_publisher`, `official`, `cncf`, `operators` | Boolean filters |
| `deprecated` | Include deprecated packages, excluded by default |
| `license`, `capabilities` | Arrays of licenses or capabilities |
| `sort` | `relevance`, `stars`, or `last_updated` |
| `facets` | Include grouped counts |
| `limit`, `offset` | Page size from 1 to 60, and pagination offset |

### `search_repositories`

Search repositories. All inputs are optional.

- `name` is a repository-name expression, interpreted as a regex upstream.
- `url` matches an exact repository URL.
- `kind`, `user`, and `org` take arrays.
- `limit` accepts 1 to 60. `offset` controls pagination.

## Package details

### `get_package`

Requires `kind`, `repo`, and `name`. Optional `version`.

Returns a metadata summary with the package ID, version, repository, stats, keywords, links, containers, and security metadata. It does not include the README or available version list.

### `get_package_readme`

Requires `kind`, `repo`, and `name`. Optional `version`.

Returns the package README. Responses can exceed 100 KB. Prefer `get_package` unless you need the documentation.

### `get_package_versions`

Requires `kind`, `repo`, and `name`. Optional `limit`.

Returns available versions. By default it returns the whole list. Unlike search pagination, this limit truncates the fetched result locally.

### `get_package_changelog`

Requires `kind`, `repo`, and `name`. Optional `from` and `to`.

Returns structured changelog entries. Semver filtering excludes the `from` version and includes the `to` version. Filtering happens in the client.

### `get_changelog_md`

Requires `kind`, `repo`, and `name`.

Returns the pre-formatted Markdown changelog in the response's `changelog` field. This tool does not accept `from` or `to`.

### `get_package_star_stats`

Requires `kind`, `repo`, and `name`.

Returns package star statistics.

## Helm chart data

Artifact Hub's values and template endpoints support `helm` and `kagent`. Other kinds return an upstream error. Helm charts hosted in OCI registries still use `helm`.

### `get_package_values`

Requires `kind`, `repo`, and `name`. Optional `version`.

Returns the chart's default `values.yaml`. It does not return your release's configured values.

### `get_package_values_schema`

Requires `package_id` and `version`.

Returns the chart's values schema in the `schema` field. That field can be null if no schema is available.

### `get_package_templates`

Requires `package_id` and `version`.

Lists template names without the template source. Use those exact names for the next lookup.

### `get_package_template`

Requires `package_id`, `version`, and `name`.

Returns one template's name and decoded source in `data`. A missing template or one with no data returns an error.

### `get_package_template_data`

Requires `package_id`, `version`, and `name`.

Returns only the decoded template source text. The source contains Helm expressions, not rendered manifests.

## Security

### `get_package_security_report`

Requires `package_id` and `version`.

Returns Artifact Hub's security report for that release, including reported vulnerability data. It does not run a scan. Report availability depends on the package and version.

## Server information

### `get_server_info`

Takes an empty object, `{}`.

Returns the MCP server's name, version, and description.

## Supported package kinds

The current client recognizes these canonical kind names:

```text
helm, falco, opa, olm, tbaction, krew, helm-plugin,
tekton-task, keda-scaler, coredns, keptn, tekton-pipeline,
container, kubewarden, gatekeeper, kyverno,
knative-client-plugin, backstage, argo-template,
kubearmor, kcl, headlamp, inspektor-gadget,
tekton-stepaction, meshery, opencost, radius, bootc, kagent
```

Aliases include `tinkerbell` for `tbaction`, `tekton` for `tekton-task`, `keda` for `keda-scaler`, `coredns-plugin` for `coredns`, `meshery-design` for `meshery`, and `bootable-container` for `bootc`.

The [kind mappings in the Rust client](https://github.com/luxass/artifacthub-rs/blob/main/crates/artifacthub-client/src/kind.rs) are the source of truth.

## Tool filtering

All tools are enabled by default. Use `--tools` to expose only selected names, or `--exclude-tools` to remove names from the default set. Do not pass both.

See [client setup and filtering examples](/getting-started/#limit-the-exposed-tools).
