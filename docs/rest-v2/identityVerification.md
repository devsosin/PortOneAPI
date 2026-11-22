# 본인인증 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `POST /identity-verifications/{identityVerificationId}/confirm` — 본인인증 확인
- `POST /identity-verifications/{identityVerificationId}/resend` — SMS 본인인증 요청 재전송
- `POST /identity-verifications/{identityVerificationId}/send` — 본인인증 요청 전송
- `GET /identity-verifications/{identityVerificationId}` — 본인인증 단건 조회
- `GET /identity-verifications` — 본인인증 내역 다건 조회

## POST /identity-verifications/{identityVerificationId}/confirm

**본인인증 확인** (operationId: `confirmIdentityVerification`)

요청된 본인인증에 대한 확인을 진행합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `identityVerificationId` | path | string | Y | 본인인증 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ConfirmIdentityVerificationBody](#confirmidentityverificationbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `otp` | string |  | OTP (One-Time Password) — SMS 방식에서만 사용됩니다. |

### 응답

- **200** 성공 응답: [ConfirmIdentityVerificationResponse](#confirmidentityverificationresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`IdentityVerificationNotFoundError`](errors.md#identityverificationnotfounderror): 요청된 본인인증 건이 존재하지 않는 경우
  * [`IdentityVerificationNotSentError`](errors.md#identityverificationnotsenterror): 본인인증 건이 API로 요청된 상태가 아닌 경우
- **409**
  * [`IdentityVerificationAlreadyVerifiedError`](errors.md#identityverificationalreadyverifiederror): 본인인증 건이 이미 인증 완료된 상태인 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /identity-verifications/{identityVerificationId}/resend

**SMS 본인인증 요청 재전송** (operationId: `resendIdentityVerification`)

SMS 본인인증 요청을 재전송합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `identityVerificationId` | path | string | Y | 본인인증 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답: [ResendIdentityVerificationResponse](#resendidentityverificationresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`IdentityVerificationNotFoundError`](errors.md#identityverificationnotfounderror): 요청된 본인인증 건이 존재하지 않는 경우
  * [`IdentityVerificationNotSentError`](errors.md#identityverificationnotsenterror): 본인인증 건이 API로 요청된 상태가 아닌 경우
- **409**
  * [`IdentityVerificationAlreadyVerifiedError`](errors.md#identityverificationalreadyverifiederror): 본인인증 건이 이미 인증 완료된 상태인 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /identity-verifications/{identityVerificationId}/send

**본인인증 요청 전송** (operationId: `sendIdentityVerification`)

SMS 또는 APP 방식을 이용하여 본인인증 요청을 전송합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `identityVerificationId` | path | string | Y | 본인인증 아이디 |

### 요청 본문 (application/json, 필수)

타입: [SendIdentityVerificationBody](#sendidentityverificationbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `channelKey` | string | Y | 채널 키 |
| `customer` | [SendIdentityVerificationBodyCustomer](#sendidentityverificationbodycustomer) | Y | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `operator` | [IdentityVerificationOperator](#identityverificationoperator) | Y | 통신사 |
| `method` | [IdentityVerificationMethod](#identityverificationmethod) | Y | 본인인증 방식 |

### 응답

- **200** 성공 응답: [SendIdentityVerificationResponse](#sendidentityverificationresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`MaxTransactionCountReachedError`](errors.md#maxtransactioncountreachederror): 결제 혹은 본인인증 시도 횟수가 최대에 도달한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`IdentityVerificationNotFoundError`](errors.md#identityverificationnotfounderror): 요청된 본인인증 건이 존재하지 않는 경우
  * [`ChannelNotFoundError`](errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **409**
  * [`IdentityVerificationAlreadyVerifiedError`](errors.md#identityverificationalreadyverifiederror): 본인인증 건이 이미 인증 완료된 상태인 경우
  * [`IdentityVerificationAlreadySentError`](errors.md#identityverificationalreadysenterror): 본인인증 건이 이미 API로 요청된 상태인 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## GET /identity-verifications/{identityVerificationId}

**본인인증 단건 조회** (operationId: `getIdentityVerification`)

주어진 아이디에 대응되는 본인인증 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `identityVerificationId` | path | string | Y | 조회할 본인인증 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답으로 본인 인증 객체를 반환합니다.: [IdentityVerification](#identityverification)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`IdentityVerificationNotFoundError`](errors.md#identityverificationnotfounderror): 요청된 본인인증 건이 존재하지 않는 경우

## GET /identity-verifications

**본인인증 내역 다건 조회** (operationId: `getIdentityVerifications`)

주어진 조건에 맞는 본인인증 내역들을 페이지 기반으로 조회합니다. 이용시 tech.support@portone.io에 이용 목적과 기간을 보내 주세요.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetIdentityVerificationsBody](#getidentityverificationsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [IdentityVerificationSortInput](#identityverificationsortinput) |  | 정렬 조건 — 미 입력 시 sortBy: REQUESTED_AT, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [IdentityVerificationFilterInput](#identityverificationfilterinput) |  | 조회할 본인인증 내역 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 본인인증 내역 리스트와 페이지 정보가 반환됩니다.: [GetIdentityVerificationsResponse](#getidentityverificationsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

---

# 스키마

### Carrier

통신사

- `SKT` — SKT
- `KT` — KT
- `LGU` — LG 유플러스
- `SKT_MVNO` — SKT 알뜰폰
- `KT_MVNO` — KT 알뜰폰
- `LGU_MVNO` — LGU 알뜰폰

### ConfirmIdentityVerificationBody

본인인증 확인을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `otp` | string |  | OTP (One-Time Password) — SMS 방식에서만 사용됩니다. |

### ConfirmIdentityVerificationResponse

본인인증 확인 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `identityVerification` | [VerifiedIdentityVerification](#verifiedidentityverification) | Y | 완료된 본인인증 내역 |

### FailedIdentityVerification

실패한 본인인증 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 본인인증 상태 |
| `id` | string | Y | 고객사 본인인증 번호 |
| `channel` | [SelectedChannel](schemas-common.md#selectedchannel) |  | 사용된 본인인증 채널 |
| `requestedCustomer` | [IdentityVerificationRequestedCustomer](#identityverificationrequestedcustomer) | Y | 요청 시 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `requestedAt` | string(date-time) | Y | 본인인증 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `failure` | [IdentityVerificationFailure](#identityverificationfailure) | Y | 본인인증 실패 정보 |
| `version` | [PortOneVersion](schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `pgTxId` | string |  | 본인인증 내역 PG사 아이디 |

### GetIdentityVerificationsBody

GetIdentityVerificationsBody — 본인인증 내역 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [IdentityVerificationSortInput](#identityverificationsortinput) |  | 정렬 조건 — 미 입력 시 sortBy: REQUESTED_AT, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [IdentityVerificationFilterInput](#identityverificationfilterinput) |  | 조회할 본인인증 내역 조건 필터 |

### GetIdentityVerificationsResponse

본인인증 내역 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [IdentityVerification](#identityverification)[] | Y | 조회된 본인인증 내역 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### IdentityVerification

본인인증 내역

oneOf, discriminator `status`:

- `FAILED` → [FailedIdentityVerification](#failedidentityverification)
- `READY` → [ReadyIdentityVerification](#readyidentityverification)
- `VERIFIED` → [VerifiedIdentityVerification](#verifiedidentityverification)

### IdentityVerificationFailure

본인인증 실패 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `reason` | string |  | 실패 사유 |
| `pgCode` | string |  | PG사 실패 코드 |
| `pgMessage` | string |  | PG사 실패 메시지 |

### IdentityVerificationFilterCustomerInput

본인인증 다건 조회를 위한 고객 정보 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 이름 |
| `birthYear` | string |  | 출생 연도 |
| `birthMonth` | string |  | 출생월 |
| `birthDay` | string |  | 출생일 |
| `phoneNumber` | string |  | 전화번호 — 특수 문자(-) 없이 숫자로만 이루어진 번호 형식입니다. |
| `gender` | [Gender](schemas-common.md#gender) |  | 성별 |

### IdentityVerificationFilterInput

본인인증 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 본인인증 내역을 조회합니다. |
| `timeRangeField` | [IdentityVerificationTimeRangeField](#identityverificationtimerangefield) |  | 조회 기준 시점 유형 — 값을 입력하지 않으면 REQUESTED_AT으로 설정됩니다. |
| `from` | string(date-time) |  | 조회 기준 시점 범위의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 조회 기준 시점 범위의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `identityVerificationId` | string |  | 고객사 본인인증 번호 — V1 본인인증 건의 경우 `imp_uid`에 대응됩니다. |
| `identityVerificationTxId` | string |  | 포트원 본인인증 시도 번호 — V1 본인인증 건의 경우 `imp_uid`에 대응됩니다. |
| `isTest` | boolean |  | 테스트 결제 필터링 |
| `statuses` | [IdentityVerificationStatus](#identityverificationstatus)[] |  | 본인인증 상태 리스트 — 값을 입력하지 않으면 필터링이 적용되지 않습니다. |
| `pgTxId` | string |  | PG사 본인인증 번호 |
| `pgMerchantId` | string |  | PG 상점아이디 |
| `pgProviders` | [PgProvider](schemas-common.md#pgprovider)[] |  | PG사 결제 모듈 리스트 — 값을 입력하지 않으면 PG사 결제 모듈 필터링이 적용되지 않습니다. |
| `pgCompanies` | [PgCompany](schemas-common.md#pgcompany)[] |  | PG사 리스트 — 값을 입력하지 않으면 PG사 필터링이 적용되지 않습니다. |
| `carriers` | [Carrier](#carrier)[] |  | 통신사 리스트 |
| `version` | [PortOneVersion](schemas-common.md#portoneversion) |  | 포트원 버전 |
| `customer` | [IdentityVerificationFilterCustomerInput](#identityverificationfiltercustomerinput) |  | 고객 정보 — 인증이 완료되지 않은 본인인증 내역의 경우 요청 시 고객 정보로, 인증이 완료된 본인인증 내역의 경우 인증된 고객 정보로 필터링합니다. |

### IdentityVerificationMethod

본인인증 방식

- `SMS`
- `APP`

### IdentityVerificationOperator

본인인증 통신사

- `SKT` — SKT
- `KT` — KT
- `LGU` — LGU
- `SKT_MVNO` — SKT 알뜰폰
- `KT_MVNO` — KT 알뜰폰
- `LGU_MVNO` — LGU 알뜰폰

### IdentityVerificationRequestedCustomer

요청 시 고객 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 식별 아이디 |
| `name` | string |  | 이름 |
| `phoneNumber` | string |  | 전화번호 — 특수 문자(-) 없이 숫자로만 이루어진 번호 형식입니다. |
| `birthYear` | string |  | 출생연도 |
| `birthMonth` | string |  | 출생월 |
| `birthDay` | string |  | 출생일 |

### IdentityVerificationSortBy

본인인증 내역 정렬 기준

- `REQUESTED_AT` — 요청 시각
- `VERIFIED_AT` — 완료 시각
- `FAILED_AT` — 실패 시각
- `STATUS_UPDATED_AT` — 상태 변경 시각

### IdentityVerificationSortInput

본인인증 내역 다건 조회 시 정렬 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `by` | [IdentityVerificationSortBy](#identityverificationsortby) |  | 정렬 기준 필드 — 어떤 필드를 기준으로 정렬할 지 결정합니다. 비워서 보낼 경우, REQUESTED_AT이 기본값으로 설정됩니다. |
| `order` | [SortOrder](schemas-common.md#sortorder) |  | 정렬 순서 — 어떤 순서로 정렬할 지 결정합니다. 비워서 보낼 경우, DESC(내림차순)가 기본값으로 설정됩니다. |

### IdentityVerificationStatus

본인인증 상태

- `READY` — 요청 상태
- `VERIFIED` — 완료 상태
- `FAILED` — 실패 상태

### IdentityVerificationTimeRangeField

본인인증 다건 조회 시, 시각 범위를 적용할 필드

- `REQUESTED_AT` — 요청 시각
- `VERIFIED_AT` — 완료 시각
- `FAILED_AT` — 실패 시각
- `STATUS_UPDATED_AT` — 상태 변경 시각

### IdentityVerificationVerifiedCustomer

인증된 고객 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 식별 아이디 |
| `name` | string | Y | 이름 |
| `operator` | [IdentityVerificationOperator](#identityverificationoperator) |  | 통신사 — 다날: 별도 계약이 필요합니다.<br>KG이니시스: 제공하지 않습니다. |
| `phoneNumber` | string |  | 전화번호 — 특수 문자(-) 없이 숫자로만 이루어진 번호 형식입니다.<br>다날: 별도 계약이 필요합니다.<br>KG이니시스: 항상 제공합니다. |
| `birthDate` | string(date) |  | 생년월일 (yyyy-MM-dd) — 포트원 V2 본인인증 건의 경우 항상 존재합니다. |
| `gender` | [Gender](schemas-common.md#gender) |  | 성별 — 다날: 항상 제공합니다.<br>KG이니시스: 항상 제공합니다. |
| `isForeigner` | boolean |  | 외국인 여부 — 다날: 별도 계약이 필요합니다.<br>KG이니시스: 항상 제공합니다. |
| `ci` | string |  | CI (개인 고유 식별키) — 개인을 식별하기 위한 고유 정보입니다.<br>다날: 항상 제공합니다.<br>KG이니시스: 카카오를 제외한 인증사에서 제공합니다. |
| `di` | string |  | DI (사이트별 개인 고유 식별키) — 중복 가입을 방지하기 위해 개인을 식별하는 사이트별 고유 정보입니다.<br>다날: 항상 제공합니다.<br>KG이니시스: 제공하지 않습니다. |

### ReadyIdentityVerification

준비 상태의 본인인증 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 본인인증 상태 |
| `id` | string | Y | 고객사 본인인증 번호 |
| `channel` | [SelectedChannel](schemas-common.md#selectedchannel) |  | 사용된 본인인증 채널 |
| `requestedCustomer` | [IdentityVerificationRequestedCustomer](#identityverificationrequestedcustomer) | Y | 요청 시 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `requestedAt` | string(date-time) | Y | 본인인증 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `version` | [PortOneVersion](schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `pgTxId` | string |  | 본인인증 내역 PG사 아이디 |

### ResendIdentityVerificationResponse

본인인증 요청 재전송 성공 응답

type: object

### SendIdentityVerificationBody

본인인증 요청을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `channelKey` | string | Y | 채널 키 |
| `customer` | [SendIdentityVerificationBodyCustomer](#sendidentityverificationbodycustomer) | Y | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `operator` | [IdentityVerificationOperator](#identityverificationoperator) | Y | 통신사 |
| `method` | [IdentityVerificationMethod](#identityverificationmethod) | Y | 본인인증 방식 |

### SendIdentityVerificationBodyCustomer

본인인증 요청을 위한 고객 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 식별 아이디 |
| `name` | string | Y | 이름 |
| `phoneNumber` | string | Y | 전화번호 — 특수 문자(-) 없이 숫자만 입력합니다. |
| `identityNumber` | string |  | 주민등록번호 앞 7자리 — SMS 방식의 경우 필수로 입력합니다. |
| `ipAddress` | string | Y | IP 주소 — 고객의 요청 속도 제한에 사용됩니다. |

### SendIdentityVerificationResponse

본인인증 요청 전송 성공 응답

type: object

### VerifiedIdentityVerification

완료된 본인인증 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 본인인증 상태 |
| `id` | string | Y | 고객사 본인인증 번호 |
| `channel` | [SelectedChannel](schemas-common.md#selectedchannel) |  | 사용된 본인인증 채널 |
| `verifiedCustomer` | [IdentityVerificationVerifiedCustomer](#identityverificationverifiedcustomer) | Y | 인증된 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `requestedAt` | string(date-time) | Y | 본인인증 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `verifiedAt` | string(date-time) | Y | 본인인증 완료 시점 |
| `pgTxId` | string | Y | 본인인증 내역 PG사 아이디 |
| `pgRawResponse` | string | Y | PG사 응답 데이터 |
| `version` | [PortOneVersion](schemas-common.md#portoneversion) | Y | 포트원 버전 |
