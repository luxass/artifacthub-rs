//! Rust client library for the Artifact Hub API.
//!
//! Provides an HTTP client for making requests to the Artifact Hub API
//! and package kind mappings.

/// HTTP client for the Artifact Hub API.
pub mod client;

/// Endpoint namespaces (packages, repositories, helm, stats, security).
pub mod endpoints;

/// Package kind mappings (Helm, Falco, OPA, etc.).
pub mod kind;

/// Response models for Artifact Hub API endpoints.
pub mod models;

/// Request parameter types for endpoint methods.
pub mod params;

pub use client::{ArtifactHubClient, ArtifactHubClientBuilder};
