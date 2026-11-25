pub mod dto;

use reqwest::Method;

use crate::{
    AuthedPortOneAPI,
    errors::Error,
    features::b2b::{B2bError, test_query},
    types::ClientResult,
};
use dto::{
    request::{CancelTaxInvoiceIssuanceBody, GetTaxInvoicesBody, IssueTaxInvoiceImmediatelyBody},
    response::{
        CancelTaxInvoiceIssuanceResponse, GetTaxInvoicesResponse,
        IssueTaxInvoiceImmediatelyResponse, TaxInvoice, TaxInvoiceUrlResponse,
    },
};

pub use dto::common::TaxInvoiceKeyType;
pub use dto::request::TaxInvoiceLookup;

/// B2B 세금계산서 API. 발행·취소는 실제 세금계산서를 만들거나 취소하므로 호출 전 확인이 필요하다.
///
/// 상태를 바꾸는 호출의 `idempotency_key`(선택)는 `Idempotency-Key` 헤더로 보내 재시도 시 중복 처리를
/// 막는다. 16~256자 ASCII 문자열이어야 하며(어기면 요청 없이 `InvalidRequestError`), 같은 키로 다른
/// 요청을 보내서는 안 된다. 처리 중인 요청을 같은 키로 다시 보내면
/// `PortOneError::IdempotencyOutstandingRequestError`(409)가 오고, 크레이트는 재시도하지 않으므로 호출
/// 측이 잠시 뒤 **같은 키로** 다시 호출한다. 문서화된 거절은 `Error::B2b(B2bError::…)` 로 돌려준다.
///
/// 지원하지 않는 호출(임시저장, 역발행 요청, 국세청 즉시 전송, 삭제, 첨부, 일괄 발행 등)은
/// `docs/research/cash-receipt-tax-invoice.md` 의 "남은 작업" 참고.
pub trait PortOneTaxInvoiceService: Send + Sync {
    /// `POST /b2b/tax-invoices/issue-immediately` 즉시 정발행
    ///
    /// `body.tax_invoice.issuance_type` 의 서버 기본값은 역발행이므로 `Some(IssuanceType::Normal)` 을
    /// 넣는다. `test` 가 true 면 테스트 모드.
    fn issue_tax_invoice_immediately(
        &self,
        body: &IssueTaxInvoiceImmediatelyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<IssueTaxInvoiceImmediatelyResponse>> + Send;

    /// `POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-issuance` 공급자 취소(국세청 전송 전, `ISSUED` 상태만)
    fn cancel_tax_invoice_issuance(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
        body: &CancelTaxInvoiceIssuanceBody,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<CancelTaxInvoiceIssuanceResponse>> + Send;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}`
    fn get_tax_invoice(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> impl Future<Output = ClientResult<TaxInvoice>> + Send;

    /// `GET /b2b/tax-invoices` 다건 조회 (조건은 `requestBody` 쿼리에 JSON 으로 전달)
    fn get_tax_invoices(
        &self,
        body: &GetTaxInvoicesBody,
    ) -> impl Future<Output = ClientResult<GetTaxInvoicesResponse>> + Send;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}/pdf-download-url`
    fn get_tax_invoice_pdf_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> impl Future<Output = ClientResult<TaxInvoiceUrlResponse>> + Send;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}/print-url`
    fn get_tax_invoice_print_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> impl Future<Output = ClientResult<TaxInvoiceUrlResponse>> + Send;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}/popup-url`
    fn get_tax_invoice_popup_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> impl Future<Output = ClientResult<TaxInvoiceUrlResponse>> + Send;
}

impl PortOneTaxInvoiceService for AuthedPortOneAPI {
    async fn issue_tax_invoice_immediately(
        &self,
        body: &IssueTaxInvoiceImmediatelyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> ClientResult<IssueTaxInvoiceImmediatelyResponse> {
        let url = self.build_url(
            &["b2b", "tax-invoices", "issue-immediately"],
            &test_query(test),
        )?;
        self.request(Method::POST, url, Some(body), idempotency_key)
            .await
            .map_err(B2bError::refine)
    }

    async fn cancel_tax_invoice_issuance(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
        body: &CancelTaxInvoiceIssuanceBody,
        idempotency_key: Option<&str>,
    ) -> ClientResult<CancelTaxInvoiceIssuanceResponse> {
        let url = self.build_url(
            &["b2b", "tax-invoices", key, "cancel-issuance"],
            &lookup_query(lookup),
        )?;
        self.request(Method::POST, url, Some(body), idempotency_key)
            .await
            .map_err(B2bError::refine)
    }

    async fn get_tax_invoice(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoice> {
        let url = self.build_url(&["b2b", "tax-invoices", key], &lookup_query(lookup))?;
        self.request::<(), _>(Method::GET, url, None, None)
            .await
            .map_err(B2bError::refine)
    }

    async fn get_tax_invoices(
        &self,
        body: &GetTaxInvoicesBody,
    ) -> ClientResult<GetTaxInvoicesResponse> {
        let json = serde_json::to_string(body).map_err(|e| Error::InternalError(e.to_string()))?;
        let url = self.build_url(&["b2b", "tax-invoices"], &[("requestBody", json)])?;
        self.request::<(), _>(Method::GET, url, None, None)
            .await
            .map_err(B2bError::refine)
    }

    async fn get_tax_invoice_pdf_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        self.get_url_response(key, "pdf-download-url", lookup).await
    }

    async fn get_tax_invoice_print_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        self.get_url_response(key, "print-url", lookup).await
    }

