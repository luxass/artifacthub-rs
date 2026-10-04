---
title: Getting started
navTitle: Getting started
order: 1
description: Install the binary, connect your MCP client, and make your first package lookup.
---

## Install the server

Choose one installation method. These commands install the MCP binary, not this documentation site.

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

### Claude Code

```sh
claude mcp add artifacthub -- artifacthub-mcp
```

### Codex

```sh
codex mcp add artifacthub -- artifacthub-mcp
```

Or add this to `~/.codex/config.toml`:

```toml
[mcp_servers.artifacthub]
command = "artifacthub-mcp"
```

### Clients using mcpServers

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

### OpenCode

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

### VS Code

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
