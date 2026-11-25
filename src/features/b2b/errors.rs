use thiserror::Error;

use crate::errors::{Error, PortOneError};

/// Rejections that only the B2B endpoints (tax invoice, counterparty) return.
///
/// Every variant is a response `type` named in `docs/rest-v2/cash-receipt-tax-invoice/`; the
/// comment on a variant is the status code the docs list for it. Shared rejections
/// (`InvalidRequestError`, `UnauthorizedError`, `ForbiddenError`) stay in [`PortOneError`]; a
/// `type` that is not listed here stays [`PortOneError::Unclassified`], so match with a `_` arm.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum B2bError {
    #[error("B2B not enabled")]
    B2bNotEnabledError, // 403

    #[error("B2B external service")]
    B2bExternalServiceError, // 502

    #[error("B2B supplier not found")]
    B2bSupplierNotFoundError, // 404

    #[error("B2B recipient not found")]
    B2bRecipientNotFoundError, // 404

    #[error("B2B counterparty not found")]
    B2bCounterpartyNotFoundError, // 404

    #[error("B2B original tax invoice not found")]
    B2bOriginalTaxInvoiceNotFoundError, // 404

    #[error("B2B tax invoice not found")]
    B2bTaxInvoiceNotFoundError, // 404

    #[error("B2B bulk tax invoice not found")]
    B2bBulkTaxInvoiceNotFoundError, // 404

    #[error("B2B tax invoice attachment not found")]
    B2bTaxInvoiceAttachmentNotFoundError, // 404

    #[error("B2B file not found")]
    B2bFileNotFoundError, // 404

    #[error("B2B certificate unregistered")]
    B2bCertificateUnregisteredError, // 404

    #[error("B2B counterparty verification not found")]
    B2bCounterpartyVerificationNotFoundError, // 404

    #[error("B2B id already exists")]
    B2bIdAlreadyExistsError, // 409

    #[error("B2B tax invoice supplier document key already used")]
    B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError, // 400

    #[error("B2B tax invoice recipient document key already used")]
    B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError, // 400

    #[error("B2B tax invoice no supplier document key")]
    B2bTaxInvoiceNoSupplierDocumentKeyError, // 400

    #[error("B2B tax invoice no recipient document key")]
    B2bTaxInvoiceNoRecipientDocumentKeyError, // 400

    #[error("B2B document key cannot be changed")]
    B2bDocumentKeyCannotBeChangedError, // 400

    #[error("B2B modification not provided")]
    B2bModificationNotProvidedError, // 400

    #[error("B2B cannot change tax type")]
    B2BCannotChangeTaxTypeError, // 400

    #[error("B2B tax invoice status not sending completed")]
    B2BTaxInvoiceStatusNotSendingCompletedError, // 400

    #[error("B2B issuance type mismatch")]
    B2bIssuanceTypeMismatchError, // 400

    #[error("B2B tax invoice not issued status")]
    B2bTaxInvoiceNotIssuedStatusError, // 400

    #[error("B2B tax invoice not drafted status")]
    B2bTaxInvoiceNotDraftedStatusError, // 400

    #[error("B2B tax invoice not requested status")]
    B2bTaxInvoiceNotRequestedStatusError, // 400

    #[error("B2B tax invoice non deletable status")]
    B2bTaxInvoiceNonDeletableStatusError, // 400

    #[error("B2B counterparty nts not connected")]
    B2bCounterpartyNtsNotConnectedError, // 400

    #[error("B2B counterparty nts connection failed")]
    B2bCounterpartyNtsConnectionFailedError, // 502

    #[error("B2B counterparty brn invalid")]
    B2bCounterpartyBrnInvalidError, // 400

    #[error("B2B counterparty brn modification not allowed")]
    B2bCounterpartyBrnModificationNotAllowedError, // 400

    #[error("B2B counterparty self origin brn mismatch")]
    B2bCounterpartySelfOriginBrnMismatchError, // 400

    #[error("B2B counterparty too many additional contacts")]
    B2bCounterpartyTooManyAdditionalContactsError, // 400

    #[error("B2B counterparty missing required fields")]
    B2bCounterpartyMissingRequiredFieldsError, // 400

    #[error("B2B counterparty verification brn mismatch")]
    B2bCounterpartyVerificationBrnMismatchError, // 400

    #[error("B2B counterparty verification type mismatch")]
    B2bCounterpartyVerificationTypeMismatchError, // 400

    #[error("B2B counterparty verification invalid")]
    B2bCounterpartyVerificationInvalidError, // 400

    #[error("B2B counterparty partner not connectable")]
    B2bCounterpartyPartnerNotConnectableError, // 403

    #[error("B2B counterparty partner not deletable")]
    B2bCounterpartyPartnerNotDeletableError, // 403

    #[error("B2B counterparty partner not updatable")]
    B2bCounterpartyPartnerNotUpdatableError, // 403

    #[error("B2B counterparty ongoing tax invoice exists")]
    B2bCounterpartyOngoingTaxInvoiceExistsError, // 409

    #[error("B2B counterparty id already exists")]
    B2bCounterpartyIdAlreadyExistsError, // 409

    #[error("B2B counterparty id already exists by partner")]
    B2bCounterpartyIdAlreadyExistsByPartnerError, // 409
}

