//! Rust client library for the Artifact Hub API.
//!
//! Provides an HTTP client for making requests to the Artifact Hub API
//! and package kind mappings.

/// HTTP client for the Artifact Hub API.
pub mod client;

/// Package kind mappings (Helm, Falco, OPA, etc.).
pub mod kind;
