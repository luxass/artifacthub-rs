---
title: Example workflows
navTitle: Examples
order: 2
description: Prompts you can use with your assistant, and the tool calls behind them.
---

## Find a chart

> Find five Helm charts for PostgreSQL on Artifact Hub. Compare their repositories, versions, and descriptions. Check which results are from verified publishers.

Start with `search_packages`:

```json
{
  "q": "postgresql",
  "kind": ["helm"],
  "verified_publisher": true,
  "facets": true,
  "limit": 5
}
```

This example filters to verified publishers. Omit `verified_publisher` to compare all results. Verification and stars are discovery signals, not security guarantees.

Use `get_package` for the packages you want to inspect. Search accepts arrays for `kind`, `repo`, `org`, and `user`, even when a filter has only one entry.

## Configure a Helm chart

> Look up prometheus-community/prometheus. Read its default values and values schema, if available. Suggest a values.yaml for persistence, explaining every key against the published chart data. Do not deploy it.

First, call `get_package`:

```json
{
  "kind": "helm",
  "repo": "prometheus-community",
  "name": "prometheus"
}
```

Use the returned `version` to pin `get_package_values`:

```json
{
  "kind": "helm",
  "repo": "prometheus-community",
  "name": "prometheus",
  "version": "<version returned by get_package>"
}
```

To call `get_package_values_schema`, use both `package_id` and `version` from that same metadata response:

```json
{
  "package_id": "<package_id returned by get_package>",
  "version": "<version returned by get_package>"
}
```

The angle-bracket values are placeholders. Replace them with the actual response fields. Pinning one version across calls avoids mixing configuration from different releases.

If the package has no schema, use its default values and README. Review the suggested configuration yourself before running Helm.

## Inspect chart templates

> What resources does this chart define? List its templates, then read the workload and service templates. Explain how the values affect them.

1. Resolve the package with `get_package`.
2. Call `get_package_templates` with its `package_id` and `version`.
3. Copy an exact `name` from the template list.
4. Call `get_package_template` with the same ID, version, and template name.

The final call's arguments look like:

```json
{
  "package_id": "<package_id returned by get_package>",
  "version": "<version returned by get_package>",
  "name": "<exact name returned by get_package_templates>"
}
```

`get_package_template` returns the name and decoded source. Use `get_package_template_data` if you only want the source text.

These tools do not render templates. Conditional resources, dependency charts, and cluster capabilities can affect the final manifests. Run Helm separately if you need rendered output.

## Review an upgrade

> Look up the chart versions available for my package. Compare my current version with the target version using the published changelog. Flag security updates and missing data. Do not assume compatibility.

Use `get_package_versions` with `kind`, `repo`, and `name`. Its optional `limit` trims the returned list.

For `get_package_changelog`, supply the same package coordinates and the release range:

```json
{
  "kind": "helm",
  "repo": "<repository name from search>",
  "name": "<package name from search>",
  "from": "<current semver>",
  "to": "<target semver>"
}
```

The range excludes `from` and includes `to`. Filtering happens in the client using semver. `get_changelog_md` gives you Artifact Hub's formatted Markdown changelog instead, without range parameters.

Changelogs reflect what maintainers publish. They are not a full diff of chart behavior.

## Read a security report

> Fetch the security report for this chart version. Summarize the reported vulnerabilities and affected images. If there is no report, say so.

Resolve the target version with `get_package`, then call `get_package_security_report`:

```json
{
  "package_id": "<package_id returned by get_package>",
  "version": "<version returned by get_package>"
}
```

The tool retrieves Artifact Hub's report. It does not run a new vulnerability scan or inspect your deployed images. Check the report's scope and available timestamps before using it to make a deployment decision.

## Find a repository

> What Helm repositories does the prometheus-community organization publish on Artifact Hub?

Call `search_repositories`:

```json
{
  "kind": ["helm"],
  "org": ["prometheus-community"],
  "limit": 10
}
```

Use returned repository names in a subsequent `search_packages` call with a `repo` array. Repository search also supports a name expression, an exact URL, and user aliases.
