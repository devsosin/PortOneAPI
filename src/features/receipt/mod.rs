pub mod dto;

use reqwest::Method;

use crate::{
    AuthedPortOneAPI,
    features::receipt::dto::{
        request::IssueCashReceiptBody,
        response::{CancelCashReceiptResponse, CashReceipt, IssueCashReceiptResponse},
    },
    types::ClientResult,
};

/// 현금영수증 API. 발급·취소는 실제 영수증을 만들거나 취소하므로 호출 전 확인이 필요하다.
pub trait PortOneCashReceiptService: Send + Sync {
    /// `POST /cash-receipts` 수동 발급
    fn issue_cash_receipt(
        &self,
        body: &IssueCashReceiptBody,
    ) -> impl Future<Output = ClientResult<IssueCashReceiptResponse>> + Send;

    /// `GET /payments/{paymentId}/cash-receipt` 결제 건별 조회
    fn get_cash_receipt(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> impl Future<Output = ClientResult<CashReceipt>> + Send;

    /// `POST /payments/{paymentId}/cash-receipt/cancel` 취소
    fn cancel_cash_receipt(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> impl Future<Output = ClientResult<CancelCashReceiptResponse>> + Send;
}

impl PortOneCashReceiptService for AuthedPortOneAPI {
    async fn issue_cash_receipt(
        &self,
        body: &IssueCashReceiptBody,
    ) -> ClientResult<IssueCashReceiptResponse> {
        let url = self.build_url(&["cash-receipts"], &[])?;
        self.request(Method::POST, url, Some(body), None).await
    }

    async fn get_cash_receipt(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> ClientResult<CashReceipt> {
        let url = self.build_url(
            &["payments", payment_id, "cash-receipt"],
            &receipt_query(store_id),
        )?;
        self.request::<(), _>(Method::GET, url, None, None).await
    }

    async fn cancel_cash_receipt(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> ClientResult<CancelCashReceiptResponse> {
        let url = self.build_url(
            &["payments", payment_id, "cash-receipt", "cancel"],
            &receipt_query(store_id),
        )?;
        self.request::<(), _>(Method::POST, url, None, None).await
    }
}

fn receipt_query(store_id: Option<&str>) -> Vec<(&'static str, String)> {
    store_id
        .map(|s| vec![("storeId", s.to_string())])
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    fn test_api() -> crate::AuthedPortOneAPI {
        crate::PortOneAPI::new().authed_api("k")
    }

    use super::*;
    use crate::features::receipt::dto::request::{
        CashReceiptType, IssueCashReceiptCustomerInput, PaymentAmountInput,
    };
    use serde_json::json;

    #[test]
    fn query_only_when_store_id_given() {
        assert!(receipt_query(None).is_empty());
        let url = test_api()
            .build_url(
                &["payments", "p 1", "cash-receipt"],
                &receipt_query(Some("s1")),
            )
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/payments/p%201/cash-receipt?storeId=s1"
        );
    }

    #[test]
    fn path_segment_cannot_escape() {
        let url = test_api()
            .build_url(&["payments", "a/b", "cash-receipt", "cancel"], &[])
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/payments/a%2Fb/cash-receipt/cancel"
        );
    }

    #[test]
    fn issue_body_serializes_camel_case_and_skips_none() {
        let body = IssueCashReceiptBody {
            store_id: None,
            payment_id: "pay-1".into(),
            channel_key: "ch-1".into(),
            receipt_type: CashReceiptType::Personal,
            order_name: "order".into(),
            currency: "KRW".into(),
            amount: PaymentAmountInput {
                total: 11000,
                tax_free: None,
                vat: None,
            },
            product_type: None,
            customer: IssueCashReceiptCustomerInput {
                identity_number: "01000000000".into(),
                name: None,
                email: None,
                phone_number: None,
            },
            paid_at: None,
            business_registration_number: None,
            payment_method: None,
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "paymentId": "pay-1",
                "channelKey": "ch-1",
                "type": "PERSONAL",
                "orderName": "order",
                "currency": "KRW",
                "amount": {"total": 11000},
                "customer": {"identityNumber": "01000000000"}
            })
        );
    }

    #[test]
    fn deserializes_issue_and_cancel_responses() {
        let issued: IssueCashReceiptResponse = serde_json::from_value(json!({
            "cashReceipt": {"issueNumber": "123", "url": "https://example.com/r", "pgReceiptId": "pg1"}
        }))
        .unwrap();
        assert_eq!(issued.cash_receipt.issue_number, "123");

        let cancelled: CancelCashReceiptResponse = serde_json::from_value(json!({
            "cancelledAmount": 11000, "cancelledAt": "2026-01-01T00:00:00Z"
        }))
        .unwrap();
        assert_eq!(cancelled.cancelled_amount, 11000);
    }

    #[test]
    fn deserializes_cash_receipt_by_status() {
        let base = json!({
            "merchantId": "m", "storeId": "s", "paymentId": "p",
            "channel": {"type": "LIVE"}, "amount": 1000, "currency": "KRW",
            "orderName": "o", "isManual": false, "issueNumber": "1",
            "issuedAt": "2026-01-01T00:00:00Z"
        });

        let mut issued = base.clone();
        issued["status"] = json!("ISSUED");
        assert!(matches!(
            serde_json::from_value::<CashReceipt>(issued).unwrap(),
            CashReceipt::Issued(r) if r.url.is_none() && r.amount == 1000
        ));

        let mut cancelled = base;
        cancelled["status"] = json!("CANCELLED");
        cancelled["cancelledAt"] = json!("2026-01-02T00:00:00Z");
        assert!(matches!(
            serde_json::from_value::<CashReceipt>(cancelled).unwrap(),
            CashReceipt::Cancelled(_)
        ));

        let failed = json!({
            "status": "ISSUE_FAILED", "merchantId": "m", "storeId": "s",
            "paymentId": "p", "orderName": "o", "isManual": true
        });
        assert!(matches!(
            serde_json::from_value::<CashReceipt>(failed).unwrap(),
            CashReceipt::IssueFailed(_)
        ));
    }
}
