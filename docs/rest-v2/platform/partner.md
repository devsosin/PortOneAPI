# 파트너 정산: 파트너·계좌·사업자·설정

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `POST /b2b/companies/business-info` — 사업자등록 정보 조회 (platform.company)
- `GET /platform/bank-accounts/{bank}/{accountNumber}/holder` — 예금주 조회 (platform.account)
- `GET /platform/companies/{businessRegistrationNumber}/state` — 사업자 조회 (platform.company)
- `GET /platform/partner-filter-options` — 
- `POST /platform/partners/batch` — 파트너 다건 생성
- `POST /platform/partners/counterparty-connect/{id}` — 파트너 거래처 연동
- `POST /platform/partners/counterparty-connect` — 파트너 일괄 거래처 연동
- `POST /platform/partners/counterparty-disconnect/{id}` — 파트너 거래처 연동 해제
- `POST /platform/partners/counterparty-disconnect` — 파트너 일괄 거래처 연동 해제
- `POST /platform/partners/schedule` — 
- `POST /platform/partners/{id}/archive` — 파트너 보관
- `POST /platform/partners/{id}/recover` — 파트너 복원
- `GET /platform/partners/{id}/schedule` — 
- `PUT /platform/partners/{id}/schedule` — 
- `POST /platform/partners/{id}/schedule` — 
- `DELETE /platform/partners/{id}/schedule` — 
- `GET /platform/partners/{id}` — 파트너 조회
- `PATCH /platform/partners/{id}` — 파트너 수정
- `GET /platform/partners` — 파트너 다건 조회
- `POST /platform/partners` — 파트너 생성
- `GET /platform/setting` — 플랫폼 설정 조회
- `PATCH /platform/setting` — 플랫폼 설정 업데이트

## POST /b2b/companies/business-info

**사업자등록 정보 조회** (operationId: `getB2bBusinessInfos`)

