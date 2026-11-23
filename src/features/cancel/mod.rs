pub mod dto;
mod errors;

use crate::{
    AuthedPortOneAPI,
    errors::PortOneError,
    features::cancel::dto::{CancelPaymentBody, CancelPaymentResponse},
    types::{ClientResult, PortOneResponse},
};

pub use errors::CancelError;

pub trait PortOneCancelService: Send + Sync {
    /// 결제를 취소한다. `body` 의 `amount` 가 없으면 전액, 있으면 부분 취소이다.
    ///
    /// `idempotency_key` 를 주면 `Idempotency-Key` 헤더로 보내 재시도 시 중복 취소를 막는다.
    /// 16~256자의 ASCII 문자열이어야 하며, 같은 키로 다른 요청을 보내서는 안 된다.
    fn cancel_payment(
        &self,
        payment_id: &str,
        body: &CancelPaymentBody,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<PortOneResponse<CancelPaymentResponse>>> + Send;
}

impl PortOneCancelService for AuthedPortOneAPI {
    async fn cancel_payment(
        &self,
        payment_id: &str,
        body: &CancelPaymentBody,
        idempotency_key: Option<&str>,
    ) -> ClientResult<PortOneResponse<CancelPaymentResponse>> {
        if let Some(key) = idempotency_key
            && !is_valid_idempotency_key(key)
        {
            return Err(PortOneError::InvalidRequestError.into());
        }

        self.send_post(&["payments", payment_id, "cancel"], body, idempotency_key)
            .await
            .map_err(CancelError::refine)
    }
}

fn is_valid_idempotency_key(key: &str) -> bool {
    (16..=256).contains(&key.len()) && key.chars().all(|c| c.is_ascii_graphic() && c != '"')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_key_rules() {
        assert!(is_valid_idempotency_key("0123456789abcdef"));
        assert!(!is_valid_idempotency_key("short"));
        assert!(!is_valid_idempotency_key(&"a".repeat(257)));
        assert!(!is_valid_idempotency_key("0123456789abcde\""));
        assert!(!is_valid_idempotency_key("0123456789abcde f"));
        assert!(!is_valid_idempotency_key("0123456789abcdé!"));
    }
}
