use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use super::common::{
    AdditionalContact, Company, IssuanceType, Item, ModificationType, PurposeType,
    TaxInvoiceStatus, TaxationType,
};
use crate::features::b2b::PageInfo;

/// 문서 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentModificationType {
    /// 정상발행
    Normal,
    /// 수정발행
    Modification,
    #[serde(other)]
    Unknown,
}

/// 수정 사유 기재
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modification {
    #[serde(rename = "type")]
    pub modification_type: ModificationType,
    pub original_nts_approval_number: String,
    pub original_tax_invoice_id: String,
    pub root_tax_invoice_id: String,
}

/// 세금계산서 (`B2bTaxInvoice`)
///
/// `status` 가 [`TaxInvoiceStatus::Unknown`] 이면 알 수 없는 상태이므로 발행 성공으로 취급하지 않는다.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInvoice {
    pub id: String,
    pub status: TaxInvoiceStatus,
    pub taxation_type: TaxationType,
    pub document_modification_type: DocumentModificationType,
    pub is_delayed: Option<bool>,
    pub issuance_type: IssuanceType,
    pub bulk_tax_invoice_id: Option<String>,
    pub serial_number: Option<String>,
    pub book_volume: Option<i32>,
    pub book_issue: Option<i32>,
    pub write_date: NaiveDate,
    pub issuance_due_date: NaiveDate,
    pub purpose_type: PurposeType,
    pub total_supply_amount: i64,
    pub total_tax_amount: i64,
    pub total_amount: i64,
    pub cash_amount: Option<i64>,
    pub check_amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub note_amount: Option<i64>,
    pub remarks: Vec<String>,
    pub supplier_document_key: Option<String>,
    pub supplier: Company,
    pub recipient_document_key: Option<String>,
    pub recipient: Company,
    pub send_sms: Option<bool>,
    pub modification: Option<Modification>,
    pub items: Vec<Item>,
    pub additional_contacts: Vec<AdditionalContact>,
    pub memo: Option<String>,
    pub drafted_at: Option<DateTime<Utc>>,
    pub requested_at: Option<DateTime<Utc>>,
    pub issued_at: Option<DateTime<Utc>>,
    pub status_updated_at: Option<DateTime<Utc>>,
    pub nts_sent_at: Option<DateTime<Utc>>,
    /// 국세청 승인번호 (발행 시 자동 부여)
    pub nts_approval_number: Option<String>,
    pub nts_result: Option<String>,
    pub nts_result_code: Option<String>,
    pub nts_result_received_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 공급받는자 영업 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompanyStateBusinessStatus {
    InBusiness,
    Closed,
    Suspended,
    #[serde(other)]
    Unknown,
}

/// 세금계산서 요약 (다건 조회 항목)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInvoiceSummary {
    pub id: String,
    pub taxation_type: TaxationType,
    pub document_modification_type: DocumentModificationType,
    pub is_delayed: Option<bool>,
    pub issuance_type: IssuanceType,
    pub bulk_tax_invoice_id: Option<String>,
    pub payout_id: Option<String>,
    pub total_supply_amount: i64,
    pub total_tax_amount: i64,
    pub total_amount: i64,
    pub purpose_type: PurposeType,
    pub supplier_brn: String,
    pub supplier_name: String,
    pub supplier_representative_name: String,
    pub supplier_document_key: Option<String>,
    pub supplier_counterparty_id: Option<String>,
    pub recipient_brn: String,
    pub recipient_name: String,
    pub recipient_representative_name: String,
    pub recipient_document_key: Option<String>,
    pub recipient_counterparty_id: Option<String>,
    pub recipient_business_status: Option<CompanyStateBusinessStatus>,
    pub recipient_closed_suspended_date: Option<NaiveDate>,
    pub write_date: NaiveDate,
    pub issuance_due_date: NaiveDate,
    pub status: TaxInvoiceStatus,
    pub drafted_at: Option<DateTime<Utc>>,
    pub requested_at: Option<DateTime<Utc>>,
    pub issued_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub status_updated_at: DateTime<Utc>,
    pub nts_sent_at: Option<DateTime<Utc>>,
    pub nts_approval_number: Option<String>,
    pub nts_result: Option<String>,
    pub nts_result_received_at: Option<DateTime<Utc>>,
    pub nts_result_code: Option<String>,
    pub memo: Option<String>,
    pub items: Vec<Item>,
}

/// `POST /b2b/tax-invoices/issue-immediately` 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTaxInvoiceImmediatelyResponse {
    pub tax_invoice: TaxInvoice,
}

/// `POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-issuance` 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTaxInvoiceIssuanceResponse {
    pub tax_invoice: TaxInvoice,
}

/// `GET /b2b/tax-invoices` 응답
#[derive(Debug, Clone, Deserialize)]
pub struct GetTaxInvoicesResponse {
    pub items: Vec<TaxInvoiceSummary>,
    pub page: PageInfo,
}

/// PDF·팝업·프린트 URL 응답
#[derive(Debug, Clone, Deserialize)]
pub struct TaxInvoiceUrlResponse {
    pub url: String,
}
