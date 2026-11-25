use thiserror::Error;

use crate::errors::{Error, PortOneError};

/// Rejections that only the cash receipt endpoints return.
///
/// The crate does not check for duplicates before issuing; avoiding them is the caller's job
/// (look up the receipt with `get_cash_receipt` or your own records first).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ReceiptError {
    /// `POST /cash-receipts` was rejected because the cash receipt already exists.
    #[error("Cash Receipt Already Issued")]
    CashReceiptAlreadyIssuedError, // 409
}

impl ReceiptError {
    /// Maps the `type` of an error response; `None` for types the receipt endpoints do not define.
    pub fn from_type(kind: &str) -> Option<Self> {
        Some(match kind {
            "CashReceiptAlreadyIssuedError" => Self::CashReceiptAlreadyIssuedError,
            _ => return None,
        })
    }

    /// Replaces a generic unclassified response error with the receipt specific one, if any.
    pub(super) fn refine(err: Error) -> Error {
        match err {
            Error::PortOneError(PortOneError::Unclassified { status, kind }) => {
                match Self::from_type(&kind) {
                    Some(receipt) => receipt.into(),
                    None => PortOneError::Unclassified { status, kind }.into(),
                }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unclassified(status: u16, kind: &str) -> Error {
        PortOneError::Unclassified {
            status,
            kind: kind.into(),
        }
        .into()
    }

    #[test]
    fn already_issued_maps_to_own_variant() {
        assert!(matches!(
            ReceiptError::refine(unclassified(409, "CashReceiptAlreadyIssuedError")),
            Error::Receipt(ReceiptError::CashReceiptAlreadyIssuedError)
        ));
    }

    #[test]
    fn unknown_and_shared_errors_pass_through() {
        assert!(matches!(
            ReceiptError::refine(unclassified(409, "SomethingNew")),
            Error::PortOneError(PortOneError::Unclassified { .. })
        ));
        assert!(matches!(
            ReceiptError::refine(PortOneError::ForbiddenError.into()),
            Error::PortOneError(PortOneError::ForbiddenError)
        ));
        assert!(ReceiptError::from_type("PaymentAlreadyCancelledError").is_none());
    }

    #[test]
    fn parses_response_body_into_variant() {
        let body = r#"{"type":"CashReceiptAlreadyIssuedError","message":"already"}"#;
        let err = PortOneError::from_response(reqwest::StatusCode::CONFLICT, body);
        assert!(matches!(
            ReceiptError::refine(err.into()),
            Error::Receipt(ReceiptError::CashReceiptAlreadyIssuedError)
        ));
    }
}
