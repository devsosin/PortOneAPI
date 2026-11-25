use chrono::NaiveDate;
use serde::Serialize;

use super::common::{
    AdditionalContact, Company, IssuanceType, Item, ModificationType, PurposeType,
    TaxInvoiceKeyType, TaxInvoiceStatus, TaxationType,
};
use crate::features::b2b::SortOrder;

/// 단건 조작 공통 쿼리 (`brn`, `taxInvoiceKeyType`, `test`)
#[derive(Debug, Clone, Default)]
pub struct TaxInvoiceLookup<'a> {
    /// 사업자등록번호. `key_type` 이 `TaxInvoiceId` 가 아니면 필요하다.
    pub brn: Option<&'a str>,
    /// 미지정 시 `TAX_INVOICE_ID`
    pub key_type: Option<TaxInvoiceKeyType>,
    /// 테스트 모드
    pub test: bool,
}

/// 세금계산서 생성 요청 정보 (`B2bTaxInvoiceInput`)
///
/// `issuance_type` 미지정 시 서버 기본값은 **역발행(`REVERSE`)** 이다. 즉시 정발행
/// (`issue_tax_invoice_immediately`)에는 `Some(IssuanceType::Normal)` 을 넣는다
/// (맞지 않으면 `B2bError::B2bIssuanceTypeMismatchError`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInvoiceInput {
    pub taxation_type: TaxationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuance_type: Option<IssuanceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_volume: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_issue: Option<i32>,
    /// 작성일 (`yyyy-MM-dd`)
    pub write_date: NaiveDate,
    pub purpose_type: PurposeType,
    pub total_supply_amount: i64,
    pub total_tax_amount: i64,
    pub total_amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_amount: Option<i64>,
    /// 비고 (최대 3개)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    /// 영문 대소문자, 숫자, `-`, `_` 만 가능
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_document_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Company>,
    /// 영문 대소문자, 숫자, `-`, `_` 만 가능
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_document_key: Option<String>,
    pub recipient: Company,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_sms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    /// 추가 담당자 (최대 5명)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contacts: Option<Vec<AdditionalContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_counterparty_additional_contacts: Option<bool>,
}

/// 수정 세금계산서 생성 입력 정보
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModificationCreateBody {
    #[serde(rename = "type")]
    pub modification_type: ModificationType,
    /// `tax_invoice_key_type` 이 `TaxInvoiceId` 가 아니면 필수
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brn: Option<String>,
    pub tax_invoice_key: String,
    /// 기본 `RECIPIENT` (`SUPPLIER`, `RECIPIENT` 만 지원)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_invoice_key_type: Option<TaxInvoiceKeyType>,
}

/// `POST /b2b/tax-invoices/issue-immediately` 요청 본문
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTaxInvoiceImmediatelyBody {
    pub tax_invoice: TaxInvoiceInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification: Option<ModificationCreateBody>,
    /// true 면 공급받는자 정보로 거래처를 자동 생성
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_recipient_counterparty: Option<bool>,
}

/// `POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-issuance` 요청 본문
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelTaxInvoiceIssuanceBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// 조회 기간 기준 (미지정 시 등록일)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchDateType {
    Register,
    Write,
    Issue,
}

/// 날짜 범위 (`yyyy-MM-dd`)
#[derive(Debug, Clone, Default, Serialize)]
pub struct DateRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<NaiveDate>,
}

/// 조회 기간 필터
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_type: Option<SearchDateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<Vec<DateRange>>,
}

/// 상위 필터. 필드 중 **한 개만** 채운다.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_filter: Option<DateFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_tax_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nts_approval_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_document_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_document_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_invoice_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<String>,
}

/// 다건 조회 필터. 모든 필드는 선택이다.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInvoiceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_filter: Option<PrimaryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_brn: Option<String>,
    /// 역발행이면 공급자, 정발행이면 공급받는자 사업자등록번호
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_brn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<TaxInvoiceStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxation_types: Option<Vec<TaxationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delayed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuance_types: Option<Vec<IssuanceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose_types: Option<Vec<PurposeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_brn_match: Option<bool>,
}

/// 정렬 기준
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxInvoiceSortBy {
    WriteDate,
    IssuanceDueDate,
    TotalAmount,
    TotalSupplyAmount,
    TotalTaxAmount,
    RequestedAt,
    IssuedAt,
    NtsSentAt,
    StatusUpdatedAt,
}

/// 정렬 조건 (미지정 시 상태 변경 일시 내림차순)
#[derive(Debug, Clone, Default, Serialize)]
pub struct TaxInvoiceSortInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<TaxInvoiceSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
}

/// `GET /b2b/tax-invoices` 조건 (`requestBody` 쿼리로 전달)
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaxInvoicesBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// 0부터 시작, 기본 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 기본 500, 최대 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TaxInvoiceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TaxInvoiceSortInput>,
}
