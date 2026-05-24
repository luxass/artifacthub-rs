//! MCP server for [Artifact Hub](https://artifacthub.io).
//!
//! Provides tools for searching packages, getting package details,
//! viewing changelogs, and extracting Helm chart values.

/// HTTP client for the Artifact Hub API.
pub mod client;

/// Package kind mappings (Helm, Falco, OPA, etc.).
pub mod kind;

/// MCP tool implementations and server struct.
pub mod tools;
