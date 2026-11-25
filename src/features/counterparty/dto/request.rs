use serde::Serialize;

use super::response::{BusinessStatus, NtsConnectionStatus};
use crate::features::b2b::PageInput;

/// 거래처 담당자 입력 정보
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInput {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// 거래처 입력 정보
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterpartyInput {
    /// 사업자등록번호 (`-` 없이 숫자만)
    pub brn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 업태
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    /// 업종
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInput>,
    /// 최대 5명
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contacts: Option<Vec<ContactInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// 거래처 생성·수정 옵션 (사업자 정보·휴폐업 상태 조회)
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_business_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_business_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info_verification_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_status_verification_id: Option<String>,
}

/// `POST /b2b/counterparties` 요청 본문
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCounterpartyBody {
    /// 미지정 시 서버가 임의 ID 를 채번
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_id: Option<String>,
    pub counterparty: CounterpartyInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateOptions>,
}

/// `PATCH /b2b/counterparties/{counterpartyId}` 요청 본문
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCounterpartyBody {
    pub counterparty: CounterpartyInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateOptions>,
}

/// 거래처 검색 필터. 모든 필드는 선택이다.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterpartyFilter {
    /// prefix 검색
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brn: Option<String>,
    /// 포함 검색
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_statuses: Option<Vec<BusinessStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nts_connection_statuses: Option<Vec<NtsConnectionStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_ids: Option<Vec<String>>,
}

/// `GET /b2b/counterparties` 조건 (`requestBody` 쿼리로 전달)
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCounterpartiesBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CounterpartyFilter>,
}
