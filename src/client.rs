use std::time::Duration;

use reqwest::{ClientBuilder, Method, Url, header};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    AuthedPortOneAPI, PortOneAPI,
    errors::{Error, PortOneError},
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
                // 공식 정책: PG 응답 지연을 고려해 읽기 타임아웃은 최소 60초 (docs/rest-v2/policy.md)
                .connect_timeout(Duration::from_secs(10))
                .read_timeout(Duration::from_secs(60))
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

impl Default for PortOneAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthedPortOneAPI {
    /// API 기본 주소를 바꾼다 (프록시·테스트용 서버). 기본값은 `https://api.portone.io`.
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.trim_end_matches('/').into();
        self
    }

    /// GET 요청. `uri` 는 기본 주소 뒤에 그대로 붙는다.
    pub async fn send<U: for<'de> Deserialize<'de>>(
        &self,
        uri: &str,
    ) -> ClientResult<PortOneResponse<U>> {
        let url = Url::parse(&format!("{}/{}", self.base_url, uri))
            .map_err(|e| Error::InternalError(e.to_string()))?;
        self.request::<(), _>(Method::GET, url, None, None).await
    }

    /// JSON 본문을 POST 한다. `path_segments` 는 각각 퍼센트 인코딩되어 경로가 된다.
    /// `idempotency_key` 는 RFC 8941 규칙에 따라 쌍따옴표로 감싸 `Idempotency-Key` 헤더로 보낸다.
    pub async fn send_post<B: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        path_segments: &[&str],
        body: &B,
        idempotency_key: Option<&str>,
    ) -> ClientResult<PortOneResponse<U>> {
        let url = self.build_url(path_segments, &[])?;
        self.request(Method::POST, url, Some(body), idempotency_key)
            .await
    }

    /// 경로 조각과 쿼리로 요청 URL을 만든다. 조각은 퍼센트 인코딩된다.
    pub(crate) fn build_url(
        &self,
        segments: &[&str],
        query: &[(&str, String)],
    ) -> ClientResult<Url> {
        let mut url = Url::parse(&self.base_url)
            .map_err(|e| Error::InternalError(format!("invalid base url: {e}")))?;
        url.path_segments_mut()
            .map_err(|_| Error::InternalError("base url cannot have segments".into()))?
            .pop_if_empty()
            .extend(segments);
        if !query.is_empty() {
            // encodeURIComponent 와 같은 규칙으로 인코딩한다 (공백은 '+' 가 아니라 %20).
            let encoded = query
                .iter()
                .map(|(k, v)| format!("{}={}", encode_component(k), encode_component(v)))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&encoded));
        }
        Ok(url)
    }

    /// 모든 요청의 공통 경로: 인증 헤더, 멱등키, 2xx 외 응답의 오류 변환.
    pub(crate) async fn request<B: Serialize, U: DeserializeOwned>(
        &self,
        method: Method,
        url: Url,
        body: Option<&B>,
        idempotency_key: Option<&str>,
    ) -> ClientResult<U> {
        let mut req = self
            .api
            .client
            .request(method, url)
            .header("Authorization", format!("PortOne {}", self.key));
        if let Some(body) = body {
            req = req.json(body);
        }
        if let Some(key) = idempotency_key {
            req = req.header("Idempotency-Key", format!("\"{key}\""));
        }

        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(PortOneError::from_response(status, &text).into());
        }
        Ok(res.json().await?)
    }
}

/// `encodeURIComponent` 와 같은 퍼센트 인코딩: `A-Za-z0-9 - _ . ! ~ * ' ( )` 만 그대로 둔다.
fn encode_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
