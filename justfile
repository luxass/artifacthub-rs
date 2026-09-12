_default:
  @just --list

# ==================== DEVELOPMENT ====================

# Check code compiles
check:
  cargo check

# Run tests
test:
  cargo test --locked

# Test the live sweep's comparison logic without network access
sweep-test:
  python3 -B -m unittest discover -s "{{justfile_directory()}}/scripts" -p "test_*.py"

# Run e2e tests against real Artifact Hub API
e2e:
  cargo test --locked --features e2e --test e2e -- --include-ignored

# Run live-API conformance sweep over all MCP tools (requires network)
sweep:
  cargo build --locked
  python3 "{{justfile_directory()}}/scripts/mcp_sweep.py" "{{justfile_directory()}}/target/debug/artifacthub-mcp"

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
