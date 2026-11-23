use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    AuthError(#[from] AuthError),

    #[error("PortOne error: {0}")]
    PortOneError(#[from] PortOneError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidTokenInfo,
}

#[derive(Debug, Error)]
pub enum PortOneError {
    #[error("Invalid Request Body")]
    InvalidRequestError, // 400

    #[error("Unauthorized")]
    UnauthorizedError, // 401

    #[error("Forbidden Request")]
    ForbiddenError, // 403

    #[error("Payment Not Found")]
    PaymentNotFoundError, // 404

    #[error("Payment Not Paid")]
    PaymentNotPaidError, // 409

    #[error("Payment Already Cancelled")]
    PaymentAlreadyCancelledError, // 409

    #[error("Cancellable Amount Consistency Broken")]
    CancellableAmountConsistencyBrokenError, // 409

    #[error("Cancel Amount Exceeds Cancellable Amount")]
    CancelAmountExceedsCancellableAmountError, // 409

    #[error("Sum Of Parts Exceeds Cancel Amount")]
    SumOfPartsExceedsCancelAmountError, // 409

    #[error("Cancel Tax Free Amount Exceeds Cancellable Tax Free Amount")]
    CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError, // 409

    #[error("Cancel Tax Amount Exceeds Cancellable Tax Amount")]
    CancelTaxAmountExceedsCancellableTaxAmountError, // 409

    #[error("Max Cancel Count Reached")]
    MaxCancelCountReachedError, // 409

    #[error("Negative Promotion Adjusted Cancel Amount")]
    NegativePromotionAdjustedCancelAmountError, // 400

    #[error("Promotion Discount Retain Option Should Not Be Changed")]
    PromotionDiscountRetainOptionShouldNotBeChangedError, // 400

    #[error("Idempotent request is still outstanding")]
    IdempotencyOutstandingRequestError, // 409

    #[error("PG provider error: {code:?} {message:?}")]
    PgProviderError {
        code: Option<String>,
        message: Option<String>,
    }, // 502

    #[error("Unknown")]
    UnknownError,
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

impl PortOneError {
    /// 상태 코드와 오류 응답 본문(`{"type": ...}`)으로 오류를 구분한다.
    /// 본문이 없거나 알 수 없는 `type` 이면 상태 코드만으로 매핑한다.
    pub fn from_response(code: StatusCode, body: &str) -> Self {
        let parsed: Option<ErrorBody> = serde_json::from_str(body).ok();
        let Some(parsed) = parsed else {
            return code.into();
        };

        match parsed.r#type.as_deref() {
            Some("InvalidRequestError") => Self::InvalidRequestError,
            Some("UnauthorizedError") => Self::UnauthorizedError,
            Some("ForbiddenError") => Self::ForbiddenError,
            Some("PaymentNotFoundError") => Self::PaymentNotFoundError,
            Some("PaymentNotPaidError") => Self::PaymentNotPaidError,
            Some("PaymentAlreadyCancelledError") => Self::PaymentAlreadyCancelledError,
            Some("CancellableAmountConsistencyBrokenError") => {
                Self::CancellableAmountConsistencyBrokenError
            }
            Some("CancelAmountExceedsCancellableAmountError") => {
                Self::CancelAmountExceedsCancellableAmountError
            }
            Some("SumOfPartsExceedsCancelAmountError") => Self::SumOfPartsExceedsCancelAmountError,
            Some("CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError") => {
                Self::CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError
            }
            Some("CancelTaxAmountExceedsCancellableTaxAmountError") => {
                Self::CancelTaxAmountExceedsCancellableTaxAmountError
            }
            Some("MaxCancelCountReachedError") => Self::MaxCancelCountReachedError,
            Some("NegativePromotionAdjustedCancelAmountError") => {
                Self::NegativePromotionAdjustedCancelAmountError
            }
            Some("PromotionDiscountRetainOptionShouldNotBeChangedError") => {
                Self::PromotionDiscountRetainOptionShouldNotBeChangedError
            }
            Some("IDEMPOTENCY_OUTSTANDING_REQUEST") => Self::IdempotencyOutstandingRequestError,
            Some("PgProviderError") => Self::PgProviderError {
                code: parsed.pg_code,
                message: parsed.pg_message,
            },
            _ => code.into(),
        }
    }
}