impl B2bError {
    /// Maps the `type` of an error response; `None` for types the B2B endpoints do not define.
    pub fn from_type(kind: &str) -> Option<Self> {
        Some(match kind {
            "B2bNotEnabledError" => Self::B2bNotEnabledError,
            "B2bExternalServiceError" => Self::B2bExternalServiceError,
            "B2bSupplierNotFoundError" => Self::B2bSupplierNotFoundError,
            "B2bRecipientNotFoundError" => Self::B2bRecipientNotFoundError,
            "B2bCounterpartyNotFoundError" => Self::B2bCounterpartyNotFoundError,
            "B2bOriginalTaxInvoiceNotFoundError" => Self::B2bOriginalTaxInvoiceNotFoundError,
            "B2bTaxInvoiceNotFoundError" => Self::B2bTaxInvoiceNotFoundError,
            "B2bBulkTaxInvoiceNotFoundError" => Self::B2bBulkTaxInvoiceNotFoundError,
            "B2bTaxInvoiceAttachmentNotFoundError" => Self::B2bTaxInvoiceAttachmentNotFoundError,
            "B2bFileNotFoundError" => Self::B2bFileNotFoundError,
            "B2bCertificateUnregisteredError" => Self::B2bCertificateUnregisteredError,
            "B2bCounterpartyVerificationNotFoundError" => {
                Self::B2bCounterpartyVerificationNotFoundError
            }
            "B2bIdAlreadyExistsError" => Self::B2bIdAlreadyExistsError,
            "B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError" => {
                Self::B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError
            }
            "B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError" => {
                Self::B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError
            }
            "B2bTaxInvoiceNoSupplierDocumentKeyError" => {
                Self::B2bTaxInvoiceNoSupplierDocumentKeyError
            }
            "B2bTaxInvoiceNoRecipientDocumentKeyError" => {
                Self::B2bTaxInvoiceNoRecipientDocumentKeyError
            }
            "B2bDocumentKeyCannotBeChangedError" => Self::B2bDocumentKeyCannotBeChangedError,
            "B2bModificationNotProvidedError" => Self::B2bModificationNotProvidedError,
            "B2BCannotChangeTaxTypeError" => Self::B2BCannotChangeTaxTypeError,
            "B2BTaxInvoiceStatusNotSendingCompletedError" => {
                Self::B2BTaxInvoiceStatusNotSendingCompletedError
            }
            "B2bIssuanceTypeMismatchError" => Self::B2bIssuanceTypeMismatchError,
            "B2bTaxInvoiceNotIssuedStatusError" => Self::B2bTaxInvoiceNotIssuedStatusError,
            "B2bTaxInvoiceNotDraftedStatusError" => Self::B2bTaxInvoiceNotDraftedStatusError,
            "B2bTaxInvoiceNotRequestedStatusError" => Self::B2bTaxInvoiceNotRequestedStatusError,
            "B2bTaxInvoiceNonDeletableStatusError" => Self::B2bTaxInvoiceNonDeletableStatusError,
            "B2bCounterpartyNtsNotConnectedError" => Self::B2bCounterpartyNtsNotConnectedError,
            "B2bCounterpartyNtsConnectionFailedError" => {
                Self::B2bCounterpartyNtsConnectionFailedError
            }
            "B2bCounterpartyBrnInvalidError" => Self::B2bCounterpartyBrnInvalidError,
            "B2bCounterpartyBrnModificationNotAllowedError" => {
                Self::B2bCounterpartyBrnModificationNotAllowedError
            }
            "B2bCounterpartySelfOriginBrnMismatchError" => {
                Self::B2bCounterpartySelfOriginBrnMismatchError
            }
            "B2bCounterpartyTooManyAdditionalContactsError" => {
                Self::B2bCounterpartyTooManyAdditionalContactsError
            }
            "B2bCounterpartyMissingRequiredFieldsError" => {
                Self::B2bCounterpartyMissingRequiredFieldsError
            }
            "B2bCounterpartyVerificationBrnMismatchError" => {
                Self::B2bCounterpartyVerificationBrnMismatchError
            }
            "B2bCounterpartyVerificationTypeMismatchError" => {
                Self::B2bCounterpartyVerificationTypeMismatchError
            }
            "B2bCounterpartyVerificationInvalidError" => {
                Self::B2bCounterpartyVerificationInvalidError
            }
            "B2bCounterpartyPartnerNotConnectableError" => {
                Self::B2bCounterpartyPartnerNotConnectableError
            }
            "B2bCounterpartyPartnerNotDeletableError" => {
                Self::B2bCounterpartyPartnerNotDeletableError
            }
            "B2bCounterpartyPartnerNotUpdatableError" => {
                Self::B2bCounterpartyPartnerNotUpdatableError
            }
            "B2bCounterpartyOngoingTaxInvoiceExistsError" => {
                Self::B2bCounterpartyOngoingTaxInvoiceExistsError
            }
            "B2bCounterpartyIdAlreadyExistsError" => Self::B2bCounterpartyIdAlreadyExistsError,
            "B2bCounterpartyIdAlreadyExistsByPartnerError" => {
                Self::B2bCounterpartyIdAlreadyExistsByPartnerError
            }
            _ => return None,
        })
    }

