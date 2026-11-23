//! 로컬 TCP 서버로 HTTP 를 모킹한다. 실제 PortOne API 는 호출하지 않는다.

use portone::{
    PortOneAPI,
    errors::{Error, PortOneError},
    features::cancel::CancelError,
    features::cancel::{
        PortOneCancelService,
        dto::{CancelPaymentBody, CancelRefundAccount, CancelRequester, PaymentCancellationStatus},
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const KEY: &str = "0123456789abcdef0123";

const SUCCESS: &str = r#"{"cancellation":{"status":"SUCCEEDED","id":"c-1","pgCancellationId":"pg-1","totalAmount":3000,"taxFreeAmount":0,"vatAmount":273,"reason":"test","cancelledAt":"2026-01-01T00:00:00Z","requestedAt":"2026-01-01T00:00:00Z","receiptUrl":"https://example.com/r","trigger":"API"}}"#;

/// 요청 하나를 받아 원문을 돌려주고 지정한 응답을 보내는 서버를 띄운다.
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

async fn cancel_with(
    status: &str,
    body: &str,
    key: Option<&str>,
) -> (
    Result<
        portone::types::PortOneResponse<portone::features::cancel::dto::CancelPaymentResponse>,
        Error,
    >,
    String,
) {
    let (url, handle) = mock_server(status, body).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    let req = CancelPaymentBody::new("test").amount(3000);
    let result = api.cancel_payment("pay/1 a", &req, key).await;
    (result, handle.await.unwrap())
}

fn port_one_error(r: Result<impl std::fmt::Debug, Error>) -> PortOneError {
    match r.unwrap_err() {
        Error::PortOneError(e) => e,
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn partial_cancel_builds_request_and_parses_response() {
    let (url, handle) = mock_server("200 OK", SUCCESS).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    let body = CancelPaymentBody::new("test")
        .amount(3000)
        .current_cancellable_amount(9000)
        .requester(CancelRequester::Admin)
        .skip_webhook(true)
        .refund_account(CancelRefundAccount {
            bank: "KDB".into(),
            number: "123".into(),
            holder_name: "name".into(),
            holder_phone_number: None,
        });

    let res = api
        .cancel_payment("pay/1 a", &body, Some(KEY))
        .await
        .unwrap()
        .get_data()
        .unwrap();
    let raw = handle.await.unwrap();

    assert!(
        raw.starts_with("POST /payments/pay%2F1%20a/cancel HTTP/1.1"),
        "{raw}"
    );
    assert!(raw.contains("authorization: PortOne secret"));
    assert!(raw.contains(&format!("idempotency-key: \"{KEY}\"")));
    let json: serde_json::Value =
        serde_json::from_str(raw.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "reason": "test",
            "amount": 3000,
            "currentCancellableAmount": 9000,
            "requester": "ADMIN",
            "skipWebhook": true,
            "refundAccount": {"bank": "KDB", "number": "123", "holderName": "name"}
        })
    );

    let c = res.get_cancellation();
    assert!(c.is_succeeded());
    assert_eq!(c.get_status(), PaymentCancellationStatus::Succeeded);
    assert_eq!(c.get_total_amount(), 3000);
    assert_eq!(c.get_vat_amount(), 273);
    assert_eq!(c.get_receipt_url(), Some("https://example.com/r"));
}

#[tokio::test]
async fn full_cancel_omits_amount() {
    let (url, handle) = mock_server("200 OK", SUCCESS).await;
    let api = PortOneAPI::new().authed_api("secret").with_base_url(&url);
    api.cancel_payment("p1", &CancelPaymentBody::new("full"), None)
        .await
        .unwrap();
    let raw = handle.await.unwrap();
    assert!(!raw.to_ascii_lowercase().contains("idempotency-key"));
    assert!(raw.ends_with(r#"{"reason":"full"}"#), "{raw}");
}

#[tokio::test]
async fn invalid_idempotency_key_is_rejected_without_request() {
    let api = PortOneAPI::new()
        .authed_api("secret")
        .with_base_url("http://127.0.0.1:1");
    let r = api
        .cancel_payment("p1", &CancelPaymentBody::new("x"), Some("short"))
        .await;
    assert!(matches!(
        port_one_error(r),
        PortOneError::InvalidRequestError
    ));
}

#[tokio::test]
async fn maps_conflict_errors_by_type() {
    let cases = [
        ("PaymentAlreadyCancelledError", "already"),
        ("CancelAmountExceedsCancellableAmountError", "exceeds"),
        ("CancellableAmountConsistencyBrokenError", "consistency"),
        ("PaymentNotPaidError", "notpaid"),
        ("MaxCancelCountReachedError", "maxcount"),
        ("IDEMPOTENCY_OUTSTANDING_REQUEST", "outstanding"),
    ];
    for (ty, tag) in cases {
        let body = format!(r#"{{"type":"{ty}","message":"m"}}"#);
        let (r, _) = cancel_with("409 Conflict", &body, None).await;
        let e = r.unwrap_err();
        let ok = match tag {
            "already" => matches!(e, Error::Cancel(CancelError::PaymentAlreadyCancelledError)),
            "exceeds" => matches!(
                e,
                Error::Cancel(CancelError::CancelAmountExceedsCancellableAmountError)
            ),
            "consistency" => matches!(
                e,
                Error::Cancel(CancelError::CancellableAmountConsistencyBrokenError)
            ),
            "notpaid" => matches!(e, Error::Cancel(CancelError::PaymentNotPaidError)),
            "maxcount" => matches!(e, Error::Cancel(CancelError::MaxCancelCountReachedError)),
            _ => matches!(
                e,
                Error::PortOneError(PortOneError::IdempotencyOutstandingRequestError)
            ),
        };
        assert!(ok, "{ty} -> {e:?}");
    }
}

#[tokio::test]
async fn maps_not_found_auth_and_pg_errors() {
    let (r, _) = cancel_with("404 Not Found", r#"{"type":"PaymentNotFoundError"}"#, None).await;
    assert!(matches!(
        port_one_error(r),
        PortOneError::PaymentNotFoundError
    ));

    let (r, _) = cancel_with("401 Unauthorized", r#"{"type":"UnauthorizedError"}"#, None).await;
    assert!(matches!(port_one_error(r), PortOneError::UnauthorizedError));

    let (r, _) = cancel_with("403 Forbidden", r#"{"type":"ForbiddenError"}"#, None).await;
    assert!(matches!(port_one_error(r), PortOneError::ForbiddenError));

    let body = r#"{"type":"PgProviderError","message":"m","pgCode":"E1","pgMessage":"denied"}"#;
    let (r, _) = cancel_with("502 Bad Gateway", body, None).await;
    match port_one_error(r) {
        PortOneError::PgProviderError { code, message } => {
            assert_eq!(code.as_deref(), Some("E1"));
            assert_eq!(message.as_deref(), Some("denied"));
        }
        e => panic!("{e:?}"),
    }
}

#[tokio::test]
async fn falls_back_to_status_when_body_is_not_typed() {
    let (r, _) = cancel_with("404 Not Found", "not json", None).await;
    assert!(matches!(
        port_one_error(r),
        PortOneError::PaymentNotFoundError
    ));
    let (r, _) = cancel_with("409 Conflict", r#"{"type":"Other"}"#, None).await;
    assert!(matches!(
        port_one_error(r),
        PortOneError::Unclassified { status: 409, .. }
    ));
}
