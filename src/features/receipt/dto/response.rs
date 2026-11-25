use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::request::{CashReceiptType, PgProvider};

/// 선택된 채널 타입. 알 수 없는 값은 `Unknown` 이며 `LIVE` 로 취급하지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum SelectedChannelType {
    #[serde(rename = "LIVE")]
    Live,
    #[serde(rename = "TEST")]
    Test,
    #[serde(other)]
    Unknown,
}

/// 채널의 PG사. 문서에 없는 값은 원문을 담은 `Unknown` 이다(성공·실연동으로 취급하지 않는다).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum SelectedPgProvider {
    Known(PgProvider),
    Unknown(String),
}

/// 선택된 채널 정보 (`docs/rest-v2/00-basics/schemas-common.md` `SelectedChannel`)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedChannel {
    #[serde(rename = "type")]
    pub channel_type: SelectedChannelType,
    pub id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub pg_provider: SelectedPgProvider,
    pub pg_merchant_id: String,
}

impl SelectedChannel {
    /// 실 연동 채널인지. 알 수 없는 타입은 `false`.
    pub fn is_live(&self) -> bool {
        self.channel_type == SelectedChannelType::Live
    }
}

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
    /// 발급에 사용된 채널
    pub channel: SelectedChannel,
    pub amount: i64,
    pub tax_free_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub currency: String,
    pub order_name: String,
    pub is_manual: bool,
    #[serde(rename = "type")]
    pub receipt_type: Option<CashReceiptType>,
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
    pub channel: SelectedChannel,
    pub amount: i64,
    pub tax_free_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub currency: String,
    pub order_name: String,
    pub is_manual: bool,
    #[serde(rename = "type")]
    pub receipt_type: Option<CashReceiptType>,
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
    pub channel: Option<SelectedChannel>,
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

/// 페이지 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub number: i32,
    pub size: i32,
    pub total_count: i32,
}

/// `GET /cash-receipts` 응답
#[derive(Debug, Clone, Deserialize)]
pub struct GetCashReceiptsResponse {
    pub items: Vec<CashReceipt>,
    pub page: PageInfo,
}
