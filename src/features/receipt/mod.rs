pub mod dto;
mod errors;

use reqwest::Method;

use crate::{
    AuthedPortOneAPI,
    features::receipt::dto::{
        request::{GetCashReceiptsBody, IssueCashReceiptBody},
        response::{
            CancelCashReceiptResponse, CashReceipt, GetCashReceiptsResponse,
            IssueCashReceiptResponse,
        },
    },
    types::ClientResult,
};

pub use errors::ReceiptError;

/// 현금영수증 API. 발급·취소는 실제 영수증을 만들거나 취소하므로 호출 전 확인이 필요하다.
///
/// 중복 발급 방지는 호출 측 책임이다(`get_cash_receipt` 나 자체 기록으로 확인). 이미 발급된 경우
/// `issue_cash_receipt` 는 `Error::Receipt(ReceiptError::CashReceiptAlreadyIssuedError)` 를 돌려준다.
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

    /// `GET /cash-receipts` 다건 조회 (조건은 `requestBody` 쿼리에 JSON 으로 전달)
    fn get_cash_receipts(
        &self,
        body: &GetCashReceiptsBody,
    ) -> impl Future<Output = ClientResult<GetCashReceiptsResponse>> + Send;

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
        self.request(Method::POST, url, Some(body), None)
            .await
            .map_err(ReceiptError::refine)
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

    async fn get_cash_receipts(
        &self,
        body: &GetCashReceiptsBody,
    ) -> ClientResult<GetCashReceiptsResponse> {
        let json = serde_json::to_string(body)
            .map_err(|e| crate::errors::Error::InternalError(e.to_string()))?;
        let url = self.build_url(&["cash-receipts"], &[("requestBody", json)])?;
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
                identity_number_type: None,
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

    #[test]
    fn list_url_carries_encoded_request_body() {
        use crate::features::receipt::dto::request::*;
        let body = GetCashReceiptsBody {
            filter: Some(CashReceiptFilterInput {
                order_name: Some("a b&c=d/한".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        let url = test_api()
            .build_url(&["cash-receipts"], &[("requestBody", json.clone())])
            .unwrap();
        assert_eq!(url.path(), "/cash-receipts");
        let raw = url.query().unwrap();
        assert!(raw.starts_with("requestBody=%7B%22filter%22%3A%7B%22orderName%22%3A%22a%20b%26c%3Dd%2F%ED%95%9C%22%7D%7D"));
        assert!(!raw.contains(' ') && !raw.contains('+') && !raw.contains('{'));
        let (_, decoded) = url.query_pairs().next().unwrap();
        assert_eq!(decoded, json);
    }

    #[test]
    fn list_filter_pg_fields_serialize() {
        use crate::features::receipt::dto::request::*;
        let f = CashReceiptFilterInput {
            pg_providers: Some(vec![PgProvider::Html5Inicis, PgProvider::NiceV2]),
            pg_companies: Some(vec![PgCompany::Tosspayments, PgCompany::TripleA]),
            version: Some(PortOneVersion::V2),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&f).unwrap(),
            json!({
                "pgProviders": ["HTML5_INICIS", "NICE_V2"],
                "pgCompanies": ["TOSSPAYMENTS", "TRIPLE_A"],
                "version": "V2"
            })
        );
    }

    #[test]
    fn identity_number_type_serializes() {
        use crate::features::receipt::dto::request::IdentityNumberType;
        let c = IssueCashReceiptCustomerInput {
            identity_number: "1".into(),
            identity_number_type: Some(IdentityNumberType::Phone),
            name: None,
            email: None,
            phone_number: None,
        };
        assert_eq!(
            serde_json::to_value(&c).unwrap(),
            json!({"identityNumber": "1", "identityNumberType": "PHONE"})
        );
    }

    #[test]
    fn list_body_serializes_filter_and_defaults() {
        use crate::features::receipt::dto::request::*;
        assert_eq!(
            serde_json::to_value(GetCashReceiptsBody::default()).unwrap(),
            json!({})
        );
        let body = GetCashReceiptsBody {
            page: Some(PageInput {
                number: Some(1),
                size: None,
            }),
            sort: Some(CashReceiptSortInput {
                by: Some(CashReceiptSortBy::CancelledAt),
                order: Some(SortOrder::Asc),
            }),
            filter: Some(CashReceiptFilterInput {
                statuses: Some(vec![CashReceiptStatus::Failed]),
                types: Some(vec![CashReceiptType::Corporate]),
                time_range_field: Some(CashReceiptTimeRangeField::StatusUpdatedAt),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "page": {"number": 1},
                "sort": {"by": "CANCELLED_AT", "order": "ASC"},
                "filter": {"timeRangeField": "STATUS_UPDATED_AT", "statuses": ["FAILED"], "types": ["CORPORATE"]}
            })
        );
    }

    #[test]
    fn deserializes_list_response() {
        let r: GetCashReceiptsResponse = serde_json::from_value(json!({
            "items": [{
                "status": "ISSUED", "merchantId": "m", "storeId": "s", "paymentId": "p",
                "channel": {"type": "TEST"}, "amount": 1000, "currency": "KRW",
                "orderName": "o", "isManual": true, "type": "CORPORATE",
                "issueNumber": "1", "issuedAt": "2026-01-01T00:00:00Z"
            }],
            "page": {"number": 0, "size": 10, "totalCount": 1}
        }))
        .unwrap();
        assert_eq!(r.page.total_count, 1);
        assert!(matches!(
            &r.items[0],
            CashReceipt::Issued(i) if i.receipt_type == Some(CashReceiptType::Corporate)
        ));
    }
}
