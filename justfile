_default:
  @just --list

# ==================== DEVELOPMENT ====================

# Check code compiles
check:
  cargo check

# Run tests
test:
  cargo test --locked

# Run e2e tests against real Artifact Hub API
e2e:
  cargo test --locked --features e2e --test e2e -- --include-ignored

# Diff Hub API vs MCP server: just compare-mcp
compare-mcp:
  cargo build --locked
  python3 "{{justfile_directory()}}/scripts/mcp_compare.py"

# Build
build:
  cargo build --locked

# Format code
fmt:
  cargo fmt

fmt-check:
  cargo fmt --check

# Run clippy
lint:
  cargo clippy -- -D warnings

# Run all CI checks
ci:
  just fmt-check
  just lint
  just test

# ==================== LOCAL INSTALL ====================

# Build debug binary and symlink into ~/.local/bin for opencode
link:
  cargo build --locked
  mkdir -p ~/.local/bin
  ln -sfn "{{justfile_directory()}}/target/debug/artifacthub-mcp" ~/.local/bin/artifacthub-mcp
  @echo "linked ~/.local/bin/artifacthub-mcp"

# Remove ~/.local/bin symlink
unlink:
  rm -f ~/.local/bin/artifacthub-mcp
  @echo "unlinked ~/.local/bin/artifacthub-mcp"
