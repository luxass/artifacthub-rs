---
title: Artifact Hub MCP
navTitle: Setup
order: 0
description: Search Artifact Hub packages and read Helm chart data through your AI assistant.
---

`artifacthub-mcp` connects your assistant to [Artifact Hub](https://artifacthub.io) with 15 read-only tools. It runs locally over stdio, needs no API key, and does not install charts or access your cluster.

## Install the server

Choose one installation method.

### Homebrew

For macOS and Linux:

```sh
brew install luxass/homebrew-tap/artifacthub-mcp
```

### Cargo

With a Rust toolchain installed:

```sh
cargo install --locked artifacthub-mcp
```

### Pre-built binaries

Download the binary for your platform from [GitHub Releases](https://github.com/luxass/artifacthub-rs/releases). Pre-built binaries are available for Linux and macOS.

Put the binary on your `PATH`, then check it:

```sh
artifacthub-mcp --version
artifacthub-mcp --help
```

## Connect a client

The server uses stdio. Configure your client to launch `artifacthub-mcp` as a command, not to connect to a URL.

<details>
<summary>Claude Code</summary>

```sh
claude mcp add artifacthub -- artifacthub-mcp
```

</details>

<details>
<summary>Codex</summary>

```sh
codex mcp add artifacthub -- artifacthub-mcp
```

Or add this to `~/.codex/config.toml`:

```toml
[mcp_servers.artifacthub]
command = "artifacthub-mcp"
```

</details>

<details>
<summary>Clients using mcpServers</summary>

For clients that support the `mcpServers` configuration format, add this entry to the client's MCP configuration file:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

The file location depends on your client. This format is not interchangeable with the OpenCode or VS Code formats below.

</details>

<details>
<summary>OpenCode</summary>

Add this entry to `opencode.json`:

```json
{
  "mcp": {
    "artifacthub": {
      "type": "local",
      "command": ["artifacthub-mcp"]
    }
  }
}
```

</details>

<details>
<summary>VS Code</summary>

Add a server entry to your workspace's `.vscode/mcp.json`:

```json
{
  "servers": {
    "artifacthub": {
      "type": "stdio",
      "command": "artifacthub-mcp"
    }
  }
}
```

</details>

If your editor cannot find the binary, set `command` to its absolute path. Restart or reconnect the MCP server after editing the client configuration.

## Make your first lookup

Ask your assistant:

> Use Artifact Hub to find Helm charts for PostgreSQL. Return five results with their repository names and chart versions.

The corresponding `search_packages` arguments are:

```json
{
  "q": "postgresql",
  "kind": ["helm"],
  "limit": 5
}
```

These JSON examples are tool arguments sent by an MCP client. They are not shell commands or HTTP requests.

Use the repository and package names returned by search for subsequent lookups. Ask for a README only when you need it. Some READMEs exceed 100 KB.

## Examples

Once connected, try these prompts. Expand an example to see the tool calls behind it. The [tool reference](/tools/) lists all tool inputs.

<details>
<summary>Find a chart</summary>

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

</details>

<details>
<summary>Configure a Helm chart</summary>

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

</details>

<details>
<summary>Inspect chart templates</summary>

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

</details>

<details>
<summary>Review an upgrade</summary>

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

</details>

<details>
<summary>Read a security report</summary>

> Fetch the security report for this chart version. Summarize the reported vulnerabilities and affected images. If there is no report, say so.

Resolve the target version with `get_package`, then call `get_package_security_report`:

```json
{
  "package_id": "<package_id returned by get_package>",
  "version": "<version returned by get_package>"
}
```

The tool retrieves Artifact Hub's report. It does not run a new vulnerability scan or inspect your deployed images. Check the report's scope and available timestamps before using it to make a deployment decision.

</details>

<details>
<summary>Find a repository</summary>

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

</details>

## Limit the exposed tools

All 15 tools are enabled by default. To expose a smaller set:

```sh
artifacthub-mcp --tools search_packages,get_package,get_package_versions
```

To keep the default set except for selected tools:

```sh
artifacthub-mcp --exclude-tools get_package_star_stats,get_package_security_report
```

`--tools` and `--exclude-tools` are mutually exclusive. Unknown tool names cause the server to exit with an error.

Pass those flags through your client's arguments configuration. For example:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp",
      "args": [
        "--tools",
        "search_packages,get_package,get_package_values"
      ]
    }
  }
}
```

## Troubleshooting

- **Command not found:** verify the binary's path in the environment your MCP client uses. A desktop editor may not inherit your terminal's `PATH`.
- **Running the binary appears to hang:** without `--help` or `--version`, it waits for MCP messages on stdin. Let the MCP client launch it.
- **Tools are missing:** check the configured `--tools` or `--exclude-tools` flags, then reconnect.
- **Lookup fails:** confirm the kind, repository, package name, and version from search results. The server needs outbound HTTPS access to Artifact Hub and uses a 30-second HTTP request timeout.
- **Schema or report is absent:** check whether Artifact Hub publishes that data for the selected version. Do not substitute invented values or a clean security verdict.
