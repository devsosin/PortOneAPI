//! Crate-level error type.
//!
//! [`Error`] is what every public call returns. Features whose failures need more detail
//! (`cancel`, `receipt`, `webhook`) define their own error enum next to the feature and convert it
//! into [`Error`] with `From`.

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use crate::features::{cancel::CancelError, receipt::ReceiptError, webhook::WebhookError};

/// Result alias used by every request method.
pub type Result<T> = std::result::Result<T, Error>;

/// Former name of [`Error`], kept so existing code keeps compiling.
pub type ClientError = Error;

/// Any failure of this crate.
///
/// Reactions: `ReqwestError` (network/timeout) may be retried only for reads or requests
/// sent with an idempotency key; `AuthError` and `PortOneError::Unauthorized`/`Forbidden`
/// mean the API key must be fixed; `Webhook` means the message must be rejected.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    AuthError(#[from] AuthError),

    #[error("PortOne error: {0}")]
    PortOneError(#[from] PortOneError),

    #[error("Cancel error: {0}")]
    Cancel(#[from] CancelError),

    #[error("Cash receipt error: {0}")]
    Receipt(#[from] ReceiptError),

    #[error("Webhook error: {0}")]
    Webhook(#[from] WebhookError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidTokenInfo,
}

/// Error responses shared by every endpoint. Endpoint specific rejections are
/// [`PortOneError::Unclassified`] until the feature turns them into its own error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PortOneError {
    #[error("Invalid Request Body")]
    InvalidRequestError, // 400

    #[error("Unauthorized")]
    UnauthorizedError, // 401

    #[error("Forbidden Request")]
    ForbiddenError, // 403

    #[error("Payment Not Found")]
    PaymentNotFoundError, // 404

    #[error("Idempotent request is still outstanding")]
    IdempotencyOutstandingRequestError, // 409

    #[error("PG provider error: {code:?} {message:?}")]
    PgProviderError {
        code: Option<String>,
        message: Option<String>,
    }, // 502

    /// The body named an error `type` that is not shared by all endpoints.
    #[error("Unclassified PortOne error: {kind} (status {status})")]
    Unclassified { status: u16, kind: String },

    #[error("Unknown")]
    UnknownError,
}

impl PortOneError {
    /// 상태 코드와 오류 응답 본문(`{"type": ...}`)으로 오류를 구분한다.
    /// 본문이 없으면 상태 코드만으로 매핑하고, 공통이 아닌 `type` 은 [`Self::Unclassified`] 로 둔다.
    pub fn from_response(code: StatusCode, body: &str) -> Self {
        let Some(parsed) = serde_json::from_str::<ErrorBody>(body).ok() else {
            return code.into();
        };

        match parsed.r#type {
            Some(kind) => match kind.as_str() {
                "InvalidRequestError" => Self::InvalidRequestError,
                "UnauthorizedError" => Self::UnauthorizedError,
                "ForbiddenError" => Self::ForbiddenError,
                "PaymentNotFoundError" => Self::PaymentNotFoundError,
                "IDEMPOTENCY_OUTSTANDING_REQUEST" => Self::IdempotencyOutstandingRequestError,
                "PgProviderError" => Self::PgProviderError {
                    code: parsed.pg_code,
                    message: parsed.pg_message,
                },
                _ => Self::Unclassified {
                    status: code.as_u16(),
                    kind,
                },
            },
            None => code.into(),
        }
    }
}

impl From<StatusCode> for PortOneError {
    fn from(code: StatusCode) -> Self {
        match code.as_u16() {
            400 => Self::InvalidRequestError,
            401 => Self::UnauthorizedError,
            403 => Self::ForbiddenError,
            404 => Self::PaymentNotFoundError,
            502 => Self::PgProviderError {
                code: None,
                message: None,
            },
            _ => Self::UnknownError,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    r#type: Option<String>,
    #[serde(rename = "pgCode")]
    pg_code: Option<String>,
    #[serde(rename = "pgMessage")]
    pg_message: Option<String>,
}
