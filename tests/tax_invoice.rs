//! 로컬 모의 서버 테스트. 실제 PortOne API 는 호출하지 않는다.

mod common;

use chrono::NaiveDate;
use portone::{
    PortOneAPI,
    errors::{Error, PortOneError},
    features::{
        b2b::B2bError,
        tax_invoice::{
            PortOneTaxInvoiceService, TaxInvoiceKeyType, TaxInvoiceLookup,
            dto::{
                common::{Company, IssuanceType, PurposeType, TaxInvoiceStatus, TaxationType},
                request::{
                    CancelTaxInvoiceIssuanceBody, GetTaxInvoicesBody,
                    IssueTaxInvoiceImmediatelyBody, TaxInvoiceInput,
                },
            },
        },
    },
};

use common::{KEY, mock_server};

const INVOICE: &str = r#"{"id":"txi-1","status":"ISSUED","taxationType":"TAXABLE","documentModificationType":"NORMAL","issuanceType":"NORMAL","writeDate":"2026-01-31","issuanceDueDate":"2026-02-10","purposeType":"INVOICE","totalSupplyAmount":100,"totalTaxAmount":10,"totalAmount":110,"remarks":[],"supplier":{"brn":"1234567890"},"recipient":{"brn":"0987654321"},"items":[],"additionalContacts":[]}"#;

fn issue_body() -> IssueTaxInvoiceImmediatelyBody {
    IssueTaxInvoiceImmediatelyBody {
        tax_invoice: TaxInvoiceInput {
            taxation_type: TaxationType::Taxable,
            issuance_type: Some(IssuanceType::Normal),
            serial_number: None,
            book_volume: None,
            book_issue: None,
            write_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            purpose_type: PurposeType::Invoice,
            total_supply_amount: 100,
            total_tax_amount: 10,
            total_amount: 110,
            cash_amount: None,
            check_amount: None,
            credit_amount: None,
            note_amount: None,
            remarks: None,
            supplier_document_key: None,
            supplier: None,
            recipient_document_key: None,
            recipient: Company {
                brn: Some("0987654321".into()),
                ..Default::default()
            },
            send_sms: None,
            items: None,
            additional_contacts: None,
            use_counterparty_additional_contacts: None,
        },
        memo: None,
        modification: None,
        create_recipient_counterparty: None,
    }
}

fn api(url: &str) -> portone::AuthedPortOneAPI {
    PortOneAPI::new().authed_api("secret").with_base_url(url)
}

