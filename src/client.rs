use std::time::Duration;

use reqwest::{ClientBuilder, StatusCode, header};
use serde::{Deserialize, Serialize};

use crate::{
    AuthedPortOneAPI, PortOneAPI,
    errors::{ClientError, PortOneError},
    types::{ClientResult, PortOneResponse},
};

const PORTONE_BASE_URL: &str = "https://api.portone.io";

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

    pub fn authed_api(self, key: &str) -> AuthedPortOneAPI {
        AuthedPortOneAPI {
            api: self,
            key: key.into(),
            base_url: PORTONE_BASE_URL.into(),
        }
    }
}

impl AuthedPortOneAPI {
    /// API 기본 주소를 바꾼다 (프록시·테스트용 서버). 기본값은 `https://api.portone.io`.
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.trim_end_matches('/').into();
        self
    }

    pub async fn send<U: for<'de> Deserialize<'de>>(
        &self,
        uri: &str,
    ) -> ClientResult<PortOneResponse<U>> {
        let res = self
            .api
            .client
            .get(format!("{}/{}", self.base_url, uri))
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

    /// JSON 본문을 POST 한다. `path_segments` 는 각각 퍼센트 인코딩되어 경로가 된다.
    /// `idempotency_key` 는 RFC 8941 규칙에 따라 쌍따옴표로 감싸 `Idempotency-Key` 헤더로 보낸다.
    pub async fn send_post<B: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        path_segments: &[&str],
        body: &B,
        idempotency_key: Option<&str>,
    ) -> ClientResult<PortOneResponse<U>> {
        let mut url = reqwest::Url::parse(&self.base_url)
            .map_err(|e| ClientError::InternalError(e.to_string()))?;
        url.path_segments_mut()
            .map_err(|_| ClientError::InternalError("invalid base url".into()))?
            .pop_if_empty()
            .extend(path_segments);

        let mut req = self
            .api
            .client
            .post(url)
            .header("Authorization", format!("PortOne {}", &self.key))
            .json(body);
        if let Some(key) = idempotency_key {
            req = req.header("Idempotency-Key", format!("\"{}\"", key));
        }

        let res = req.send().await?;

        let status = res.status();
        if status.ne(&StatusCode::OK) {
            let text = res.text().await.unwrap_or_default();
            let err = PortOneError::from_response(status, &text);
            return Err(err.into());
        }

        let response = res.json().await?;
        Ok(response)
    }
}
