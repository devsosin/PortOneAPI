//! Tax invoice webhook payload (`docs/rest-v2/webhook/platform-webhook-tax-invoice.md`).
//!
//! The body is `{ "type", "timestamp", "data" }`. Unknown `type` and `status` values are kept as
//! [`TaxInvoiceEventType::Unknown`] / [`TaxInvoiceStatus::Unknown`] and must not be treated as success.

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::errors::WebhookError;
use crate::features::tax_invoice::dto::common::TaxInvoiceStatus;

/// Event that triggered the webhook (`TaxInvoice.*`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "String")]
#[non_exhaustive]
pub enum TaxInvoiceEventType {
    /// `TaxInvoice.Requested`: 역발행 요청 완료
    Requested,
    /// `TaxInvoice.Issued`: 공급자가 발행 승인
    Issued,
    /// `TaxInvoice.RequestCancelled`: 공급받는자가 역발행 요청 취소
    RequestCancelled,
    /// `TaxInvoice.IssuanceCancelled`: 공급자가 역발행 승인 이후 취소
    IssuanceCancelled,
    /// `TaxInvoice.Refused`: 공급자가 역발행 요청 거부
    Refused,
    /// `TaxInvoice.SendingCompleted`: 국세청 전송 완료
    SendingCompleted,
    /// `TaxInvoice.Manual`: 수동 웹훅 재발송
    Manual,
    /// A `type` this crate does not know; carries the raw value.
    Unknown(String),
}

impl From<String> for TaxInvoiceEventType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "TaxInvoice.Requested" => Self::Requested,
            "TaxInvoice.Issued" => Self::Issued,
            "TaxInvoice.RequestCancelled" => Self::RequestCancelled,
            "TaxInvoice.IssuanceCancelled" => Self::IssuanceCancelled,
            "TaxInvoice.Refused" => Self::Refused,
            "TaxInvoice.SendingCompleted" => Self::SendingCompleted,
            "TaxInvoice.Manual" => Self::Manual,
            _ => Self::Unknown(value),
        }
    }
}

/// `data` of a tax invoice webhook.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInvoiceEventData {
    pub tax_invoice_id: String,
    pub supplier_document_key: String,
    pub recipient_document_key: String,
    pub status: TaxInvoiceStatus,
    /// 공급자 사업자등록번호
    pub supplier_brn: String,
    /// 합계금액
    pub total_amount: i64,
    /// 공급가액
    pub total_supply_amount: i64,
    /// 세액
    pub total_tax_amount: i64,
}

/// A parsed tax invoice webhook body.
#[derive(Debug, Clone, Deserialize)]
pub struct TaxInvoiceEvent {
    #[serde(rename = "type")]
    pub event_type: TaxInvoiceEventType,
    pub timestamp: DateTime<Utc>,
    pub data: TaxInvoiceEventData,
}

impl TaxInvoiceEvent {
    /// Parses the raw body without checking any signature.
    ///
    /// Use it only when the deliveries carry no signature headers, and then protect the endpoint
    /// otherwise (IP allowlist, see `docs/research/cash-receipt-tax-invoice.md`). Prefer
    /// `WebhookVerifier::verify_and_parse_tax_invoice`.
    pub fn parse(body: &[u8]) -> Result<Self, WebhookError> {
        serde_json::from_slice(body).map_err(|_| WebhookError::InvalidPayload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = r#"{
        "type": "TaxInvoice.SendingCompleted",
        "timestamp": "2025-03-20T14:25:10Z",
        "data": {
            "taxInvoiceId": "txi-test", "supplierDocumentKey": "sdk-test",
            "recipientDocumentKey": "rdk-test", "status": "SENDING_COMPLETED",
            "supplierBrn": "1234567890", "totalAmount": 110000,
            "totalSupplyAmount": 100000, "totalTaxAmount": 10000
        }
    }"#;

    #[test]
    fn parses_documented_example() {
        let e = TaxInvoiceEvent::parse(BODY.as_bytes()).unwrap();
        assert_eq!(e.event_type, TaxInvoiceEventType::SendingCompleted);
        assert_eq!(e.data.status, TaxInvoiceStatus::SendingCompleted);
        assert_eq!(e.data.total_amount, 110000);
        assert_eq!(e.timestamp.to_rfc3339(), "2025-03-20T14:25:10+00:00");
    }

    #[test]
    fn all_documented_types_are_known() {
        for (raw, want) in [
            ("TaxInvoice.Requested", TaxInvoiceEventType::Requested),
            ("TaxInvoice.Issued", TaxInvoiceEventType::Issued),
            (
                "TaxInvoice.RequestCancelled",
                TaxInvoiceEventType::RequestCancelled,
            ),
            (
                "TaxInvoice.IssuanceCancelled",
                TaxInvoiceEventType::IssuanceCancelled,
            ),
            ("TaxInvoice.Refused", TaxInvoiceEventType::Refused),
            ("TaxInvoice.Manual", TaxInvoiceEventType::Manual),
        ] {
            assert_eq!(TaxInvoiceEventType::from(raw.to_string()), want);
        }
    }

    #[test]
    fn unknown_type_and_status_stay_unknown() {
        let body = BODY
            .replace("TaxInvoice.SendingCompleted", "TaxInvoice.Brand")
            .replace("SENDING_COMPLETED", "BRAND_NEW");
        let e = TaxInvoiceEvent::parse(body.as_bytes()).unwrap();
        assert_eq!(
            e.event_type,
            TaxInvoiceEventType::Unknown("TaxInvoice.Brand".into())
        );
        assert_eq!(e.data.status, TaxInvoiceStatus::Unknown);
    }

    #[test]
    fn rejects_malformed_payload() {
        assert_eq!(
            TaxInvoiceEvent::parse(b"{}").unwrap_err(),
            WebhookError::InvalidPayload
        );
        assert_eq!(
            TaxInvoiceEvent::parse(b"not json").unwrap_err(),
            WebhookError::InvalidPayload
        );
    }
}
