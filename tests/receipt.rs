//! 로컬 TCP 서버로 HTTP 를 모킹한다. 실제 PortOne API 는 호출하지 않는다.

use portone::{
    PortOneAPI,
    errors::{Error, PortOneError},
    features::receipt::{
        PortOneCashReceiptService, ReceiptError,
        dto::request::{
            CashReceiptType, IssueCashReceiptBody, IssueCashReceiptCustomerInput,
            PaymentAmountInput,
        },
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const KEY: &str = "0123456789abcdef0123";
const ISSUED: &str =
    r#"{"cashReceipt":{"issueNumber":"1","url":"https://example.com/r","pgReceiptId":"pg"}}"#;
const CANCELLED: &str = r#"{"cancelledAmount":1000,"cancelledAt":"2026-01-01T00:00:00Z"}"#;

/// 요청 하나를 받아 원문(헤더+본문)을 돌려주고 지정한 응답을 보낸다.
async fn mock_server(status: &str, body: &str) -> (String, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    let handle = tokio::spawn(async move {
        let (mut sock, _) = listener.accept().await.unwrap();
        let mut buf = Vec::new();
        let mut chunk = [0u8; 1024];
        loop {
            let n = sock.read(&mut chunk).await.unwrap();
            buf.extend_from_slice(&chunk[..n]);
            let text = String::from_utf8_lossy(&buf).to_string();
            if let Some(pos) = text.find("\r\n\r\n") {
                let len = text[..pos]
                    .lines()
                    .find_map(|l| {
                        l.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|v| v.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                if buf.len() >= pos + 4 + len {
                    break;
                }
            }
            if n == 0 {
                break;
            }
        }
        sock.write_all(response.as_bytes()).await.unwrap();
        sock.shutdown().await.ok();
        String::from_utf8_lossy(&buf).to_string()
    });
    (url, handle)
}

fn issue_body() -> IssueCashReceiptBody {
    IssueCashReceiptBody {
        store_id: None,
        payment_id: "pay-1".into(),
        channel_key: "ch-1".into(),
        receipt_type: CashReceiptType::Personal,
        order_name: "order".into(),
        currency: "KRW".into(),
        amount: PaymentAmountInput {
            total: 1000,
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
    }
}

#[tokio::test]
async fn issue_sends_quoted_idempotency_key() {
    let (url, handle) = mock_server("200 OK", ISSUED).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    api.issue_cash_receipt(&issue_body(), Some(KEY))
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(raw.starts_with("POST /cash-receipts HTTP/1.1"), "{raw}");
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );
}

#[tokio::test]
async fn issue_omits_header_without_key() {
    let (url, handle) = mock_server("200 OK", ISSUED).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    api.issue_cash_receipt(&issue_body(), None).await.unwrap();
    let raw = handle.await.unwrap();
    assert!(
        !raw.to_ascii_lowercase().contains("idempotency-key"),
        "{raw}"
    );
}

#[tokio::test]
async fn cancel_sends_key_and_store_id_and_omits_without_key() {
    let (url, handle) = mock_server("200 OK", CANCELLED).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    api.cancel_cash_receipt("pay/1", Some("s1"), Some(KEY))
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(
        raw.starts_with("POST /payments/pay%2F1/cash-receipt/cancel?storeId=s1 HTTP/1.1"),
        "{raw}"
    );
    assert!(
        raw.contains(&format!("idempotency-key: \"{KEY}\"")),
        "{raw}"
    );

    let (url, handle) = mock_server("200 OK", CANCELLED).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    api.cancel_cash_receipt("p1", None, None).await.unwrap();
    let raw = handle.await.unwrap();
    assert!(
        !raw.to_ascii_lowercase().contains("idempotency-key"),
        "{raw}"
    );
}

#[tokio::test]
async fn invalid_key_is_rejected_without_request() {
    let api = PortOneAPI::new()
        .authed_api("secret")
        .with_base_url("http://127.0.0.1:1");
    for bad in [
        "short",
        &"a".repeat(257),
        "0123456789abcde\"x",
        "0123456789abcde fgh",
    ] {
        let issue = api.issue_cash_receipt(&issue_body(), Some(bad)).await;
        assert!(matches!(
            issue,
            Err(Error::PortOneError(PortOneError::InvalidRequestError))
        ));
        let cancel = api.cancel_cash_receipt("p1", None, Some(bad)).await;
        assert!(matches!(
            cancel,
            Err(Error::PortOneError(PortOneError::InvalidRequestError))
        ));
    }
}

#[tokio::test]
async fn outstanding_request_409_is_distinct_and_not_retried() {
    let body = r#"{"type":"IDEMPOTENCY_OUTSTANDING_REQUEST","message":"m"}"#;
    // 서버는 연결 한 번만 받는다: 크레이트가 재시도하면 두 번째 연결이 거부되어 다른 오류가 난다.
    let (url, handle) = mock_server("409 Conflict", body).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    let r = api.issue_cash_receipt(&issue_body(), Some(KEY)).await;
    assert!(matches!(
        r,
        Err(Error::PortOneError(
            PortOneError::IdempotencyOutstandingRequestError
        ))
    ));
    handle.await.unwrap();

    let (url, _h) = mock_server("409 Conflict", body).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    let r = api.cancel_cash_receipt("p1", None, Some(KEY)).await;
    assert!(matches!(
        r,
        Err(Error::PortOneError(
            PortOneError::IdempotencyOutstandingRequestError
        ))
    ));
}

#[tokio::test]
async fn already_issued_still_maps_with_key() {
    let body = r#"{"type":"CashReceiptAlreadyIssuedError","message":"m"}"#;
    let (url, _h) = mock_server("409 Conflict", body).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    let r = api.issue_cash_receipt(&issue_body(), Some(KEY)).await;
    assert!(matches!(
        r,
        Err(Error::Receipt(ReceiptError::CashReceiptAlreadyIssuedError))
    ));
}
