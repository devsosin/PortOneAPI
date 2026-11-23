use chrono::{DateTime, Utc};

use super::{HEADER_ID, HEADER_SIGNATURE, HEADER_TIMESTAMP, errors::WebhookError};

/// The three signature headers of one webhook request.
#[derive(Debug, Clone, Copy)]
pub struct WebhookHeaders<'a> {
    pub id: &'a str,
    pub timestamp: &'a str,
    pub signature: &'a str,
}

impl<'a> WebhookHeaders<'a> {
    /// Builds the headers through a lookup function (header name in lowercase -> value),
    /// so any HTTP framework's header map can be used.
    pub fn from_lookup<F>(get: F) -> Result<Self, WebhookError>
    where
        F: Fn(&str) -> Option<&'a str>,
    {
        let need = |name: &'static str| {
            get(name)
                .filter(|v| !v.is_empty())
                .ok_or(WebhookError::MissingHeader(name))
        };
        Ok(Self {
            id: need(HEADER_ID)?,
            timestamp: need(HEADER_TIMESTAMP)?,
            signature: need(HEADER_SIGNATURE)?,
        })
    }
}

/// Result of a successful verification. `id` is unique per event, so callers can use it
/// to deduplicate redelivered webhooks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedWebhook {
    pub id: String,
    pub timestamp: DateTime<Utc>,
}
