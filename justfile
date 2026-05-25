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