#[tokio::test]
async fn issue_immediately_posts_body_test_flag_and_key() {
    let response = format!(r#"{{"taxInvoice":{INVOICE}}}"#);
    let (url, handle) = mock_server("200 OK", &response).await;
    let res = api(&url)
        .issue_tax_invoice_immediately(&issue_body(), true, Some(KEY))
        .await
        .unwrap();
    assert_eq!(res.tax_invoice.status, TaxInvoiceStatus::Issued);
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("POST /b2b/tax-invoices/issue-immediately?test=true HTTP/1.1"),
        "{raw}"
    );
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );
    assert!(raw.contains("authorization: PortOne secret"), "{raw}");
    assert!(raw.contains(r#""issuanceType":"NORMAL""#), "{raw}");
}

#[tokio::test]
async fn issue_immediately_without_key_omits_header() {
    let response = format!(r#"{{"taxInvoice":{INVOICE}}}"#);
    let (url, handle) = mock_server("200 OK", &response).await;
    api(&url)
        .issue_tax_invoice_immediately(&issue_body(), false, None)
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("POST /b2b/tax-invoices/issue-immediately HTTP/1.1"),
        "{raw}"
    );
    assert!(
        !raw.to_ascii_lowercase().contains("idempotency-key"),
        "{raw}"
    );
}

#[tokio::test]
async fn invalid_idempotency_key_is_rejected_without_request() {
    let api = api("http://127.0.0.1:1");
    let issue = api
        .issue_tax_invoice_immediately(&issue_body(), false, Some("short"))
        .await;
    assert!(matches!(
        issue,
        Err(Error::PortOneError(PortOneError::InvalidRequestError))
    ));
    let cancel = api
        .cancel_tax_invoice_issuance(
            "k",
            &TaxInvoiceLookup::default(),
            &CancelTaxInvoiceIssuanceBody::default(),
            Some("bad key with spaces!"),
        )
        .await;
    assert!(matches!(
        cancel,
        Err(Error::PortOneError(PortOneError::InvalidRequestError))
    ));
}

#[tokio::test]
async fn cancel_issuance_encodes_key_and_sends_lookup_and_memo() {
    let response = format!(r#"{{"taxInvoice":{INVOICE}}}"#);
    let (url, handle) = mock_server("200 OK", &response).await;
    let lookup = TaxInvoiceLookup {
        brn: Some("1234567890"),
        key_type: Some(TaxInvoiceKeyType::Supplier),
        test: true,
    };
    let body = CancelTaxInvoiceIssuanceBody {
        memo: Some("typo".into()),
    };
    api(&url)
        .cancel_tax_invoice_issuance("sup/1", &lookup, &body, Some(KEY))
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with(
            "POST /b2b/tax-invoices/sup%2F1/cancel-issuance?brn=1234567890&taxInvoiceKeyType=SUPPLIER&test=true HTTP/1.1"
        ),
        "{raw}"
    );
    assert!(raw.contains(r#"{"memo":"typo"}"#), "{raw}");
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );
}

#[tokio::test]
async fn not_issued_status_error_is_matchable() {
    let body = r#"{"type":"B2bTaxInvoiceNotIssuedStatusError","message":"m"}"#;
    let (url, _h) = mock_server("400 Bad Request", body).await;
    let r = api(&url)
        .cancel_tax_invoice_issuance(
            "k",
            &TaxInvoiceLookup::default(),
            &CancelTaxInvoiceIssuanceBody::default(),
            None,
        )
        .await;
    assert!(matches!(
        r,
        Err(Error::B2b(B2bError::B2bTaxInvoiceNotIssuedStatusError))
    ));
}

#[tokio::test]
async fn issue_errors_map_to_dedicated_variants() {
    for (status, kind, want) in [
        (
            "400 Bad Request",
            "B2bIssuanceTypeMismatchError",
            B2bError::B2bIssuanceTypeMismatchError,
        ),
        (
            "400 Bad Request",
            "B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError",
            B2bError::B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError,
        ),
        (
            "404 Not Found",
            "B2bRecipientNotFoundError",
            B2bError::B2bRecipientNotFoundError,
        ),
        (
            "409 Conflict",
            "B2bIdAlreadyExistsError",
            B2bError::B2bIdAlreadyExistsError,
        ),
        (
            "502 Bad Gateway",
            "B2bExternalServiceError",
            B2bError::B2bExternalServiceError,
        ),
        (
            "403 Forbidden",
            "B2bNotEnabledError",
            B2bError::B2bNotEnabledError,
        ),
    ] {
        let body = format!(r#"{{"type":"{kind}","message":"m"}}"#);
        let (url, _h) = mock_server(status, &body).await;
        let r = api(&url)
            .issue_tax_invoice_immediately(&issue_body(), false, None)
            .await;
        match r {
            Err(Error::B2b(got)) => assert_eq!(got, want),
            other => panic!("{kind}: {other:?}"),
        }
    }
}

#[tokio::test]
async fn shared_errors_and_outstanding_409_stay_generic() {
    let (url, _h) = mock_server("403 Forbidden", r#"{"type":"ForbiddenError"}"#).await;
    let r = api(&url)
        .issue_tax_invoice_immediately(&issue_body(), false, None)
        .await;
    assert!(matches!(
        r,
        Err(Error::PortOneError(PortOneError::ForbiddenError))
    ));

    let body = r#"{"type":"IDEMPOTENCY_OUTSTANDING_REQUEST","message":"m"}"#;
    let (url, handle) = mock_server("409 Conflict", body).await;
    let r = api(&url)
        .issue_tax_invoice_immediately(&issue_body(), false, Some(KEY))
        .await;
    assert!(matches!(
        r,
        Err(Error::PortOneError(
            PortOneError::IdempotencyOutstandingRequestError
        ))
    ));
    handle.await.unwrap();
}

#[tokio::test]
async fn get_tax_invoice_is_typed() {
    let (url, handle) = mock_server("200 OK", INVOICE).await;
    let t = api(&url)
        .get_tax_invoice("txi-1", &TaxInvoiceLookup::default())
        .await
        .unwrap();
    assert_eq!(t.id, "txi-1");
    assert_eq!(t.total_amount, 110);
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("GET /b2b/tax-invoices/txi-1 HTTP/1.1"),
        "{raw}"
    );

    let (url, _h) = mock_server("404 Not Found", r#"{"type":"B2bTaxInvoiceNotFoundError"}"#).await;
    let r = api(&url)
        .get_tax_invoice("x", &TaxInvoiceLookup::default())
        .await;
    assert!(matches!(
        r,
        Err(Error::B2b(B2bError::B2bTaxInvoiceNotFoundError))
    ));
}

#[tokio::test]
async fn list_sends_encoded_request_body_in_query() {
    let response = r#"{"items":[],"page":{"number":0,"size":500,"totalCount":0}}"#;
    let (url, handle) = mock_server("200 OK", response).await;
    let body = GetTaxInvoicesBody {
        page_size: Some(10),
        test: Some(true),
        ..Default::default()
    };
    let r = api(&url).get_tax_invoices(&body).await.unwrap();
    assert!(r.items.is_empty());
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with(
            "GET /b2b/tax-invoices?requestBody=%7B%22test%22%3Atrue%2C%22pageSize%22%3A10%7D HTTP/1.1"
        ),
        "{raw}"
    );
}

#[tokio::test]
async fn url_endpoints_hit_their_paths() {
    for (suffix, which) in [("pdf-download-url", 0), ("print-url", 1), ("popup-url", 2)] {
        let (url, handle) = mock_server("200 OK", r#"{"url":"https://example.com/x"}"#).await;
        let api = api(&url);
        let lookup = TaxInvoiceLookup::default();
        let r = match which {
            0 => api.get_tax_invoice_pdf_url("k1", &lookup).await,
            1 => api.get_tax_invoice_print_url("k1", &lookup).await,
            _ => api.get_tax_invoice_popup_url("k1", &lookup).await,
        }
        .unwrap();
        assert_eq!(r.url, "https://example.com/x");
        let raw = handle.await.unwrap();
        assert!(
            raw.starts_with(&format!("GET /b2b/tax-invoices/k1/{suffix} HTTP/1.1")),
            "{raw}"
        );
    }
}
