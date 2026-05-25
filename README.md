# artifacthub-rs

Rust crates for the [Artifact Hub](https://artifacthub.io) API.

## Crates

| Crate | Description | Version |
|-------|-------------|---------|
| [`artifacthub-client`](crates/artifacthub-client) | Rust client library for the Artifact Hub API | [![crates.io](https://img.shields.io/crates/v/artifacthub-client.svg)](https://crates.io/crates/artifacthub-client) |
| [`artifacthub-mcp`](crates/artifacthub-mcp-server) | MCP server for Artifact Hub | [![crates.io](https://img.shields.io/crates/v/artifacthub-mcp.svg)](https://crates.io/crates/artifacthub-mcp) |

## Development

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

## Contributing

Contributions are welcome! Report bugs or request features via [GitHub Issues](https://github.com/luxass/artifacthub-rs/issues).

## License

Published under [MIT License](./LICENSE).