    async fn get_tax_invoice_popup_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        self.get_url_response(key, "popup-url", lookup).await
    }
}

fn lookup_query(lookup: &TaxInvoiceLookup<'_>) -> Vec<(&'static str, String)> {
    let mut query = Vec::new();
    if let Some(brn) = lookup.brn {
        query.push(("brn", brn.to_string()));
    }
    if let Some(key_type) = lookup.key_type {
        query.push(("taxInvoiceKeyType", key_type.as_str().to_string()));
    }
    query.extend(test_query(lookup.test));
    query
}

impl AuthedPortOneAPI {
    async fn get_url_response(
        &self,
        key: &str,
        last: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        let url = self.build_url(&["b2b", "tax-invoices", key, last], &lookup_query(lookup))?;
        self.request::<(), _>(Method::GET, url, None, None)
            .await
            .map_err(B2bError::refine)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use serde_json::json;

    use super::*;
    use dto::common::{Company, IssuanceType, PurposeType, TaxationType};
    use dto::request::{DateFilter, DateRange, PrimaryFilter, TaxInvoiceFilter, TaxInvoiceInput};

    fn test_api() -> AuthedPortOneAPI {
        crate::PortOneAPI::new().authed_api("k")
    }

    fn input() -> TaxInvoiceInput {
        TaxInvoiceInput {
            taxation_type: TaxationType::Taxable,
            issuance_type: Some(IssuanceType::Normal),
            serial_number: None,
            book_volume: None,
            book_issue: None,
            write_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            purpose_type: PurposeType::Invoice,
            total_supply_amount: 100000,
            total_tax_amount: 10000,
            total_amount: 110000,
            cash_amount: None,
            check_amount: None,
            credit_amount: None,
            note_amount: None,
            remarks: None,
            supplier_document_key: Some("sup-key_1".into()),
            supplier: None,
            recipient_document_key: None,
            recipient: Company {
                brn: Some("1234567890".into()),
                ..Default::default()
            },
            send_sms: None,
            items: None,
            additional_contacts: None,
            use_counterparty_additional_contacts: None,
        }
    }

    #[test]
    fn default_lookup_has_no_query() {
        assert!(lookup_query(&TaxInvoiceLookup::default()).is_empty());
    }

    #[test]
    fn lookup_query_and_url() {
        let lookup = TaxInvoiceLookup {
            brn: Some("1234567890"),
            key_type: Some(TaxInvoiceKeyType::Supplier),
            test: true,
        };
        let url = test_api()
            .build_url(
                &["b2b", "tax-invoices", "k1", "print-url"],
                &lookup_query(&lookup),
            )
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/b2b/tax-invoices/k1/print-url?brn=1234567890&taxInvoiceKeyType=SUPPLIER&test=true"
        );
    }

