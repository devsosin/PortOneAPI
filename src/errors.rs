use reqwest::StatusCode;
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
            _ => Self::UnknownError,
        }
    }
}
