//! Types used in both requests and responses of the tax invoice endpoints.
//!
//! Enums that can appear in a response have an `Unknown` variant for values this crate does not
//! know yet. `Unknown` is never a success state; never send it in a request either.

use serde::{Deserialize, Serialize};

/// 과세 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxationType {
    /// 과세
    Taxable,
    /// 영세
    ZeroRated,
    /// 면세
    Free,
    #[serde(other)]
    Unknown,
}

/// 발행 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssuanceType {
    /// 정발행
    Normal,
    /// 역발행
    Reverse,
    #[serde(other)]
    Unknown,
}

/// 영수/청구
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PurposeType {
    /// 영수
    Receipt,
    /// 청구
    Invoice,
    /// 없음
    None,
    #[serde(other)]
    Unknown,
}

/// 세금계산서 상태. 알 수 없는 값은 [`TaxInvoiceStatus::Unknown`] 이며 발행 성공으로 보지 않는다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxInvoiceStatus {
    Drafted,
    DraftPending,
    DraftFailed,
    Requested,
    RequestPending,
    RequestFailed,
    RequestCancelled,
    RequestRefused,
    Issued,
    IssuePending,
    IssueFailed,
    IssuanceCancelled,
    CancelPending,
    BeforeSending,
    WaitingSending,
    SendingPending,
    Sending,
    SendingCompleted,
    SendingFailed,
    #[serde(other)]
    Unknown,
}

/// 수정 세금계산서 사유
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModificationType {
    CorrectionOfEntryErrors,
    ChangeInSupplyCost,
    Return,
    CancellationOfContract,
    DuplicateIssuanceDueToError,
    #[serde(other)]
    Unknown,
}

/// 세금계산서 식별자 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxInvoiceKeyType {
    /// 공급자용 문서 번호
    Supplier,
    /// 공급받는자용 문서 번호
    Recipient,
    /// 세금계산서 아이디
    TaxInvoiceId,
    #[serde(other)]
    Unknown,
}

impl TaxInvoiceKeyType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Supplier => "SUPPLIER",
            Self::Recipient => "RECIPIENT",
            Self::TaxInvoiceId | Self::Unknown => "TAX_INVOICE_ID",
        }
    }
}

/// 세금계산서 담당자
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<String>,
    pub email: String,
}

/// 공급자·공급받는자 정보
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    /// 사업자등록번호 (`-` 제외 10자리)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brn: Option<String>,
    /// 거래처 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_id: Option<String>,
    /// 종사업자 식별 번호 (4자리)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 업태
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    /// 종목
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}

/// 추가 담당자
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// 소수 (`value * 10^-scale`, 예: 123.45 는 `value` 12345, `scale` 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal {
    pub value: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<i32>,
}

/// 품목 (최대 99개)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// 결제일 (`yyyy-MM-dd`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 규격
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    /// 단가
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_cost_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}
