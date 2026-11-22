use reqwest::{Method, Url};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    AuthedPortOneAPI,
    errors::{ClientError, PortOneError},
    types::ClientResult,
};

const PORTONE_BASE: &str = "https://api.portone.io";

/// 경로 조각과 쿼리로 요청 URL을 만든다. 조각은 퍼센트 인코딩된다.
pub(crate) fn build_url(segments: &[&str], query: &[(&str, String)]) -> ClientResult<Url> {
    let mut url = Url::parse(PORTONE_BASE)
        .map_err(|e| ClientError::InternalError(format!("invalid base url: {e}")))?;
    url.path_segments_mut()
        .map_err(|_| ClientError::InternalError("base url cannot have segments".into()))?
        .extend(segments);
    if !query.is_empty() {
        url.query_pairs_mut()
            .extend_pairs(query.iter().map(|(k, v)| (*k, v.as_str())));
    }
    Ok(url)
}

impl AuthedPortOneAPI {
    /// 메서드·본문을 지정하는 요청. 200 외 응답은 상태 코드별 `PortOneError`로 변환한다.
    pub(crate) async fn request<B: Serialize, U: DeserializeOwned>(
        &self,
        method: Method,
        url: Url,
        body: Option<&B>,
    ) -> ClientResult<U> {
        let mut req = self
            .api
            .client
            .request(method, url)
            .header("Authorization", format!("PortOne {}", &self.key));
        if let Some(body) = body {
            req = req.json(body);
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let err: PortOneError = res.status().into();
            return Err(err.into());
        }
        Ok(res.json().await?)
    }
}
