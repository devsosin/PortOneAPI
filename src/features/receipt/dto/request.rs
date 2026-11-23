use chrono::{DateTime, Utc};
use serde::Serialize;

/// 현금영수증 유형 (`PERSONAL` 소득공제용, `CORPORATE` 지출증빙용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
