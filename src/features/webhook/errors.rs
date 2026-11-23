use thiserror::Error;

/// Why a webhook could not be verified. Every variant means the message must be rejected.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum WebhookError {
    #[error("Invalid webhook secret")]
    InvalidSecret,

    #[error("Missing webhook header: {0}")]
    MissingHeader(&'static str),

    #[error("Malformed webhook timestamp")]
    MalformedTimestamp,

    #[error("Malformed webhook signature header")]
    MalformedSignature,

    #[error("Webhook timestamp is too old")]
    TimestampTooOld,

    #[error("Webhook timestamp is too far in the future")]
    TimestampTooNew,

    #[error("Webhook signature mismatch")]
    SignatureMismatch,
}
