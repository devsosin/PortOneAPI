# 파트너 정산 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `POST /b2b/companies/business-info` — 사업자등록 정보 조회 (platform.company)
- `GET /platform/account-transfers` — 이체 내역 다건 조회 (platform.accountTransfer)
- `POST /platform/additional-fee-policies/{id}/archive` — 추가 수수료 정책 보관 (platform.policy)
- `POST /platform/additional-fee-policies/{id}/recover` — 추가 수수료 정책 복원 (platform.policy)
- `GET /platform/additional-fee-policies/{id}/schedule` — 
- `PUT /platform/additional-fee-policies/{id}/schedule` — 
- `POST /platform/additional-fee-policies/{id}/schedule` — 
- `DELETE /platform/additional-fee-policies/{id}/schedule` — 
- `GET /platform/additional-fee-policies/{id}` — 추가 수수료 정책 조회 (platform.policy)
- `PATCH /platform/additional-fee-policies/{id}` — 추가 수수료 정책 수정 (platform.policy)
- `GET /platform/additional-fee-policies` — 추가 수수료 정책 다건 조회 (platform.policy)
- `POST /platform/additional-fee-policies` — 추가 수수료 정책 생성 (platform.policy)
- `GET /platform/bank-accounts/{bank}/{accountNumber}/holder` — 예금주 조회 (platform.account)
- `GET /platform/bulk-account-transfers` — 일괄 이체 내역 다건 조회 (platform.bulkAccountTransfer)
- `GET /platform/bulk-payouts` — 일괄 지급 내역 다건 조회 (platform.bulkPayout)
- `GET /platform/companies/{businessRegistrationNumber}/state` — 사업자 조회 (platform.company)
- `POST /platform/contracts/{id}/archive` — 계약 보관 (platform.policy)
- `POST /platform/contracts/{id}/recover` — 계약 복원 (platform.policy)
- `GET /platform/contracts/{id}/schedule` — 
- `PUT /platform/contracts/{id}/schedule` — 
- `POST /platform/contracts/{id}/schedule` — 
- `DELETE /platform/contracts/{id}/schedule` — 
- `GET /platform/contracts/{id}` — 계약 조회 (platform.policy)
- `PATCH /platform/contracts/{id}` — 계약 수정 (platform.policy)
- `GET /platform/contracts` — 계약 다건 조회 (platform.policy)
- `POST /platform/contracts` — 계약 생성 (platform.policy)
- `POST /platform/discount-share-policies/{id}/archive` — 할인 분담 정책 보관 (platform.policy)
- `POST /platform/discount-share-policies/{id}/recover` — 할인 분담 정책 복원 (platform.policy)
- `GET /platform/discount-share-policies/{id}/schedule` — 
- `PUT /platform/discount-share-policies/{id}/schedule` — 
- `POST /platform/discount-share-policies/{id}/schedule` — 
- `DELETE /platform/discount-share-policies/{id}/schedule` — 
- `GET /platform/discount-share-policies/{id}` — 할인 분담 정책 조회 (platform.policy)
- `PATCH /platform/discount-share-policies/{id}` — 할인 분담 정책 수정 (platform.policy)
- `GET /platform/discount-share-policies` — 할인 분담 정책 다건 조회 (platform.policy)
- `POST /platform/discount-share-policies` — 할인 분담 정책 생성 (platform.policy)
- `GET /platform/discount-share-policy-filter-options` — 
- `GET /platform/partner-filter-options` — 
- `POST /platform/partner-settlements/complete-payout` — 일괄 지급 완료 처리 (platform.payout)
- `POST /platform/partner-settlements/delete` — 정산내역 삭제 (platform.partnerSettlement)
- `GET /platform/partner-settlements` — 정산 내역 다건 조회 (platform.partnerSettlement)
- `POST /platform/partners/batch` — 파트너 다건 생성 (platform.partner)
- `POST /platform/partners/counterparty-connect/{id}` — 파트너 거래처 연동 (platform.partner)
- `POST /platform/partners/counterparty-connect` — 파트너 일괄 거래처 연동 (platform.partner)
- `POST /platform/partners/counterparty-disconnect/{id}` — 파트너 거래처 연동 해제 (platform.partner)
- `POST /platform/partners/counterparty-disconnect` — 파트너 일괄 거래처 연동 해제 (platform.partner)
- `POST /platform/partners/schedule` — 
- `POST /platform/partners/{id}/archive` — 파트너 보관 (platform.partner)
- `POST /platform/partners/{id}/recover` — 파트너 복원 (platform.partner)
- `GET /platform/partners/{id}/schedule` — 
- `PUT /platform/partners/{id}/schedule` — 
- `POST /platform/partners/{id}/schedule` — 
- `DELETE /platform/partners/{id}/schedule` — 
- `GET /platform/partners/{id}` — 파트너 조회 (platform.partner)
- `PATCH /platform/partners/{id}` — 파트너 수정 (platform.partner)
- `GET /platform/partners` — 파트너 다건 조회 (platform.partner)
- `POST /platform/partners` — 파트너 생성 (platform.partner)
- `GET /platform/payouts` — 지급 내역 다건 조회 (platform.payout)
- `GET /platform/setting` — 플랫폼 설정 조회
- `PATCH /platform/setting` — 플랫폼 설정 업데이트
- `GET /platform/transfer-summaries/sheet-file` — 정산 상세 내역 다운로드 (platform.transfer)
- `GET /platform/transfer-summaries` — 정산건 다건 조회 (platform.transfer)
- `POST /platform/transfers/manual` — 수기 정산건 생성 (platform.transfer)
- `POST /platform/transfers/order-cancel` — 주문 취소 정산건 생성 (platform.transfer)
- `POST /platform/transfers/order` — 주문 정산건 생성 (platform.transfer)
- `GET /platform/transfers/{id}` — 정산건 조회 (platform.transfer)
- `DELETE /platform/transfers/{id}` — 정산건 삭제 (platform.transfer)

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /platform/account-transfers

**이체 내역 다건 조회** (operationId: `getPlatformAccountTransfers`)

