use reqwest::Client;

pub mod client;
pub mod errors;
pub mod features;
pub mod types;

pub struct PortOneAPI {
    client: Client,
}

pub struct AuthedPortOneAPI<'a> {
    api: &'a PortOneAPI,
    key: &'a str,
}
