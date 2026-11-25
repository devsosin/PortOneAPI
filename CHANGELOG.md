# Changelog

## 0.2.0 (unreleased)

Compared with 0.1.0.

### Added
- `features::cancel`: payment cancel / partial cancel (`PortOneCancelService`).
- `features::webhook` (also at `portone::webhook`): Standard Webhooks signature verification.
- `features::receipt`: cash receipt issue / get / cancel / list (`get_cash_receipts`, filter incl. `pgProviders` / `pgCompanies` / `version`, paging DTOs; query is sent as URL-encoded `requestBody`), `identityNumberType` input. `features::tax_invoice`: read-only lookups.
- `docs/rest-v2/policy.md` (official common API policy, must-read) and `docs/research/cash-receipt-tax-invoice.md`: research record, tax review result, follow-up issues #12–#15.
- `errors::Error` as the crate error (`errors::ClientError` stays as an alias), `errors::Result`.
- `PaymentDetailData`: optional `receiptUrl` / `receiptUrlWithoutAuthentication` with `get_receipt_url` / `get_receipt_url_without_authentication` (backward compatible; `None` when absent).
- `features::receipt::ReceiptError::CashReceiptAlreadyIssuedError` (409 of `issue_cash_receipt`) and `Error::Receipt`. The crate does not check for duplicates; that is the caller's job (see `docs/research/cash-receipt-tax-invoice.md`).
- `features::receipt::dto::response::SelectedChannel` (+ `SelectedChannelType`, `SelectedPgProvider`) for the receipt `channel` (PORT-007). Unknown `type` / `pgProvider` values deserialize into `Unknown` and are never treated as live/success (`SelectedChannel::is_live`). `PgProvider` also implements `Deserialize`.
- `Idempotency-Key` for cash receipt issue and cancel (PORT-008). Key validation (16–256 ASCII chars, no quote/space/control) now lives in `client::request`, so every call rejects a bad key with `InvalidRequestError` before sending. A 409 `IDEMPOTENCY_OUTSTANDING_REQUEST` is surfaced as `PortOneError::IdempotencyOutstandingRequestError` and is not retried by the crate: retry later with the same key.
- `PortOneAPI: Default`; CI workflow, clippy settings, `CLAUDE.md`.

### Breaking
- `issue_cash_receipt` and `cancel_cash_receipt` take an extra `idempotency_key: Option<&str>` argument (pass `None` to keep the old behavior).
- `channel` of `IssuedCashReceipt` / `CancelledCashReceipt` / `IssueFailedCashReceipt` is `SelectedChannel` / `Option<SelectedChannel>` (was `serde_json::Value`).
- `features::receipt` response `type` fields are `Option<CashReceiptType>` (was `Option<String>`); `CashReceiptType` also deserializes. `IssueCashReceiptCustomerInput` has a new field `identity_number_type`.
- `errors::Error` (`ClientError`) and `PortOneError` are `#[non_exhaustive]`; `Error` gained `Cancel`, `Receipt` and `Webhook` variants. Add a `_` arm when matching.
- Cancel-specific rejections (`PaymentAlreadyCancelledError`, …) are in `features::cancel::CancelError`, not `PortOneError`. Response `type`s that are not shared by all endpoints are `PortOneError::Unclassified { status, kind }` (before: mapped by status code).
- `PaymentDetailData::get_customer_name` / `get_customer_email` return `Option<&str>` (they panicked when the field was absent).
- Service traits declare `fn … -> impl Future<Output = …> + Send` instead of `async fn` (callers unaffected; custom implementors keep using `async fn`).
- `PaymentAmount` fields are public.

### Fixed
- `get_payment_detail` percent-encodes `paymentId`; query values use `encodeURIComponent` rules (`%20`, not `+`).
- HTTP client uses a 60s read timeout (was 10s total), as the API policy recommends.
- `with_base_url` is honored by every request (receipt and tax invoice calls used the production URL).
- `tests/portone.rs` (live call) is `#[ignore]`.
