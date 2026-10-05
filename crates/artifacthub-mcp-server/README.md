# artifacthub-mcp

An MCP server for [Artifact Hub](https://artifacthub.io) that gives AI coding assistants current package and Helm chart data. Search packages, inspect metadata, check versions and changelogs, and read a chart's `values.yaml`.

This helps your assistant use the chart information that Artifact Hub publishes instead of guessing at package versions or Helm values.

## Install

### Homebrew (macOS and Linux)

```sh
brew tap luxass/tap
brew install luxass/tap/artifacthub-mcp
```

### Cargo

```sh
cargo install --locked artifacthub-mcp
```

### GitHub Releases

Pre-built binaries are available for Linux and macOS at [github.com/luxass/artifacthub-rs/releases](https://github.com/luxass/artifacthub-rs/releases).

## Setup

For clients that use the `mcpServers` format, add this to their MCP configuration file. Other clients use different formats; see the examples below.

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

### Tool Filtering

Control which tools are exposed to the MCP client:

```bash
# Enable only specific tools
artifacthub-mcp --tools search_packages,get_package,get_package_versions

# Exclude specific tools from the default set
artifacthub-mcp --exclude-tools get_package_star_stats,get_package_security_report
```

`--tools` and `--exclude-tools` are mutually exclusive. Run `artifacthub-mcp --help` for the full list of available tools.

<details>
<summary>Claude Code</summary>

Add via the Claude Code CLI:

```bash
claude mcp add --transport stdio artifacthub -- artifacthub-mcp
```

This defaults to local scope for the current project. Add `--scope user` before the server name to register it across projects, or use `--scope project` to write a shared `.mcp.json`.

For manual project setup, add this to `.mcp.json` at your project root, not `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

</details>

<details>
<summary>Codex</summary>

Add via the Codex CLI:

```bash
codex mcp add artifacthub -- artifacthub-mcp
```

Or add to your `~/.codex/config.toml`:

```toml
[mcp_servers.artifacthub]
command = "artifacthub-mcp"
```

</details>

<details>
<summary>Cursor</summary>

Add this to `.cursor/mcp.json` for the current project or `~/.cursor/mcp.json` for all projects:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

</details>

<details>
<summary>OpenCode</summary>

Add to your `opencode.json`:

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

For quick installation, use one of the one-click install buttons below...

[![Install in VS Code](https://img.shields.io/badge/VS_Code-Install_Server-0098FF?style=flat-square&logo=visualstudiocode&logoColor=white)](https://vscode.dev/redirect/mcp/install?name=artifacthub&config=%7B%22command%22%3A%22artifacthub-mcp%22%7D) [![Install in VS Code Insiders](https://img.shields.io/badge/VS_Code_Insiders-Install_Server-24bfa5?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=artifacthub&config=%7B%22command%22%3A%22artifacthub-mcp%22%7D&quality=insiders)

For manual installation, add this to `.mcp.json` at your project root:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

For user-profile configuration, run **MCP: Open User Configuration** and add the server to the file's top-level `servers` object.

</details>

Check that `artifacthub-mcp --version` works before connecting a client. If the client cannot find the binary, use its absolute path. OpenCode takes a command array, so replace its first item.

Registering the server does not test the connection. In Claude Code or Codex, use `/mcp` to check its status.

The [setup docs](https://artifacthub-mcp.luxass.dev) have more details. Client references: [Claude Code](https://code.claude.com/docs/en/mcp), [Codex](https://developers.openai.com/codex/mcp/), [Cursor](https://cursor.com/docs/context/mcp), [OpenCode](https://opencode.ai/docs/mcp-servers/), and [VS Code](https://code.visualstudio.com/docs/copilot/customization/mcp-servers).

## Tools

### Discovery

| Tool | Description |
|------|-------------|
| `search_packages` | Search for packages by query, kind, repo, or org |
| `search_repositories` | Search repositories by name, kind, user, or org |

Search filters such as `kind`, `repo`, and `org` take arrays. For example:

```json
{"q": "nginx", "kind": ["helm"], "facets": true, "limit": 5}
```

Set `facets` to `true` to include Hub's grouped counts in `search_packages` results.

### Package Details

| Tool | Description |
|------|-------------|
| `get_package` | Get metadata summary for a package |
| `get_package_readme` | Get the README content for a package |
| `get_package_versions` | List all available versions for a package |
| `get_package_changelog` | Get changelog between versions (JSON) |
| `get_changelog_md` | Get changelog as pre-formatted markdown |
| `get_package_star_stats` | View package star count |

### Helm Charts

| Tool | Description |
|------|-------------|
| `get_package_values` | Extract `values.yaml` from a Helm chart |
| `get_package_values_schema` | Get JSON schema for Helm chart values using `package_id` and `version` from `get_package` |
| `get_package_templates` | List chart template names and metadata using `package_id` and `version` from `get_package` |
| `get_package_template` | Get one decoded chart template by exact name after listing templates |
| `get_package_template_data` | Get only decoded chart template source text by exact name |

### Security

| Tool | Description |
|------|-------------|
| `get_package_security_report` | Get detailed security report with CVEs using `package_id` and `version` from `get_package` |

## Examples

Once connected, ask your assistant things like:

- "Find me a Helm chart for PostgreSQL"
- "What Helm repositories does Bitnami maintain?"
- "What versions of cert-manager are available?"
- "Show me the changelog from nginx 1.0.0 to 1.1.0"
- "Get the values.yaml for prometheus-community/prometheus"
- "Show me the values schema for the nginx chart"
- "What Kubernetes resources does the redis chart create?"
- "Are there any security vulnerabilities in the latest envoy chart?"
- "How many stars does the falco chart have?"

## Supported Package Kinds

Helm charts, including charts hosted in OCI registries, use `helm`.
See the [kind mappings](../artifacthub-client/src/kind.rs) for all supported kinds and aliases.

## Contributing

Contributions are welcome! Here are some ways to help:

- Report bugs or request features via [GitHub Issues](https://github.com/luxass/artifacthub-rs/issues)
- Submit pull requests for bug fixes or new tools
- Improve documentation and examples

### Development

```sh
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Run all CI checks
just ci
```

## License

Published under [MIT License](../../LICENSE).
