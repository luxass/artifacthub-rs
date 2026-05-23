set shell := ["bash", "-c"]

_default:
  @just --list

# ==================== DEVELOPMENT ====================

# Check code compiles
check:
  cargo check

# Run tests
test:
  cargo test

# Format code
fmt:
  cargo fmt

# Run clippy
lint:
  cargo clippy -- -D warnings

# Run all CI checks
ci:
  cargo fmt --check
  just lint
  just test

# ==================== RELEASE ====================

# Release a new version: just release 0.2.1
release version:
  #!/usr/bin/env bash
  set -euo pipefail

  # Validate version format
  if ! echo "{{version}}" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "Error: Invalid version format '{{version}}'. Use semver (e.g., 0.2.1)"
    exit 1
  fi

  # Check working tree is clean
  if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Working tree is not clean. Commit or stash changes first."
    exit 1
  fi

  current_version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
  echo "Current version: $current_version"
  echo "New version: {{version}}"

  # Update version in Cargo.toml
  sed -i '' "s/^version = \".*\"/version = \"{{version}}\"/" Cargo.toml

  # Let cargo update lockfile to match
  cargo check --quiet

  # Commit changes
  git add Cargo.toml Cargo.lock
  git commit -m "chore: update to v{{version}}"

  # Create and push tag
  git tag "v{{version}}"
  git push origin main
  git push origin "v{{version}}"

  echo "Released v{{version}}"
  echo "GitHub Actions will now build and publish to crates.io"
