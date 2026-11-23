# 파트너 정산: 정책·계약

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `POST /platform/additional-fee-policies/{id}/archive` — 추가 수수료 정책 보관
- `POST /platform/additional-fee-policies/{id}/recover` — 추가 수수료 정책 복원
- `GET /platform/additional-fee-policies/{id}/schedule` — 
- `PUT /platform/additional-fee-policies/{id}/schedule` — 
- `POST /platform/additional-fee-policies/{id}/schedule` — 
- `DELETE /platform/additional-fee-policies/{id}/schedule` — 
- `GET /platform/additional-fee-policies/{id}` — 추가 수수료 정책 조회
- `PATCH /platform/additional-fee-policies/{id}` — 추가 수수료 정책 수정
- `GET /platform/additional-fee-policies` — 추가 수수료 정책 다건 조회
- `POST /platform/additional-fee-policies` — 추가 수수료 정책 생성
- `POST /platform/contracts/{id}/archive` — 계약 보관
- `POST /platform/contracts/{id}/recover` — 계약 복원
- `GET /platform/contracts/{id}/schedule` — 
- `PUT /platform/contracts/{id}/schedule` — 
- `POST /platform/contracts/{id}/schedule` — 
- `DELETE /platform/contracts/{id}/schedule` — 
- `GET /platform/contracts/{id}` — 계약 조회
- `PATCH /platform/contracts/{id}` — 계약 수정
- `GET /platform/contracts` — 계약 다건 조회
- `POST /platform/contracts` — 계약 생성
- `POST /platform/discount-share-policies/{id}/archive` — 할인 분담 정책 보관
- `POST /platform/discount-share-policies/{id}/recover` — 할인 분담 정책 복원
- `GET /platform/discount-share-policies/{id}/schedule` — 
- `PUT /platform/discount-share-policies/{id}/schedule` — 
- `POST /platform/discount-share-policies/{id}/schedule` — 
- `DELETE /platform/discount-share-policies/{id}/schedule` — 
- `GET /platform/discount-share-policies/{id}` — 할인 분담 정책 조회
- `PATCH /platform/discount-share-policies/{id}` — 할인 분담 정책 수정
- `GET /platform/discount-share-policies` — 할인 분담 정책 다건 조회
- `POST /platform/discount-share-policies` — 할인 분담 정책 생성
- `GET /platform/discount-share-policy-filter-options` — 

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledAdditionalFeePolicyError`](../00-basics/errors.md#platformcannotarchivescheduledadditionalfeepolicyerror): 예약된 업데이트가 있는 추가 수수료 정책을 보관하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)

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

- **200** 성공 응답으로 예약된 추가 수수료 정책을 반환합니다.: [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformAdditionalFeePolicyScheduleAlreadyExistsError`](../00-basics/errors.md#platformadditionalfeepolicyschedulealreadyexistserror)
  * [`PlatformArchivedAdditionalFeePolicyError`](../00-basics/errors.md#platformarchivedadditionalfeepolicyerror): 보관된 추가 수수료 정책을 업데이트하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)

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

- **200** 성공 응답으로 추가 수수료 정책을 반환합니다.: [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)

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
| `vatPayer` | [PlatformPayer](schemas.md#platformpayer) |  | 부가세를 부담할 주체 |

### 응답

- **200** 성공 응답으로 업데이트된 추가 수수료 정책이 반환됩니다.: [UpdatePlatformAdditionalFeePolicyResponse](#updateplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformAdditionalFeePolicyNotFoundError`](../00-basics/errors.md#platformadditionalfeepolicynotfounderror)
- **409**
  * [`PlatformArchivedAdditionalFeePolicyError`](../00-basics/errors.md#platformarchivedadditionalfeepolicyerror): 보관된 추가 수수료 정책을 업데이트하려고 하는 경우

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
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformAdditionalFeePolicyFilterInput](#platformadditionalfeepolicyfilterinput) |  | 조회할 추가 수수료 정책 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 추가 수수료 정책 리스트와 페이지 정보를 반환합니다.: [GetPlatformAdditionalFeePoliciesResponse](#getplatformadditionalfeepoliciesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

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
| `vatPayer` | [PlatformPayer](schemas.md#platformpayer) | Y | 부가세 부담 주체 |

### 응답

- **200** 성공 응답: [CreatePlatformAdditionalFeePolicyResponse](#createplatformadditionalfeepolicyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformAdditionalFeePolicyAlreadyExistsError`](../00-basics/errors.md#platformadditionalfeepolicyalreadyexistserror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledContractError`](../00-basics/errors.md#platformcannotarchivescheduledcontracterror): 예약된 업데이트가 있는 계약을 보관하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)

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

- **200** 성공 응답으로 예약된 계약 객체를 반환합니다.: [PlatformContract](schemas.md#platformcontract)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformContractScheduleAlreadyExistsError`](../00-basics/errors.md#platformcontractschedulealreadyexistserror)
  * [`PlatformArchivedContractError`](../00-basics/errors.md#platformarchivedcontracterror): 보관된 계약을 업데이트하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)

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

- **200** 성공 응답으로 계약 객체를 반환합니다.: [PlatformContract](schemas.md#platformcontract)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)

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
| `platformFeeVatPayer` | [PlatformPayer](schemas.md#platformpayer) |  | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean |  | 정산 시 결제금액 부가세 감액 여부 |

### 응답

- **200** 성공 응답으로 업데이트된 계약 객체가 반환됩니다.: [UpdatePlatformContractResponse](#updateplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
- **409**
  * [`PlatformArchivedContractError`](../00-basics/errors.md#platformarchivedcontracterror): 보관된 계약을 업데이트하려고 하는 경우

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
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformContractFilterInput](#platformcontractfilterinput) |  | 조회할 계약 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 계약 리스트와 페이지 정보를 반환합니다.: [GetPlatformContractsResponse](#getplatformcontractsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

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
| `platformFeeVatPayer` | [PlatformPayer](schemas.md#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 |

### 응답

- **200** 성공 응답으로 생성된 계약 객체가 반환됩니다.: [CreatePlatformContractResponse](#createplatformcontractresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformContractAlreadyExistsError`](../00-basics/errors.md#platformcontractalreadyexistserror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformCannotArchiveScheduledDiscountSharePolicyError`](../00-basics/errors.md#platformcannotarchivescheduleddiscountsharepolicyerror): 예약된 업데이트가 있는 할인 분담 정책을 보관하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)

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

- **200** 성공 응답으로 예약된 할인 분담 정책을 반환합니다.: [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformDiscountSharePolicyScheduleAlreadyExistsError`](../00-basics/errors.md#platformdiscountsharepolicyschedulealreadyexistserror)
  * [`PlatformArchivedDiscountSharePolicyError`](../00-basics/errors.md#platformarchiveddiscountsharepolicyerror): 보관된 할인 분담 정책을 업데이트하려고 하는 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)

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

- **200** 성공 응답으로 할인 분담 정책을 반환합니다.: [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformdiscountsharepolicynotfounderror)
- **409**
  * [`PlatformArchivedDiscountSharePolicyError`](../00-basics/errors.md#platformarchiveddiscountsharepolicyerror): 보관된 할인 분담 정책을 업데이트하려고 하는 경우

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
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformDiscountSharePolicyFilterInput](#platformdiscountsharepolicyfilterinput) |  | 조회할 할인 분담 정책 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 할인 분담 정책 리스트와 페이지 정보가 반환됩니다.: [GetPlatformDiscountSharePoliciesResponse](#getplatformdiscountsharepoliciesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`PlatformDiscountSharePolicyAlreadyExistsError`](../00-basics/errors.md#platformdiscountsharepolicyalreadyexistserror)

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
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

---

# 스키마

### ArchivePlatformAdditionalFeePolicyResponse

추가 수수료 정책 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 보관된 추가 수수료 정책 |

### ArchivePlatformContractResponse

계약 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 보관된 계약 |

### ArchivePlatformDiscountSharePolicyResponse

할인 분담 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 보관된 할인 분담 |

### CancelPlatformAdditionalFeePolicyScheduleResponse

추가 수수료 정책 예약 업데이트 취소 성공 응답

type: object

### CancelPlatformContractScheduleResponse

계약 예약 업데이트 취소 성공 응답

type: object

### CancelPlatformDiscountSharePolicyScheduleResponse

할인 분담 정책 예약 업데이트 취소 성공 응답

type: object

### CreatePlatformAdditionalFeePolicyBody

추가 수수료 정책 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 생성할 추가 수수료 정책 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `name` | string | Y | 이름 |
| `fee` | [PlatformFeeInput](#platformfeeinput) | Y | 수수료 정보 |
| `memo` | string |  | 메모 |
| `vatPayer` | [PlatformPayer](schemas.md#platformpayer) | Y | 부가세 부담 주체 |

### CreatePlatformAdditionalFeePolicyResponse

플랫폼 생성 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 생성된 추가 수수료 정책 |

### CreatePlatformContractBody

계약 객체 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 계약에 부여할 고유 아이디 — 명시하지 않는 경우 포트원이 임의의 아이디를 발급해드립니다. |
| `name` | string | Y | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) | Y | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) | Y | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](schemas.md#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 |

### CreatePlatformContractResponse

계약 객체 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 생성된 계약 객체 |

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
| `discountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 생성된 할인 분담 정책 |

### GetPlatformAdditionalFeePoliciesBody

추가 수수료 정책 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformAdditionalFeePolicyFilterInput](#platformadditionalfeepolicyfilterinput) |  | 조회할 추가 수수료 정책 조건 필터 |

### GetPlatformAdditionalFeePoliciesResponse

추가 수수료 정책 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy)[] | Y | 조회된 추가 수수료 정책 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformContractsBody

계약 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformContractFilterInput](#platformcontractfilterinput) |  | 조회할 계약 조건 필터 |

### GetPlatformContractsResponse

계약 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformContract](schemas.md#platformcontract)[] | Y | 조회된 계약 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### GetPlatformDiscountSharePoliciesBody

할인 분담 정책 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformDiscountSharePolicyFilterInput](#platformdiscountsharepolicyfilterinput) |  | 조회할 할인 분담 정책 조건 필터 |

### GetPlatformDiscountSharePoliciesResponse

할인 분담 정책 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy)[] | Y | 조회된 할인 분담 정책 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### PlatformAdditionalFeePolicyFilterInput

추가 수수료 정책 다건 조회를 위한 필터 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 추가 수수료 정책의 필터 옵션을 조회하고, false 이면 보관되지 않은 추가 수수료 정책의 필터 옵션을 조회합니다. 기본값은 false 입니다. |
| `vatPayers` | [PlatformPayer](schemas.md#platformpayer)[] |  | 금액 부담 주체 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 부가세 부담 주체에 해당하는 추가 수수료 정책만 조회합니다. |
| `keyword` | [PlatformAdditionalFeePolicyFilterInputKeyword](#platformadditionalfeepolicyfilterinputkeyword) |  | 검색 키워드 |

### PlatformAdditionalFeePolicyFilterInputKeyword

검색 키워드 입력 정보 — 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 추가 수수료 정책만 조회합니다. 하위 필드는 명시된 값 중 한 가지만 적용됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 해당 값이 포함된 name 을 가진 추가 수수료 정책만 조회합니다. |
| `id` | string |  | 해당 값이 포함된 id 를 가진 추가 수수료 정책만 조회합니다. |
| `fee` | string |  | 해당 값과 같은 수수료 를 가진 추가 수수료 정책만 조회합니다. |

### PlatformContractFilterInput

계약 다건 조회를 위한 필터 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `platformFeePayers` | [PlatformPayer](schemas.md#platformpayer)[] |  | 금액 부담 주체 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 수수료 부담 주체를 가진 계약만 조회합니다. |
| `cycleTypes` | [PlatformSettlementCycleType](#platformsettlementcycletype)[] |  | 플랫폼 정산 주기 계산 방식 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 주기 계산 방식을 가진 계약만 조회합니다. |
| `datePolicies` | [PlatformSettlementCycleDatePolicy](schemas.md#platformsettlementcycledatepolicy)[] |  | 플랫폼 정산 기준일 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 기준일을 가진 계약만 조회합니다. |
| `isArchived` | boolean |  | 보관 조회 여부 — true 이면 보관된 계약을 조회하고, false 이면 보관되지 않은 계약을 조회합니다. 기본값은 false 입니다. |
| `keyword` | [PlatformContractFilterInputKeyword](#platformcontractfilterinputkeyword) |  | 검색 키워드 |

### PlatformContractFilterInputKeyword

검색 키워드 입력 정보 — 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 계약만 조회합니다. 하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 해당 값이 포함된 id 를 가진 계약만 조회합니다. |
| `name` | string |  | 해당 값이 포함된 name 을 가진 계약만 조회합니다. |

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

### PlatformFeeInput

수수료 계산 방식을 특정하기 위한 입력 정보 — 수수료 계산 방식을 특정하기 위한 입력 정보<br>정률 수수료를 설정하고 싶은 경우 `fixedRate` 필드에, 정액 수수료를 설정하고 싶은 경우 `fixedAmount` 필드에 값을 명시해 요청합니다.<br>두 필드 모두 값이 들어있지 않은 경우 요청이 거절됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fixedRate` | integer(int32) |  | 정률 수수료 |
| `fixedAmount` | integer(int64) |  | 정액 수수료 |

### PlatformSettlementCycleInput

플랫폼 정산 주기 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `lagDays` | integer(int32) | Y | 지체일 (d+n 의 n) — 정산시작일(통상 주문완료일)로부터 더해진 다음 날짜로부터 가장 가까운 날에 정산이 됩니다. 최소 1 에서 최대 10 까지 지정할 수 있습니다. |
| `datePolicy` | [PlatformSettlementCycleDatePolicy](schemas.md#platformsettlementcycledatepolicy) | Y | 기준일로, 정산일 계산 시 공휴일을 고려하기 위한 정보입니다. |
| `method` | [PlatformSettlementCycleMethodInput](#platformsettlementcyclemethodinput) | Y | 정산 주기 계산 방식 |

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

### PlatformSettlementCycleMethodManualDatesInput

PlatformSettlementCycleMethodManualDatesInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dates` | [MonthDay](schemas.md#monthday)[] | Y | 월 및 일자 정보 |

### PlatformSettlementCycleMethodMonthlyInput

PlatformSettlementCycleMethodMonthlyInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `daysOfMonth` | integer(int32)[] | Y |  |

### PlatformSettlementCycleMethodWeeklyInput

PlatformSettlementCycleMethodWeeklyInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `daysOfWeek` | [DayOfWeek](schemas.md#dayofweek)[] | Y | 요일 |

### PlatformSettlementCycleType

플랫폼 정산 주기 계산 방식

- `DAILY` — 매일 정산
- `WEEKLY` — 매주 정해진 요일에 정산
- `MONTHLY` — 매월 정해진 날(일)에 정산
- `MANUAL_DATES` — 정해진 날짜(월, 일)에 정산

### RecoverPlatformAdditionalFeePolicyResponse

추가 수수료 정책 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 복원된 추가 수수료 정책 |

### RecoverPlatformContractResponse

계약 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 복원된 계약 |

### RecoverPlatformDiscountSharePolicyResponse

할인 분담 복원 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 복원된 할인 분담 |

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
| `scheduledAdditionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 예약된 추가 수수료 정책 |

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
| `scheduledContract` | [PlatformContract](schemas.md#platformcontract) | Y | 예약된 계약 정보 |

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
| `scheduledDiscountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 예약된 할인 분담 정보 |

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
| `scheduledAdditionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 예약된 추가 수수료 정책 |

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
| `scheduledContract` | [PlatformContract](schemas.md#platformcontract) | Y | 예약된 계약 정보 |

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
| `scheduledDiscountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 예약된 할인 분담 정보 |

### UpdatePlatformAdditionalFeePolicyBody

추가 수수료 정책 업데이트를 위한 입력 정보 — 추가 수수료 정책 업데이트를 위한 입력 정보<br>값이 명시하지 않은 필드는 업데이트되지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fee` | [PlatformFeeInput](#platformfeeinput) |  | 책정 수수료 |
| `name` | string |  | 추가 수수료 정책 이름 |
| `memo` | string |  | 해당 추가 수수료 정책에 대한 메모 |
| `vatPayer` | [PlatformPayer](schemas.md#platformpayer) |  | 부가세를 부담할 주체 |

### UpdatePlatformAdditionalFeePolicyResponse

추가 수수료 정책 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `additionalFeePolicy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 업데이트된 추가 수수료 정책 |

### UpdatePlatformContractBody

계약 업데이트를 위한 입력 정보. 값이 명시되지 않은 필드는 업데이트되지 않습니다. — 계약 업데이트를 위한 입력 정보. 값이 명시되지 않은 필드는 업데이트되지 않습니다.<br>값이 명시되지 않은 필드는 업데이트되지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFeeInput](#platformfeeinput) |  | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycleInput](#platformsettlementcycleinput) |  | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](schemas.md#platformpayer) |  | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean |  | 정산 시 결제금액 부가세 감액 여부 |

### UpdatePlatformContractResponse

계약 객체 업데이트 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 업데이트된 계약 객체 |

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
| `discountSharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 업데이트된 할인 분담 정책 |
