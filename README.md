# artifacthub-mcp

MCP server for [Artifact Hub](https://artifacthub.io) — search packages, get details, view changelogs, and extract Helm chart values directly from your AI coding assistant.

## Tools

| Tool | Description |
|------|-------------|
| `search_packages` | Search for packages by query, kind, repo, or org |
| `get_package` | Get full details including readme, versions, and maintainers |
| `get_package_versions` | List all available versions for a package |
| `get_package_changelog` | Get changelog between versions |
| `get_package_star_stats` | View star history and growth |
| `get_package_values` | Extract `values.yaml` from a Helm chart |

## Install

### Homebrew (macOS and Linux)

```sh
brew install luxass/homebrew-tap/artifacthub-mcp
```

### Cargo

```sh
cargo install --locked artifacthub-mcp
```

### Install Script

macOS and Linux:

```sh
curl -fsSL https://raw.githubusercontent.com/luxass/artifacthub-mcp/main/install.sh | sh
```

The installer uses Homebrew if available, otherwise downloads the correct release for your platform and installs `artifacthub-mcp` into `~/.local/bin` by default.

You can override the target directory with `ARTIFACTHUB_MCP_INSTALL_DIR`, and pin a specific release with `ARTIFACTHUB_MCP_VERSION`.

## Usage

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

## Examples

Once connected, ask your assistant things like:

- "Find me a Helm chart for PostgreSQL"
- "What versions of cert-manager are available?"
- "Show me the changelog from nginx 1.0.0 to 1.1.0"
- "Get the values.yaml for prometheus-community/prometheus"
- "How many stars does the falco chart have?"

## Supported Package Kinds

helm, falco, opa, olm, tekton, krew, helm-plugin, gatekeeper, keptn, tinkerbell, cni, contour, keda, coredns, operator, kubewarden, inspektor-gadget, kubearmor, backstage, headlamp, kpt, kubeescape, argo-template, helm-oci

## License

MIT
