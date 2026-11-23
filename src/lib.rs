//! PortOne V2 REST API client.
//!
//! Start with [`PortOneAPI::new`] and [`PortOneAPI::authed_api`]; each feature adds its
//! calls to [`AuthedPortOneAPI`] through a trait in [`features`]. Every call returns
//! [`errors::Error`]. API reference: `docs/rest-v2/README.md`.

pub mod client;
pub mod errors;
pub mod features;
pub mod types;

use reqwest::Client;

/// Kept at the old path; the module moved to [`features::webhook`].
pub use features::webhook;

pub struct PortOneAPI {
    client: Client,
}

pub struct AuthedPortOneAPI {
    api: PortOneAPI,
    key: String,
    base_url: String,
}
