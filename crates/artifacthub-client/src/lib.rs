//! Rust client library for the Artifact Hub API.
//!
//! Provides an HTTP client for making requests to the Artifact Hub API
//! and package kind mappings.

/// HTTP client for the Artifact Hub API.
pub mod client;

/// API handlers and operation builders.
pub mod api;

/// Error types returned by the client.
pub mod error;

/// Package kind mappings (Helm, Falco, OPA, etc.).
pub mod kind;

/// Response models for Artifact Hub API endpoints.
pub mod models;

pub use client::{ArtifactHubClient, ArtifactHubClientBuilder};
pub use error::{ArtifactHubError, Result};
