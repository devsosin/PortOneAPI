use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 취소 요청 주체.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRequester {
    Customer,
    Admin,
}

/// 프로모션 할인율 유지 옵션.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PromotionDiscountRetainOption {
    Retain,
    Release,
}

/// 계좌 환불이 필요한 결제수단(가상계좌 등)의 환불 계좌.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRefundAccount {
    /// 은행 코드 (예: `KDB`). 문서의 Bank 열거값을 그대로 사용한다.
    pub bank: String,
    pub number: String,
    pub holder_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_phone_number: Option<String>,
}

/// `POST /payments/{paymentId}/cancel` 요청 본문.
///
/// `amount` 를 생략하면 전액 취소, 지정하면 부분 취소이다.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPaymentBody {
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_free_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester: Option<CancelRequester>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promotion_discount_retain_option: Option<PromotionDiscountRetainOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_cancellable_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_account: Option<CancelRefundAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_webhook: Option<bool>,
}

impl CancelPaymentBody {
    /// 전액 취소 본문. 사유는 필수이다.
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            store_id: None,
            amount: None,
            tax_free_amount: None,
            vat_amount: None,
            requester: None,
            promotion_discount_retain_option: None,
            current_cancellable_amount: None,
            refund_account: None,
            refund_email: None,
            skip_webhook: None,
        }
    }

    /// 부분 취소 금액을 지정한다.
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn store_id(mut self, store_id: impl Into<String>) -> Self {
        self.store_id = Some(store_id.into());
        self
    }

    pub fn tax_free_amount(mut self, amount: i64) -> Self {
        self.tax_free_amount = Some(amount);
        self
    }

    pub fn vat_amount(mut self, amount: i64) -> Self {
        self.vat_amount = Some(amount);
        self
    }

    pub fn requester(mut self, requester: CancelRequester) -> Self {
        self.requester = Some(requester);
        self
    }

    pub fn promotion_discount_retain_option(
        mut self,
        option: PromotionDiscountRetainOption,
    ) -> Self {
        self.promotion_discount_retain_option = Some(option);
        self
    }

    /// 요청 이전의 취소 가능 잔액. 서버의 잔액과 일치할 때만 취소가 진행된다.
    pub fn current_cancellable_amount(mut self, amount: i64) -> Self {
        self.current_cancellable_amount = Some(amount);
        self
    }

    pub fn refund_account(mut self, account: CancelRefundAccount) -> Self {
        self.refund_account = Some(account);
        self
    }

    pub fn refund_email(mut self, email: impl Into<String>) -> Self {
        self.refund_email = Some(email.into());
        self
    }

    /// 취소 성공 시 웹훅 전송을 생략한다.
    pub fn skip_webhook(mut self, skip: bool) -> Self {
        self.skip_webhook = Some(skip);
        self
    }
}

/// 결제 취소 내역의 상태.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentCancellationStatus {
    Failed,
    Requested,
    Succeeded,
}

/// 결제 취소 내역. 상태(`status`)별 스키마의 공통 필드를 담는다.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCancellation {
    id: String,
    status: PaymentCancellationStatus,
    pg_cancellation_id: Option<String>,
    total_amount: i64,
    tax_free_amount: i64,
    vat_amount: i64,
    easy_pay_discount_amount: Option<i64>,
    reason: String,
    cancelled_at: Option<DateTime<Utc>>,
    requested_at: DateTime<Utc>,
    receipt_url: Option<String>,
}

impl PaymentCancellation {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_status(&self) -> PaymentCancellationStatus {
        self.status
    }
    pub fn is_succeeded(&self) -> bool {
        self.status == PaymentCancellationStatus::Succeeded
    }
    pub fn get_pg_cancellation_id(&self) -> Option<&str> {
        self.pg_cancellation_id.as_deref()
    }
    pub fn get_total_amount(&self) -> i64 {
        self.total_amount
    }
    pub fn get_tax_free_amount(&self) -> i64 {
        self.tax_free_amount
    }
    pub fn get_vat_amount(&self) -> i64 {
        self.vat_amount
    }
    pub fn get_easy_pay_discount_amount(&self) -> Option<i64> {
        self.easy_pay_discount_amount
    }
    pub fn get_reason(&self) -> &str {
        &self.reason
    }
    pub fn get_cancelled_at(&self) -> Option<DateTime<Utc>> {
        self.cancelled_at
    }
    pub fn get_requested_at(&self) -> DateTime<Utc> {
        self.requested_at
    }
    /// 취소 영수증 URL (취소 완료 상태에서만 존재할 수 있다).
    pub fn get_receipt_url(&self) -> Option<&str> {
        self.receipt_url.as_deref()
    }
}

/// 결제 취소 성공 응답.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelPaymentResponse {
    cancellation: PaymentCancellation,
}

impl CancelPaymentResponse {
    pub fn get_cancellation(&self) -> &PaymentCancellation {
        &self.cancellation
    }
    pub fn into_cancellation(self) -> PaymentCancellation {
        self.cancellation
    }
}
