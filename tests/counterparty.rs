//! 로컬 모의 서버 테스트. 실제 PortOne API 는 호출하지 않는다.

mod common;

use portone::{
    PortOneAPI,
    errors::{Error, PortOneError},
    features::{
        b2b::B2bError,
        counterparty::{
            PortOneCounterpartyService,
            dto::{
                request::{
                    CounterpartyFilter, CounterpartyInput, CreateCounterpartyBody,
                    GetCounterpartiesBody, UpdateCounterpartyBody,
                },
                response::NtsConnectionStatus,
            },
        },
    },
};

use common::{KEY, mock_server};

const COUNTERPARTY: &str = r#"{"id":"cp-1","graphqlId":"g","isForTest":false,"brn":"1234567890","companyName":"Example Co","representativeName":"Kim","contact":{"name":"Kim","email":"a@example.com"},"additionalContacts":[],"ntsConnectionStatus":"CONNECTED"}"#;

fn input() -> CounterpartyInput {
    CounterpartyInput {
        brn: "1234567890".into(),
        name: None,
        representative_name: None,
        address: None,
        business_type: None,
        business_class: None,
        contact: None,
        additional_contacts: None,
        memo: None,
    }
}

fn api(url: &str) -> portone::AuthedPortOneAPI {
    PortOneAPI::new().authed_api("secret").with_base_url(url)
}

#[tokio::test]
async fn create_posts_with_key_and_test_flag() {
    let response = format!(r#"{{"counterparty":{COUNTERPARTY}}}"#);
    let (url, handle) = mock_server("200 OK", &response).await;
    let body = CreateCounterpartyBody {
        counterparty_id: Some("cp-1".into()),
        counterparty: input(),
        options: None,
    };
    let r = api(&url)
        .create_counterparty(&body, true, Some(KEY))
        .await
        .unwrap();
    assert_eq!(
        r.counterparty.nts_connection_status,
        NtsConnectionStatus::Connected
    );
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("POST /b2b/counterparties?test=true HTTP/1.1"),
        "{raw}"
    );
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );
    assert!(raw.contains(r#""counterpartyId":"cp-1""#), "{raw}");
}

#[tokio::test]
async fn create_conflict_and_nts_errors_are_matchable() {
    let (url, _h) = mock_server(
        "409 Conflict",
        r#"{"type":"B2bCounterpartyIdAlreadyExistsError"}"#,
    )
    .await;
    let body = CreateCounterpartyBody {
        counterparty_id: None,
        counterparty: input(),
        options: None,
    };
    let r = api(&url).create_counterparty(&body, false, None).await;
    assert!(matches!(
        r,
        Err(Error::B2b(B2bError::B2bCounterpartyIdAlreadyExistsError))
    ));

    let (url, _h) = mock_server(
        "502 Bad Gateway",
        r#"{"type":"B2bCounterpartyNtsConnectionFailedError"}"#,
    )
    .await;
    let r = api(&url).create_counterparty(&body, false, None).await;
    assert!(matches!(
        r,
        Err(Error::B2b(
            B2bError::B2bCounterpartyNtsConnectionFailedError
        ))
    ));
}

#[tokio::test]
async fn get_encodes_id_and_maps_not_found() {
    let (url, handle) = mock_server("200 OK", COUNTERPARTY).await;
    let c = api(&url).get_counterparty("a/b", false).await.unwrap();
    assert_eq!(c.id, "cp-1");
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("GET /b2b/counterparties/a%2Fb HTTP/1.1"),
        "{raw}"
    );

    let (url, _h) = mock_server(
        "404 Not Found",
        r#"{"type":"B2bCounterpartyNotFoundError"}"#,
    )
    .await;
    let r = api(&url).get_counterparty("x", false).await;
    assert!(matches!(
        r,
        Err(Error::B2b(B2bError::B2bCounterpartyNotFoundError))
    ));
}

#[tokio::test]
async fn search_sends_test_and_request_body() {
    let response = r#"{"page":{"number":0,"size":10,"totalCount":0},"items":[]}"#;
    let (url, handle) = mock_server("200 OK", response).await;
    let body = GetCounterpartiesBody {
        filter: Some(CounterpartyFilter {
            brn: Some("1234567890".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    api(&url).get_counterparties(&body, true).await.unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with(
            "GET /b2b/counterparties?test=true&requestBody=%7B%22filter%22%3A%7B%22brn%22%3A%221234567890%22%7D%7D HTTP/1.1"
        ),
        "{raw}"
    );
}

#[tokio::test]
async fn update_uses_patch_with_key() {
    let response = format!(r#"{{"counterparty":{COUNTERPARTY}}}"#);
    let (url, handle) = mock_server("200 OK", &response).await;
    let body = UpdateCounterpartyBody {
        counterparty: input(),
        options: None,
    };
    api(&url)
        .update_counterparty("cp-1", &body, false, Some(KEY))
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("PATCH /b2b/counterparties/cp-1 HTTP/1.1"),
        "{raw}"
    );
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );
}

#[tokio::test]
async fn delete_uses_delete_and_maps_ongoing_invoice_conflict() {
    let (url, handle) = mock_server("200 OK", "{}").await;
    api(&url)
        .delete_counterparty("cp-1", true, Some(KEY))
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("DELETE /b2b/counterparties/cp-1?test=true HTTP/1.1"),
        "{raw}"
    );
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );

    let (url, _h) = mock_server(
        "409 Conflict",
        r#"{"type":"B2bCounterpartyOngoingTaxInvoiceExistsError"}"#,
    )
    .await;
    let r = api(&url).delete_counterparty("cp-1", false, None).await;
    assert!(matches!(
        r,
        Err(Error::B2b(
            B2bError::B2bCounterpartyOngoingTaxInvoiceExistsError
        ))
    ));
}

#[tokio::test]
async fn invalid_key_is_rejected_without_request() {
    let api = api("http://127.0.0.1:1");
    let r = api.delete_counterparty("cp-1", false, Some("short")).await;
    assert!(matches!(
        r,
        Err(Error::PortOneError(PortOneError::InvalidRequestError))
    ));
}
