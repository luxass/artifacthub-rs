# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.9](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.8...artifacthub-mcp@0.2.9) - 2026-09-12

### 🐛 Bug Fixes

- preserve Helm read data and repair conformance checks ([#30](https://github.com/luxass/artifacthub-rs/pull/30)) (by @luxass)
- align mcp server tools with hub behavior ([#25](https://github.com/luxass/artifacthub-rs/pull/25)) (by @luxass)
- align artifacthub-client with hub server behavior ([#27](https://github.com/luxass/artifacthub-rs/pull/27)) (by @luxass)

### 🧪 Testing

- prove mock catches client drift (expected red) ([#24](https://github.com/luxass/artifacthub-rs/pull/24)) (by @luxass)

### Contributors

* @luxass

## [0.2.8](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.7...artifacthub-mcp@0.2.8) - 2026-09-06

### 🐛 Bug Fixes

- tolerate missing fields in Artifact Hub API responses ([#17](https://github.com/luxass/artifacthub-rs/pull/17)) (by @luxass)

### 🚀 Features

- mark MCP tools as read-only with open-world hint ([#18](https://github.com/luxass/artifacthub-rs/pull/18)) (by @luxass)

### Contributors

* @luxass

## [0.2.7](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.6...artifacthub-mcp@0.2.7) - 2026-05-30

### 🐛 Bug Fixes

- *(mcp)* remove Rust uint schema formats ([#14](https://github.com/luxass/artifacthub-rs/pull/14)) (by @luxass)

### Contributors

* @luxass

## [0.2.6](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.5...artifacthub-mcp@0.2.6) - 2026-05-30

### 🚜 Refactor

- route MCP tools through typed client ([#12](https://github.com/luxass/artifacthub-rs/pull/12)) (by @luxass)

### Contributors

* @luxass

## [0.2.5](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.4...artifacthub-mcp@0.2.5) - 2026-05-28

### 🐛 Bug Fixes

- require versions for package id endpoints (by @luxass)
- align Artifact Hub search and metadata parsing (by @luxass)

### 🚀 Features

- add focused Helm template tools (by @luxass)

### Contributors

* @luxass

## [0.2.4](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.3...artifacthub-mcp@0.2.4) - 2026-05-25

### 🚀 Features

- *(artifacthub-client)* restructure client resources ([#9](https://github.com/luxass/artifacthub-rs/pull/9)) (by @luxass)

### Contributors

* @luxass

## [0.2.2](https://github.com/luxass/artifacthub-rs/compare/artifacthub-mcp@0.2.1...artifacthub-mcp@0.2.2) - 2026-05-25

### 🐛 Bug Fixes

- add artifacthub-client to workspace dependencies with version constraint (by @luxass)

### 🚜 Refactor

- move response models to artifacthub-client crate ([#4](https://github.com/luxass/artifacthub-rs/pull/4)) (by @luxass)
- use workspace dependency inheritance (by @luxass)
- convert to workspace with separate client and server crates (by @luxass)

### Contributors

* @luxass
