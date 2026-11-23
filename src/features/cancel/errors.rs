use thiserror::Error;

use crate::errors::{Error, PortOneError};

/// Rejections that only the cancel endpoint returns.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CancelError {
    #[error("Payment Not Paid")]
    PaymentNotPaidError, // 409

    #[error("Payment Already Cancelled")]
    PaymentAlreadyCancelledError, // 409

    #[error("Cancellable Amount Consistency Broken")]
    CancellableAmountConsistencyBrokenError, // 409

    #[error("Cancel Amount Exceeds Cancellable Amount")]
    CancelAmountExceedsCancellableAmountError, // 409

    #[error("Sum Of Parts Exceeds Cancel Amount")]
    SumOfPartsExceedsCancelAmountError, // 409

    #[error("Cancel Tax Free Amount Exceeds Cancellable Tax Free Amount")]
    CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError, // 409

    #[error("Cancel Tax Amount Exceeds Cancellable Tax Amount")]
    CancelTaxAmountExceedsCancellableTaxAmountError, // 409

    #[error("Max Cancel Count Reached")]
    MaxCancelCountReachedError, // 409

    #[error("Negative Promotion Adjusted Cancel Amount")]
    NegativePromotionAdjustedCancelAmountError, // 400

    #[error("Promotion Discount Retain Option Should Not Be Changed")]
    PromotionDiscountRetainOptionShouldNotBeChangedError, // 400
}

impl CancelError {
    /// Maps the `type` of an error response; `None` for types the cancel endpoint does not define.
    pub fn from_type(kind: &str) -> Option<Self> {
        Some(match kind {
            "PaymentNotPaidError" => Self::PaymentNotPaidError,
            "PaymentAlreadyCancelledError" => Self::PaymentAlreadyCancelledError,
            "CancellableAmountConsistencyBrokenError" => {
                Self::CancellableAmountConsistencyBrokenError
            }
            "CancelAmountExceedsCancellableAmountError" => {
                Self::CancelAmountExceedsCancellableAmountError
            }
            "SumOfPartsExceedsCancelAmountError" => Self::SumOfPartsExceedsCancelAmountError,
            "CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError" => {
                Self::CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError
            }
            "CancelTaxAmountExceedsCancellableTaxAmountError" => {
                Self::CancelTaxAmountExceedsCancellableTaxAmountError
            }
            "MaxCancelCountReachedError" => Self::MaxCancelCountReachedError,
            "NegativePromotionAdjustedCancelAmountError" => {
                Self::NegativePromotionAdjustedCancelAmountError
            }
            "PromotionDiscountRetainOptionShouldNotBeChangedError" => {
                Self::PromotionDiscountRetainOptionShouldNotBeChangedError
            }
            _ => return None,
        })
    }

    /// Replaces a generic unclassified response error with the cancel specific one, if any.
    pub(super) fn refine(err: Error) -> Error {
        match err {
            Error::PortOneError(PortOneError::Unclassified { status, kind }) => {
                match Self::from_type(&kind) {
                    Some(cancel) => cancel.into(),
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

    #[test]
    fn refine_maps_known_types_only() {
        let known = PortOneError::Unclassified {
            status: 409,
            kind: "PaymentAlreadyCancelledError".into(),
        };
        assert!(matches!(
            CancelError::refine(known.into()),
            Error::Cancel(CancelError::PaymentAlreadyCancelledError)
        ));

        let unknown = PortOneError::Unclassified {
            status: 409,
            kind: "SomethingNew".into(),
        };
        assert!(matches!(
            CancelError::refine(unknown.into()),
            Error::PortOneError(PortOneError::Unclassified { .. })
        ));
        assert!(matches!(
            CancelError::refine(PortOneError::ForbiddenError.into()),
            Error::PortOneError(PortOneError::ForbiddenError)
        ));
    }
}
