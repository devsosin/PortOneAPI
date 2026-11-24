use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 현금영수증 유형 (`PERSONAL` 소득공제용, `CORPORATE` 지출증빙용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptType {
    Personal,
    Corporate,
}

/// 상품 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentProductType {
    Physical,
    Digital,
}

/// 수동 발급 가능 결제 수단 (일부 PG에서만 입력)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueCashReceiptPaymentMethodType {
    Transfer,
    VirtualAccount,
}

/// 고객 식별값 유형 (갤럭시아머니트리에서만 필요)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentityNumberType {
    Phone,
    Card,
    Business,
}

/// 금액 세부 입력 정보
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmountInput {
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_free: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<i64>,
}

/// 현금영수증 발급 시 고객 정보
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCashReceiptCustomerInput {
    /// 고객 식별값(휴대폰번호, 사업자등록번호 등)
    pub identity_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_number_type: Option<IdentityNumberType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// `POST /cash-receipts` 요청 본문
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCashReceiptBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    pub payment_id: String,
    pub channel_key: String,
    #[serde(rename = "type")]
    pub receipt_type: CashReceiptType,
    pub order_name: String,
    /// ISO 4217 통화 코드 (예: `KRW`)
    pub currency: String,
    pub amount: PaymentAmountInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PaymentProductType>,
    pub customer: IssueCashReceiptCustomerInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<IssueCashReceiptPaymentMethodType>,
}

/// 목록 조회 기준 시점 필드
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptTimeRangeField {
    IssuedAt,
    CancelledAt,
    StatusUpdatedAt,
}

/// 목록 정렬 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptSortBy {
    IssuedAt,
    CancelledAt,
    StatusUpdatedAt,
}

/// 정렬 순서
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    Desc,
    Asc,
}

/// 목록 필터용 발급 상태 (응답의 `ISSUE_FAILED` 와 달리 필터는 `FAILED`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptStatus {
    Issued,
    Cancelled,
    Failed,
}

/// 페이지 입력 (기본 number 0, size 10)
#[derive(Debug, Clone, Default, Serialize)]
pub struct PageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// 정렬 조건 (기본 `ISSUED_AT` `DESC`)
#[derive(Debug, Clone, Default, Serialize)]
pub struct CashReceiptSortInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<CashReceiptSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
}

/// 목록 필터. PG사 모듈·PG사·버전 필터(`pgProviders`, `pgCompanies`, `version`)는 아직 지원하지 않는다.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashReceiptFilterInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_field: Option<CashReceiptTimeRangeField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<CashReceiptStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_receipt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<CashReceiptType>>,
}

/// `GET /cash-receipts` 조회 조건. JSON 으로 직렬화해 `requestBody` 쿼리로 보낸다.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCashReceiptsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CashReceiptSortInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CashReceiptFilterInput>,
}
