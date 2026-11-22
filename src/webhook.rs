//! Webhook signature verification following the Standard Webhooks specification.
//!
//! The PortOne V2 documentation states that webhook messages follow Standard Webhooks
//! and that verification code may be written according to that spec. Everything below
//! the "from the spec" notes comes from the Standard Webhooks specification, not from
//! the PortOne documentation:
//!
//! - headers `webhook-id`, `webhook-timestamp` (unix seconds), `webhook-signature`
//! - signed content: `"{id}.{timestamp}.{raw body}"`
//! - signature: HMAC-SHA256, base64 encoded, sent as `v1,<base64>`; several
//!   space-separated signatures may be present (secret rotation)
//! - secret: `whsec_` prefix followed by base64 of the key bytes
//!
//! The body must be the raw request bytes, exactly as received.

use std::time::Duration;

use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use thiserror::Error;

pub const HEADER_ID: &str = "webhook-id";
pub const HEADER_TIMESTAMP: &str = "webhook-timestamp";
pub const HEADER_SIGNATURE: &str = "webhook-signature";

/// Default allowed difference between `webhook-timestamp` and the current time.
pub const DEFAULT_TOLERANCE: Duration = Duration::from_secs(5 * 60);

const SECRET_PREFIX: &str = "whsec_";
const SIGNATURE_VERSION: &str = "v1";

type HmacSha256 = Hmac<Sha256>;

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

pub struct WebhookVerifier {
    key: Vec<u8>,
}

impl WebhookVerifier {
    /// Creates a verifier from a webhook secret.
    ///
    /// A `whsec_` prefixed secret is base64-decoded after the prefix (Standard Webhooks
    /// format). A secret without the prefix is used as raw UTF-8 key bytes.
    pub fn new(secret: &str) -> Result<Self, WebhookError> {
        let key = match secret.strip_prefix(SECRET_PREFIX) {
            Some(encoded) => STANDARD
                .decode(encoded)
                .map_err(|_| WebhookError::InvalidSecret)?,
            None => secret.as_bytes().to_vec(),
        };
        Self::from_bytes(key)
    }

    /// Creates a verifier from already decoded key bytes.
    pub fn from_bytes(key: impl Into<Vec<u8>>) -> Result<Self, WebhookError> {
        let key = key.into();
        if key.is_empty() {
            return Err(WebhookError::InvalidSecret);
        }
        Ok(Self { key })
    }

    /// Verifies with the default 5 minute tolerance at the current time.
    pub fn verify(
        &self,
        body: &[u8],
        headers: &WebhookHeaders<'_>,
    ) -> Result<VerifiedWebhook, WebhookError> {
        self.verify_at(body, headers, DEFAULT_TOLERANCE, Utc::now())
    }

    /// Verifies with a caller supplied tolerance at the current time.
    pub fn verify_with_tolerance(
        &self,
        body: &[u8],
        headers: &WebhookHeaders<'_>,
        tolerance: Duration,
    ) -> Result<VerifiedWebhook, WebhookError> {
        self.verify_at(body, headers, tolerance, Utc::now())
    }

    /// Verifies against an explicit `now`. Timestamps older than `now - tolerance` or
    /// newer than `now + tolerance` are rejected.
    pub fn verify_at(
        &self,
        body: &[u8],
        headers: &WebhookHeaders<'_>,
        tolerance: Duration,
        now: DateTime<Utc>,
    ) -> Result<VerifiedWebhook, WebhookError> {
        if headers.id.is_empty() {
            return Err(WebhookError::MissingHeader(HEADER_ID));
        }
        if headers.timestamp.is_empty() {
            return Err(WebhookError::MissingHeader(HEADER_TIMESTAMP));
        }
        if headers.signature.is_empty() {
            return Err(WebhookError::MissingHeader(HEADER_SIGNATURE));
        }

        let seconds: i64 = headers
            .timestamp
            .parse()
            .map_err(|_| WebhookError::MalformedTimestamp)?;
        let timestamp =
            DateTime::from_timestamp(seconds, 0).ok_or(WebhookError::MalformedTimestamp)?;

        let tolerance = i64::try_from(tolerance.as_secs()).unwrap_or(i64::MAX);
        let diff = now.timestamp().saturating_sub(seconds);
        if diff > tolerance {
            return Err(WebhookError::TimestampTooOld);
        }
        if diff < tolerance.saturating_neg() {
            return Err(WebhookError::TimestampTooNew);
        }

        let mut candidates = Vec::new();
        for part in headers.signature.split_ascii_whitespace() {
            let Some((version, encoded)) = part.split_once(',') else {
                return Err(WebhookError::MalformedSignature);
            };
            if version != SIGNATURE_VERSION {
                continue;
            }
            candidates.push(
                STANDARD
                    .decode(encoded)
                    .map_err(|_| WebhookError::MalformedSignature)?,
            );
        }
        if candidates.is_empty() {
            return Err(WebhookError::MalformedSignature);
        }

        let mut matched = false;
        for candidate in &candidates {
            let mac = self.mac(headers.id, headers.timestamp, body);
            // `verify_slice` compares in constant time.
            matched |= mac.verify_slice(candidate).is_ok();
        }
        if !matched {
            return Err(WebhookError::SignatureMismatch);
        }

        Ok(VerifiedWebhook {
            id: headers.id.to_string(),
            timestamp,
        })
    }