    /// Replaces a generic unclassified response error with the B2B specific one, if any.
    pub(crate) fn refine(err: Error) -> Error {
        match err {
            Error::PortOneError(PortOneError::Unclassified { status, kind }) => {
                match Self::from_type(&kind) {
                    Some(b2b) => b2b.into(),
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
    fn documented_types_map_to_variants() {
        assert_eq!(
            B2bError::from_type("B2bTaxInvoiceNotFoundError"),
            Some(B2bError::B2bTaxInvoiceNotFoundError)
        );
        assert_eq!(
            B2bError::from_type("B2BCannotChangeTaxTypeError"),
            Some(B2bError::B2BCannotChangeTaxTypeError)
        );
        assert!(B2bError::from_type("InvalidRequestError").is_none());
        assert!(B2bError::from_type("CashReceiptAlreadyIssuedError").is_none());
    }

    #[test]
    fn refine_maps_unclassified_and_passes_the_rest() {
        let err: Error = PortOneError::Unclassified {
            status: 404,
            kind: "B2bCounterpartyNotFoundError".into(),
        }
        .into();
        assert!(matches!(
            B2bError::refine(err),
            Error::B2b(B2bError::B2bCounterpartyNotFoundError)
        ));
        let err: Error = PortOneError::Unclassified {
            status: 409,
            kind: "SomethingNew".into(),
        }
        .into();
        assert!(matches!(
            B2bError::refine(err),
            Error::PortOneError(PortOneError::Unclassified { .. })
        ));
        assert!(matches!(
            B2bError::refine(PortOneError::ForbiddenError.into()),
            Error::PortOneError(PortOneError::ForbiddenError)
        ));
    }

    #[test]
    fn parses_response_body_into_variant() {
        let body = r#"{"type":"B2bNotEnabledError","message":"off"}"#;
        let err = PortOneError::from_response(reqwest::StatusCode::FORBIDDEN, body);
        assert!(matches!(
            B2bError::refine(err.into()),
            Error::B2b(B2bError::B2bNotEnabledError)
        ));
    }
}