요청된 사업자등록번호 리스트에 해당하는 사업자등록 정보를 조회합니다.
해당 API 사용을 위해서는 별도 문의가 필요합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [GetB2bBusinessInfosBody](#getb2bbusinessinfosbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brnList` | string[] | Y | 조회할 사업자등록번호 리스트 |

### 응답

- **200** 성공 응답으로 사업자등록 정보조회 결과 리스트를 반환합니다.: [GetB2bBusinessInfosResponse](#getb2bbusinessinfosresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /platform/bank-accounts/{bank}/{accountNumber}/holder

**예금주 조회** (operationId: `getPlatformAccountHolder`)

계좌의 예금주를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `bank` | path | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `accountNumber` | path | string | Y | '-'를 제외한 계좌 번호 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `birthdate` | query | string |  | 실명 조회를 위해 추가로 보낼 수 있습니다. birthdate과 businessRegistrationNumber 중 하나만 사용해야 합니다. |
| `businessRegistrationNumber` | query | string |  | 실명 조회를 위해 추가로 보낼 수 있습니다. birthdate과 businessRegistrationNumber 중 하나만 사용해야 합니다. |

### 응답

- **200** 성공 응답으로 조회된 예금주 명이 반환됩니다.: [PlatformAccountHolder](#platformaccountholder)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNotSupportedBankError`](../00-basics/errors.md#platformnotsupportedbankerror): 지원하지 않는 은행인 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **503**
  * [`PlatformExternalApiTemporarilyFailedError`](../00-basics/errors.md#platformexternalapitemporarilyfailederror): 외부 api의 일시적인 오류
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## GET /platform/companies/{businessRegistrationNumber}/state

**사업자 조회** (operationId: `getPlatformCompanyState`)

사업자 정보를 조회합니다. 포트원 서비스에 연동 및 등록되지 않은 사업자도 조회 가능합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `businessRegistrationNumber` | path | string | Y | 사업자등록번호 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 조회된 사업자 상태가 반환됩니다.: [GetPlatformCompanyStatePayload](#getplatformcompanystatepayload)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformCompanyNotFoundError`](../00-basics/errors.md#platformcompanynotfounderror): 사업자 정보를 찾을 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## GET /platform/partner-filter-options

**** (operationId: `getPlatformPartnerFilterOptions`)

파트너 다건 조회 시 필요한 필터 옵션을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `isArchived` | query | boolean |  | true 이면 보관된 파트너의 필터 옵션을 조회하고, false 이면 보관되지 않은 파트너의 필터 옵션을 조회합니다. 기본값은 false 입니다. |

### 응답

- **200** 성공 응답으로 조회된 파트너 필터 옵션을 반환합니다.: [PlatformPartnerFilterOptions](#platformpartnerfilteroptions)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/partners/batch

**파트너 다건 생성** (operationId: `createPlatformPartners`)

새로운 파트너를 다건 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformPartnersBody](#createplatformpartnersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partners` | [CreatePlatformPartnerBody](#createplatformpartnerbody)[] | Y | 생성할 파트너 리스트 정보 |

### 응답

- **200** 성공 응답: [CreatePlatformPartnersResponse](#createplatformpartnersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerIdsDuplicatedError`](../00-basics/errors.md#platformpartneridsduplicatederror)
  * [`PlatformCurrencyNotSupportedError`](../00-basics/errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractsNotFoundError`](../00-basics/errors.md#platformcontractsnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerIdsAlreadyExistError`](../00-basics/errors.md#platformpartneridsalreadyexisterror)

## POST /platform/partners/counterparty-connect/{id}

**파트너 거래처 연동** (operationId: `connectPartnerCounterparty`)

파트너를 거래처에 연동합니다. 팝빌 자동 가입이 진행되니 거래처 연동 전 파트너에게 안내가 필요합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 파트너 객체를 반환합니다.: [ConnectPartnerCounterpartyResponse](#connectpartnercounterpartyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerTypeIsNotBusinessError`](../00-basics/errors.md#platformpartnertypeisnotbusinesserror): 파트너 유형이 사업자가 아닌 경우
  * [`PlatformPartnerTaxationTypeIsSimpleError`](../00-basics/errors.md#platformpartnertaxationtypeissimpleerror): 파트너의 과세 유형이 간이 과세 세금계산서 미발행 유형인 경우
  * [`PlatformCounterpartyNotConnectableStatusError`](../00-basics/errors.md#platformcounterpartynotconnectablestatuserror): 파트너 거래처 연동 상태가 연동 가능한 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](../00-basics/errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformPartnerScheduleExistsError`](../00-basics/errors.md#platformpartnerscheduleexistserror): 파트너 수정 예약 건이 존재하는 경우
  * [`PlatformArchivedPartnerError`](../00-basics/errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformArchivedPartnerNtsNotAllowedError`](../00-basics/errors.md#platformarchivedpartnerntsnotallowederror): 보관된 파트너는 국세청 연동/연동해제를 할 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## POST /platform/partners/counterparty-connect

**파트너 일괄 거래처 연동** (operationId: `connectBulkPartnerCounterparty`)

파트너들을 일괄 거래처 연동합니다. 팝빌 자동 가입이 진행되니 거래처 연동 전 파트너에게 안내가 필요합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [ConnectBulkPartnerCounterpartyBody](#connectbulkpartnercounterpartybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 일괄 거래처 연동할 파트너 조건 필터 |

### 응답

- **200** 성공 응답으로 일괄 작업 객체를 반환합니다.: [ConnectBulkPartnerCounterpartyResponse](#connectbulkpartnercounterpartyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](../00-basics/errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformTargetPartnerNotFoundError`](../00-basics/errors.md#platformtargetpartnernotfounderror): 처리 대상 파트너가 존재하지 않는 경우
- **503**
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## POST /platform/partners/counterparty-disconnect/{id}

**파트너 거래처 연동 해제** (operationId: `disconnectPartnerCounterparty`)

파트너를 거래처 연동 해제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 파트너 객체를 반환합니다.: [DisconnectPartnerCounterpartyResponse](#disconnectpartnercounterpartyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerTypeIsNotBusinessError`](../00-basics/errors.md#platformpartnertypeisnotbusinesserror): 파트너 유형이 사업자가 아닌 경우
  * [`PlatformPartnerTaxationTypeIsSimpleError`](../00-basics/errors.md#platformpartnertaxationtypeissimpleerror): 파트너의 과세 유형이 간이 과세 세금계산서 미발행 유형인 경우
  * [`PlatformCounterpartyNotConnectedError`](../00-basics/errors.md#platformcounterpartynotconnectederror): 파트너가 거래처로 연동 되어있지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](../00-basics/errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformOngoingTaxInvoiceExistsError`](../00-basics/errors.md#platformongoingtaxinvoiceexistserror): 진행 중인 세금계산서가 존재하는 경우
  * [`PlatformArchivedPartnerNtsNotAllowedError`](../00-basics/errors.md#platformarchivedpartnerntsnotallowederror): 보관된 파트너는 국세청 연동/연동해제를 할 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## POST /platform/partners/counterparty-disconnect

**파트너 일괄 거래처 연동 해제** (operationId: `disconnectBulkPartnerCounterparty`)

파트너들을 일괄 거래처 연동 해제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [DisconnectBulkPartnerCounterpartyBody](#disconnectbulkpartnercounterpartybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 일괄 거래처 연동 해제할 파트너 조건 필터 |

### 응답

- **200** 성공 응답으로 일괄 작업 객체를 반환합니다.: [DisconnectBulkPartnerCounterpartyResponse](#disconnectbulkpartnercounterpartyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](../00-basics/errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformTargetPartnerNotFoundError`](../00-basics/errors.md#platformtargetpartnernotfounderror): 처리 대상 파트너가 존재하지 않는 경우
- **503**
  * [`PlatformExternalApiFailedError`](../00-basics/errors.md#platformexternalapifailederror): 외부 api 오류

## POST /platform/partners/schedule

**** (operationId: `schedulePlatformPartners`)

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [SchedulePlatformPartnersBody](#scheduleplatformpartnersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  |  |
| `update` | [SchedulePlatformPartnersBodyUpdate](#scheduleplatformpartnersbodyupdate) | Y |  |
| `appliedAt` | string(date-time) | Y |  |

### 응답

- **200** : [SchedulePlatformPartnersResponse](#scheduleplatformpartnersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerSchedulesAlreadyExistError`](../00-basics/errors.md#platformpartnerschedulesalreadyexisterror)
  * [`PlatformArchivedPartnersCannotBeScheduledError`](../00-basics/errors.md#platformarchivedpartnerscannotbeschedulederror): 보관된 파트너들을 예약 업데이트하려고 하는 경우
  * [`PlatformMemberCompanyConnectedPartnersCannotBeScheduledError`](../00-basics/errors.md#platformmembercompanyconnectedpartnerscannotbeschedulederror): 연동 사업자로 연동된 파트너들을 예약 수정하려고 시도한 경우

## POST /platform/partners/{id}/archive

**파트너 보관** (operationId: `archivePlatformPartner`)

주어진 아이디에 대응되는 파트너를 보관합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 보관된 파트너 객체를 반환합니다.: [ArchivePlatformPartnerResponse](#archiveplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledPartnerError`](../00-basics/errors.md#platformcannotarchivescheduledpartnererror): 예약된 업데이트가 있는 파트너를 보관하려고 하는 경우
  * [`PlatformCounterpartyOngoingTaxInvoiceExistsError`](../00-basics/errors.md#platformcounterpartyongoingtaxinvoiceexistserror): 연동된 거래처에 진행 중인 세금계산서가 있는 경우
  * [`PlatformPartnerPendingNtsOperationError`](../00-basics/errors.md#platformpartnerpendingntsoperationerror): 파트너의 국세청 연동/해제가 진행 중인 경우

## POST /platform/partners/{id}/recover

**파트너 복원** (operationId: `recoverPlatformPartner`)

주어진 아이디에 대응되는 파트너를 복원합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 복원된 파트너 객체를 반환합니다.: [RecoverPlatformPartnerResponse](#recoverplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformPartnerPendingNtsOperationError`](../00-basics/errors.md#platformpartnerpendingntsoperationerror): 파트너의 국세청 연동/해제가 진행 중인 경우

## GET /platform/partners/{id}/schedule

**** (operationId: `getPlatformPartnerSchedule`)

주어진 아이디에 대응되는 파트너의 예약 업데이트를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 예약된 파트너 객체를 반환합니다.: [PlatformPartner](schemas.md#platformpartner)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)

## PUT /platform/partners/{id}/schedule

**** (operationId: `reschedulePartner`)

주어진 아이디에 대응되는 파트너에 예약 업데이트를 재설정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [ReschedulePlatformPartnerBody](#rescheduleplatformpartnerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformPartnerBody](#updateplatformpartnerbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 파트너 객체를 반환합니다.: [ReschedulePlatformPartnerResponse](#rescheduleplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`](../00-basics/errors.md#platformmembercompanyconnectedpartnercannotbeschedulederror): 연동 사업자로 연동된 파트너를 예약 수정하려고 시도한 경우

## POST /platform/partners/{id}/schedule

**** (operationId: `schedulePartner`)

주어진 아이디에 대응되는 파트너에 업데이트를 예약합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [SchedulePlatformPartnerBody](#scheduleplatformpartnerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformPartnerBody](#updateplatformpartnerbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 파트너 객체가 반환됩니다.: [SchedulePlatformPartnerResponse](#scheduleplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](../00-basics/errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformInsufficientDataToChangePartnerTypeError`](../00-basics/errors.md#platforminsufficientdatatochangepartnertypeerror): 파트너 타입 수정에 필요한 데이터가 부족한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](../00-basics/errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerScheduleAlreadyExistsError`](../00-basics/errors.md#platformpartnerschedulealreadyexistserror)
  * [`PlatformArchivedPartnerError`](../00-basics/errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformAccountVerificationAlreadyUsedError`](../00-basics/errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`](../00-basics/errors.md#platformmembercompanyconnectedpartnerbrnunchangeableerror): 연동 사업자로 연동된 파트너의 사업자등록번호를 변경하려고 시도한 경우
  * [`PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`](../00-basics/errors.md#platformmembercompanyconnectedpartnertypeunchangeableerror): 연동 사업자로 연동된 파트너의 파트너 유형을 변경하려고 시도한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](../00-basics/errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`](../00-basics/errors.md#platformmembercompanyconnectedpartnercannotbeschedulederror): 연동 사업자로 연동된 파트너를 예약 수정하려고 시도한 경우

## DELETE /platform/partners/{id}/schedule

**** (operationId: `cancelPlatformPartnerSchedule`)

주어진 아이디에 대응되는 파트너의 예약 업데이트를 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답: [CancelPlatformPartnerScheduleResponse](#cancelplatformpartnerscheduleresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)

## GET /platform/partners/{id}

**파트너 조회** (operationId: `getPlatformPartner`)

파트너 객체를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회하고 싶은 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 파트너 객체가 반환됩니다.: [PlatformPartner](schemas.md#platformpartner)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)

## PATCH /platform/partners/{id}

**파트너 수정** (operationId: `updatePlatformPartner`)

주어진 아이디에 대응되는 파트너 정보를 업데이트합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 업데이트할 파트너 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdatePlatformPartnerBody](#updateplatformpartnerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 파트너 법인명 혹은 이름 |
| `contact` | [UpdatePlatformPartnerBodyContact](#updateplatformpartnerbodycontact) |  | 파트너 담당자 연락 정보 |
| `account` | [UpdatePlatformPartnerBodyAccount](#updateplatformpartnerbodyaccount) |  | 정산 계좌 |
| `defaultContractId` | string |  | 파트너에 설정된 기본 계약 아이디 |
| `memo` | string |  | 파트너에 대한 메모 |
| `tags` | string[] |  | 파트너의 태그 리스트 |
| `type` | [UpdatePlatformPartnerBodyType](#updateplatformpartnerbodytype) |  | 파트너 유형별 정보 |
| `userDefinedProperties` | [PlatformProperties](schemas.md#platformproperties) |  | 사용자 정의 속성 |

### 응답

- **200** 성공 응답으로 업데이트된 파트너 객체가 반환됩니다.: [UpdatePlatformPartnerResponse](#updateplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](../00-basics/errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformInsufficientDataToChangePartnerTypeError`](../00-basics/errors.md#platforminsufficientdatatochangepartnertypeerror): 파트너 타입 수정에 필요한 데이터가 부족한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](../00-basics/errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformArchivedPartnerError`](../00-basics/errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformAccountVerificationAlreadyUsedError`](../00-basics/errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`](../00-basics/errors.md#platformmembercompanyconnectedpartnerbrnunchangeableerror): 연동 사업자로 연동된 파트너의 사업자등록번호를 변경하려고 시도한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](../00-basics/errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`](../00-basics/errors.md#platformmembercompanyconnectedpartnertypeunchangeableerror): 연동 사업자로 연동된 파트너의 파트너 유형을 변경하려고 시도한 경우

## GET /platform/partners

**파트너 다건 조회** (operationId: `getPlatformPartners`)

여러 파트너를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformPartnersBody](#getplatformpartnersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 조회할 파트너 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 파트너 리스트와 페이지 정보가 반환됩니다.: [GetPlatformPartnersResponse](#getplatformpartnersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/partners

**파트너 생성** (operationId: `createPlatformPartner`)

새로운 파트너를 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformPartnerBody](#createplatformpartnerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 파트너에 부여할 고유 아이디 — 고객사 서버에 등록된 파트너 지칭 아이디와 동일하게 설정하는 것을 권장합니다. 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 파트너 법인명 혹은 이름 |
| `contact` | [CreatePlatformPartnerBodyContact](#createplatformpartnerbodycontact) | Y | 파트너 담당자 연락 정보 |
| `account` | [CreatePlatformPartnerBodyAccount](#createplatformpartnerbodyaccount) | Y | 정산 계좌 — 파트너의 사업자등록번호가 존재하는 경우 명시합니다. 별도로 검증하지는 않으며, 번호와 기호 모두 입력 가능합니다. |
| `defaultContractId` | string | Y | 기본 계약 아이디 — 이미 존재하는 계약 아이디를 등록해야 합니다. |
| `memo` | string |  | 파트너에 대한 메모 — 총 256자까지 입력할 수 있습니다. |
| `tags` | string[] | Y | 파트너에 부여할 태그 리스트 — 최대 10개까지 입력할 수 있습니다. |
| `type` | [CreatePlatformPartnerBodyType](#createplatformpartnerbodytype) | Y | 파트너 유형별 추가 정보 — 사업자/원천징수 대상자 중 추가할 파트너의 유형에 따른 정보를 입력해야 합니다. |
| `userDefinedProperties` | [PlatformProperties](schemas.md#platformproperties) |  | 사용자 정의 속성 |

### 응답

- **200** 성공 응답으로 생성된 파트너 객체가 반환됩니다.: [CreatePlatformPartnerResponse](#createplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](../00-basics/errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformCurrencyNotSupportedError`](../00-basics/errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](../00-basics/errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerIdAlreadyExistsError`](../00-basics/errors.md#platformpartneridalreadyexistserror)
  * [`PlatformAccountVerificationAlreadyUsedError`](../00-basics/errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](../00-basics/errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우

## GET /platform/setting

**플랫폼 설정 조회** (operationId: `getPlatformSetting`)

설정 정보를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 플랫폼 설정 정보를 반환합니다.: [PlatformSetting](#platformsetting)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## PATCH /platform/setting

**플랫폼 설정 업데이트** (operationId: `updatePlatformSetting`)

설정 정보를 업데이트합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdatePlatformSettingBody](#updateplatformsettingbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `defaultWithdrawalMemo` | string |  | 기본 보내는 이 통장 메모 |
| `defaultDepositMemo` | string |  | 기본 받는 이 통장 메모 |
| `supportsMultipleOrderTransfersPerPartner` | boolean |  | paymentId, storeId, partnerId가 같은 주문 정산건에 대한 중복 정산 지원 여부 |
| `adjustSettlementDateAfterHolidayIfEarlier` | boolean |  | 정산일이 정산시작일보다 작거나 같을 경우 공휴일 후 영업일로 정산일 다시 계산 여부 |
| `deductWht` | boolean |  | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](schemas.md#settlementamounttype) |  | 정산 금액 취급 기준 |
| `manualSettlementPerTransfer` | boolean |  | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### 응답

- **200** 성공 응답으로 플랫폼 설정 정보를 반환합니다.: [UpdatePlatformSettingResponse](#updateplatformsettingresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

---

# 스키마

### ArchivePlatformPartnerResponse

파트너 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 보관된 파트너 |

### B2bBusinessInfo

사업자등록 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string | Y | 사업자등록번호 |
| `name` | string | Y | 상호 |
| `ceoName` | string | Y | 대표자명 |
| `zipCode` | string | Y | 우편번호 |
| `address` | string | Y | 주소 |
| `businessEntityType` | string | Y | 사업자 유형 |
| `businessStatus` | string | Y | 사업 상태 |
| `taxationType` | string | Y | 과세 유형 |
| `simplifiedTaxationTypeDate` | string |  | 간이과세-일반과세 전환일 |
| `closingDate` | string |  | 폐업일 |
| `openingDate` | string | Y | 개업일 |
| `businessType` | string | Y | 업태 |
| `businessClass` | string | Y | 종목 |
| `businessCategoryCode` | string | Y | 업종코드 |
| `corpRegNo` | string |  | 법인등록번호 |
| `phoneNumber` | string |  | 전화번호 |
| `taxOfficeCode` | string |  | 관할세무서코드 |
| `taxOfficeName` | string |  | 관할세무서명 |

### B2bBusinessInfoResult

사업자등록 정보조회 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string | Y | 사업자등록번호 |
| `businessInfo` | [B2bBusinessInfo](#b2bbusinessinfo) |  | 사업자등록 정보 |
| `error` | string |  | 조회 실패 시 에러 메시지 |
| `verificationId` | string |  | 조회 결과 ID — 거래처 생성/수정 시 사업자 정보 조회 결과를 재사용하기 위한 ID입니다. 조회 성공 시에만 설정됩니다. |

### CancelPlatformPartnerScheduleResponse

파트너 예약 업데이트 취소 성공 응답

type: object

### ConnectBulkPartnerCounterpartyBody

파트너 일괄 거래처 연동 요청 정보 — 파트너 일괄 거래처 연동 요청 정보<br>파트너들을 일괄 거래처 연동합니다. 팝빌 자동 가입이 진행되니 거래처 연동 전 파트너에게 안내가 필요합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 일괄 거래처 연동할 파트너 조건 필터 |

### ConnectBulkPartnerCounterpartyResponse

파트너 일괄 거래처 연동 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkTask` | [PlatformBulkTask](#platformbulktask) | Y |  |

### ConnectPartnerCounterpartyResponse

파트너 거래처 연동 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y |  |

### CreatePlatformPartnerBody

파트너 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 파트너에 부여할 고유 아이디 — 고객사 서버에 등록된 파트너 지칭 아이디와 동일하게 설정하는 것을 권장합니다. 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 파트너 법인명 혹은 이름 |
| `contact` | [CreatePlatformPartnerBodyContact](#createplatformpartnerbodycontact) | Y | 파트너 담당자 연락 정보 |
| `account` | [CreatePlatformPartnerBodyAccount](#createplatformpartnerbodyaccount) | Y | 정산 계좌 — 파트너의 사업자등록번호가 존재하는 경우 명시합니다. 별도로 검증하지는 않으며, 번호와 기호 모두 입력 가능합니다. |
| `defaultContractId` | string | Y | 기본 계약 아이디 — 이미 존재하는 계약 아이디를 등록해야 합니다. |
| `memo` | string |  | 파트너에 대한 메모 — 총 256자까지 입력할 수 있습니다. |
| `tags` | string[] | Y | 파트너에 부여할 태그 리스트 — 최대 10개까지 입력할 수 있습니다. |
| `type` | [CreatePlatformPartnerBodyType](#createplatformpartnerbodytype) | Y | 파트너 유형별 추가 정보 — 사업자/원천징수 대상자 중 추가할 파트너의 유형에 따른 정보를 입력해야 합니다. |
| `userDefinedProperties` | [PlatformProperties](schemas.md#platformproperties) |  | 사용자 정의 속성 |

### CreatePlatformPartnerBodyAccount

파트너 계좌 등록을 위한 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산에 사용할 통화 |
| `number` | string | Y | 계좌번호 |
| `holder` | string | Y | 예금주명 |
| `accountVerificationId` | string |  | 계좌 검증 아이디 |

### CreatePlatformPartnerBodyContact

파트너 담당자 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 담당자 이름 |
| `phoneNumber` | string |  | 담당자 휴대폰 번호 |
| `email` | string | Y | 담당자 이메일 |

### CreatePlatformPartnerBodyType

파트너 생성을 위한 유형별 추가 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `business` | [CreatePlatformPartnerBodyTypeBusiness](#createplatformpartnerbodytypebusiness) |  | 사업자 추가 정보 |
| `whtPayer` | [CreatePlatformPartnerBodyTypeWhtPayer](#createplatformpartnerbodytypewhtpayer) |  | 원천징수 대상자 추가 정보 |
| `nonWhtPayer` | [CreatePlatformPartnerBodyTypeNonWhtPayer](#createplatformpartnerbodytypenonwhtpayer) |  | 원천징수 비대상자 추가 정보 |

### CreatePlatformPartnerBodyTypeBusiness

CreatePlatformPartnerBodyTypeBusiness

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyName` | string | Y | 상호명 |
| `taxationType` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype) |  | 사업자 유형 — 값을 입력하지 않으면 일반 과세로 설정됩니다. |
| `businessRegistrationNumber` | string | Y | 사업자등록번호 |
| `representativeName` | string | Y | 대표자 이름 |
| `companyAddress` | string |  | 사업장 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `companyVerificationId` | string |  | 사업자 조회 검증 아이디 |

### CreatePlatformPartnerBodyTypeNonWhtPayer

CreatePlatformPartnerBodyTypeNonWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### CreatePlatformPartnerBodyTypeWhtPayer

CreatePlatformPartnerBodyTypeWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### CreatePlatformPartnerResponse

파트너 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 생성된 파트너 |

### CreatePlatformPartnersBody

파트너 다건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partners` | [CreatePlatformPartnerBody](#createplatformpartnerbody)[] | Y | 생성할 파트너 리스트 정보 |

### CreatePlatformPartnersResponse

파트너 다건 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partners` | [PlatformPartner](schemas.md#platformpartner)[] | Y | 생성된 파트너 리스트 |

### DisconnectBulkPartnerCounterpartyBody

파트너 일괄 거래처 연동 해제 요청 정보 — 파트너 일괄 거래처 연동 해제 요청 정보<br>파트너들을 일괄 거래처 연동 해제합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 일괄 거래처 연동 해제할 파트너 조건 필터 |

### DisconnectBulkPartnerCounterpartyResponse

파트너 일괄 거래처 연동 해제 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkTask` | [PlatformBulkTask](#platformbulktask) | Y |  |

### DisconnectPartnerCounterpartyResponse

파트너 거래처 연동 해제 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y |  |

### GetB2bBusinessInfosBody

사업자등록 정보 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brnList` | string[] | Y | 조회할 사업자등록번호 리스트 |

### GetB2bBusinessInfosResponse

사업자등록 정보 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `result` | [B2bBusinessInfoResult](#b2bbusinessinforesult)[] | Y | 사업자등록 정보 리스트 |

### GetPlatformCompanyStatePayload

사업자 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyState` | [PlatformCompanyState](#platformcompanystate) | Y | 사업자 정보 |
| `companyVerificationId` | string | Y | 사업자 검증 아이디 |

### GetPlatformPartnersBody

파트너 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 조회할 파트너 조건 필터 |

### GetPlatformPartnersResponse

파트너 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPartner](schemas.md#platformpartner)[] | Y | 조회된 파트너 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### PlatformAccountHolder

예금주 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `holderName` | string | Y | 계좌 예금주 이름 |
| `accountVerificationId` | string | Y | 계좌 검증 아이디 |

### PlatformBulkTask

PlatformBulkTask

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `status` | [PlatformBulkTaskStatus](#platformbulktaskstatus) | Y |  |
| `type` | [PlatformBulkTaskType](#platformbulktasktype) | Y |  |
| `progressStats` | [PlatformBulkTaskProgressStats](#platformbulktaskprogressstats) | Y |  |
| `isForTest` | boolean | Y |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `createdAt` | string(date-time) | Y |  |
| `updatedAt` | string(date-time) | Y |  |

### PlatformBulkTaskProgressStats

PlatformBulkTaskProgressStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `preparedCount` | integer(int64) | Y |  |
| `processingCount` | integer(int64) | Y |  |
| `succeededCount` | integer(int64) | Y |  |
| `failedCount` | integer(int64) | Y |  |
| `canceledCount` | integer(int64) | Y |  |

### PlatformBulkTaskStatus

PlatformBulkTaskStatus

- `PREPARED`
- `PROCESSING`
- `COMPLETED`
- `CANCELED`

### PlatformBulkTaskType

PlatformBulkTaskType

- `CREATE_TRANSFERS` — 정산건 일괄 등록
- `CREATE_PARTNERS` — 파트너 일괄 등록
- `CONNECT_MEMBER_COMPANIES` — 파트너 일괄 국세청 연동
- `DISCONNECT_MEMBER_COMPANIES` — 파트너 일괄 국세청 연동 해제
- `SEND_PAYOUT_SETTLEMENT_STATEMENTS` — 정산 내역서 일괄 발송

### PlatformBusinessStatus

플랫폼 사업자 상태

- `IN_BUSINESS` — 사업 중
- `CLOSED` — 폐업
- `SUSPENDED` — 휴업

### PlatformCompanyState

PlatformCompanyState

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxationType` | [PlatformTaxationType](#platformtaxationtype) | Y |  |
| `taxationTypeDate` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `businessStatus` | [PlatformBusinessStatus](#platformbusinessstatus) | Y |  |
| `closedSuspendedDate` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPartnerContractSummary

파트너 계약 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 계약 고유 아이디 |
| `name` | string | Y | 계약 이름 |

### PlatformPartnerFilterInput

파트너 필터 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 파트너를 조회하고, false 이면 보관되지 않은 파트너를 조회합니다. 기본값은 false 입니다. |
| `tags` | string[] |  | 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 태그를 하나 이상 가지는 파트너만 조회합니다. |
| `banks` | [Bank](../00-basics/schemas-common.md#bank)[] |  | 은행 — 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 계좌 은행을 가진 파트너만 조회합니다. |
| `accountCurrencies` | [Currency](../00-basics/schemas-common.md#currency)[] |  | 통화 — 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 계좌 통화를 가진 파트너만 조회합니다. |
| `ids` | string[] |  | 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 아이디를 가진 파트너만 조회합니다. |
| `contractIds` | string[] |  | 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 기본 계약 id를 가진 파트너만 조회합니다. |
| `accountStatuses` | [PlatformAccountStatus](schemas.md#platformaccountstatus)[] |  | 플랫폼 계좌 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 계좌 상태를 가진 파트너만 조회합니다. |
| `businessStatuses` | [PlatformPartnerBusinessStatus](schemas.md#platformpartnerbusinessstatus)[] |  | 플랫폼 파트너 사업자 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 사업자 상태를 가진 파트너만 조회합니다. |
| `types` | [PlatformPartnerTypeName](schemas.md#platformpartnertypename)[] |  | 플랫폼 파트너 유형 이름 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 사업자 유형을 가진 파트너만 조회합니다. |
| `taxationTypes` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype)[] |  | 플랫폼 파트너 과세 유형 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 과세 유형을 가진 파트너만 조회합니다. |
| `memberCompanyConnectionStatuses` | [PlatformPartnerMemberCompanyConnectionStatus](schemas.md#platformpartnermembercompanyconnectionstatus)[] |  | 플랫폼 파트너 연동 사업자 연결 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 연동 사업자 연결 상태를 가진 파트너만 조회합니다. |
| `keyword` | [PlatformPartnerFilterInputKeyword](#platformpartnerfilterinputkeyword) |  | 검색 키워드 |

### PlatformPartnerFilterInputKeyword

파트너 검색 키워드 입력 정보 — 파트너 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 파트너만 조회합니다. 하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 해당 값이 포함된 id 를 가진 파트너만 조회합니다. |
| `name` | string |  | 해당 값이 포함된 이름 을 가진 파트너만 조회합니다. |
| `email` | string |  | 해당 값이 포함된 이메일 주소를 가진 파트너만 조회합니다. |
| `businessRegistrationNumber` | string |  | 해당 값이 포함된 사업자등록번호를 가진 파트너만 조회합니다. |
| `defaultContractId` | string |  | 해당 값이 포함된 기본 계약 아이디를 가진 파트너만 조회합니다. |
| `memo` | string |  | 해당 값이 포함된 메모를 가진 파트너만 조회합니다. |
| `accountNumber` | string |  | 해당 값이 포함된 계좌번호를 가진 파트너만 조회합니다. |
| `accountHolder` | string |  | 해당 값이 포함된 계좌 예금주명을 가진 파트너만 조회합니다. |

### PlatformPartnerFilterOptions

파트너 필터 옵션 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `tags` | string[] | Y | 조회된 태그 리스트 |
| `contractSummary` | [PlatformPartnerContractSummary](#platformpartnercontractsummary)[] | Y | 조회된 파트너 계약 요약 정보 리스트 |

### PlatformSetting

플랫폼 설정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `defaultWithdrawalMemo` | string |  | 기본 보내는 이 통장 메모 |
| `defaultDepositMemo` | string |  | 기본 받는 이 통장 메모 |
| `supportsMultipleOrderTransfersPerPartner` | boolean | Y | paymentId, storeId, partnerId가 같은 주문 정산건에 대한 중복 정산 지원 여부 |
| `adjustSettlementDateAfterHolidayIfEarlier` | boolean | Y | 정산일이 정산시작일보다 작거나 같을 경우 공휴일 후 영업일로 정산일 다시 계산 여부 |
| `deductWht` | boolean | Y | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](schemas.md#settlementamounttype) | Y | 정산 금액 취급 기준 |
| `isForTest` | boolean | Y |  |
| `manualSettlementPerTransfer` | boolean | Y | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### PlatformTaxationType

플랫폼 과세 유형

- `NORMAL` — 일반 과세
- `SIMPLE_TAX_INVOICE_ISSUER` — 간이과세(세금계산서 발행)
- `SIMPLE` — 간이과세(세금계산서 미발행)
- `TAX_FREE` — 면세
- `ASSIGNED_ID_NUMBER` — 고유 번호 부여 사업자 (비영리, 국가 등 납세 의무가 없는)
- `SPECIAL` — 과세 특례자

### RecoverPlatformPartnerResponse

파트너 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 복원된 파트너 |

### ReschedulePlatformPartnerBody

파트너 예약 업데이트 재설정을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformPartnerBody](#updateplatformpartnerbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### ReschedulePlatformPartnerResponse

파트너 예약 업데이트 재설정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledPartner` | [PlatformPartner](schemas.md#platformpartner) | Y | 예약된 파트너 정보 |

### SchedulePlatformPartnerBody

파트너 업데이트 예약을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformPartnerBody](#updateplatformpartnerbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### SchedulePlatformPartnerResponse

파트너 업데이트 예약 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledPartner` | [PlatformPartner](schemas.md#platformpartner) | Y | 예약된 파트너 정보 |

### SchedulePlatformPartnersBody

SchedulePlatformPartnersBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  |  |
| `update` | [SchedulePlatformPartnersBodyUpdate](#scheduleplatformpartnersbodyupdate) | Y |  |
| `appliedAt` | string(date-time) | Y |  |

### SchedulePlatformPartnersBodyUpdate

SchedulePlatformPartnersBodyUpdate

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  |  |
| `contact` | [SchedulePlatformPartnersBodyUpdateContact](#scheduleplatformpartnersbodyupdatecontact) |  |  |
| `type` | [SchedulePlatformPartnersBodyUpdateType](#scheduleplatformpartnersbodyupdatetype) |  |  |
| `account` | [SchedulePlatformPartnersBodyUpdateAccount](#scheduleplatformpartnersbodyupdateaccount) |  |  |
| `defaultContractId` | string |  |  |
| `memo` | string |  |  |
| `tags` | string[] |  |  |
| `userDefinedProperties` | [PlatformProperties](schemas.md#platformproperties) |  |  |

### SchedulePlatformPartnersBodyUpdateAccount

파트너 계좌 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산에 사용할 통화 |
| `number` | string | Y | 계좌번호 |
| `holder` | string | Y | 예금주명 |
| `accountVerificationId` | string |  | 계좌 검증 아이디 |

### SchedulePlatformPartnersBodyUpdateContact

파트너 업데이트를 위한 유형별 추가 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 담당자 이름 |
| `phoneNumber` | string |  | 담당자 휴대폰 번호 |
| `email` | string |  | 담당자 이메일 |

### SchedulePlatformPartnersBodyUpdateType

파트너 유형별 정보 업데이트를 위한 입력 정보 — 파트너 유형별 정보 업데이트를 위한 입력 정보<br>파트너 유형별 추가 정보를 수정합니다.<br>최초 생성된 유형 내에서 세부 정보만 수정할 수 있고 파트너의 유형 자체를 수정할 수는 없습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `business` | [SchedulePlatformPartnersBodyUpdateTypeBusiness](#scheduleplatformpartnersbodyupdatetypebusiness) |  | 사업자 추가 정보 |
| `whtPayer` | [SchedulePlatformPartnersBodyUpdateTypeWhtPayer](#scheduleplatformpartnersbodyupdatetypewhtpayer) |  | 원천징수 대상자 추가 정보 |
| `nonWhtPayer` | [SchedulePlatformPartnersBodyUpdateTypeNonWhtPayer](#scheduleplatformpartnersbodyupdatetypenonwhtpayer) |  | 원천징수 비대상자 추가 정보 |

### SchedulePlatformPartnersBodyUpdateTypeBusiness

SchedulePlatformPartnersBodyUpdateTypeBusiness

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyName` | string |  | 상호명 |
| `taxationType` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype) |  | 사업자 유형 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `representativeName` | string |  | 대표자 이름 |
| `companyAddress` | string |  | 사업장 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `companyVerificationId` | string |  | 사업자 조회 검증 아이디 |

### SchedulePlatformPartnersBodyUpdateTypeNonWhtPayer

SchedulePlatformPartnersBodyUpdateTypeNonWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### SchedulePlatformPartnersBodyUpdateTypeWhtPayer

SchedulePlatformPartnersBodyUpdateTypeWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### SchedulePlatformPartnersResponse

SchedulePlatformPartnersResponse

type: object

### UpdatePlatformPartnerBody

파트너 업데이트를 위한 입력 정보 — 파트너 업데이트를 위한 입력 정보<br>값이 명시되지 않은 필드는 업데이트되지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 파트너 법인명 혹은 이름 |
| `contact` | [UpdatePlatformPartnerBodyContact](#updateplatformpartnerbodycontact) |  | 파트너 담당자 연락 정보 |
| `account` | [UpdatePlatformPartnerBodyAccount](#updateplatformpartnerbodyaccount) |  | 정산 계좌 |
| `defaultContractId` | string |  | 파트너에 설정된 기본 계약 아이디 |
| `memo` | string |  | 파트너에 대한 메모 |
| `tags` | string[] |  | 파트너의 태그 리스트 |
| `type` | [UpdatePlatformPartnerBodyType](#updateplatformpartnerbodytype) |  | 파트너 유형별 정보 |
| `userDefinedProperties` | [PlatformProperties](schemas.md#platformproperties) |  | 사용자 정의 속성 |

### UpdatePlatformPartnerBodyAccount

파트너 계좌 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산에 사용할 통화 |
| `number` | string | Y | 계좌번호 |
| `holder` | string | Y | 예금주명 |
| `accountVerificationId` | string |  | 계좌 검증 아이디 |

### UpdatePlatformPartnerBodyContact

파트너 담당자 업데이트를 위한 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 담당자 이름 |
| `phoneNumber` | string |  | 담당자 휴대폰 번호 |
| `email` | string |  | 담당자 이메일 |

### UpdatePlatformPartnerBodyType

파트너 업데이트를 위한 유형별 추가 정보 — 파트너 업데이트를 위한 유형별 추가 정보<br>파트너 유형별 추가 정보를 수정합니다.<br>기존과 다른 파트너 유형 정보가 입력된 경우, 파트너의 유형 자체가 변경됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `business` | [UpdatePlatformPartnerBodyTypeBusiness](#updateplatformpartnerbodytypebusiness) |  | 사업자 추가 정보 |
| `whtPayer` | [UpdatePlatformPartnerBodyTypeWhtPayer](#updateplatformpartnerbodytypewhtpayer) |  | 원천징수 대상자 추가 정보 |
| `nonWhtPayer` | [UpdatePlatformPartnerBodyTypeNonWhtPayer](#updateplatformpartnerbodytypenonwhtpayer) |  | 원천징수 비대상자 추가 정보 |

### UpdatePlatformPartnerBodyTypeBusiness

UpdatePlatformPartnerBodyTypeBusiness

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyName` | string |  | 상호명 |
| `taxationType` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype) |  | 사업자 유형 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `representativeName` | string |  | 대표자 이름 |
| `companyAddress` | string |  | 사업장 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `companyVerificationId` | string |  | 사업자 조회 검증 아이디 |

### UpdatePlatformPartnerBodyTypeNonWhtPayer

UpdatePlatformPartnerBodyTypeNonWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### UpdatePlatformPartnerBodyTypeWhtPayer

UpdatePlatformPartnerBodyTypeWhtPayer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### UpdatePlatformPartnerResponse

파트너 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 업데이트된 파트너 |

### UpdatePlatformSettingBody

플랫폼 설정 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `defaultWithdrawalMemo` | string |  | 기본 보내는 이 통장 메모 |
| `defaultDepositMemo` | string |  | 기본 받는 이 통장 메모 |
| `supportsMultipleOrderTransfersPerPartner` | boolean |  | paymentId, storeId, partnerId가 같은 주문 정산건에 대한 중복 정산 지원 여부 |
| `adjustSettlementDateAfterHolidayIfEarlier` | boolean |  | 정산일이 정산시작일보다 작거나 같을 경우 공휴일 후 영업일로 정산일 다시 계산 여부 |
| `deductWht` | boolean |  | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](schemas.md#settlementamounttype) |  | 정산 금액 취급 기준 |
| `manualSettlementPerTransfer` | boolean |  | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### UpdatePlatformSettingResponse

플랫폼 설정 업데이트 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `setting` | [PlatformSetting](#platformsetting) | Y |  |