    #[test]
    fn key_cannot_escape_path() {
        let url = test_api()
            .build_url(&["b2b", "tax-invoices", "a/b c", "cancel-issuance"], &[])
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/b2b/tax-invoices/a%2Fb%20c/cancel-issuance"
        );
    }

    #[test]
    fn issue_body_serializes_camel_case_and_skips_none() {
        let body = IssueTaxInvoiceImmediatelyBody {
            tax_invoice: input(),
            memo: None,
            modification: None,
            create_recipient_counterparty: Some(true),
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "taxInvoice": {
                    "taxationType": "TAXABLE",
                    "issuanceType": "NORMAL",
                    "writeDate": "2026-01-31",
                    "purposeType": "INVOICE",
                    "totalSupplyAmount": 100000,
                    "totalTaxAmount": 10000,
                    "totalAmount": 110000,
                    "supplierDocumentKey": "sup-key_1",
                    "recipient": {"brn": "1234567890"}
                },
                "createRecipientCounterparty": true
            })
        );
    }

    #[test]
    fn list_body_serializes_and_goes_into_encoded_query() {
        let body = GetTaxInvoicesBody {
            page_size: Some(100),
            filter: Some(TaxInvoiceFilter {
                primary_filter: Some(PrimaryFilter {
                    date_filter: Some(DateFilter {
                        date_type: None,
                        date_range: Some(vec![DateRange {
                            from: NaiveDate::from_ymd_opt(2026, 1, 1),
                            until: None,
                        }]),
                    }),
                    ..Default::default()
                }),
                statuses: Some(vec![
                    dto::common::TaxInvoiceStatus::Issued,
                    dto::common::TaxInvoiceStatus::SendingCompleted,
                ]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(
            value,
            json!({
                "pageSize": 100,
                "filter": {
                    "primaryFilter": {"dateFilter": {"dateRange": [{"from": "2026-01-01"}]}},
                    "statuses": ["ISSUED", "SENDING_COMPLETED"]
                }
            })
        );
        let url = test_api()
            .build_url(
                &["b2b", "tax-invoices"],
                &[("requestBody", value.to_string())],
            )
            .unwrap();
        assert_eq!(url.path(), "/b2b/tax-invoices");
        assert!(!url.query().unwrap().contains('{'));
        let (name, decoded) = url.query_pairs().next().unwrap();
        assert_eq!(name, "requestBody");
        assert_eq!(decoded, value.to_string());
    }

    fn invoice_json(status: &str) -> serde_json::Value {
        json!({
            "id": "txi-1", "status": status, "taxationType": "TAXABLE",
            "documentModificationType": "NORMAL", "issuanceType": "NORMAL",
            "writeDate": "2026-01-31", "issuanceDueDate": "2026-02-10",
            "purposeType": "INVOICE", "totalSupplyAmount": 100000,
            "totalTaxAmount": 10000, "totalAmount": 110000, "remarks": [],
            "supplier": {"brn": "1234567890"},
            "recipient": {"brn": "0987654321", "contact": {"email": "a@example.com"}},
            "items": [{"name": "x", "quantity": {"value": 150, "scale": 2}}],
            "additionalContacts": [],
            "statusUpdatedAt": "2026-01-31T01:00:00Z"
        })
    }

    #[test]
    fn deserializes_tax_invoice() {
        let t: TaxInvoice = serde_json::from_value(invoice_json("ISSUED")).unwrap();
        assert_eq!(t.status, dto::common::TaxInvoiceStatus::Issued);
        assert_eq!(t.items[0].quantity.unwrap().scale, Some(2));
        assert!(t.status_updated_at.is_some());
        assert!(t.nts_approval_number.is_none());
    }

    #[test]
    fn unknown_status_is_never_a_known_state() {
        let t: TaxInvoice = serde_json::from_value(invoice_json("SOMETHING_NEW")).unwrap();
        assert_eq!(t.status, dto::common::TaxInvoiceStatus::Unknown);
        assert_ne!(t.status, dto::common::TaxInvoiceStatus::Issued);
    }

    #[test]
    fn deserializes_responses() {
        let r: IssueTaxInvoiceImmediatelyResponse =
            serde_json::from_value(json!({"taxInvoice": invoice_json("ISSUED")})).unwrap();
        assert_eq!(r.tax_invoice.id, "txi-1");
        let u: TaxInvoiceUrlResponse =
            serde_json::from_str(r#"{"url":"https://example.com/x"}"#).unwrap();
        assert_eq!(u.url, "https://example.com/x");
        let l: GetTaxInvoicesResponse = serde_json::from_value(json!({
            "items": [{
                "id": "txi-1", "taxationType": "FREE", "documentModificationType": "MODIFICATION",
                "issuanceType": "REVERSE", "totalSupplyAmount": 1, "totalTaxAmount": 0,
                "totalAmount": 1, "purposeType": "NONE", "supplierBrn": "1",
                "supplierName": "s", "supplierRepresentativeName": "r", "recipientBrn": "2",
                "recipientName": "n", "recipientRepresentativeName": "r2",
                "writeDate": "2026-01-01", "issuanceDueDate": "2026-01-10",
                "status": "DRAFTED", "statusUpdatedAt": "2026-01-01T00:00:00Z", "items": []
            }],
            "page": {"number": 0, "size": 500, "totalCount": 1}
        }))
        .unwrap();
        assert_eq!(l.items[0].status, dto::common::TaxInvoiceStatus::Drafted);
        assert_eq!(l.page.total_count, 1);
    }
}
