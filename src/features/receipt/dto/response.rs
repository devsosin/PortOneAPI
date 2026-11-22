use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

/// 발급 성공 응답 요약
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashReceiptSummary {
    pub issue_number: String,
    pub url: String,
    pub pg_receipt_id: String,
}

/// `POST /cash-receipts` 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCashReceiptResponse {
    pub cash_receipt: CashReceiptSummary,
}

/// `POST /payments/{paymentId}/cash-receipt/cancel` 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelCashReceiptResponse {
    pub cancelled_amount: i64,
    pub cancelled_at: DateTime<Utc>,
}

/// 발급 완료
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuedCashReceipt {
    pub merchant_id: String,
    pub store_id: String,
    pub payment_id: String,
    /// 발급에 사용된 채널(스키마가 넓어 원본 JSON 유지)
    pub channel: Value,
    pub amount: i64,
    pub tax_free_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub currency: String,
    pub order_name: String,
    pub is_manual: bool,
    #[serde(rename = "type")]
    pub receipt_type: Option<String>,
    pub pg_receipt_id: Option<String>,
    pub issue_number: String,
    pub url: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status_updated_at: Option<DateTime<Utc>>,
}

/// 발급 취소
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledCashReceipt {
    pub merchant_id: String,
    pub store_id: String,
    pub payment_id: String,
    pub channel: Value,
    pub amount: i64,
    pub tax_free_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub currency: String,
    pub order_name: String,
    pub is_manual: bool,
    #[serde(rename = "type")]
    pub receipt_type: Option<String>,
    pub pg_receipt_id: Option<String>,
    pub issue_number: String,
    pub url: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub cancelled_at: DateTime<Utc>,
    pub status_updated_at: Option<DateTime<Utc>>,
}

/// 발급 실패
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFailedCashReceipt {
    pub merchant_id: String,
    pub store_id: String,
    pub payment_id: String,
    pub channel: Option<Value>,
    pub order_name: String,
    pub is_manual: bool,
    pub status_updated_at: Option<DateTime<Utc>>,
}

/// 현금영수증 내역 (`status` 로 구분)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum CashReceipt {
    #[serde(rename = "ISSUED")]
    Issued(IssuedCashReceipt),
    #[serde(rename = "CANCELLED")]
    Cancelled(CancelledCashReceipt),
    #[serde(rename = "ISSUE_FAILED")]
    IssueFailed(IssueFailedCashReceipt),
}