여러 이체 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetAccountTransfersBody1](#getaccounttransfersbody1)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformAccountTransferFilter](#platformaccounttransferfilter) |  |  |

### 응답

- **200** 성공 응답으로 조회된 이체 내역과 페이지 정보를 반환합니다.: [GetPlatformAccountTransfersResponse](#getplatformaccounttransfersresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/additional-fee-policies/{id}/archive

**추가 수수료 정책 보관** (operationId: `archivePlatformAdditionalFeePolicy`)

주어진 아이디에 대응되는 추가 수수료 정책을 보관합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 보관된 추가 수수료 정책 객체를 반환합니다.: [ArchivePlatformAdditionalFeePolicyResponse](#archiveplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledAdditionalFeePolicyError`](errors.md#platformcannotarchivescheduledadditionalfeepolicyerror): 예약된 업데이트가 있는 추가 수수료 정책을 보관하려고 하는 경우

## POST /platform/additional-fee-policies/{id}/recover

**추가 수수료 정책 복원** (operationId: `recoverPlatformAdditionalFeePolicy`)

주어진 아이디에 대응되는 추가 수수료 정책을 복원합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 복원된 추가 수수료 정책 객체를 반환합니다.: [RecoverPlatformAdditionalFeePolicyResponse](#recoverplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)

## GET /platform/additional-fee-policies/{id}/schedule

**** (operationId: `getPlatformAdditionalFeePolicySchedule`)

주어진 아이디에 대응되는 추가 수수료 정책의 예약 업데이트를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 예약된 추가 수수료 정책을 반환합니다.: [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)

## PUT /platform/additional-fee-policies/{id}/schedule

**** (operationId: `rescheduleAdditionalFeePolicy`)

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [ReschedulePlatformAdditionalFeePolicyBody](#rescheduleplatformadditionalfeepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformAdditionalFeePolicyBody](#updateplatformadditionalfeepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 추가 수수료 정책이 반환됩니다.: [ReschedulePlatformAdditionalFeePolicyResponse](#rescheduleplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)

## POST /platform/additional-fee-policies/{id}/schedule

**** (operationId: `scheduleAdditionalFeePolicy`)

주어진 아이디에 대응되는 추가 수수료 정책에 업데이트를 예약합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [SchedulePlatformAdditionalFeePolicyBody](#scheduleplatformadditionalfeepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformAdditionalFeePolicyBody](#updateplatformadditionalfeepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 추가 수수료 정책을 반환합니다.: [SchedulePlatformAdditionalFeePolicyResponse](#scheduleplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformAdditionalFeePolicyScheduleAlreadyExistsError`](errors.md#platformadditionalfeepolicyschedulealreadyexistserror)
  * [`PlatformArchivedAdditionalFeePolicyError`](errors.md#platformarchivedadditionalfeepolicyerror): 보관된 추가 수수료 정책을 업데이트하려고 하는 경우

## DELETE /platform/additional-fee-policies/{id}/schedule

**** (operationId: `cancelPlatformAdditionalFeePolicySchedule`)

주어진 아이디에 대응되는 추가 수수료 정책의 예약 업데이트를 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답: [CancelPlatformAdditionalFeePolicyScheduleResponse](#cancelplatformadditionalfeepolicyscheduleresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)

## GET /platform/additional-fee-policies/{id}

**추가 수수료 정책 조회** (operationId: `getPlatformAdditionalFeePolicy`)

주어진 아이디에 대응되는 추가 수수료 정책을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회할 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 추가 수수료 정책을 반환합니다.: [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)

## PATCH /platform/additional-fee-policies/{id}

**추가 수수료 정책 수정** (operationId: `updatePlatformAdditionalFeePolicy`)

주어진 아이디에 대응되는 추가 수수료 정책을 업데이트합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 업데이트할 추가 수수료 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdatePlatformAdditionalFeePolicyBody](#updateplatformadditionalfeepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fee` | [PlatformFeeInput](#platformfeeinput) |  | 책정 수수료 |
| `name` | string |  | 추가 수수료 정책 이름 |
| `memo` | string |  | 해당 추가 수수료 정책에 대한 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) |  | 부가세를 부담할 주체 |

### 응답

- **200** 성공 응답으로 업데이트된 추가 수수료 정책이 반환됩니다.: [UpdatePlatformAdditionalFeePolicyResponse](#updateplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformArchivedAdditionalFeePolicyError`](errors.md#platformarchivedadditionalfeepolicyerror): 보관된 추가 수수료 정책을 업데이트하려고 하는 경우

## GET /platform/additional-fee-policies

**추가 수수료 정책 다건 조회** (operationId: `getPlatformAdditionalFeePolicies`)

여러 추가 수수료 정책을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformAdditionalFeePoliciesBody](#getplatformadditionalfeepoliciesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformAdditionalFeePolicyFilterInput](#platformadditionalfeepolicyfilterinput) |  | 조회할 추가 수수료 정책 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 추가 수수료 정책 리스트와 페이지 정보를 반환합니다.: [GetPlatformAdditionalFeePoliciesResponse](#getplatformadditionalfeepoliciesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/additional-fee-policies

**추가 수수료 정책 생성** (operationId: `createPlatformAdditionalFeePolicy`)

새로운 추가 수수료 정책을 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformAdditionalFeePolicyBody](#createplatformadditionalfeepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 생성할 추가 수수료 정책 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `name` | string | Y | 이름 |
| `fee` | [PlatformFeeInput](#platformfeeinput) | Y | 수수료 정보 |
| `memo` | string |  | 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) | Y | 부가세 부담 주체 |

### 응답

- **200** 성공 응답: [CreatePlatformAdditionalFeePolicyResponse](#createplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformAdditionalFeePolicyAlreadyExistsError`](errors.md#platformadditionalfeepolicyalreadyexistserror)

## GET /platform/bank-accounts/{bank}/{accountNumber}/holder

**예금주 조회** (operationId: `getPlatformAccountHolder`)

계좌의 예금주를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `bank` | path | [Bank](schemas-common.md#bank) | Y | 은행 |
| `accountNumber` | path | string | Y | '-'를 제외한 계좌 번호 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `birthdate` | query | string |  | 실명 조회를 위해 추가로 보낼 수 있습니다. birthdate과 businessRegistrationNumber 중 하나만 사용해야 합니다. |
| `businessRegistrationNumber` | query | string |  | 실명 조회를 위해 추가로 보낼 수 있습니다. birthdate과 businessRegistrationNumber 중 하나만 사용해야 합니다. |

### 응답

- **200** 성공 응답으로 조회된 예금주 명이 반환됩니다.: [PlatformAccountHolder](#platformaccountholder)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNotSupportedBankError`](errors.md#platformnotsupportedbankerror): 지원하지 않는 은행인 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **503**
  * [`PlatformExternalApiTemporarilyFailedError`](errors.md#platformexternalapitemporarilyfailederror): 외부 api의 일시적인 오류
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

## GET /platform/bulk-account-transfers

**일괄 이체 내역 다건 조회** (operationId: `getPlatformBulkAccountTransfers`)

성공 응답으로 조회된 일괄 이체 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformBulkAccountTransfersBody](#getplatformbulkaccounttransfersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkAccountTransferFilterInput](#platformbulkaccounttransferfilterinput) |  |  |

### 응답

- **200** : [GetPlatformBulkAccountTransfersResponse](#getplatformbulkaccounttransfersresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/bulk-payouts

**일괄 지급 내역 다건 조회** (operationId: `getPlatformBulkPayouts`)

성공 응답으로 조회된 일괄 지급 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformBulkPayoutsBody](#getplatformbulkpayoutsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkPayoutFilterInput](#platformbulkpayoutfilterinput) |  |  |

### 응답

- **200** : [GetPlatformBulkPayoutsResponse](#getplatformbulkpayoutsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformCompanyNotFoundError`](errors.md#platformcompanynotfounderror): 사업자 정보를 찾을 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

## POST /platform/contracts/{id}/archive

**계약 보관** (operationId: `archivePlatformContract`)

주어진 아이디에 대응되는 계약을 보관합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 보관된 계약 객체를 반환합니다.: [ArchivePlatformContractResponse](#archiveplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledContractError`](errors.md#platformcannotarchivescheduledcontracterror): 예약된 업데이트가 있는 계약을 보관하려고 하는 경우

## POST /platform/contracts/{id}/recover

**계약 복원** (operationId: `recoverPlatformContract`)

주어진 아이디에 대응되는 계약을 복원합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 복원된 계약 객체를 반환합니다.: [RecoverPlatformContractResponse](#recoverplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)

## GET /platform/contracts/{id}/schedule

**** (operationId: `getPlatformContractSchedule`)

주어진 아이디에 대응되는 계약의 예약 업데이트를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 예약된 계약 객체를 반환합니다.: [PlatformContract](#platformcontract)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)

## PUT /platform/contracts/{id}/schedule

**** (operationId: `rescheduleContract`)

주어진 아이디에 대응되는 계약에 예약 업데이트를 재설정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [ReschedulePlatformContractBody](#rescheduleplatformcontractbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformContractBody](#updateplatformcontractbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 계약 객체를 반환합니다.: [ReschedulePlatformContractResponse](#rescheduleplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)

## POST /platform/contracts/{id}/schedule

**** (operationId: `scheduleContract`)

주어진 아이디에 대응되는 계약에 업데이트를 예약합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [SchedulePlatformContractBody](#scheduleplatformcontractbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformContractBody](#updateplatformcontractbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 계약 객체를 반환합니다.: [SchedulePlatformContractResponse](#scheduleplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformContractScheduleAlreadyExistsError`](errors.md#platformcontractschedulealreadyexistserror)
  * [`PlatformArchivedContractError`](errors.md#platformarchivedcontracterror): 보관된 계약을 업데이트하려고 하는 경우

## DELETE /platform/contracts/{id}/schedule

**** (operationId: `cancelPlatformContractSchedule`)

주어진 아이디에 대응되는 계약의 예약 업데이트를 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답: [CancelPlatformContractScheduleResponse](#cancelplatformcontractscheduleresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)

## GET /platform/contracts/{id}

**계약 조회** (operationId: `getPlatformContract`)

주어진 아이디에 대응되는 계약을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회할 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 계약 객체를 반환합니다.: [PlatformContract](#platformcontract)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)

## PATCH /platform/contracts/{id}

**계약 수정** (operationId: `updatePlatformContract`)

주어진 아이디에 대응되는 계약을 업데이트합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 업데이트할 계약 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdatePlatformContractBody](#updateplatformcontractbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) |  | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) |  | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) |  | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean |  | 정산 시 결제금액 부가세 감액 여부 |

### 응답

- **200** 성공 응답으로 업데이트된 계약 객체가 반환됩니다.: [UpdatePlatformContractResponse](#updateplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformArchivedContractError`](errors.md#platformarchivedcontracterror): 보관된 계약을 업데이트하려고 하는 경우

## GET /platform/contracts

**계약 다건 조회** (operationId: `getPlatformContracts`)

여러 계약을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformContractsBody](#getplatformcontractsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformContractFilterInput](#platformcontractfilterinput) |  | 조회할 계약 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 계약 리스트와 페이지 정보를 반환합니다.: [GetPlatformContractsResponse](#getplatformcontractsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/contracts

**계약 생성** (operationId: `createPlatformContract`)

새로운 계약을 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformContractBody](#createplatformcontractbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 계약에 부여할 고유 아이디 — 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) | Y | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) | Y | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 |

### 응답

- **200** 성공 응답으로 생성된 계약 객체가 반환됩니다.: [CreatePlatformContractResponse](#createplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformContractAlreadyExistsError`](errors.md#platformcontractalreadyexistserror)

## POST /platform/discount-share-policies/{id}/archive

**할인 분담 정책 보관** (operationId: `archivePlatformDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담을 보관합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 보관된 할인 분담 객체를 반환합니다.: [ArchivePlatformDiscountSharePolicyResponse](#archiveplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledDiscountSharePolicyError`](errors.md#platformcannotarchivescheduleddiscountsharepolicyerror): 예약된 업데이트가 있는 할인 분담 정책을 보관하려고 하는 경우

## POST /platform/discount-share-policies/{id}/recover

**할인 분담 정책 복원** (operationId: `recoverPlatformDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담을 복원합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 복원된 할인 분담 객체를 반환합니다.: [RecoverPlatformDiscountSharePolicyResponse](#recoverplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)

## GET /platform/discount-share-policies/{id}/schedule

**** (operationId: `getPlatformDiscountSharePolicySchedule`)

주어진 아이디에 대응되는 할인 분담의 예약 업데이트를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 예약된 할인 분담 정책을 반환합니다.: [PlatformDiscountSharePolicy](#platformdiscountsharepolicy)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)

## PUT /platform/discount-share-policies/{id}/schedule

**** (operationId: `rescheduleDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담에 예약 업데이트를 재설정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [ReschedulePlatformDiscountSharePolicyBody](#rescheduleplatformdiscountsharepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformDiscountSharePolicyBody](#updateplatformdiscountsharepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 할인 분담 정책을 반환합니다.: [ReschedulePlatformDiscountSharePolicyResponse](#rescheduleplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)

## POST /platform/discount-share-policies/{id}/schedule

**** (operationId: `scheduleDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담에 업데이트를 예약합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [SchedulePlatformDiscountSharePolicyBody](#scheduleplatformdiscountsharepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformDiscountSharePolicyBody](#updateplatformdiscountsharepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### 응답

- **200** 성공 응답으로 예약된 할인 분담 정책이 반환됩니다.: [SchedulePlatformDiscountSharePolicyResponse](#scheduleplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformDiscountSharePolicyScheduleAlreadyExistsError`](errors.md#platformdiscountsharepolicyschedulealreadyexistserror)
  * [`PlatformArchivedDiscountSharePolicyError`](errors.md#platformarchiveddiscountsharepolicyerror): 보관된 할인 분담 정책을 업데이트하려고 하는 경우

## DELETE /platform/discount-share-policies/{id}/schedule

**** (operationId: `cancelPlatformDiscountSharePolicySchedule`)

주어진 아이디에 대응되는 할인 분담의 예약 업데이트를 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답: [CancelPlatformDiscountSharePolicyScheduleResponse](#cancelplatformdiscountsharepolicyscheduleresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)

## GET /platform/discount-share-policies/{id}

**할인 분담 정책 조회** (operationId: `getPlatformDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회할 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 할인 분담 정책을 반환합니다.: [PlatformDiscountSharePolicy](#platformdiscountsharepolicy)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)

## PATCH /platform/discount-share-policies/{id}

**할인 분담 정책 수정** (operationId: `updatePlatformDiscountSharePolicy`)

주어진 아이디에 대응되는 할인 분담을 업데이트합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 업데이트할 할인 분담 정책 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdatePlatformDiscountSharePolicyBody](#updateplatformdiscountsharepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 할인 분담 정책 이름 |
| `partnerShareRate` | integer(int32) |  | 할인 분담율 — 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 |

### 응답

- **200** 성공 응답으로 업데이트된 할인 분담 정책을 반환합니다.: [UpdatePlatformDiscountSharePolicyResponse](#updateplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformArchivedDiscountSharePolicyError`](errors.md#platformarchiveddiscountsharepolicyerror): 보관된 할인 분담 정책을 업데이트하려고 하는 경우

## GET /platform/discount-share-policies

**할인 분담 정책 다건 조회** (operationId: `getPlatformDiscountSharePolicies`)

여러 할인 분담을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformDiscountSharePoliciesBody](#getplatformdiscountsharepoliciesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformDiscountSharePolicyFilterInput](#platformdiscountsharepolicyfilterinput) |  | 조회할 할인 분담 정책 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 할인 분담 정책 리스트와 페이지 정보가 반환됩니다.: [GetPlatformDiscountSharePoliciesResponse](#getplatformdiscountsharepoliciesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/discount-share-policies

**할인 분담 정책 생성** (operationId: `createPlatformDiscountSharePolicy`)

새로운 할인 분담을 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformDiscountSharePolicyBody](#createplatformdiscountsharepolicybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 할인 분담에 부여할 고유 아이디 — 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 할인 분담에 부여할 이름 |
| `partnerShareRate` | integer(int32) | Y | 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 ex) 파트너 브랜드 쿠폰 |

### 응답

- **200** 성공 응답으로 생성된 할인 분담 정책이 반환됩니다.: [CreatePlatformDiscountSharePolicyResponse](#createplatformdiscountsharepolicyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformDiscountSharePolicyAlreadyExistsError`](errors.md#platformdiscountsharepolicyalreadyexistserror)

## GET /platform/discount-share-policy-filter-options

**** (operationId: `getPlatformDiscountSharePolicyFilterOptions`)

할인 분담 정책 다건 조회 시 필요한 필터 옵션을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `isArchived` | query | boolean |  | true 이면 보관된 할인 분담의 필터 옵션을 조회하고, false 이면 보관되지 않은 할인 분담의 필터 옵션을 조회합니다. 기본값은 false 입니다. |

### 응답

- **200** 성공 응답으로 조회된 할인 분담 정책 필터 옵션을 반환합니다.: [PlatformDiscountSharePolicyFilterOptions](#platformdiscountsharepolicyfilteroptions)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/partner-settlements/complete-payout

**일괄 지급 완료 처리** (operationId: `completePlatformPayoutByPartnerSettlementIds`)

선택한 정산내역 아이디들로 일괄 지급을 완료 처리 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CompletePlatformPayoutByPartnerSettlementIdsBody](#completeplatformpayoutbypartnersettlementidsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string | Y |  |
| `name` | string |  |  |
| `partnerSettlementIds` | string[] | Y |  |
| `completedAt` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [CompletePlatformPayoutByPartnerSettlementIdsResponse](#completeplatformpayoutbypartnersettlementidsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNoSelectedPartnerSettlementsError`](errors.md#platformnoselectedpartnersettlementserror): 선택된 정산건이 없는 경우
  * [`PlatformDuplicatedPartnerSettlementIdsError`](errors.md#platformduplicatedpartnersettlementidserror): 선택된 정산건 아이디에 중복이 있는 경우
  * [`PlatformNonPayablePartnerSettlementsError`](errors.md#platformnonpayablepartnersettlementserror): 지급할 수 없는 정산건이 포함된 경우
  * [`PlatformNegativePayoutAmountPartnersError`](errors.md#platformnegativepayoutamountpartnerserror): 지급 금액의 총합이 음수인 파트너가 존재하는 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerSettlementsNotFoundError`](errors.md#platformpartnersettlementsnotfounderror): 요청한 정산건 목록을 찾을 수 없는 경우
- **409**
  * [`PlatformBulkPayoutIdAlreadyExistsError`](errors.md#platformbulkpayoutidalreadyexistserror): 일괄 지급 아이디가 이미 존재하는 경우

## POST /platform/partner-settlements/delete

**정산내역 삭제** (operationId: `deletePlatformPartnerSettlements`)

선택한 정산내역들을 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [DeletePlatformPartnerSettlementsBody](#deleteplatformpartnersettlementsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementIds` | string[] | Y |  |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [DeletePlatformPartnerSettlementsResponse](#deleteplatformpartnersettlementsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNonDeletablePartnerSettlementsError`](errors.md#platformnondeletablepartnersettlementserror): 삭제할 수 없는 정산건이 포함된 경우
  * [`PlatformReferencedCancelOrderTransfersExistError`](errors.md#platformreferencedcancelordertransfersexisterror): 취소 정산건이 참조 중인 정산건이 포함된 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerSettlementsNotFoundError`](errors.md#platformpartnersettlementsnotfounderror): 요청한 정산건 목록을 찾을 수 없는 경우

## GET /platform/partner-settlements

**정산 내역 다건 조회** (operationId: `getPlatformPartnerSettlements`)

여러 정산 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformPartnerSettlementsBody](#getplatformpartnersettlementsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerSettlementFilterInput](#platformpartnersettlementfilterinput) | Y | 조회할 정산내역 조건 필터 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 조회된 정산 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.: [GetPlatformPartnerSettlementsResponse](#getplatformpartnersettlementsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerIdsDuplicatedError`](errors.md#platformpartneridsduplicatederror)
  * [`PlatformCurrencyNotSupportedError`](errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractsNotFoundError`](errors.md#platformcontractsnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerIdsAlreadyExistError`](errors.md#platformpartneridsalreadyexisterror)

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerTypeIsNotBusinessError`](errors.md#platformpartnertypeisnotbusinesserror): 파트너 유형이 사업자가 아닌 경우
  * [`PlatformPartnerTaxationTypeIsSimpleError`](errors.md#platformpartnertaxationtypeissimpleerror): 파트너의 과세 유형이 간이 과세 세금계산서 미발행 유형인 경우
  * [`PlatformCounterpartyNotConnectableStatusError`](errors.md#platformcounterpartynotconnectablestatuserror): 파트너 거래처 연동 상태가 연동 가능한 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformPartnerScheduleExistsError`](errors.md#platformpartnerscheduleexistserror): 파트너 수정 예약 건이 존재하는 경우
  * [`PlatformArchivedPartnerError`](errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformArchivedPartnerNtsNotAllowedError`](errors.md#platformarchivedpartnerntsnotallowederror): 보관된 파트너는 국세청 연동/연동해제를 할 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformTargetPartnerNotFoundError`](errors.md#platformtargetpartnernotfounderror): 처리 대상 파트너가 존재하지 않는 경우
- **503**
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformPartnerTypeIsNotBusinessError`](errors.md#platformpartnertypeisnotbusinesserror): 파트너 유형이 사업자가 아닌 경우
  * [`PlatformPartnerTaxationTypeIsSimpleError`](errors.md#platformpartnertaxationtypeissimpleerror): 파트너의 과세 유형이 간이 과세 세금계산서 미발행 유형인 경우
  * [`PlatformCounterpartyNotConnectedError`](errors.md#platformcounterpartynotconnectederror): 파트너가 거래처로 연동 되어있지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformOngoingTaxInvoiceExistsError`](errors.md#platformongoingtaxinvoiceexistserror): 진행 중인 세금계산서가 존재하는 경우
  * [`PlatformArchivedPartnerNtsNotAllowedError`](errors.md#platformarchivedpartnerntsnotallowederror): 보관된 파트너는 국세청 연동/연동해제를 할 수 없는 경우
- **503**
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * [`PlatformBtxNotEnabledError`](errors.md#platformbtxnotenablederror): BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformTargetPartnerNotFoundError`](errors.md#platformtargetpartnernotfounderror): 처리 대상 파트너가 존재하지 않는 경우
- **503**
  * [`PlatformExternalApiFailedError`](errors.md#platformexternalapifailederror): 외부 api 오류

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerSchedulesAlreadyExistError`](errors.md#platformpartnerschedulesalreadyexisterror)
  * [`PlatformArchivedPartnersCannotBeScheduledError`](errors.md#platformarchivedpartnerscannotbeschedulederror): 보관된 파트너들을 예약 업데이트하려고 하는 경우
  * [`PlatformMemberCompanyConnectedPartnersCannotBeScheduledError`](errors.md#platformmembercompanyconnectedpartnerscannotbeschedulederror): 연동 사업자로 연동된 파트너들을 예약 수정하려고 시도한 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledPartnerError`](errors.md#platformcannotarchivescheduledpartnererror): 예약된 업데이트가 있는 파트너를 보관하려고 하는 경우
  * [`PlatformCounterpartyOngoingTaxInvoiceExistsError`](errors.md#platformcounterpartyongoingtaxinvoiceexistserror): 연동된 거래처에 진행 중인 세금계산서가 있는 경우
  * [`PlatformPartnerPendingNtsOperationError`](errors.md#platformpartnerpendingntsoperationerror): 파트너의 국세청 연동/해제가 진행 중인 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
- **409**
  * [`PlatformPartnerPendingNtsOperationError`](errors.md#platformpartnerpendingntsoperationerror): 파트너의 국세청 연동/해제가 진행 중인 경우

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

- **200** 성공 응답으로 예약된 파트너 객체를 반환합니다.: [PlatformPartner](#platformpartner)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`](errors.md#platformmembercompanyconnectedpartnercannotbeschedulederror): 연동 사업자로 연동된 파트너를 예약 수정하려고 시도한 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformInsufficientDataToChangePartnerTypeError`](errors.md#platforminsufficientdatatochangepartnertypeerror): 파트너 타입 수정에 필요한 데이터가 부족한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerScheduleAlreadyExistsError`](errors.md#platformpartnerschedulealreadyexistserror)
  * [`PlatformArchivedPartnerError`](errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformAccountVerificationAlreadyUsedError`](errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`](errors.md#platformmembercompanyconnectedpartnerbrnunchangeableerror): 연동 사업자로 연동된 파트너의 사업자등록번호를 변경하려고 시도한 경우
  * [`PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`](errors.md#platformmembercompanyconnectedpartnertypeunchangeableerror): 연동 사업자로 연동된 파트너의 파트너 유형을 변경하려고 시도한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`](errors.md#platformmembercompanyconnectedpartnercannotbeschedulederror): 연동 사업자로 연동된 파트너를 예약 수정하려고 시도한 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)

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

- **200** 성공 응답으로 파트너 객체가 반환됩니다.: [PlatformPartner](#platformpartner)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)

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
| `userDefinedProperties` | [PlatformProperties](#platformproperties) |  | 사용자 정의 속성 |

### 응답

- **200** 성공 응답으로 업데이트된 파트너 객체가 반환됩니다.: [UpdatePlatformPartnerResponse](#updateplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformInsufficientDataToChangePartnerTypeError`](errors.md#platforminsufficientdatatochangepartnertypeerror): 파트너 타입 수정에 필요한 데이터가 부족한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformArchivedPartnerError`](errors.md#platformarchivedpartnererror): 보관된 파트너를 업데이트하려고 하는 경우
  * [`PlatformAccountVerificationAlreadyUsedError`](errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`](errors.md#platformmembercompanyconnectedpartnerbrnunchangeableerror): 연동 사업자로 연동된 파트너의 사업자등록번호를 변경하려고 시도한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우
  * [`PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`](errors.md#platformmembercompanyconnectedpartnertypeunchangeableerror): 연동 사업자로 연동된 파트너의 파트너 유형을 변경하려고 시도한 경우

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
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 조회할 파트너 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 파트너 리스트와 페이지 정보가 반환됩니다.: [GetPlatformPartnersResponse](#getplatformpartnersresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
| `userDefinedProperties` | [PlatformProperties](#platformproperties) |  | 사용자 정의 속성 |

### 응답

- **200** 성공 응답으로 생성된 파트너 객체가 반환됩니다.: [CreatePlatformPartnerResponse](#createplatformpartnerresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformAccountVerificationFailedError`](errors.md#platformaccountverificationfailederror): 파트너 계좌 인증이 실패한 경우
  * [`PlatformCurrencyNotSupportedError`](errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
  * [`PlatformAccountVerificationNotFoundError`](errors.md#platformaccountverificationnotfounderror): 파트너 계좌 검증 아이디를 찾을 수 없는 경우
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformPartnerIdAlreadyExistsError`](errors.md#platformpartneridalreadyexistserror)
  * [`PlatformAccountVerificationAlreadyUsedError`](errors.md#platformaccountverificationalreadyusederror): 파트너 계좌 검증 아이디를 이미 사용한 경우
  * [`PlatformCompanyVerificationAlreadyUsedError`](errors.md#platformcompanyverificationalreadyusederror): 파트너 사업자 검증 아이디를 이미 사용한 경우

## GET /platform/payouts

**지급 내역 다건 조회** (operationId: `getPlatformPayouts`)

여러 지급 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformPayoutsBody](#getplatformpayoutsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformPayoutFilterInput](#platformpayoutfilterinput) |  |  |

### 응답

- **200** 성공 응답으로 조회된 지급 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.: [GetPlatformPayoutsResponse](#getplatformpayoutsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

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
| `settlementAmountType` | [SettlementAmountType](#settlementamounttype) |  | 정산 금액 취급 기준 |
| `manualSettlementPerTransfer` | boolean |  | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### 응답

- **200** 성공 응답으로 플랫폼 설정 정보를 반환합니다.: [UpdatePlatformSettingResponse](#updateplatformsettingresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/transfer-summaries/sheet-file

**정산 상세 내역 다운로드** (operationId: `downloadPlatformTransferSheet`)

정산 상세 내역을 csv 파일로 다운로드 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [DownloadPlatformTransferSheetBody](#downloadplatformtransfersheetbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 컬럼 키 목록 — - TRANSFER_MEMO:  메모 <br>- TRANSFER_TYPE: 정산 유형<br>- TRANSFER_STATUS:  상태 <br>- TRANSFER_ID: 정산 아이디<br>- TRANSFER_SETTLEMENT_DATE:  정산일 <br>- TRANSFER_SETTLEMENT_AMOUNT: 정산 금액<br>- TRANSFER_SETTLEMENT_TAX_FREE_AMOUNT: 정산 면세액<br>- TRANSFER_SETTLEMENT_CURRENCY: 정산 통화<br>- TRANSFER_SETTLEMENT_START_DATE: 정산 시작일<br>- TRANSFER_ORDER_NAME:  주문명 <br>- TRANSFER_ORDER_AMOUNT: 주문 금액<br>- TRANSFER_ORDER_TAX_FREE_AMOUNT: 주문 면세액<br>- TRANSFER_PAYMENT_ID: 주문 번호<br>- TRANSFER_PAYMENT_METHOD: 결제 수단<br>- TRANSFER_PAYMENT_AMOUNT: 결제 금액<br>- TRANSFER_PAYMENT_SUPPLY_AMOUNT: 결제 공급가액<br>- TRANSFER_PAYMENT_VAT_AMOUNT: 결제 부가세액<br>- TRANSFER_PAYMENT_TAX_FREE_AMOUNT: 결제 면세액<br>- TRANSFER_PAYMENT_VAT_BURDEN_AMOUNT: 결제 부가세 부담금<br>- TRANSFER_PLATFORM_FEE:  중개수수료 <br>- TRANSFER_PLATFORM_FEE_VAT: 중개수수료 부가세 부담금<br>- TRANSFER_CONTRACT_ID: 계약 아이디<br>- TRANSFER_CONTRACT_NAME: 계약 이름<br>- TRANSFER_DISCOUNT_AMOUNT: 할인 금액<br>- TRANSFER_DISCOUNT_TAX_FREE_AMOUNT: 할인 면세액<br>- TRANSFER_DISCOUNT_SHARE_AMOUNT: 할인 분담금<br>- TRANSFER_DISCOUNT_SHARE_TAX_FREE_AMOUNT: 할인 면세 분담금<br>- TRANSFER_ADDITIONAL_FEE:  추가수수료 <br>- TRANSFER_ADDITIONAL_FEE_VAT: 추가수수료 부가세 부담금<br>- TRANSFER_{UserDefinedProperty.Key}<br>- FORMULA_{UserDefinedFormula.Key}<br>- PARTNER_* : 파트너 컬럼 키 사용 가능(w/o PARTNER_STATUS_UPDATED_AT) |
| `fields` | string[] |  |  |

### 응답

- **200** 
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

## GET /platform/transfer-summaries

**정산건 다건 조회** (operationId: `getPlatformTransferSummaries`)

성공 응답으로 조회된 정산건 요약 리스트와 페이지 정보가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformTransferSummariesBody](#getplatformtransfersummariesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 조회할 정산건 조건 필터 |

### 응답

- **200** : [GetPlatformTransferSummariesResponse](#getplatformtransfersummariesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/transfers/manual

**수기 정산건 생성** (operationId: `createPlatformManualTransfer`)

성공 응답으로 생성된 수기 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformManualTransferBody](#createplatformmanualtransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `memo` | string |  | 메모 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) |  | 정산 면세 금액 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) |  | 정산 통화 |

### 응답

- **200** : [CreateManualTransferResponse](#createmanualtransferresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformCurrencyNotSupportedError`](errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
  * [`PlatformNegativeAmountNotAllowedError`](errors.md#platformnegativeamountnotallowederror): 정산 건별 옵션이 켜진 플랫폼에서 음수 금액 수기 정산 생성을 시도한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformTransferIdAlreadyUsedError`](errors.md#platformtransferidalreadyusederror)

## POST /platform/transfers/order-cancel

**주문 취소 정산건 생성** (operationId: `createPlatformOrderCancelTransfer`)

성공 응답으로 생성된 주문 취소 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformOrderCancelTransferBody](#createplatformordercanceltransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string |  | 파트너 아이디 |
| `paymentId` | string |  | 결제 아이디 |
| `transferId` | string |  | 정산건 아이디 |
| `cancellationId` | string | Y | 취소 내역 아이디 |
| `memo` | string |  | 메모 |
| `orderDetail` | [CreatePlatformOrderCancelTransferBodyOrderDetail](#createplatformordercanceltransferbodyorderdetail) |  | 주문 취소 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 취소 면세 금액 — 주문 취소 항목과 취소 면세 금액을 같이 전달하시면 최종 취소 면세 금액은 주문 취소 항목의 면세 금액이 아닌 전달해주신 취소 면세 금액으로 적용됩니다. |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 할인 정보 |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `externalCancellationDetail` | [CreatePlatformOrderCancelTransferBodyExternalCancellationDetail](#createplatformordercanceltransferbodyexternalcancellationdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 취소 정산건으로 등록되고, 존재하지않은 경우 포트원 결제 취소 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 취소 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### 응답

- **200** : [CreateOrderCancelTransferResponse](#createordercanceltransferresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformOrderDetailMismatchedError`](errors.md#platformorderdetailmismatchederror)
  * [`PlatformDiscountSharePolicyIdDuplicatedError`](errors.md#platformdiscountsharepolicyidduplicatederror)
  * [`PlatformCancellableAmountExceededError`](errors.md#platformcancellableamountexceedederror): 취소 가능한 금액이 초과한 경우
  * [`PlatformCancellableDiscountAmountExceededError`](errors.md#platformcancellablediscountamountexceedederror)
  * [`PlatformCancellableDiscountTaxFreeAmountExceededError`](errors.md#platformcancellablediscounttaxfreeamountexceedederror)
  * [`PlatformProductIdDuplicatedError`](errors.md#platformproductidduplicatederror)
  * [`PlatformCancellableProductQuantityExceededError`](errors.md#platformcancellableproductquantityexceedederror)
  * [`PlatformOrderTransferAlreadyCancelledError`](errors.md#platformordertransferalreadycancellederror)
  * [`PlatformSettlementAmountExceededError`](errors.md#platformsettlementamountexceedederror): 정산 가능한 금액을 초과한 경우
  * [`PlatformCancellationAndPaymentTypeMismatchedError`](errors.md#platformcancellationandpaymenttypemismatchederror)
  * [`PlatformSettlementCancelAmountExceededPortOneCancelError`](errors.md#platformsettlementcancelamountexceededportonecancelerror): 정산 취소 요청 금액이 포트원 결제 취소 내역의 취소 금액을 초과한 경우
  * [`PlatformCannotSpecifyTransferError`](errors.md#platformcannotspecifytransfererror): 정산 건 식별에 실패한 경우
  * [`PlatformSettlementDateEarlierThanSettlementStartDateError`](errors.md#platformsettlementdateearlierthansettlementstartdateerror): 정산일이 정산 시작일보다 빠른 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](errors.md#platformtransfernotfounderror)
  * [`PlatformCancellationNotFoundError`](errors.md#platformcancellationnotfounderror)
  * [`PlatformPaymentNotFoundError`](errors.md#platformpaymentnotfounderror)
  * [`PlatformProductIdNotFoundError`](errors.md#platformproductidnotfounderror)
  * [`PlatformTransferDiscountSharePolicyNotFoundError`](errors.md#platformtransferdiscountsharepolicynotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformTransferAlreadyExistsError`](errors.md#platformtransferalreadyexistserror)
  * [`PlatformTransferIdAlreadyUsedError`](errors.md#platformtransferidalreadyusederror)

## POST /platform/transfers/order

**주문 정산건 생성** (operationId: `createPlatformOrderTransfer`)

성공 응답으로 생성된 주문 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformOrderTransferBody](#createplatformordertransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `contractId` | string |  | 계약 아이디 — 기본값은 파트너의 기본 계약 아이디 입니다. |
| `memo` | string |  | 메모 |
| `paymentId` | string | Y | 결제 아이디 |
| `orderDetail` | [CreatePlatformOrderTransferBodyOrderDetail](#createplatformordertransferbodyorderdetail) | Y | 주문 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 면세 금액 — 주문 항목과 면세 금액을 같이 전달하시면 최종 면세 금액은 주문 항목의 면세 금액이 아닌 전달해주신 면세 금액으로 적용됩니다. |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 기본값은 결제 일시 입니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 추가 수수료 정보 |
| `externalPaymentDetail` | [CreatePlatformOrderTransferBodyExternalPaymentDetail](#createplatformordertransferbodyexternalpaymentdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 정산건 으로 등록되고, 존재하지않은 경우 포트원 결제 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `parameters` | [TransferParameters](#transferparameters) |  | 정산 파라미터 (실험기능) |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### 응답

- **200** : [CreateOrderTransferResponse](#createordertransferresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformProductIdDuplicatedError`](errors.md#platformproductidduplicatederror)
  * [`PlatformSettlementPaymentAmountExceededPortOnePaymentError`](errors.md#platformsettlementpaymentamountexceededportonepaymenterror): 정산 요청 결제 금액이 포트원 결제 내역의 결제 금액을 초과한 경우
  * [`PlatformSettlementTaxFreeAmountExceededPortOnePaymentError`](errors.md#platformsettlementtaxfreeamountexceededportonepaymenterror): 정산 요청 면세 금액이 포트원 결제 내역의 면세 금액을 초과한 경우
  * [`PlatformSettlementSupplyWithVatAmountExceededPortOnePaymentError`](errors.md#platformsettlementsupplywithvatamountexceededportonepaymenterror): 정산 요청 공급대가가 포트원 결제 내역의 공급대가를 초과한 경우
  * [`PlatformSettlementAmountExceededError`](errors.md#platformsettlementamountexceedederror): 정산 가능한 금액을 초과한 경우
  * [`PlatformContractPlatformFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`](errors.md#platformcontractplatformfixedamountfeecurrencyandsettlementcurrencymismatchederror)
  * [`PlatformAdditionalFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`](errors.md#platformadditionalfixedamountfeecurrencyandsettlementcurrencymismatchederror)
  * [`PlatformCurrencyNotSupportedError`](errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
  * [`PlatformSettlementDateEarlierThanSettlementStartDateError`](errors.md#platformsettlementdateearlierthansettlementstartdateerror): 정산일이 정산 시작일보다 빠른 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](errors.md#platformcontractnotfounderror)
  * [`PlatformAdditionalFeePoliciesNotFoundError`](errors.md#platformadditionalfeepoliciesnotfounderror)
  * [`PlatformDiscountSharePoliciesNotFoundError`](errors.md#platformdiscountsharepoliciesnotfounderror)
  * [`PlatformPaymentNotFoundError`](errors.md#platformpaymentnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
  * [`PlatformSettlementParameterNotFoundError`](errors.md#platformsettlementparameternotfounderror): 정산 파라미터가 존재하지 않는 경우
- **409**
  * [`PlatformTransferAlreadyExistsError`](errors.md#platformtransferalreadyexistserror)
  * [`PlatformTransferIdAlreadyUsedError`](errors.md#platformtransferidalreadyusederror)

## GET /platform/transfers/{id}

**정산건 조회** (operationId: `getPlatformTransfer`)

정산건을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회하고 싶은 정산건 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 정산건 객체가 반환됩니다.: [PlatformTransfer](#platformtransfer)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](errors.md#platformtransfernotfounderror)

## DELETE /platform/transfers/{id}

**정산건 삭제** (operationId: `deletePlatformTransfer`)

scheduled, in_process 상태의 정산건만 삭제가능합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 정산건 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [DeletePlatformTransferResponse](#deleteplatformtransferresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformCancelOrderTransfersExistsError`](errors.md#platformcancelordertransfersexistserror)
  * [`PlatformTransferNonDeletableStatusError`](errors.md#platformtransfernondeletablestatuserror)
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](errors.md#platformtransfernotfounderror)

---

# 스키마

### ArchivePlatformAdditionalFeePolicyResponse

추가 수수료 정책 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 보관된 추가 수수료 정책 |

### ArchivePlatformContractResponse

계약 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](#platformcontract) | Y | 보관된 계약 |

### ArchivePlatformDiscountSharePolicyResponse

할인 분담 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 보관된 할인 분담 |

### ArchivePlatformPartnerResponse

파트너 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](#platformpartner) | Y | 보관된 파트너 |

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

### CancelPlatformAdditionalFeePolicyScheduleResponse

추가 수수료 정책 예약 업데이트 취소 성공 응답

type: object

### CancelPlatformContractScheduleResponse

계약 예약 업데이트 취소 성공 응답

type: object

### CancelPlatformDiscountSharePolicyScheduleResponse

할인 분담 정책 예약 업데이트 취소 성공 응답

type: object

### CancelPlatformPartnerScheduleResponse

파트너 예약 업데이트 취소 성공 응답

type: object

### CompletePlatformPayoutByPartnerSettlementIdsBody

CompletePlatformPayoutByPartnerSettlementIdsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string | Y |  |
| `name` | string |  |  |
| `partnerSettlementIds` | string[] | Y |  |
| `completedAt` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### CompletePlatformPayoutByPartnerSettlementIdsResponse

일괄 지급 완료 처리 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string |  |  |
| `bulkPayoutGraphqlId` | string |  |  |
| `payoutCount` | integer(int32) | Y |  |
| `partnerSettlementCount` | integer(int32) | Y |  |

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
| `partner` | [PlatformPartner](#platformpartner) | Y |  |

### CreateManualTransferResponse

CreateManualTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformManualTransfer](#platformmanualtransfer) | Y |  |

### CreateOrderCancelTransferResponse

CreateOrderCancelTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformOrderCancelTransfer](#platformordercanceltransfer) | Y |  |

### CreateOrderTransferResponse

CreateOrderTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformOrderTransfer](#platformordertransfer) | Y |  |

### CreatePlatformAdditionalFeePolicyBody

추가 수수료 정책 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 생성할 추가 수수료 정책 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `name` | string | Y | 이름 |
| `fee` | [PlatformFeeInput](#platformfeeinput) | Y | 수수료 정보 |
| `memo` | string |  | 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) | Y | 부가세 부담 주체 |

### CreatePlatformAdditionalFeePolicyResponse

플랫폼 생성 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 생성된 추가 수수료 정책 |

### CreatePlatformContractBody

계약 객체 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 계약에 부여할 고유 아이디 — 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) | Y | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) | Y | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 |

### CreatePlatformContractResponse

계약 객체 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](#platformcontract) | Y | 생성된 계약 객체 |

### CreatePlatformDiscountSharePolicyBody

할인 분담 정책 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 할인 분담에 부여할 고유 아이디 — 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 할인 분담에 부여할 이름 |
| `partnerShareRate` | integer(int32) | Y | 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 ex) 파트너 브랜드 쿠폰 |

### CreatePlatformDiscountSharePolicyResponse

할인 분담 정책 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 생성된 할인 분담 정책 |

### CreatePlatformManualTransferBody

수기 정산건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `memo` | string |  | 메모 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) |  | 정산 면세 금액 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) |  | 정산 통화 |

### CreatePlatformOrderCancelTransferBody

주문 취소 정산 등록을 위한 입력 정보 — 주문 취소 정산 등록을 위한 입력 정보<br>하나의 payment에 하나의 정산 건만 존재하는 경우에는 (partnerId, paymentId)로 취소 정산을 등록하실 수 있습니다.<br>하나의 payment에 여러 개의 정산 건이 존재하는 경우에는 transferId를 필수로 입력해야 합니다.<br>transferId를 입력한 경우 (partnerId, paymentId)는 생략 가능합니다.<br>id를 입력한 경우 생성될 취소 정산건 아이디로 사용됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string |  | 파트너 아이디 |
| `paymentId` | string |  | 결제 아이디 |
| `transferId` | string |  | 정산건 아이디 |
| `cancellationId` | string | Y | 취소 내역 아이디 |
| `memo` | string |  | 메모 |
| `orderDetail` | [CreatePlatformOrderCancelTransferBodyOrderDetail](#createplatformordercanceltransferbodyorderdetail) |  | 주문 취소 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 취소 면세 금액 — 주문 취소 항목과 취소 면세 금액을 같이 전달하시면 최종 취소 면세 금액은 주문 취소 항목의 면세 금액이 아닌 전달해주신 취소 면세 금액으로 적용됩니다. |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 할인 정보 |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `externalCancellationDetail` | [CreatePlatformOrderCancelTransferBodyExternalCancellationDetail](#createplatformordercanceltransferbodyexternalcancellationdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 취소 정산건으로 등록되고, 존재하지않은 경우 포트원 결제 취소 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 취소 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### CreatePlatformOrderCancelTransferBodyDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicyId` | string | Y | 할인 분담 정책 아이디 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세 할인 금액 |

### CreatePlatformOrderCancelTransferBodyExternalCancellationDetail

외부 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cancelledAt` | string(date-time) |  | 취소 일시 |

### CreatePlatformOrderCancelTransferBodyOrderDetail

주문 취소 정보 — 주문 취소 정보<br>orderAmount, orderLines, all 중에서 하나만 입력하여야 합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `orderAmount` | integer(int64) |  | 주문 취소 금액 |
| `orderLines` | [CreatePlatformOrderCancelTransferBodyOrderLine](#createplatformordercanceltransferbodyorderline)[] |  | 주문 취소 항목 리스트 |
| `all` | [CreatePlatformOrderCancelTransferBodyOrderDetailAll](#createplatformordercanceltransferbodyorderdetailall) |  | 전체 금액 취소 |

### CreatePlatformOrderCancelTransferBodyOrderDetailAll

전체 금액 취소

type: object

### CreatePlatformOrderCancelTransferBodyOrderLine

주문 취소 항목 리스트

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `productId` | string | Y | 상품 아이디 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 상품 할인 정보 |

### CreatePlatformOrderTransferBody

주문 정산건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `contractId` | string |  | 계약 아이디 — 기본값은 파트너의 기본 계약 아이디 입니다. |
| `memo` | string |  | 메모 |
| `paymentId` | string | Y | 결제 아이디 |
| `orderDetail` | [CreatePlatformOrderTransferBodyOrderDetail](#createplatformordertransferbodyorderdetail) | Y | 주문 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 면세 금액 — 주문 항목과 면세 금액을 같이 전달하시면 최종 면세 금액은 주문 항목의 면세 금액이 아닌 전달해주신 면세 금액으로 적용됩니다. |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 기본값은 결제 일시 입니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 추가 수수료 정보 |
| `externalPaymentDetail` | [CreatePlatformOrderTransferBodyExternalPaymentDetail](#createplatformordertransferbodyexternalpaymentdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 정산건 으로 등록되고, 존재하지않은 경우 포트원 결제 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `parameters` | [TransferParameters](#transferparameters) |  | 정산 파라미터 (실험기능) |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### CreatePlatformOrderTransferBodyAdditionalFee

추가 수수료 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `policyId` | string | Y | 추가 수수료 정책 아이디 |

### CreatePlatformOrderTransferBodyDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicyId` | string | Y | 할인 분담 정책 아이디 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세 할인 금액 |

### CreatePlatformOrderTransferBodyExternalPaymentDetail

외부 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `orderName` | string |  | 주문 명 |
| `paidAt` | string(date-time) |  | 결제 일시 |
| `method` | [PlatformPaymentMethodInput](#platformpaymentmethodinput) |  | 결제 수단 |

### CreatePlatformOrderTransferBodyOrderDetail

주문 정보 — 주문 정보<br>주문 금액 또는 주문 항목 하나만 입력 가능합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `orderAmount` | integer(int64) |  | 주문 금액 |
| `orderLines` | [CreatePlatformOrderTransferBodyOrderLine](#createplatformordertransferbodyorderline)[] |  | 주문 항목 리스트 |

### CreatePlatformOrderTransferBodyOrderLine

주문 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [CreatePlatformOrderTransferBodyProduct](#createplatformordertransferbodyproduct) | Y | 상품 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 상품 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 상품 추가 수수료 정보 |

### CreatePlatformOrderTransferBodyProduct

상품

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 아이디 |
| `name` | string | Y | 상품 이름 |
| `amount` | integer(int64) | Y | 상품 금액 |
| `taxFreeAmount` | integer(int64) |  | 상품 면세 금액 |
| `tag` | string |  | 태그 |

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
| `userDefinedProperties` | [PlatformProperties](#platformproperties) |  | 사용자 정의 속성 |

### CreatePlatformPartnerBodyAccount

파트너 계좌 등록을 위한 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 정산에 사용할 통화 |
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
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) |  | 사업자 유형 — 값을 입력하지 않으면 일반 과세로 설정됩니다. |
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
| `partner` | [PlatformPartner](#platformpartner) | Y | 생성된 파트너 |

### CreatePlatformPartnersBody

파트너 다건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partners` | [CreatePlatformPartnerBody](#createplatformpartnerbody)[] | Y | 생성할 파트너 리스트 정보 |

### CreatePlatformPartnersResponse

파트너 다건 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partners` | [PlatformPartner](#platformpartner)[] | Y | 생성된 파트너 리스트 |

### DayOfWeek

요일

- `SUN`
- `MON`
- `TUE`
- `WED`
- `THU`
- `FRI`
- `SAT`

### DeletePlatformPartnerSettlementsBody

DeletePlatformPartnerSettlementsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementIds` | string[] | Y |  |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### DeletePlatformPartnerSettlementsResponse

정산내역 삭제 결과

type: object

### DeletePlatformTransferResponse

DeletePlatformTransferResponse

type: object

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
| `partner` | [PlatformPartner](#platformpartner) | Y |  |

### DownloadPlatformTransferSheetBody

DownloadPlatformTransferSheetBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 컬럼 키 목록 — - TRANSFER_MEMO:  메모 <br>- TRANSFER_TYPE: 정산 유형<br>- TRANSFER_STATUS:  상태 <br>- TRANSFER_ID: 정산 아이디<br>- TRANSFER_SETTLEMENT_DATE:  정산일 <br>- TRANSFER_SETTLEMENT_AMOUNT: 정산 금액<br>- TRANSFER_SETTLEMENT_TAX_FREE_AMOUNT: 정산 면세액<br>- TRANSFER_SETTLEMENT_CURRENCY: 정산 통화<br>- TRANSFER_SETTLEMENT_START_DATE: 정산 시작일<br>- TRANSFER_ORDER_NAME:  주문명 <br>- TRANSFER_ORDER_AMOUNT: 주문 금액<br>- TRANSFER_ORDER_TAX_FREE_AMOUNT: 주문 면세액<br>- TRANSFER_PAYMENT_ID: 주문 번호<br>- TRANSFER_PAYMENT_METHOD: 결제 수단<br>- TRANSFER_PAYMENT_AMOUNT: 결제 금액<br>- TRANSFER_PAYMENT_SUPPLY_AMOUNT: 결제 공급가액<br>- TRANSFER_PAYMENT_VAT_AMOUNT: 결제 부가세액<br>- TRANSFER_PAYMENT_TAX_FREE_AMOUNT: 결제 면세액<br>- TRANSFER_PAYMENT_VAT_BURDEN_AMOUNT: 결제 부가세 부담금<br>- TRANSFER_PLATFORM_FEE:  중개수수료 <br>- TRANSFER_PLATFORM_FEE_VAT: 중개수수료 부가세 부담금<br>- TRANSFER_CONTRACT_ID: 계약 아이디<br>- TRANSFER_CONTRACT_NAME: 계약 이름<br>- TRANSFER_DISCOUNT_AMOUNT: 할인 금액<br>- TRANSFER_DISCOUNT_TAX_FREE_AMOUNT: 할인 면세액<br>- TRANSFER_DISCOUNT_SHARE_AMOUNT: 할인 분담금<br>- TRANSFER_DISCOUNT_SHARE_TAX_FREE_AMOUNT: 할인 면세 분담금<br>- TRANSFER_ADDITIONAL_FEE:  추가수수료 <br>- TRANSFER_ADDITIONAL_FEE_VAT: 추가수수료 부가세 부담금<br>- TRANSFER_{UserDefinedProperty.Key}<br>- FORMULA_{UserDefinedFormula.Key}<br>- PARTNER_* : 파트너 컬럼 키 사용 가능(w/o PARTNER_STATUS_UPDATED_AT) |
| `fields` | string[] |  |  |

### EasyPayMethodType

간편 결제 수단

- `CARD`
- `TRANSFER`
- `CHARGE`

### GetAccountTransfersBody1

GetAccountTransfersBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformAccountTransferFilter](#platformaccounttransferfilter) |  |  |

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

### GetPlatformAccountTransfersResponse

이체내역 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformAccountTransfer](#platformaccounttransfer)[] | Y | 조회된 이체내역 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |
| `counts` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y | 이체 내역 상태별 건 수 |

### GetPlatformAdditionalFeePoliciesBody

추가 수수료 정책 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformAdditionalFeePolicyFilterInput](#platformadditionalfeepolicyfilterinput) |  | 조회할 추가 수수료 정책 조건 필터 |

### GetPlatformAdditionalFeePoliciesResponse

추가 수수료 정책 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy)[] | Y | 조회된 추가 수수료 정책 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformBulkAccountTransfersBody

GetPlatformBulkAccountTransfersBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkAccountTransferFilterInput](#platformbulkaccounttransferfilterinput) |  |  |

### GetPlatformBulkAccountTransfersResponse

GetPlatformBulkAccountTransfersResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformBulkAccountTransfer](#platformbulkaccounttransfer)[] | Y |  |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformBulkAccountTransferStatusStats](#platformbulkaccounttransferstatusstats) | Y |  |

### GetPlatformBulkPayoutsBody

GetPlatformBulkPayoutsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkPayoutFilterInput](#platformbulkpayoutfilterinput) |  |  |

### GetPlatformBulkPayoutsResponse

GetPlatformBulkPayoutsResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformBulkPayout](#platformbulkpayout)[] | Y |  |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformBulkPayoutStatusStats](#platformbulkpayoutstatusstats) | Y |  |

### GetPlatformCompanyStatePayload

사업자 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyState` | [PlatformCompanyState](#platformcompanystate) | Y | 사업자 정보 |
| `companyVerificationId` | string | Y | 사업자 검증 아이디 |

### GetPlatformContractsBody

계약 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformContractFilterInput](#platformcontractfilterinput) |  | 조회할 계약 조건 필터 |

### GetPlatformContractsResponse

계약 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformContract](#platformcontract)[] | Y | 조회된 계약 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformDiscountSharePoliciesBody

할인 분담 정책 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformDiscountSharePolicyFilterInput](#platformdiscountsharepolicyfilterinput) |  | 조회할 할인 분담 정책 조건 필터 |

### GetPlatformDiscountSharePoliciesResponse

할인 분담 정책 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy)[] | Y | 조회된 할인 분담 정책 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformPartnerSettlementsBody

정산내역 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerSettlementFilterInput](#platformpartnersettlementfilterinput) | Y | 조회할 정산내역 조건 필터 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### GetPlatformPartnerSettlementsResponse

정산내역 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPartnerSettlement](#platformpartnersettlement)[] | Y | 조회된 정산내역 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |
| `counts` | [PlatformPartnerSettlementStatusStats](#platformpartnersettlementstatusstats) | Y | 정산내역 상태 별 갯수 |

### GetPlatformPartnersBody

파트너 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerFilterInput](#platformpartnerfilterinput) |  | 조회할 파트너 조건 필터 |

### GetPlatformPartnersResponse

파트너 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPartner](#platformpartner)[] | Y | 조회된 파트너 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformPayoutsBody

GetPlatformPayoutsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformPayoutFilterInput](#platformpayoutfilterinput) |  |  |

### GetPlatformPayoutsResponse

GetPlatformPayoutsResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPayout](#platformpayout)[] | Y |  |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |

### GetPlatformTransferSummariesBody

정산건 요약 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 조회할 정산건 조건 필터 |

### GetPlatformTransferSummariesResponse

GetPlatformTransferSummariesResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transferSummaries` | [PlatformTransferSummary](#platformtransfersummary)[] | Y |  |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y |  |

### MonthDay

월 및 일자 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `month` | integer(int32) | Y |  |
| `day` | integer(int32) | Y |  |

### PlatformAccount

플랫폼 정산 계좌 — 플랫폼 정산 계좌<br>`currency` 가 KRW 일 경우 예금주 조회 API 를 통해 올바른 계좌인지 검증합니다. 그 외의 화폐일 경우 따로 검증하지는 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 정산에 사용할 통화 |
| `number` | string | Y | 계좌번호 |
| `holder` | string | Y | 예금주명 |
| `status` | [PlatformAccountStatus](#platformaccountstatus) | Y | 계좌 상태 |

### PlatformAccountHolder

예금주 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `holderName` | string | Y | 계좌 예금주 이름 |
| `accountVerificationId` | string | Y | 계좌 검증 아이디 |

### PlatformAccountStatus

플랫폼 계좌 상태

- `VERIFIED` — 계좌 인증 완료됨
- `VERIFY_FAILED` — 계좌주 불일치
- `VERIFY_ERROR` — 계좌 인증 오류
- `NOT_VERIFIED` — 계좌 인증 안됨
- `UNKNOWN` — 알 수 없는 상태

### PlatformAccountTransfer

계좌 이체 — 계좌 이체<br>송금 대행을 통해 일어난 정산 금액 지급, 인출 목적의 계좌 이체 결과 정보입니다.

oneOf, discriminator `type`:

- `DEPOSIT` → [PlatformDepositAccountTransfer](#platformdepositaccounttransfer)
- `WITHDRAWAL` → [PlatformWithdrawalAccountTransfer](#platformwithdrawalaccounttransfer)

### PlatformAccountTransferFilter

PlatformAccountTransferFilter

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `tradeTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 이체 일시 범위 |
| `createdTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `scheduledTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 이체 예정 일시 범위 |
| `accountTransferId` | string |  | 이체 아이디 |
| `bankAccountId` | string |  | 계좌 아이디 |
| `bulkAccountTransferId` | string |  | 일괄 이체 아이디 |
| `payoutId` | string |  | 지급 아이디 |
| `accountTransferIds` | string[] |  | 이체 아이디 리스트 |
| `types` | [PlatformAccountTransferType](#platformaccounttransfertype)[] |  | 구분 |
| `statuses` | [PlatformAccountTransferStatus](#platformaccounttransferstatus)[] |  | 상태 |
| `depositorName` | string |  | 입금자명 |
| `depositAccountHolder` | string |  | 예금주 |
| `depositMemo` | string |  | 받는 이 통장 메모 |
| `withdrawalMemo` | string |  | 보내는 이 통장 메모 |

### PlatformAccountTransferStatus

계좌 이체 상태

- `PREPARED` — 대기
- `SCHEDULED` — 예약
- `CANCELLED` — 취소
- `STOPPED` — 중단
- `PROCESSING` — 처리 중
- `ASYNC_PROCESSING` — 비동기 처리 중
- `SUCCEEDED` — 성공
- `FAILED` — 실패

### PlatformAccountTransferStatusStats

PlatformAccountTransferStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `prepared` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |
| `stopped` | integer(int64) | Y |  |
| `processing` | integer(int64) | Y |  |
| `asyncProcessing` | integer(int64) | Y |  |
| `succeeded` | integer(int64) | Y |  |
| `failed` | integer(int64) | Y |  |

### PlatformAccountTransferType

계좌 이체 유형

- `DEPOSIT` — 입금
- `WITHDRAWAL` — 출금

### PlatformAdditionalFeePolicy

추가 수수료 정책 — 추가 수수료 정책<br>추가 수수료 정책는 고객사의 주문건에 대한 중개수수료에 별도로 추가로 부여되는 수수료입니다. 대표적인 사용 예시로 풀필먼트 수수료, 로켓배송 수수료, 마케팅 채널 수수료등이 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 추가 수수료 정책 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 추가 수수료 정책 이름 |
| `fee` | [PlatformFee](#platformfee) | Y | 책정 수수료 |
| `memo` | string |  | 해당 추가 수수료 정책에 대한 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) | Y | 부가세를 부담할 주체 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformAdditionalFeePolicyFilterInput

추가 수수료 정책 다건 조회를 위한 필터 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 추가 수수료 정책의 필터 옵션을 조회하고, false 이면 보관되지 않은 추가 수수료 정책의 필터 옵션을 조회합니다. 기본값은 false 입니다. |
| `vatPayers` | [PlatformPayer](#platformpayer)[] |  | 금액 부담 주체 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 부가세 부담 주체에 해당하는 추가 수수료 정책만 조회합니다. |
| `keyword` | [PlatformAdditionalFeePolicyFilterInputKeyword](#platformadditionalfeepolicyfilterinputkeyword) |  | 검색 키워드 |

### PlatformAdditionalFeePolicyFilterInputKeyword

검색 키워드 입력 정보 — 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 추가 수수료 정책만 조회합니다. 하위 필드는 명시된 값 중 한 가지만 적용됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 해당 값이 포함된 name 을 가진 추가 수수료 정책만 조회합니다. |
| `id` | string |  | 해당 값이 포함된 id 를 가진 추가 수수료 정책만 조회합니다. |
| `fee` | string |  | 해당 값과 같은 수수료 를 가진 추가 수수료 정책만 조회합니다. |

### PlatformBankAccountDetail

계좌 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `accountNumber` | string | Y | 계좌번호 |
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `provider` | [PlatformBankAccountProvider](#platformbankaccountprovider) | Y | 제공자 |
| `holder` | string |  | 예금주명 |

### PlatformBankAccountProvider

제공자

- `HYPHEN_DATA` — 하이픈 데이터
- `HYPHEN_FIRM` — 하이픈 펌뱅킹
- `DOZN` — 더즌
- `MOCK` — 모의

### PlatformBulkAccountTransfer

PlatformBulkAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 일괄 이체 고유 아이디 |
| `graphqlId` | string | Y |  |
| `creatorId` | string | Y |  |
| `bankAccountId` | string | Y | 출금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `totalAmount` | integer(int64) | Y |  |
| `status` | [PlatformBulkAccountTransferStatus](#platformbulkaccounttransferstatus) | Y |  |
| `stats` | [PlatformBulkAccountTransferStats](#platformbulkaccounttransferstats) | Y |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `createdAt` | string(date-time) | Y |  |
| `updatedAt` | string(date-time) | Y |  |
| `scheduledAt` | string(date-time) |  |  |

### PlatformBulkAccountTransferFilterInput

PlatformBulkAccountTransferFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformBulkAccountTransferStatus](#platformbulkaccounttransferstatus)[] |  |  |
| `criteria` | [PlatformBulkAccountTransferFilterInputCriteria](#platformbulkaccounttransferfilterinputcriteria) |  |  |

### PlatformBulkAccountTransferFilterInputCriteria

PlatformBulkAccountTransferFilterInputCriteria

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `bulkAccountTransferId` | string |  | 일괄 이체 아이디 |

### PlatformBulkAccountTransferStats

PlatformBulkAccountTransferStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `amount` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y |  |
| `count` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y |  |

### PlatformBulkAccountTransferStatus

PlatformBulkAccountTransferStatus

- `PREPARED`
- `SCHEDULED`
- `ONGOING`
- `COMPLETED`

### PlatformBulkAccountTransferStatusStats

PlatformBulkAccountTransferStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `prepared` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |
| `ongoing` | integer(int64) | Y |  |
| `completed` | integer(int64) | Y |  |

### PlatformBulkPayout

PlatformBulkPayout

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 일괄 지급 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 이름 |
| `creatorId` | string | Y | 생성자 아이디 |
| `method` | [PlatformPayoutMethod](#platformpayoutmethod) | Y | 지급 유형 |
| `totalPayoutAmount` | integer(int64) | Y | 총 지급 금액 |
| `totalSettlementAmount` | integer(int64) | Y | 총 정산 금액 |
| `status` | [PlatformBulkPayoutStatus](#platformbulkpayoutstatus) | Y | 상태 |
| `payoutStats` | [PlatformBulkPayoutStats](#platformbulkpayoutstats) | Y | 지급 통계 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 업데이트 일시 |

### PlatformBulkPayoutFilterInput

PlatformBulkPayoutFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformBulkPayoutStatus](#platformbulkpayoutstatus)[] |  |  |
| `methods` | [PlatformPayoutMethod](#platformpayoutmethod)[] |  |  |
| `criteria` | [PlatformBulkPayoutFilterInputCriteria](#platformbulkpayoutfilterinputcriteria) |  |  |

### PlatformBulkPayoutFilterInputCriteria

PlatformBulkPayoutFilterInputCriteria

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `bulkPayoutId` | string |  | 일괄 지급 아이디 |

### PlatformBulkPayoutStats

PlatformBulkPayoutStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `amount` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |
| `count` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |

### PlatformBulkPayoutStatus

PlatformBulkPayoutStatus

- `PREPARING`
- `PREPARED`
- `ONGOING`
- `POST_PROCESS_PENDING`
- `CANCELLED`
- `COMPLETED`

### PlatformBulkPayoutStatusStats

PlatformBulkPayoutStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `preparing` | integer(int64) | Y |  |
| `prepared` | integer(int64) | Y |  |
| `ongoing` | integer(int64) | Y |  |
| `postProcessPending` | integer(int64) | Y |  |
| `completed` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |

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

### PlatformContact

플랫폼 파트너 담당자 연락 정보 — 플랫폼 파트너 담당자 연락 정보<br>파트너 담당자에게 연락하기 위한 정보들 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 담당자 이름 |
| `phoneNumber` | string |  | 담당자 휴대폰 번호 |
| `email` | string | Y | 담당자 이메일 |

### PlatformContract

계약 — 계약<br>계약은 플랫폼 고객사가 파트너에게 정산해줄 대금과 정산일을 계산하는 데 적용되는 정보입니다.<br>고객사의 플랫폼에서 재화 및 서비스를 판매하기 위한 중개수수료와 판매금에 대한 정산일로 구성되어 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 계약 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFee](#platformfee) | Y | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycle](#platformsettlementcycle) | Y | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 — false인 경우 정산금에서 결제 금액 부가세를 감액하지 않고, true인 경우 정산금에서 결제 금액 부가세를 감액합니다. |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformContractFilterInput

계약 다건 조회를 위한 필터 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `platformFeePayers` | [PlatformPayer](#platformpayer)[] |  | 금액 부담 주체 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 수수료 부담 주체를 가진 계약만 조회합니다. |
| `cycleTypes` | [PlatformSettlementCycleType](#platformsettlementcycletype)[] |  | 플랫폼 정산 주기 계산 방식 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 주기 계산 방식을 가진 계약만 조회합니다. |
| `datePolicies` | [PlatformSettlementCycleDatePolicy](#platformsettlementcycledatepolicy)[] |  | 플랫폼 정산 기준일 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 기준일을 가진 계약만 조회합니다. |
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 계약을 조회하고, false 이면 보관되지 않은 계약을 조회합니다. 기본값은 false 입니다. |
| `keyword` | [PlatformContractFilterInputKeyword](#platformcontractfilterinputkeyword) |  | 검색 키워드 |

### PlatformContractFilterInputKeyword

검색 키워드 입력 정보 — 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 계약만 조회합니다. 하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 해당 값이 포함된 id 를 가진 계약만 조회합니다. |
| `name` | string |  | 해당 값이 포함된 name 을 가진 계약만 조회합니다. |

### PlatformDepositAccountTransfer

PlatformDepositAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 계좌 이체 유형 |
| `id` | string | Y | 계좌 이체 아이디 |
| `bankAccountId` | string | Y | 입금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `bankAccount` | [PlatformBankAccountDetail](#platformbankaccountdetail) | Y | 입금 계좌 정보 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `amount` | integer(int64) | Y | 금액 |
| `depositMemo` | string |  | 입금 계좌 적요 |
| `tradedAt` | string(date-time) |  | 이체 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 수정 일시 |
| `depositorName` | string | Y | 입금자명 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `status` | [PlatformAccountTransferStatus](#platformaccounttransferstatus) | Y | 상태 |

### PlatformDiscountSharePolicy

할인 분담 정책 — 할인 분담 정책<br>할인 분담은 고객사의 주문건에 쿠폰 및 포인트와 같은 할인금액이 적용될 때, 파트너 정산 시 할인금액에 대한 분담 정책을 가지는 객체입니다.<br>할인 유형에 대한 아이디와 메모, 그리고 파트너 분담율을 가집니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 할인 분담 정책 이름 |
| `partnerShareRate` | integer(int32) | Y | 할인 분담율 — 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformDiscountSharePolicyFilterInput

할인 분담 정책 다건 조회를 위한 필터 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 할인 분담 정책을 조회하고, false 이면 보관되지 않은 할인 분담 정책을 조회합니다. 기본값은 false 입니다. |
| `partnerShareRates` | integer(int32)[] |  | 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 파트너 분담율을 가진 할인 분담 정책만 조회합니다. |
| `keyword` | [PlatformDiscountSharePolicyFilterInputKeyword](#platformdiscountsharepolicyfilterinputkeyword) |  | 검색 키워드 |

### PlatformDiscountSharePolicyFilterInputKeyword

검색 키워드 입력 정보 — 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 할인 분담 정책만 조회합니다. 하위 필드는 명시된 값 중 한 가지만 적용됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 해당 값이 포함된 id 를 가진 할인 분담 정책만 조회합니다. |
| `name` | string |  | 해당 값이 포함된 name 을 가진 할인 분담만 조회합니다. |

### PlatformDiscountSharePolicyFilterOptions

할인 분담 정책 필터 옵션 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerShareRates` | integer(int32)[] | Y | 조회된 파트너 분담율 리스트 |

### PlatformExternalPayment

외부 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 결제 아이디 |
| `orderName` | string |  | 주문 명 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `method` | [PlatformPaymentMethod](#platformpaymentmethod) |  | 결제 수단 |
| `paidAt` | string(date-time) |  | 결제 일시 |

### PlatformFee

플랫폼 중개수수료 정보

oneOf, discriminator `type`:

- `FIXED_AMOUNT` → [PlatformFixedAmountFee](#platformfixedamountfee)
- `FIXED_RATE` → [PlatformFixedRateFee](#platformfixedratefee)

### PlatformFeeInput

수수료 계산 방식을 특정하기 위한 입력 정보 — 수수료 계산 방식을 특정하기 위한 입력 정보<br>정률 수수료를 설정하고 싶은 경우 `fixedRate` 필드에, 정액 수수료를 설정하고 싶은 경우 `fixedAmount` 필드에 값을 명시해 요청합니다.<br>두 필드 모두 값이 들어있지 않은 경우 요청이 거절됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fixedRate` | integer(int32) |  | 정률 수수료 |
| `fixedAmount` | integer(int64) |  | 정액 수수료 |

### PlatformFixedAmountFee

정액 수수료 — 정액 수수료<br>총 금액에 무관하게 정해진 수수료 금액을 책정합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `amount` | integer(int64) | Y | 고정된 수수료 금액 |

### PlatformFixedRateFee

정률 수수료 — 정률 수수료<br>총 금액에 정해진 비율을 곱한 만큼의 수수료를 책정합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `rate` | integer(int32) | Y | 수수료율 — 총 금액 대비 수수료 비율을 의미하며, 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수입니다. `총 금액 * rate * 10^5` (`rate * 10^3 %`) 만큼 수수료를 책정합니다. |

### PlatformManualTransfer

수기 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |

### PlatformManualTransferSummary

PlatformManualTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `settlementAmount` | integer(int64) | Y |  |
| `settlementTaxFreeAmount` | integer(int64) | Y |  |

### PlatformOrderCancelTransfer

주문 취소 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 정산 금액 정보 |
| `contract` | [PlatformContract](#platformcontract) | Y | 계약 |
| `payment` | [PlatformPayment](#platformpayment) | Y | 결제 정보 |
| `settlementStartDate` | string(date) | Y | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `orderLines` | [PlatformOrderTransferOrderLine](#platformordertransferorderline)[] | Y | 주문 항목 리스트 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 정산 금액 계산 시 사용된 추가 수수료 정보 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 정산 금액 계산 시 사용된 할인 정보 |
| `cancellation` | [PlatformOrderTransferCancellation](#platformordertransfercancellation) | Y | 주문 취소 정보 |
| `parameters` | [TransferParameters](#transferparameters) | Y | 정산 파라미터 (실험기능) |

### PlatformOrderCancelTransferSummary

PlatformOrderCancelTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `storeId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `contract` | [PlatformContract](#platformcontract) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y |  |
| `payment` | [PlatformTransferSummaryPayment](#platformtransfersummarypayment) | Y |  |
| `settlementStartDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformOrderSettlementAmount

정산 금액 정보 — 정산 금액 정보<br>정산 금액과 정산 금액 계산에 사용된 금액 정보들 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlement` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFree` | integer(int64) | Y | 정산 면세 금액 |
| `payment` | integer(int64) | Y | 결제 금액 |
| `paymentVat` | integer(int64) | Y | 결제 금액 부가세 |
| `paymentVatBurden` | integer(int64) | Y | 결제 금액 부가세 부담금액 — 참조된 계약의 결제 금액 부가세 감액 여부에 따라 false인 경우 0원, true인 경우 결제 금액 부가세입니다. |
| `paymentTaxFree` | integer(int64) | Y | 결제 면세 금액 |
| `paymentSupply` | integer(int64) | Y | 결제 공급가액 |
| `order` | integer(int64) | Y | 주문 금액 |
| `orderTaxFree` | integer(int64) | Y | 면세 주문 금액 |
| `platformFee` | integer(int64) | Y | 중개 수수료 |
| `platformFeeVat` | integer(int64) | Y | 중개 수수료 부가세 |
| `additionalFee` | integer(int64) | Y | 추가 수수료 |
| `additionalFeeVat` | integer(int64) | Y | 추가 수수료 부가세 |
| `discount` | integer(int64) | Y | 할인 금액 |
| `discountTaxFree` | integer(int64) | Y | 면세 할인 금액 |
| `discountShare` | integer(int64) | Y | 할인 분담 금액 |
| `discountShareTaxFree` | integer(int64) | Y | 면세 할인 분담 금액 |
| `userDefinedFormulas` | [PlatformUserDefinedFormulaResults](#platformuserdefinedformularesults) | Y | 사용자 정의 수식 결과 |

### PlatformOrderTransfer

주문 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 정산 금액 정보 |
| `contract` | [PlatformContract](#platformcontract) | Y | 계약 |
| `payment` | [PlatformPayment](#platformpayment) | Y | 결제 정보 |
| `settlementStartDate` | string(date) | Y | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `orderLines` | [PlatformOrderTransferOrderLine](#platformordertransferorderline)[] | Y | 주문 항목 리스트 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 정산 금액 계산 시 사용된 추가 수수료 정보 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 정산 금액 계산 시 사용된 할인 정보 |
| `parameters` | [TransferParameters](#transferparameters) | Y | 정산 파라미터 (실험기능) |

### PlatformOrderTransferAdditionalFee

추가 수수료 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `policy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 추가 수수료 정책 |
| `amount` | integer(int64) | Y | 추가 수수료 금액 |
| `vat` | integer(int64) | Y | 추가 수수료 부가세 금액 |

### PlatformOrderTransferCancellation

주문 취소 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 주문 취소 아이디 |
| `cancelledAt` | string(date-time) | Y | 취소 일시 |

### PlatformOrderTransferDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 할인 분담 정책 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) | Y | 면세 할인 금액 |
| `shareAmount` | integer(int64) | Y | 할인 분담 금액 |
| `shareTaxFreeAmount` | integer(int64) | Y | 면세 할인 분담 금액 |

### PlatformOrderTransferOrderLine

주문 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [PlatformOrderTransferProduct](#platformordertransferproduct) | Y | 상품 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 상품 할인 정보 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 상품 추가 수수료 정보 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 상품 정산 금액 정보 |

### PlatformOrderTransferProduct

상품

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 아이디 |
| `name` | string | Y | 상품 이름 |
| `amount` | integer(int64) | Y | 상품 금액 |
| `taxFreeAmount` | integer(int64) | Y | 상품 면세 금액 |
| `tag` | string |  | 태그 |

### PlatformOrderTransferSummary

PlatformOrderTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `storeId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `contract` | [PlatformContract](#platformcontract) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y |  |
| `payment` | [PlatformTransferSummaryPayment](#platformtransfersummarypayment) | Y |  |
| `settlementStartDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPartner

파트너 — 파트너<br>파트너는 고객사가 정산해주어야 할 대상입니다.<br>기본 사업자 정보와 정산정보, 그리고 적용될 계약의 정보를 등록 및 관리할 수 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 파트너 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 파트너 법인명 혹은 이름 |
| `contact` | [PlatformContact](#platformcontact) | Y | 파트너 담당자 연락 정보 |
| `account` | [PlatformAccount](#platformaccount) | Y | 정산 계좌 |
| `status` | [PlatformPartnerStatus](#platformpartnerstatus) | Y | 파트너의 상태 |
| `defaultContractId` | string | Y | 파트너에 설정된 기본 계약 아이디 |
| `memo` | string |  | 파트너에 대한 메모 |
| `tags` | string[] | Y | 파트너의 태그 리스트 |
| `type` | [PlatformPartnerType](#platformpartnertype) | Y | 파트너 유형별 정보 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `userDefinedProperties` | [PlatformProperties](#platformproperties) | Y | 사용자 정의 속성 |
| `isForTest` | boolean | Y |  |

### PlatformPartnerBusinessStatus

플랫폼 파트너 사업자 상태

- `NOT_VERIFIED` — 조회 되지 않음
- `VERIFY_ERROR` — 조회 오류
- `NOT_FOUND` — 대응되는 사업자 없음
- `IN_BUSINESS` — 사업 중
- `CLOSED` — 폐업
- `SUSPENDED` — 휴업

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
| `banks` | [Bank](schemas-common.md#bank)[] |  | 은행 — 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 계좌 은행을 가진 파트너만 조회합니다. |
| `accountCurrencies` | [Currency](schemas-common.md#currency)[] |  | 통화 — 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 계좌 통화를 가진 파트너만 조회합니다. |
| `ids` | string[] |  | 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 아이디를 가진 파트너만 조회합니다. |
| `contractIds` | string[] |  | 하나 이상의 값이 존재하는 경우,  해당 리스트에 포함되는 기본 계약 id를 가진 파트너만 조회합니다. |
| `accountStatuses` | [PlatformAccountStatus](#platformaccountstatus)[] |  | 플랫폼 계좌 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 계좌 상태를 가진 파트너만 조회합니다. |
| `businessStatuses` | [PlatformPartnerBusinessStatus](#platformpartnerbusinessstatus)[] |  | 플랫폼 파트너 사업자 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 사업자 상태를 가진 파트너만 조회합니다. |
| `types` | [PlatformPartnerTypeName](#platformpartnertypename)[] |  | 플랫폼 파트너 유형 이름 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 사업자 유형을 가진 파트너만 조회합니다. |
| `taxationTypes` | [PlatformPartnerTaxationType](#platformpartnertaxationtype)[] |  | 플랫폼 파트너 과세 유형 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 과세 유형을 가진 파트너만 조회합니다. |
| `memberCompanyConnectionStatuses` | [PlatformPartnerMemberCompanyConnectionStatus](#platformpartnermembercompanyconnectionstatus)[] |  | 플랫폼 파트너 연동 사업자 연결 상태 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 연동 사업자 연결 상태를 가진 파트너만 조회합니다. |
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

### PlatformPartnerManualSettlement

PlatformPartnerManualSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `amount` | integer(int64) | Y | 정산 금액 |
| `taxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerMemberCompanyConnectionStatus

플랫폼 파트너 연동 사업자 연결 상태

- `NOT_CONNECTED` — 연결되지 않음
- `CONNECT_PENDING` — 연결 대기
- `CONNECTED` — 연결됨
- `CONNECT_FAILED` — 연결 실패
- `DISCONNECT_PENDING` — 연결 해제 대기

### PlatformPartnerOrderCancelSettlement

PlatformPartnerOrderCancelSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `contract` | [PlatformContract](#platformcontract) | Y | 계약 |
| `settlementStartDateRange` | [DateRange](schemas-common.md#daterange) | Y | 정산 시작 일 범위 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 금액 정보 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerOrderSettlement

PlatformPartnerOrderSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `contract` | [PlatformContract](#platformcontract) | Y | 계약 |
| `settlementStartDateRange` | [DateRange](schemas-common.md#daterange) | Y | 정산 시작 일 범위 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 금액 정보 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerSettlement

PlatformPartnerSettlement

oneOf, discriminator `type`:

- `MANUAL` → [PlatformPartnerManualSettlement](#platformpartnermanualsettlement)
- `ORDER_CANCEL` → [PlatformPartnerOrderCancelSettlement](#platformpartnerordercancelsettlement)
- `ORDER` → [PlatformPartnerOrderSettlement](#platformpartnerordersettlement)

### PlatformPartnerSettlementFilterInput

PlatformPartnerSettlementFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlementDates` | string(date)[] |  |  |
| `contractIds` | string[] |  |  |
| `partnerTags` | string[] |  |  |
| `settlementCurrencies` | [Currency](schemas-common.md#currency)[] |  | 통화 |
| `statuses` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus)[] |  | 정산 상태 |
| `partnerIds` | string[] |  |  |
| `settlementTypes` | [PlatformPartnerSettlementType](#platformpartnersettlementtype)[] |  | 정산 유형 |
| `keyword` | [PlatformPartnerSettlementFilterKeywordInput](#platformpartnersettlementfilterkeywordinput) |  |  |

### PlatformPartnerSettlementFilterKeywordInput

PlatformPartnerSettlementFilterKeywordInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementId` | string |  |  |
| `payoutId` | string |  |  |
| `bulkPayoutId` | string |  |  |

### PlatformPartnerSettlementStatus

정산 상태

- `PAYOUT_SCHEDULED` — 지급 예약
- `PAYOUT_PREPARED` — 지급 예정
- `PAYOUT_WITHHELD` — 지급 보류
- `PAYOUT_EXCLUDED` — 지급 제외
- `PAYOUT_FAILED` — 지급 실패
- `IN_PAYOUT` — 지급 중
- `PAID_OUT` — 지급 완료
- `CANCELLED` — 지급 취소
- `CONFIRMED` — 지급 확정

### PlatformPartnerSettlementStatusStats

PlatformPartnerSettlementStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payoutScheduled` | integer(int64) | Y |  |
| `payoutPrepared` | integer(int64) | Y |  |
| `payoutWithheld` | integer(int64) | Y |  |
| `payoutExcluded` | integer(int64) | Y |  |
| `payoutFailed` | integer(int64) | Y |  |
| `payoutCancelled` | integer(int64) | Y |  |
| `payoutConfirmed` | integer(int64) | Y |  |
| `inPayout` | integer(int64) | Y |  |
| `paidOut` | integer(int64) | Y |  |

### PlatformPartnerSettlementType

정산 유형

- `MANUAL` — 수동 정산
- `ORDER` — 주문 정산
- `ORDER_CANCEL` — 주문 취소 정산

### PlatformPartnerStatus

플랫폼 파트너 상태

- `PENDING` — 승인 대기 중
- `APPROVED` — 승인 완료
- `REJECTED` — 승인 거절

### PlatformPartnerTaxationType

플랫폼 파트너 과세 유형

- `NORMAL` — 일반 과세
- `SIMPLE_TAX_INVOICE_ISSUER` — 간이과세(세금계산서 발행)
- `SIMPLE` — 간이과세(세금계산서 미발행)
- `TAX_FREE` — 면세

### PlatformPartnerType

파트너 유형별 추가 정보

oneOf, discriminator `type`:

- `BUSINESS` → [PlatformPartnerTypeBusiness](#platformpartnertypebusiness)
- `NON_WHT_PAYER` → [PlatformPartnerTypeNonWhtPayer](#platformpartnertypenonwhtpayer)
- `WHT_PAYER` → [PlatformPartnerTypeWhtPayer](#platformpartnertypewhtpayer)

### PlatformPartnerTypeBusiness

사업자 파트너 정보 — 사업자 파트너 정보<br>사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `companyName` | string | Y | 상호명 |
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) | Y | 과세 유형 |
| `businessStatus` | [PlatformPartnerBusinessStatus](#platformpartnerbusinessstatus) | Y | 사업자 상태 |
| `businessRegistrationNumber` | string | Y | 사업자등록번호 |
| `representativeName` | string | Y | 대표자 이름 |
| `companyAddress` | string |  | 사업장 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `memberCompanyConnectionStatus` | [PlatformPartnerMemberCompanyConnectionStatus](#platformpartnermembercompanyconnectionstatus) | Y | 연동사업자 연동 상태 |
| `memberCompanyConnectionFailedReason` | string |  | 연동사업자 연동 실패 사유 |

### PlatformPartnerTypeName

플랫폼 파트너 유형 이름

- `BUSINESS` — 사업자
- `WHT_PAYER` — 원천징수 대상자
- `NON_WHT_PAYER` — 원천징수 비대상자

### PlatformPartnerTypeNonWhtPayer

원천징수 비대상자 파트너 정보 — 원천징수 비대상자 파트너 정보<br>비사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPartnerTypeWhtPayer

원천징수 대상자 파트너 정보 — 원천징수 대상자 파트너 정보<br>비사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPayer

금액 부담 주체 — 금액 부담 주체<br>플랫폼에서 발생한 결제 수수료, 부가세 등 금액을 부담하는 주체를 나타냅니다.

- `PARTNER` — 파트너가 부담하는 경우
- `MERCHANT` — 고객사가 부담하는 경우

### PlatformPayment

결제 정보

oneOf, discriminator `type`:

- `EXTERNAL` → [PlatformExternalPayment](#platformexternalpayment)
- `PORT_ONE` → [PlatformPortOnePayment](#platformportonepayment)

### PlatformPaymentMethod

결제 수단

oneOf, discriminator `type`:

- `CARD` → [PlatformPaymentMethodCard](#platformpaymentmethodcard)
- `EASY_PAY` → [PlatformPaymentMethodEasyPay](#platformpaymentmethodeasypay)
- `GIFT_CERTIFICATE` → [PlatformPaymentMethodGiftCertificate](#platformpaymentmethodgiftcertificate)
- `MOBILE` → [PlatformPaymentMethodMobile](#platformpaymentmethodmobile)
- `TRANSFER` → [PlatformPaymentMethodTransfer](#platformpaymentmethodtransfer)
- `VIRTUAL_ACCOUNT` → [PlatformPaymentMethodVirtualAccount](#platformpaymentmethodvirtualaccount)

### PlatformPaymentMethodCard

카드

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodCardInput

PlatformPaymentMethodCardInput

type: object

### PlatformPaymentMethodEasyPay

간편 결제

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `provider` | [EasyPayProvider](schemas-common.md#easypayprovider) |  | 간편 결제사 |
| `methodType` | [EasyPayMethodType](#easypaymethodtype) |  | 간편 결제 수단 |

### PlatformPaymentMethodEasyPayInput

간편 결제 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `provider` | [EasyPayProvider](schemas-common.md#easypayprovider) |  | 간편 결제사 |
| `methodType` | [EasyPayMethodType](#easypaymethodtype) |  | 간편 결제 수단 |

### PlatformPaymentMethodGiftCertificate

상품권

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodGiftCertificateInput

PlatformPaymentMethodGiftCertificateInput

type: object

### PlatformPaymentMethodInput

결제 수단 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `card` | [PlatformPaymentMethodCardInput](#platformpaymentmethodcardinput) |  | 카드 |
| `transfer` | [PlatformPaymentMethodTransferInput](#platformpaymentmethodtransferinput) |  | 계좌이체 |
| `virtualAccount` | [PlatformPaymentMethodVirtualAccountInput](#platformpaymentmethodvirtualaccountinput) |  | 가상계좌 |
| `giftCertificate` | [PlatformPaymentMethodGiftCertificateInput](#platformpaymentmethodgiftcertificateinput) |  | 상품권 |
| `mobile` | [PlatformPaymentMethodMobileInput](#platformpaymentmethodmobileinput) |  | 모바일 |
| `easyPay` | [PlatformPaymentMethodEasyPayInput](#platformpaymentmethodeasypayinput) |  | 간편 결제 |

### PlatformPaymentMethodMobile

모바일

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodMobileInput

PlatformPaymentMethodMobileInput

type: object

### PlatformPaymentMethodTransfer

계좌이체

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodTransferInput

PlatformPaymentMethodTransferInput

type: object

### PlatformPaymentMethodVirtualAccount

가상계좌

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodVirtualAccountInput

PlatformPaymentMethodVirtualAccountInput

type: object

### PlatformPayout

PlatformPayout

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 지급 고유 아이디 |
| `graphqlId` | string | Y |  |
| `bulkPayoutId` | string | Y |  |
| `bulkPayoutGraphqlId` | string | Y |  |
| `method` | [PlatformPayoutMethod](#platformpayoutmethod) | Y |  |
| `status` | [PlatformPayoutStatus](#platformpayoutstatus) | Y |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `memo` | string |  |  |
| `partner` | [PlatformPartner](#platformpartner) | Y |  |
| `account` | [PlatformPayoutAccount](#platformpayoutaccount) | Y |  |
| `taxInvoiceId` | string |  | 세금계산서 아이디 |
| `taxInvoiceStatus` | [PlatformPayoutTaxInvoiceStatus](#platformpayouttaxinvoicestatus) | Y | 세금계산서 상태 |
| `currency` | [Currency](schemas-common.md#currency) | Y |  |
| `amount` | integer(int64) | Y | 지급금액 |
| `supplyAmount` | integer(int64) | Y | 공급가액 |
| `taxFreeAmount` | integer(int64) | Y | 면세 금액 |
| `vatAmount` | integer(int64) | Y | 부가세 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |
| `incomeTaxAmount` | integer(int64) | Y | 원천징수세액 (소득세) |
| `localIncomeTaxAmount` | integer(int64) | Y | 원천징수세액 (지방소득세) |
| `withdrawalMemo` | string |  |  |
| `depositMemo` | string |  |  |
| `createdAt` | string(date-time) | Y |  |
| `scheduledAt` | string(date-time) |  |  |
| `failReason` | string |  | 실패 사유 |
| `deductWht` | boolean | Y | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](#settlementamounttype) | Y | 정산 금액 취급 기준 |
| `settlementStatement` | [PlatformPayoutSettlementStatementSummary](#platformpayoutsettlementstatementsummary) | Y | 정산 내역서 요약 정보 |

### PlatformPayoutAccount

PlatformPayoutAccount

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y |  |
| `number` | string | Y |  |
| `holder` | string | Y |  |

### PlatformPayoutFilterInput

지급 내역 필터 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformPayoutStatus](#platformpayoutstatus)[] |  | 지급 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 상태를 가진 지급 내역을 조회합니다. |
| `partnerIds` | string[] |  | 파트너 아이디 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 아이디를 가진 지급 내역을 조회합니다. |
| `criteria` | [PlatformPayoutFilterInputCriteria](#platformpayoutfilterinputcriteria) | Y | 조회 기준 |
| `payoutAccountBanks` | [Bank](schemas-common.md#bank)[] |  | 지급 계좌 은행 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 계좌 은행을 가진 지급 내역을 조회합니다. |
| `partnerTags` | string[] |  | 파트너 태그 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 태그를 하나 이상 가진 지급 내역을 조회합니다. |
| `payoutCurrencies` | [Currency](schemas-common.md#currency)[] |  | 지급 통화 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 통화를 가진 지급 내역을 조회합니다. |
| `payoutIds` | string[] |  | 지급 아이디 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 아이디를 가진 지급 내역을 조회합니다. |
| `taxInvoiceStatuses` | [PlatformPayoutTaxInvoiceStatus](#platformpayouttaxinvoicestatus)[] |  | 세금계산서 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 세금계산서 상태를 가진 지급 내역을 조회합니다. |
| `partnerTypes` | [PlatformPartnerTypeName](#platformpartnertypename)[] |  | 파트너 유형 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 유형을 가진 지급 내역을 조회합니다. |
| `partnerTaxationTypes` | [PlatformPartnerTaxationType](#platformpartnertaxationtype)[] |  | 파트너 과세 유형 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 과세 유형을 가진 지급 내역을 조회합니다. |
| `settlementStatementStatuses` | [PlatformPayoutSettlementStatementStatus](#platformpayoutsettlementstatementstatus)[] |  | 정산 내역서 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 정산 내역서 상태를 가진 지급 내역을 조회합니다. |

### PlatformPayoutFilterInputCriteria

검색 기준 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `scheduledTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 지급 예정 일시 범위 |
| `settlementStatementIssuedTimestampRange` | [DateTimeRange](schemas-common.md#datetimerange) |  | 정산 내역서 발송 일시 범위 |
| `payoutId` | string |  | 지급 아이디 |
| `bulkPayoutId` | string |  | 일괄 지급 아이디 |
| `taxInvoiceId` | string |  | 세금계산서 아이디 |
| `settlementStatementId` | string |  | 정산 내역서 아이디 |

### PlatformPayoutMethod

PlatformPayoutMethod

- `DIRECT`
- `AGENCY`

### PlatformPayoutSettlementStatementStatus

정산 내역서 발송 상태

- `UNSENT` — 미발송
- `SENT` — 발송 성공
- `SEND_FAILED` — 발송 실패
- `SEND_PREPARED` — 발송 대기

### PlatformPayoutSettlementStatementSummary

정산 내역서 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | [PlatformPayoutSettlementStatementStatus](#platformpayoutsettlementstatementstatus) | Y | 상태 |
| `id` | string |  | 아이디 |
| `issuedAt` | string(date-time) |  | 발송 일시 |

### PlatformPayoutStatus

PlatformPayoutStatus

- `CONFIRMED`
- `PREPARED`
- `CANCELLED`
- `STOPPED`
- `PROCESSING`
- `SUCCEEDED`
- `FAILED`
- `SCHEDULED`

### PlatformPayoutStatusStats

PlatformPayoutStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `confirmed` | integer(int64) | Y |  |
| `prepared` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |
| `stopped` | integer(int64) | Y |  |
| `processing` | integer(int64) | Y |  |
| `succeeded` | integer(int64) | Y |  |
| `failed` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |

### PlatformPayoutTaxInvoiceStatus

PlatformPayoutTaxInvoiceStatus

- `NONE`
- `DRAFTED`
- `DRAFT_PENDING`
- `DRAFT_FAILED`
- `REQUESTED`
- `REQUEST_PENDING`
- `REQUEST_FAILED`
- `REQUEST_CANCELLED`
- `REQUEST_REFUSED`
- `ISSUED`
- `ISSUE_PENDING`
- `ISSUE_FAILED`
- `ISSUANCE_CANCELLED`
- `CANCEL_PENDING`
- `BEFORE_SENDING`
- `WAITING_SENDING`
- `SENDING`
- `SENDING_PENDING`
- `SENDING_COMPLETED`
- `SENDING_FAILED`

### PlatformPortOnePayment

포트원 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 결제 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `channelKey` | string | Y | 채널 키 |
| `orderName` | string | Y | 주문 명 |
| `method` | [PlatformPaymentMethod](#platformpaymentmethod) |  | 결제 수단 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `paidAt` | string(date-time) | Y | 결제 일시 |

### PlatformProperties

PlatformProperties

type: map<string, [PlatformUserDefinedPropertyValue](#platformuserdefinedpropertyvalue)>

### PlatformSetting

플랫폼 설정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `defaultWithdrawalMemo` | string |  | 기본 보내는 이 통장 메모 |
| `defaultDepositMemo` | string |  | 기본 받는 이 통장 메모 |
| `supportsMultipleOrderTransfersPerPartner` | boolean | Y | paymentId, storeId, partnerId가 같은 주문 정산건에 대한 중복 정산 지원 여부 |
| `adjustSettlementDateAfterHolidayIfEarlier` | boolean | Y | 정산일이 정산시작일보다 작거나 같을 경우 공휴일 후 영업일로 정산일 다시 계산 여부 |
| `deductWht` | boolean | Y | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](#settlementamounttype) | Y | 정산 금액 취급 기준 |
| `isForTest` | boolean | Y |  |
| `manualSettlementPerTransfer` | boolean | Y | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### PlatformSettlementCycle

정산 주기 — 정산 주기<br>지체일, 정산일, 기준일로 구성되며, 해당 요소들의 조합으로 실제 정산일을 계산합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `lagDays` | integer(int32) | Y | 지체일 (d+n 의 n) — 정산시작일(통상 주문완료일)로부터 더해진 다음 날짜로부터 가장 가까운 날에 정산이 됩니다. 최소 1 에서 최대 10 까지 지정할 수 있습니다. |
| `datePolicy` | [PlatformSettlementCycleDatePolicy](#platformsettlementcycledatepolicy) | Y | 기준일로, 정산일 계산 시 공휴일을 고려하기 위한 정보입니다. |
| `method` | [PlatformSettlementCycleMethod](#platformsettlementcyclemethod) | Y | 정산 주기 계산 방식 |

### PlatformSettlementCycleDatePolicy

플랫폼 정산 기준일

- `HOLIDAY_BEFORE` — 공휴일 전 영업일
- `HOLIDAY_AFTER` — 공휴일 후 영업일
- `CALENDAR_DAY` — 달력일

### PlatformSettlementCycleInput

플랫폼 정산 주기 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `lagDays` | integer(int32) | Y | 지체일 (d+n 의 n) — 정산시작일(통상 주문완료일)로부터 더해진 다음 날짜로부터 가장 가까운 날에 정산이 됩니다. 최소 1 에서 최대 10 까지 지정할 수 있습니다. |
| `datePolicy` | [PlatformSettlementCycleDatePolicy](#platformsettlementcycledatepolicy) | Y | 기준일로, 정산일 계산 시 공휴일을 고려하기 위한 정보입니다. |
| `method` | [PlatformSettlementCycleMethodInput](#platformsettlementcyclemethodinput) | Y | 정산 주기 계산 방식 |

### PlatformSettlementCycleMethod

플랫폼 정산 주기 계산 방식

oneOf, discriminator `type`:

- `DAILY` → [PlatformSettlementCycleMethodDaily](#platformsettlementcyclemethoddaily)
- `MANUAL_DATES` → [PlatformSettlementCycleMethodManualDates](#platformsettlementcyclemethodmanualdates)
- `MONTHLY` → [PlatformSettlementCycleMethodMonthly](#platformsettlementcyclemethodmonthly)
- `WEEKLY` → [PlatformSettlementCycleMethodWeekly](#platformsettlementcyclemethodweekly)

### PlatformSettlementCycleMethodDaily

매일 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformSettlementCycleMethodDailyInput

PlatformSettlementCycleMethodDailyInput

type: object

### PlatformSettlementCycleMethodInput

플랫폼 정산 주기 계산 방식 입력 정보 — 플랫폼 정산 주기 계산 방식 입력 정보<br>하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `daily` | [PlatformSettlementCycleMethodDailyInput](#platformsettlementcyclemethoddailyinput) |  | 매일 정산 |
| `weekly` | [PlatformSettlementCycleMethodWeeklyInput](#platformsettlementcyclemethodweeklyinput) |  | 매주 정해진 요일에 정산 |
| `monthly` | [PlatformSettlementCycleMethodMonthlyInput](#platformsettlementcyclemethodmonthlyinput) |  | 매월 정해진 날(일)에 정산 |
| `manualDates` | [PlatformSettlementCycleMethodManualDatesInput](#platformsettlementcyclemethodmanualdatesinput) |  | 정해진 날짜(월, 일)에 정산 |

### PlatformSettlementCycleMethodManualDates

정해진 날짜(월, 일)에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `dates` | [MonthDay](#monthday)[] | Y | 월 및 일자 정보 |

### PlatformSettlementCycleMethodManualDatesInput

PlatformSettlementCycleMethodManualDatesInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dates` | [MonthDay](#monthday)[] | Y | 월 및 일자 정보 |

### PlatformSettlementCycleMethodMonthly

매월 정해진 날(일)에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `daysOfMonth` | integer(int32)[] | Y |  |

### PlatformSettlementCycleMethodMonthlyInput

PlatformSettlementCycleMethodMonthlyInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `daysOfMonth` | integer(int32)[] | Y |  |

### PlatformSettlementCycleMethodWeekly

매주 정해진 요일에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `daysOfWeek` | [DayOfWeek](#dayofweek)[] | Y | 요일 |

### PlatformSettlementCycleMethodWeeklyInput

PlatformSettlementCycleMethodWeeklyInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `daysOfWeek` | [DayOfWeek](#dayofweek)[] | Y | 요일 |

### PlatformSettlementCycleType

플랫폼 정산 주기 계산 방식

- `DAILY` — 매일 정산
- `WEEKLY` — 매주 정해진 요일에 정산
- `MONTHLY` — 매월 정해진 날(일)에 정산
- `MANUAL_DATES` — 정해진 날짜(월, 일)에 정산

### PlatformSettlementParameterValue

플랫폼 정산 파라미터 값

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `decimal` | integer(int64) | Y | 크기가 조정되지 않은 숫자 |
| `decimalScale` | integer(int32) |  | 소수 자리수 — 정산 시 필요한 `decimalScale`이 지정되지 않은 경우 기본값으로 0을 사용합니다.<br>입력 가능한 법위는 0 ~ 5 입니다. |

### PlatformTaxationType

플랫폼 과세 유형

- `NORMAL` — 일반 과세
- `SIMPLE_TAX_INVOICE_ISSUER` — 간이과세(세금계산서 발행)
- `SIMPLE` — 간이과세(세금계산서 미발행)
- `TAX_FREE` — 면세
- `ASSIGNED_ID_NUMBER` — 고유 번호 부여 사업자 (비영리, 국가 등 납세 의무가 없는)
- `SPECIAL` — 과세 특례자

### PlatformTransfer

정산건 — 정산건<br>정산건은 파트너에 정산해줄 정산 금액과 정산 방식 등이 포함되어 있는 정산 정보입니다.<br>정산 방식은은 주문 정산, 주문 취소 정산, 수기 정산이 있습니다.

oneOf, discriminator `type`:

- `MANUAL` → [PlatformManualTransfer](#platformmanualtransfer)
- `ORDER_CANCEL` → [PlatformOrderCancelTransfer](#platformordercanceltransfer)
- `ORDER` → [PlatformOrderTransfer](#platformordertransfer)

### PlatformTransferFilterInput

정산건 필터 입력 정보 — 정산건 필터 입력 정보<br>정산 시작일 범위와 정산 일 범위는 둘 중 하나만 입력 가능합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlementStartDateRange` | [DateRange](schemas-common.md#daterange) |  | 정산 시작일 범위 |
| `settlementDateRange` | [DateRange](schemas-common.md#daterange) |  | 정산 일 범위 |
| `partnerTags` | string[] |  | 파트너 태그 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 태그를 하나 이상 가지는 파트너에 대한 정산건만 조회합니다. |
| `contractIds` | string[] |  | 계약 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 계약 아이디를 가지는 정산건만 조회합니다. |
| `discountSharePolicyIds` | string[] |  | 할인 분담 정책 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 할인 분담 정책 아이디를 하나 이상 가지는 정산건만 조회합니다. |
| `additionalFeePolicyIds` | string[] |  | 추가 수수료 정책 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 추가 수수료 아이디를 하나 이상 가지는 정산건만 조회합니다. |
| `paymentMethodTypes` | [PaymentMethodType](schemas-common.md#paymentmethodtype)[] |  | 결제 수단 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 결제 수단을 가지는 파트너만 조회합니다. |
| `channelKeys` | string[] |  | 채널 키 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 채널 키를 가지는 정산건만 조회합니다. |
| `types` | [PlatformTransferType](#platformtransfertype)[] |  | 정산 방식 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 방식의 정산건만 조회합니다. |
| `statuses` | [PlatformTransferStatus](#platformtransferstatus)[] |  | 정산 상태 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 상태인 정산건만 조회합니다. |
| `userDefinedProperty` | [PlatformTransferPropertyExactMatchInput](#platformtransferpropertyexactmatchinput) |  | 사용자 정의 속성 exact match — 사용자 정의 속성의 key와 value가 정확히 일치하는 정산건만 조회합니다. |
| `keyword` | [PlatformTransferFilterInputKeyword](#platformtransferfilterinputkeyword) |  | 검색 키워드 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Filter의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### PlatformTransferFilterInputKeyword

정산건 검색 키워드 입력 정보 — 정산건 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 정산건만 조회합니다. 하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `all` | string |  | 해당 값이 포함된 정보를 가진 정산건만 조회합니다. |
| `paymentId` | string |  | 해당 값이랑 일치하는 paymentId 를 가진 정산건만 조회합니다. |
| `transferId` | string |  | 해당 값이랑 일치하는 transferId 를 가진 정산건만 조회합니다. |
| `transferMemo` | string |  | 해당 값이 포함된 transferMemo 를 가진 정산건만 조회합니다. |
| `productId` | string |  | 해당 값이랑 일치하는 productId 를 가진 정산건만 조회합니다. |
| `productName` | string |  | 해당 값이랑 일치하는 productName 을 가진 정산건만 조회합니다. |
| `partnerId` | string |  | 해당 값이랑 일치하는 partnerId 를 가진 정산건만 조회합니다. |
| `partnerName` | string |  | 해당 값이 포함된 partnerName 을 가진 정산건만 조회합니다. |
| `partnerMemo` | string |  | 해당 값이 포함된 partnerMemo 를 가진 정산건만 조회합니다. |
| `partnerSettlementId` | string |  | 해당 값이 포함된 partnerSettlementId 를 가진 정산건만 조회합니다. |
| `payoutId` | string |  | 해당 값과 일치하는 지급건에 연관된 정산건만 조회합니다. |

### PlatformTransferPropertyExactMatchInput

사용자 정의 속성 key/value exact match

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `key` | string | Y | 키 |
| `value` | string | Y | 값 |

### PlatformTransferStatus

정산 상태

- `SCHEDULED` — 정산 예약
- `IN_PROCESS` — 정산 중
- `SETTLED` — 정산 완료
- `IN_PAYOUT` — 지급 중
- `PAID_OUT` — 지급 완료
- `PAYOUT_CONFIRMED` — 지급 확정

### PlatformTransferSummary

PlatformTransferSummary

oneOf, discriminator `type`:

- `MANUAL` → [PlatformManualTransferSummary](#platformmanualtransfersummary)
- `ORDER_CANCEL` → [PlatformOrderCancelTransferSummary](#platformordercanceltransfersummary)
- `ORDER` → [PlatformOrderTransferSummary](#platformordertransfersummary)

### PlatformTransferSummaryExternalPayment

PlatformTransferSummaryExternalPayment

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `orderName` | string |  |  |
| `currency` | [Currency](schemas-common.md#currency) | Y |  |
| `methodType` | [PaymentMethodType](schemas-common.md#paymentmethodtype) |  |  |

### PlatformTransferSummaryPartner

PlatformTransferSummaryPartner

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `name` | string | Y |  |
| `type` | [PlatformTransferSummaryPartnerType](#platformtransfersummarypartnertype) | Y |  |
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) |  |  |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |

### PlatformTransferSummaryPartnerType

파트너 유형

- `BUSINESS` — 사업자
- `WHT_PAYER` — 원천징수 대상자
- `NON_WHT_PAYER` — 원천징수 비대상자

### PlatformTransferSummaryPayment

PlatformTransferSummaryPayment

oneOf, discriminator `type`:

- `EXTERNAL` → [PlatformTransferSummaryExternalPayment](#platformtransfersummaryexternalpayment)
- `PORT_ONE` → [PlatformTransferSummaryPortOnePayment](#platformtransfersummaryportonepayment)

### PlatformTransferSummaryPortOnePayment

PlatformTransferSummaryPortOnePayment

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `orderName` | string | Y |  |
| `currency` | [Currency](schemas-common.md#currency) | Y |  |
| `methodType` | [PaymentMethodType](schemas-common.md#paymentmethodtype) |  |  |

### PlatformTransferType

PlatformTransferType

- `ORDER`
- `ORDER_CANCEL`
- `MANUAL`

### PlatformUserDefinedFormulaResults

PlatformUserDefinedFormulaResults

type: map<string, integer(int64)>

### PlatformUserDefinedPropertyKeyValue

사용자 정의 속성

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `key` | string | Y | 사용자 정의 속성 키 |
| `value` | [PlatformUserDefinedPropertyValue](#platformuserdefinedpropertyvalue) | Y | 사용자 정의 속성 값 |

### PlatformUserDefinedPropertyValue

PlatformUserDefinedPropertyValue

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `string` | string | Y |  |

### PlatformWithdrawalAccountTransfer

PlatformWithdrawalAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 계좌 이체 유형 |
| `id` | string | Y | 계좌 이체 아이디 |
| `bankAccountId` | string | Y | 출금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `bankAccount` | [PlatformBankAccountDetail](#platformbankaccountdetail) | Y | 출금 계좌 정보 |
| `sequenceNumber` | integer(int32) |  | 거래 일련번호 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `depositBank` | [Bank](schemas-common.md#bank) | Y | 이체 계좌 은행 |
| `depositAccountNumber` | string | Y | 이체 계좌 번호 |
| `depositAccountHolder` | string | Y | 예금주 |
| `amount` | integer(int64) | Y | 금액 |
| `withdrawalMemo` | string |  | 보내는 이 통장 메모 |
| `depositMemo` | string |  | 받는 이 통장 메모 |
| `balance` | integer(int64) |  | 잔액 |
| `failReason` | string |  | 실패 사유 |
| `tradedAt` | string(date-time) |  | 이체 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 수정 일시 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `withdrawalType` | [Type](#type) | Y | 출금 유형 |
| `partnerId` | string |  | 파트너 고유 아이디 |
| `partnerGraphqlId` | string |  |  |
| `bulkPayoutId` | string |  | 일괄 지급 고유 아이디 |
| `bulkPayoutGraphqlId` | string |  |  |
| `payoutId` | string |  | 지급 고유 아이디 |
| `payoutGraphqlId` | string |  |  |
| `bulkAccountTransferId` | string |  |  |
| `bulkAccountTransferGraphqlId` | string |  |  |
| `documentId` | string |  | 전자서명 아이디 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `status` | [PlatformAccountTransferStatus](#platformaccounttransferstatus) | Y | 상태 |
| `scheduledAt` | string(date-time) |  | 예정 일시 |

### RecoverPlatformAdditionalFeePolicyResponse

추가 수수료 정책 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 복원된 추가 수수료 정책 |

### RecoverPlatformContractResponse

계약 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](#platformcontract) | Y | 복원된 계약 |

### RecoverPlatformDiscountSharePolicyResponse

할인 분담 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 복원된 할인 분담 |

### RecoverPlatformPartnerResponse

파트너 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partner` | [PlatformPartner](#platformpartner) | Y | 복원된 파트너 |

### ReschedulePlatformAdditionalFeePolicyBody

추가 수수료 정책 예약 업데이트 재설정을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformAdditionalFeePolicyBody](#updateplatformadditionalfeepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### ReschedulePlatformAdditionalFeePolicyResponse

추가 수수료 정책 예약 업데이트 재설정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledAdditionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 예약된 추가 수수료 정책 |

### ReschedulePlatformContractBody

계약 예약 업데이트 재설정을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformContractBody](#updateplatformcontractbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### ReschedulePlatformContractResponse

계약 예약 업데이트 재설정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledContract` | [PlatformContract](#platformcontract) | Y | 예약된 계약 정보 |

### ReschedulePlatformDiscountSharePolicyBody

할인 분담 정책 예약 업데이트 재설정을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformDiscountSharePolicyBody](#updateplatformdiscountsharepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### ReschedulePlatformDiscountSharePolicyResponse

할인 분담 정책 예약 업데이트 재설정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledDiscountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 예약된 할인 분담 정보 |

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
| `scheduledPartner` | [PlatformPartner](#platformpartner) | Y | 예약된 파트너 정보 |

### SchedulePlatformAdditionalFeePolicyBody

추가 수수료 정책 업데이트 예약을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformAdditionalFeePolicyBody](#updateplatformadditionalfeepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### SchedulePlatformAdditionalFeePolicyResponse

추가 수수료 정책 업데이트 예약 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledAdditionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 예약된 추가 수수료 정책 |

### SchedulePlatformContractBody

계약 업데이트 예약을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformContractBody](#updateplatformcontractbody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### SchedulePlatformContractResponse

계약 업데이트 예약 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledContract` | [PlatformContract](#platformcontract) | Y | 예약된 계약 정보 |

### SchedulePlatformDiscountSharePolicyBody

할인 분담 정책 업데이트 예약을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `update` | [UpdatePlatformDiscountSharePolicyBody](#updateplatformdiscountsharepolicybody) | Y | 반영할 업데이트 내용 |
| `appliedAt` | string(date-time) | Y | 업데이트 적용 시점 |

### SchedulePlatformDiscountSharePolicyResponse

할인 분담 정책 업데이트 예약 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `scheduledDiscountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 예약된 할인 분담 정보 |

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
| `scheduledPartner` | [PlatformPartner](#platformpartner) | Y | 예약된 파트너 정보 |

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
| `userDefinedProperties` | [PlatformProperties](#platformproperties) |  |  |

### SchedulePlatformPartnersBodyUpdateAccount

파트너 계좌 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 정산에 사용할 통화 |
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
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) |  | 사업자 유형 |
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

### SettlementAmountType

SettlementAmountType

- `NET` — 순액(공급가액)
- `GROSS` — 총액(공급가액, 부가세)

### TransferParameters

TransferParameters

type: map<string, [PlatformSettlementParameterValue](#platformsettlementparametervalue)>

### Type

Type

- `PARTNER_PAYOUT`
- `REMIT`

### UpdatePlatformAdditionalFeePolicyBody

추가 수수료 정책 업데이트를 위한 입력 정보 — 추가 수수료 정책 업데이트를 위한 입력 정보<br>값이 명시하지 않은 필드는 업데이트되지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fee` | [PlatformFeeInput](#platformfeeinput) |  | 책정 수수료 |
| `name` | string |  | 추가 수수료 정책 이름 |
| `memo` | string |  | 해당 추가 수수료 정책에 대한 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) |  | 부가세를 부담할 주체 |

### UpdatePlatformAdditionalFeePolicyResponse

추가 수수료 정책 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](#platformadditionalfeepolicy) | Y | 업데이트된 추가 수수료 정책 |

### UpdatePlatformContractBody

계약 업데이트를 위한 입력 정보. 값이 명시되지 않은 필드는 업데이트되지 않습니다. — 계약 업데이트를 위한 입력 정보. 값이 명시되지 않은 필드는 업데이트되지 않습니다.<br>값이 명시되지 않은 필드는 업데이트되지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) |  | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) |  | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) |  | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean |  | 정산 시 결제금액 부가세 감액 여부 |

### UpdatePlatformContractResponse

계약 객체 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](#platformcontract) | Y | 업데이트된 계약 객체 |

### UpdatePlatformDiscountSharePolicyBody

할인 분담 정책 업데이트를 위한 입력 정보 — 할인 분담 정책 업데이트를 위한 입력 정보<br>값이 명시되지 않은 필드는 업데이트하지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 할인 분담 정책 이름 |
| `partnerShareRate` | integer(int32) |  | 할인 분담율 — 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 |

### UpdatePlatformDiscountSharePolicyResponse

할인 분담 정책 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](#platformdiscountsharepolicy) | Y | 업데이트된 할인 분담 정책 |

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
| `userDefinedProperties` | [PlatformProperties](#platformproperties) |  | 사용자 정의 속성 |

### UpdatePlatformPartnerBodyAccount

파트너 계좌 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 정산에 사용할 통화 |
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
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) |  | 사업자 유형 |
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
| `partner` | [PlatformPartner](#platformpartner) | Y | 업데이트된 파트너 |

### UpdatePlatformSettingBody

플랫폼 설정 업데이트를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `defaultWithdrawalMemo` | string |  | 기본 보내는 이 통장 메모 |
| `defaultDepositMemo` | string |  | 기본 받는 이 통장 메모 |
| `supportsMultipleOrderTransfersPerPartner` | boolean |  | paymentId, storeId, partnerId가 같은 주문 정산건에 대한 중복 정산 지원 여부 |
| `adjustSettlementDateAfterHolidayIfEarlier` | boolean |  | 정산일이 정산시작일보다 작거나 같을 경우 공휴일 후 영업일로 정산일 다시 계산 여부 |
| `deductWht` | boolean |  | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](#settlementamounttype) |  | 정산 금액 취급 기준 |
| `manualSettlementPerTransfer` | boolean |  | 수기정산을 정산 건별로 정산내역/지급내역으로 생성 |

### UpdatePlatformSettingResponse

플랫폼 설정 업데이트 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `setting` | [PlatformSetting](#platformsetting) | Y |  |
