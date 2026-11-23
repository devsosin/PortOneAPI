# Changelog

## 0.2.0 (unreleased)

Compared with 0.1.0.

### Added
- `features::cancel`: payment cancel / partial cancel (`PortOneCancelService`).
- `features::webhook` (also at `portone::webhook`): Standard Webhooks signature verification.
- `features::receipt`: cash receipt issue / get / cancel. `features::tax_invoice`: read-only lookups.
- `errors::Error` as the crate error (`errors::ClientError` stays as an alias), `errors::Result`.
- `PortOneAPI: Default`; CI workflow, clippy settings, `CLAUDE.md`.

### Breaking
- `errors::Error` (`ClientError`) and `PortOneError` are `#[non_exhaustive]`; `Error` gained `Cancel` and `Webhook` variants. Add a `_` arm when matching.
- Cancel-specific rejections (`PaymentAlreadyCancelledError`, …) are in `features::cancel::CancelError`, not `PortOneError`. Response `type`s that are not shared by all endpoints are `PortOneError::Unclassified { status, kind }` (before: mapped by status code).
- `PaymentDetailData::get_customer_name` / `get_customer_email` return `Option<&str>` (they panicked when the field was absent).
- Service traits declare `fn … -> impl Future<Output = …> + Send` instead of `async fn` (callers unaffected; custom implementors keep using `async fn`).
- `PaymentAmount` fields are public.

### Fixed
- `with_base_url` is honored by every request (receipt and tax invoice calls used the production URL).
- `tests/portone.rs` (live call) is `#[ignore]`.
