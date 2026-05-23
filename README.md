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

### OpenCode

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

### Claude Code

Add to your `.mcp.json` or `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

### Codex

Add to your `codex.json`:

```json
{
  "mcpServers": {
    "artifacthub": {
      "command": "artifacthub-mcp"
    }
  }
}
```

## Tools

| Tool | Description |
|------|-------------|
| `search_packages` | Search for packages by query, kind, repo, or org |
| `search_repositories` | Search repositories by name, kind, user, or org |
| `get_package` | Get metadata summary for a package |
| `get_package_readme` | Get the README content for a package |
| `get_package_versions` | List all available versions for a package |
| `get_package_changelog` | Get changelog between versions (JSON) |
| `get_changelog_md` | Get changelog as pre-formatted markdown |
| `get_package_star_stats` | View star history and growth |
| `get_package_values` | Extract `values.yaml` from a Helm chart |
| `get_package_values_schema` | Get JSON schema for Helm chart values |
| `get_package_security_report` | Get detailed security report with CVEs |
| `get_package_templates` | List Kubernetes resources a chart creates |

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

## License

Published under [MIT License](./LICENSE).
