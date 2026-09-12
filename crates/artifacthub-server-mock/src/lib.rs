//! Faithful mock of the Artifact Hub API for tests.
//!
//! Parity contract: mirrors `/tmp/hub` (`artifacthub/hub` @ `248d76e6`)
//! route table (`internal/handlers/handlers.go:268-297`) so tests prove
//! wire parity instead of locking in subsets. Unknown versions 404 via
//! wiremock fallthrough, like hub's `get_package.sql` no-row 404.

#![recursion_limit = "256"]

mod fixtures;
mod routes;
mod server;

pub use fixtures::package_snapshot;
pub use server::HubMockServer;
