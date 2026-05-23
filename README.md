# artifacthub-mcp

MCP server for [Artifact Hub](https://artifacthub.io) — search packages, get details, view changelogs, and extract Helm chart values directly from your AI coding assistant.

## Install

### Homebrew (macOS and Linux)

```sh
brew install luxass/homebrew-tap/artifacthub-mcp
```

### Cargo

```sh
cargo install --locked artifacthub-mcp
```

### GitHub Releases

Pre-built binaries are available for Linux and macOS at [github.com/luxass/artifacthub-mcp/releases](https://github.com/luxass/artifacthub-mcp/releases).

## Setup

Add to your MCP client configuration:

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
claude mcp add artifacthub artifacthub-mcp
```

Or add to your `~/.claude/settings.json`:

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
codex mcp add artifacthub artifacthub-mcp
```

Or add to your `~/.codex/config.toml`:

```toml
[mcp_servers.artifacthub]
command = "artifacthub-mcp"
```

</details>

<details>
<summary>Cursor</summary>

Go to `Cursor Settings` → `MCP` → `Add new MCP Server`. Set type to `command` with the command `artifacthub-mcp`.

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

[![Install in VS Code](https://img.shields.io/badge/VS_Code-Install_Server-0098FF?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=artifacthub&config=%7B%22command%22%3A%22artifacthub-mcp%22%7D) [![Install in VS Code Insiders](https://img.shields.io/badge/VS_Code_Insiders-Install_Server-24bfa5?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=artifacthub&config=%7B%22command%22%3A%22artifacthub-mcp%22%7D&quality=insiders)

For manual installation, add to your User Settings (JSON):

```json
{
  "mcp": {
    "servers": {
      "artifacthub": {
        "command": "artifacthub-mcp"
      }
    }
  }
}
```

</details>

## Tools

### Discovery

| Tool | Description |
|------|-------------|
| `search_packages` | Search for packages by query, kind, repo, or org |
| `search_repositories` | Search repositories by name, kind, user, or org |

### Package Details

| Tool | Description |
|------|-------------|
| `get_package` | Get metadata summary for a package |
| `get_package_readme` | Get the README content for a package |
| `get_package_versions` | List all available versions for a package |
| `get_package_changelog` | Get changelog between versions (JSON) |
| `get_changelog_md` | Get changelog as pre-formatted markdown |
| `get_package_star_stats` | View star history and growth |

### Helm Charts

| Tool | Description |
|------|-------------|
| `get_package_values` | Extract `values.yaml` from a Helm chart |
| `get_package_values_schema` | Get JSON schema for Helm chart values |
| `get_package_templates` | List Kubernetes resources a chart creates |

### Security

| Tool | Description |
|------|-------------|
| `get_package_security_report` | Get detailed security report with CVEs |

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

helm, falco, opa, olm, tekton, krew, helm-plugin, gatekeeper, keptn, tinkerbell, cni, contour, keda, coredns, operator, kubewarden, inspektor-gadget, kubearmor, backstage, headlamp, kpt, kubeescape, argo-template, helm-oci

## Contributing

Contributions are welcome! Here are some ways to help:

- Report bugs or request features via [GitHub Issues](https://github.com/luxass/artifacthub-mcp/issues)
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

Published under [MIT License](./LICENSE).