    fn mac(&self, id: &str, timestamp: &str, body: &[u8]) -> HmacSha256 {
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC accepts any key length");
        mac.update(id.as_bytes());
        mac.update(b".");
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body);
        mac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fixed vector, signature computed independently with Node.js `crypto` HMAC-SHA256.
    const SECRET: &str = "whsec_plJ3nmyCDGBKInavdOK15jsl";
    const ID: &str = "msg_p5jXN8AQM9LWM0D4loKWxJek";
    const TS: &str = "1614265330";
    const BODY: &[u8] = br#"{"test": 2432232314}"#;
    const SIG: &str = "v1,3Q7B9pz3SlC1/gG4UJ269Qj4TLRRQk5tdhkvlUuj234=";

    fn now() -> DateTime<Utc> {
        DateTime::from_timestamp(1614265330, 0).unwrap()
    }

    fn sign(key: &[u8], id: &str, ts: &str, body: &[u8]) -> String {
        let verifier = WebhookVerifier::from_bytes(key).unwrap();
        let bytes = verifier.mac(id, ts, body).finalize().into_bytes();
        format!("v1,{}", STANDARD.encode(bytes))
    }

    fn headers<'a>(sig: &'a str) -> WebhookHeaders<'a> {
        WebhookHeaders {
            id: ID,
            timestamp: TS,
            signature: sig,
        }
    }

    fn verify(body: &[u8], h: &WebhookHeaders<'_>) -> Result<VerifiedWebhook, WebhookError> {
        WebhookVerifier::new(SECRET)
            .unwrap()
            .verify_at(body, h, DEFAULT_TOLERANCE, now())
    }

    #[test]
    fn fixed_vector_passes_and_exposes_id() {
        let ok = verify(BODY, &headers(SIG)).unwrap();
        assert_eq!(ok.id, ID);
        assert_eq!(ok.timestamp, now());
    }

    #[test]
    fn generated_vector_with_raw_secret_passes() {
        let sig = sign(b"raw-secret", ID, TS, BODY);
        let v = WebhookVerifier::new("raw-secret").unwrap();
        assert!(
            v.verify_at(BODY, &headers(&sig), DEFAULT_TOLERANCE, now())
                .is_ok()
        );
    }

    #[test]
    fn tampered_body_is_rejected() {
        let err = verify(br#"{"test": 2432232315}"#, &headers(SIG)).unwrap_err();
        assert_eq!(err, WebhookError::SignatureMismatch);
    }

    #[test]
    fn body_is_verified_as_raw_bytes() {
        // Re-serialized (whitespace changed) JSON must not verify.
        let err = verify(br#"{"test":2432232314}"#, &headers(SIG)).unwrap_err();
        assert_eq!(err, WebhookError::SignatureMismatch);
    }

    #[test]
    fn tampered_id_or_timestamp_is_rejected() {
        let mut h = headers(SIG);
        h.id = "msg_other";
        assert_eq!(
            verify(BODY, &h).unwrap_err(),
            WebhookError::SignatureMismatch
        );

        let mut h = headers(SIG);
        h.timestamp = "1614265331";
        assert_eq!(
            verify(BODY, &h).unwrap_err(),
            WebhookError::SignatureMismatch
        );
    }

    #[test]
    fn wrong_secret_is_rejected() {
        let v = WebhookVerifier::new("whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap();
        let err = v
            .verify_at(BODY, &headers(SIG), DEFAULT_TOLERANCE, now())
            .unwrap_err();
        assert_eq!(err, WebhookError::SignatureMismatch);
    }

    #[test]
    fn tolerance_boundaries() {
        let v = WebhookVerifier::new(SECRET).unwrap();
        let h = headers(SIG);
        let at = |secs: i64| now() + chrono::Duration::seconds(secs);

        assert!(v.verify_at(BODY, &h, DEFAULT_TOLERANCE, at(300)).is_ok());
        assert_eq!(
            v.verify_at(BODY, &h, DEFAULT_TOLERANCE, at(301))
                .unwrap_err(),
            WebhookError::TimestampTooOld
        );
        assert!(v.verify_at(BODY, &h, DEFAULT_TOLERANCE, at(-300)).is_ok());
        assert_eq!(
            v.verify_at(BODY, &h, DEFAULT_TOLERANCE, at(-301))
                .unwrap_err(),
            WebhookError::TimestampTooNew
        );
    }

    #[test]
    fn tolerance_is_adjustable() {
        let v = WebhookVerifier::new(SECRET).unwrap();
        let h = headers(SIG);
        let later = now() + chrono::Duration::seconds(60);
        assert!(
            v.verify_at(BODY, &h, Duration::from_secs(60), later)
                .is_ok()
        );
        assert_eq!(
            v.verify_at(BODY, &h, Duration::from_secs(10), later)
                .unwrap_err(),
            WebhookError::TimestampTooOld
        );
    }

    #[test]
    fn verify_uses_current_time() {
        let v = WebhookVerifier::new(SECRET).unwrap();
        // The fixed-vector timestamp is years old.
        assert_eq!(
            v.verify(BODY, &headers(SIG)).unwrap_err(),
            WebhookError::TimestampTooOld
        );
        let ts = Utc::now().timestamp().to_string();
        let sig = sign(
            &STANDARD.decode("plJ3nmyCDGBKInavdOK15jsl").unwrap(),
            ID,
            &ts,
            BODY,
        );
        let h = WebhookHeaders {
            id: ID,
            timestamp: &ts,
            signature: &sig,
        };
        assert!(v.verify(BODY, &h).is_ok());
        assert!(
            v.verify_with_tolerance(BODY, &h, Duration::from_secs(1))
                .is_ok()
        );
    }

    #[test]
    fn rotation_accepts_any_matching_signature() {
        let other = sign(b"another-key", ID, TS, BODY);
        let both = format!("{other} {SIG}");
        assert!(verify(BODY, &headers(&both)).is_ok());
        let both = format!("{SIG} {other}");
        assert!(verify(BODY, &headers(&both)).is_ok());
        assert_eq!(
            verify(BODY, &headers(&other)).unwrap_err(),
            WebhookError::SignatureMismatch
        );
    }

    #[test]
    fn unknown_signature_versions_are_skipped() {
        let both = format!("v2,AAAA {SIG}");
        assert!(verify(BODY, &headers(&both)).is_ok());
        assert_eq!(
            verify(BODY, &headers("v2,AAAA")).unwrap_err(),
            WebhookError::MalformedSignature
        );
    }

    #[test]
    fn malformed_headers_are_rejected() {
        let mut h = headers(SIG);
        h.timestamp = "not-a-number";
        assert_eq!(
            verify(BODY, &h).unwrap_err(),
            WebhookError::MalformedTimestamp
        );

        for sig in ["garbage", "v1,!!!not-base64!!!", "v1"] {
            assert_eq!(
                verify(BODY, &headers(sig)).unwrap_err(),
                WebhookError::MalformedSignature,
                "{sig}"
            );
        }
    }

    #[test]
    fn missing_headers_are_rejected() {
        let mut h = headers(SIG);
        h.id = "";
        assert_eq!(
            verify(BODY, &h).unwrap_err(),
            WebhookError::MissingHeader(HEADER_ID)
        );
        assert_eq!(
            verify(BODY, &headers("")).unwrap_err(),
            WebhookError::MissingHeader(HEADER_SIGNATURE)
        );

        let err = WebhookHeaders::from_lookup(|name| match name {
            HEADER_ID => Some(ID),
            HEADER_SIGNATURE => Some(SIG),
            _ => None,
        })
        .unwrap_err();
        assert_eq!(err, WebhookError::MissingHeader(HEADER_TIMESTAMP));

        let h = WebhookHeaders::from_lookup(|name| match name {
            HEADER_ID => Some(ID),
            HEADER_TIMESTAMP => Some(TS),
            HEADER_SIGNATURE => Some(SIG),
            _ => None,
        })
        .unwrap();
        assert!(verify(BODY, &h).is_ok());
    }

    #[test]
    fn invalid_secrets_are_rejected() {
        assert_eq!(
            WebhookVerifier::new("").err(),
            Some(WebhookError::InvalidSecret)
        );
        assert_eq!(
            WebhookVerifier::new("whsec_!!!").err(),
            Some(WebhookError::InvalidSecret)
        );
        assert_eq!(
            WebhookVerifier::new("whsec_").err(),
            Some(WebhookError::InvalidSecret)
        );
    }
}
