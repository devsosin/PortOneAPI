pub mod dto;

use reqwest::Method;

use crate::{
    AuthedPortOneAPI,
    errors::Error,
    features::b2b::{B2bError, test_query},
    types::ClientResult,
};
use dto::{
    request::{CreateCounterpartyBody, GetCounterpartiesBody, UpdateCounterpartyBody},
    response::{
        Counterparty, CreateCounterpartyResponse, DeleteCounterpartyResponse,
        GetCounterpartiesResponse, UpdateCounterpartyResponse,
    },
};

/// B2B 거래처 API. 세금계산서 발행에 쓰는 공급받는자 등록·조회·수정·삭제.
///
/// 상태를 바꾸는 호출의 `idempotency_key`(선택) 규칙은 `PortOneTaxInvoiceService` 와 같다.
/// `test` 가 true 면 테스트 모드. 문서화된 거절은 `Error::B2b(B2bError::…)` 로 돌려준다.
/// 인증서 등록·조회·검증 호출은 아직 없다(`docs/research/cash-receipt-tax-invoice.md` "남은 작업").
pub trait PortOneCounterpartyService: Send + Sync {
    /// `POST /b2b/counterparties` 거래처 생성
    fn create_counterparty(
        &self,
        body: &CreateCounterpartyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<CreateCounterpartyResponse>> + Send;

    /// `GET /b2b/counterparties/{counterpartyId}` 거래처 조회
    fn get_counterparty(
        &self,
        counterparty_id: &str,
        test: bool,
    ) -> impl Future<Output = ClientResult<Counterparty>> + Send;

    /// `GET /b2b/counterparties` 거래처 검색 (조건은 `requestBody` 쿼리에 JSON 으로 전달)
    fn get_counterparties(
        &self,
        body: &GetCounterpartiesBody,
        test: bool,
    ) -> impl Future<Output = ClientResult<GetCounterpartiesResponse>> + Send;

    /// `PATCH /b2b/counterparties/{counterpartyId}` 거래처 정보 수정
    fn update_counterparty(
        &self,
        counterparty_id: &str,
        body: &UpdateCounterpartyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<UpdateCounterpartyResponse>> + Send;

    /// `DELETE /b2b/counterparties/{counterpartyId}` 거래처 삭제
    fn delete_counterparty(
        &self,
        counterparty_id: &str,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> impl Future<Output = ClientResult<DeleteCounterpartyResponse>> + Send;
}

impl PortOneCounterpartyService for AuthedPortOneAPI {
    async fn create_counterparty(
        &self,
        body: &CreateCounterpartyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> ClientResult<CreateCounterpartyResponse> {
        let url = self.build_url(&["b2b", "counterparties"], &test_query(test))?;
        self.request(Method::POST, url, Some(body), idempotency_key)
            .await
            .map_err(B2bError::refine)
    }

    async fn get_counterparty(
        &self,
        counterparty_id: &str,
        test: bool,
    ) -> ClientResult<Counterparty> {
        let url = self.build_url(
            &["b2b", "counterparties", counterparty_id],
            &test_query(test),
        )?;
        self.request::<(), _>(Method::GET, url, None, None)
            .await
            .map_err(B2bError::refine)
    }

    async fn get_counterparties(
        &self,
        body: &GetCounterpartiesBody,
        test: bool,
    ) -> ClientResult<GetCounterpartiesResponse> {
        let json = serde_json::to_string(body).map_err(|e| Error::InternalError(e.to_string()))?;
        let mut query = test_query(test);
        query.push(("requestBody", json));
        let url = self.build_url(&["b2b", "counterparties"], &query)?;
        self.request::<(), _>(Method::GET, url, None, None)
            .await
            .map_err(B2bError::refine)
    }

    async fn update_counterparty(
        &self,
        counterparty_id: &str,
        body: &UpdateCounterpartyBody,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> ClientResult<UpdateCounterpartyResponse> {
        let url = self.build_url(
            &["b2b", "counterparties", counterparty_id],
            &test_query(test),
        )?;
        self.request(Method::PATCH, url, Some(body), idempotency_key)
            .await
            .map_err(B2bError::refine)
    }

    async fn delete_counterparty(
        &self,
        counterparty_id: &str,
        test: bool,
        idempotency_key: Option<&str>,
    ) -> ClientResult<DeleteCounterpartyResponse> {
        let url = self.build_url(
            &["b2b", "counterparties", counterparty_id],
            &test_query(test),
        )?;
        self.request::<(), _>(Method::DELETE, url, None, idempotency_key)
            .await
            .map_err(B2bError::refine)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use dto::request::{ContactInput, CounterpartyFilter, CounterpartyInput, CreateOptions};
    use dto::response::{BusinessStatus, NtsConnectionStatus};

    fn input() -> CounterpartyInput {
        CounterpartyInput {
            brn: "1234567890".into(),
            name: Some("Example Co".into()),
            representative_name: None,
            address: None,
            business_type: None,
            business_class: None,
            contact: Some(ContactInput {
                name: "Kim".into(),
                phone_number: None,
                email: "a@example.com".into(),
                memo: None,
            }),
            additional_contacts: None,
            memo: None,
        }
    }

    fn counterparty_json(nts: &str, business: &str) -> serde_json::Value {
        json!({
            "id": "cp-1", "graphqlId": "g", "isForTest": true, "brn": "1234567890",
            "companyName": "Example Co", "representativeName": "Kim",
            "contact": {"name": "Kim", "email": "a@example.com"},
            "additionalContacts": [],
            "ntsConnectionStatus": nts, "businessStatus": business
        })
    }

    #[test]
    fn create_body_serializes_and_skips_none() {
        let body = CreateCounterpartyBody {
            counterparty_id: Some("cp-1".into()),
            counterparty: input(),
            options: Some(CreateOptions {
                check_business_status: Some(true),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "counterpartyId": "cp-1",
                "counterparty": {
                    "brn": "1234567890", "name": "Example Co",
                    "contact": {"name": "Kim", "email": "a@example.com"}
                },
                "options": {"checkBusinessStatus": true}
            })
        );
    }

    #[test]
    fn search_url_has_test_and_encoded_request_body() {
        let body = GetCounterpartiesBody {
            filter: Some(CounterpartyFilter {
                company_name: Some("a b&c".into()),
                nts_connection_statuses: Some(vec![NtsConnectionStatus::Connected]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert_eq!(
            json,
            r#"{"filter":{"companyName":"a b&c","ntsConnectionStatuses":["CONNECTED"]}}"#
        );
        let api = crate::PortOneAPI::new().authed_api("k");
        let mut query = test_query(true);
        query.push(("requestBody", json.clone()));
        let url = api.build_url(&["b2b", "counterparties"], &query).unwrap();
        assert!(
            url.query()
                .unwrap()
                .starts_with("test=true&requestBody=%7B")
        );
        let pairs: Vec<_> = url.query_pairs().collect();
        assert_eq!(pairs[1].1, json);
    }

    #[test]
    fn counterparty_id_is_encoded() {
        let api = crate::PortOneAPI::new().authed_api("k");
        let url = api
            .build_url(&["b2b", "counterparties", "a/b"], &test_query(false))
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/b2b/counterparties/a%2Fb"
        );
    }

    #[test]
    fn deserializes_counterparty_and_unknown_statuses() {
        let c: Counterparty =
            serde_json::from_value(counterparty_json("CONNECTED", "UNKNOWN")).unwrap();
        assert_eq!(c.nts_connection_status, NtsConnectionStatus::Connected);
        assert_eq!(c.business_status, Some(BusinessStatus::Unchecked));
        assert!(c.partner_id.is_none());

        let c: Counterparty =
            serde_json::from_value(counterparty_json("BRAND_NEW", "BRAND_NEW")).unwrap();
        assert_eq!(c.nts_connection_status, NtsConnectionStatus::Unknown);
        assert_ne!(c.nts_connection_status, NtsConnectionStatus::Connected);
        assert_eq!(c.business_status, Some(BusinessStatus::Unrecognized));
    }

    #[test]
    fn deserializes_list_and_delete_responses() {
        let l: GetCounterpartiesResponse = serde_json::from_value(json!({
            "page": {"number": 0, "size": 10, "totalCount": 1},
            "items": [counterparty_json("NOT_CONNECTED", "IN_BUSINESS")]
        }))
        .unwrap();
        assert_eq!(l.items[0].id, "cp-1");
        let _: DeleteCounterpartyResponse = serde_json::from_str("{}").unwrap();
    }
}
