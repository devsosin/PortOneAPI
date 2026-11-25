use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::features::b2b::PageInfo;

/// 거래처 휴폐업 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessStatus {
    /// 미조회 (서버 값 `UNKNOWN`)
    #[serde(rename = "UNKNOWN")]
    Unchecked,
    InBusiness,
    Closed,
    Suspended,
    /// 사업체 미등록
    NotFound,
    CheckPending,
    CheckFailed,
    /// 이 크레이트가 모르는 값 (서버 값 `UNKNOWN`(미조회)과는 다르다). 요청에 쓰지 않는다.
    #[serde(other)]
    Unrecognized,
}

/// 국세청 연동 상태. `Connected` 외에는 연동 완료로 보지 않는다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NtsConnectionStatus {
    NotConnected,
    PendingConnect,
    Connected,
    PendingDisconnect,
    Error,
    /// 이 크레이트가 모르는 값. 요청에 쓰지 않는다.
    #[serde(other)]
    Unknown,
}

/// 거래처 담당자 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: String,
    pub phone_number: Option<String>,
    pub email: String,
    pub memo: Option<String>,
}

/// 검증 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verification {
    pub id: String,
    pub checked_at: DateTime<Utc>,
}

/// 거래처
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterparty {
    pub id: String,
    pub is_for_test: bool,
    pub brn: String,
    pub company_name: String,
    pub representative_name: String,
    pub address: Option<String>,
    pub business_type: Option<String>,
    pub business_class: Option<String>,
    pub contact: Contact,
    pub additional_contacts: Vec<Contact>,
    pub memo: Option<String>,
    pub nts_connection_status: NtsConnectionStatus,
    pub nts_connected_at: Option<DateTime<Utc>>,
    pub nts_connection_failed_reason: Option<String>,
    /// 파트너 연동 거래처인 경우에만 존재
    pub partner_id: Option<String>,
    pub business_status: Option<BusinessStatus>,
    pub business_status_checked_at: Option<DateTime<Utc>>,
    pub business_status_verification: Option<Verification>,
    pub business_info_verification: Option<Verification>,
    pub applied_at: Option<DateTime<Utc>>,
}

/// `POST /b2b/counterparties` 응답
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCounterpartyResponse {
    pub counterparty: Counterparty,
}

/// `PATCH /b2b/counterparties/{counterpartyId}` 응답
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCounterpartyResponse {
    pub counterparty: Counterparty,
}

/// `DELETE /b2b/counterparties/{counterpartyId}` 응답 (빈 객체)
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCounterpartyResponse {}

/// `GET /b2b/counterparties` 응답
#[derive(Debug, Clone, Deserialize)]
pub struct GetCounterpartiesResponse {
    pub page: PageInfo,
    pub items: Vec<Counterparty>,
}
