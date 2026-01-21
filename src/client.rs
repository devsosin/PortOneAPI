use std::time::Duration;

use reqwest::{ClientBuilder, StatusCode, header};
use serde::Deserialize;

use crate::{
    AuthedPortOneAPI, PortOneAPI,
    errors::PortOneError,
    types::{ClientResult, PortOneResponse},
};

const PORTONE_HOST: &str = "api.portone.io";

impl PortOneAPI {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: ClientBuilder::new()
                .user_agent("PortOne-Rust/1.0.0")
                .default_headers(headers)
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP Client"),
        }
    }

    pub fn authed_api<'a>(&'a self, key: &'a str) -> AuthedPortOneAPI<'a> {
        AuthedPortOneAPI { api: self, key }
    }
}

impl<'a> AuthedPortOneAPI<'a> {
    pub async fn send<U: for<'de> Deserialize<'de>>(
        &self,
        uri: &str,
    ) -> ClientResult<PortOneResponse<U>> {
        let res = self
            .api
            .client
            .get(format!("https://{}/{}", PORTONE_HOST, uri))
            .header("Authorization", format!("PortOne {}", &self.key))
            .send()
            .await?;

        if res.status().ne(&StatusCode::OK) {
            let err: PortOneError = res.status().into();
            return Err(err.into());
        }

        // let message = res.text().await.unwrap();
        // println!("{}", message);
        let response = res.json().await?;
        Ok(response)
    }
}
