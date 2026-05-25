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

pub use client::{ArtifactHubClient, ArtifactHubClientBuilder};

/// Short SDK-style alias for the Artifact Hub client.
pub type ArtifactHub = ArtifactHubClient;

// Re-export commonly used parameter types for convenience.
pub use endpoints::{
    ChangelogParams, HelmGetParams, PackageGetParams, PackageSearchParams, RepoSearchParams,
    SecurityGetParams, StatsGetParams,
};
