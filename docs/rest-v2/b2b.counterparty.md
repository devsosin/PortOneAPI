# 거래처 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /b2b/counterparties/{brn}/certificate/registration-url` — 사업자 인증서 등록 URL 조회
- `POST /b2b/counterparties/{brn}/certificate/validate` — 사업자 인증서 유효성 검증
- `GET /b2b/counterparties/{brn}/certificate` — 인증서 조회
- `GET /b2b/counterparties/{counterpartyId}` — 거래처 조회
- `DELETE /b2b/counterparties/{counterpartyId}` — 거래처 삭제
- `PATCH /b2b/counterparties/{counterpartyId}` — 거래처 정보 수정
- `GET /b2b/counterparties` — 거래처 검색
- `POST /b2b/counterparties` — 거래처 생성

## GET /b2b/counterparties/{brn}/certificate/registration-url

**사업자 인증서 등록 URL 조회** (operationId: `getB2bCounterpartyCertificateRegistrationUrl`)

연동 사업자의 인증서를 등록하기 위한 URL을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `brn` | path | string | Y | 사업자등록번호 |

### 응답

- **200** 성공 응답으로 URL을 반환합니다.: [GetB2bCounterpartyCertificateRegistrationUrlResponse](#getb2bcounterpartycertificateregistrationurlresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/counterparties/{brn}/certificate/validate

**사업자 인증서 유효성 검증** (operationId: `validateB2bCounterpartyCertificate`)

연동 사업자가 등록한 인증서의 유효성을 검증합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `brn` | path | string | Y | 사업자등록번호 |

### 응답

- **200** 성공 응답으로 유효성 검증 결과를 반환합니다.: [ValidateB2bCounterpartyCertificateResponse](#validateb2bcounterpartycertificateresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bCertificateUnregisteredError`: 인증서가 등록되어 있지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/counterparties/{brn}/certificate

**인증서 조회** (operationId: `getB2bCounterpartyCertificate`)

연동 사업자의 인증서를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `brn` | path | string | Y | 사업자등록번호 |

### 응답

- **200** 성공 응답으로 인증서 객체를 반환합니다.: [B2bCertificate](#b2bcertificate)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bCertificateUnregisteredError`: 인증서가 등록되어 있지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/counterparties/{counterpartyId}

**거래처 조회** (operationId: `getB2bCounterparty`)

거래처를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `counterpartyId` | path | string | Y | 거래처 ID |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** 성공 응답으로 거래처 객체를 반환합니다.: [B2bCounterparty](#b2bcounterparty)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## DELETE /b2b/counterparties/{counterpartyId}

**거래처 삭제** (operationId: `deleteB2bCounterparty`)

거래처를 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `counterpartyId` | path | string | Y | 거래처 ID |

### 요청 본문 (application/json)

타입: [DeleteB2bCounterpartyBody](#deleteb2bcounterpartybody)

### 응답

- **200** 성공 응답: [DeleteB2bCounterpartyResponse](#deleteb2bcounterpartyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * `B2bCounterpartyPartnerNotDeletableError`: 파트너 연동 거래처는 삭제할 수 없는 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우
- **409** * `B2bCounterpartyOngoingTaxInvoiceExistsError`: 진행 중인 세금계산서가 존재하여 거래처를 삭제할 수 없는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## PATCH /b2b/counterparties/{counterpartyId}

**거래처 정보 수정** (operationId: `updateB2bCounterparty`)

거래처 정보를 수정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `counterpartyId` | path | string | Y | 거래처 ID |

### 요청 본문 (application/json, 필수)

타입: [UpdateB2bCounterpartyBody](#updateb2bcounterpartybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterparty` | [B2bCounterpartyInput](#b2bcounterpartyinput) | Y | 거래처 정보 |
| `options` | [B2bCounterpartyCreateOptions](#b2bcounterpartycreateoptions) |  | 확인 옵션 — 사업자 정보 및 휴폐업 상태 조회 옵션입니다. |

### 응답

- **200** 성공 응답: [UpdateB2bCounterpartyResponse](#updateb2bcounterpartyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bCounterpartyBrnModificationNotAllowedError`: 사업자등록번호 수정이 허용되지 않는 경우
  * `B2bCounterpartyTooManyAdditionalContactsError`: 추가 담당자가 너무 많은 경우
  * `B2bCounterpartyMissingRequiredFieldsError`: 필수 입력 항목이 누락된 경우
  * `B2bCounterpartyVerificationBrnMismatchError`: 검증 결과의 사업자등록번호가 일치하지 않는 경우
  * `B2bCounterpartyVerificationTypeMismatchError`: 검증 유형이 일치하지 않는 경우
  * `B2bCounterpartyVerificationInvalidError`: 검증 결과가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * `B2bCounterpartyPartnerNotUpdatableError`: 파트너 연동 거래처는 수정할 수 없는 경우
- **404** * `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bCounterpartyVerificationNotFoundError`: 검증 결과를 찾을 수 없는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/counterparties

**거래처 검색** (operationId: `getB2bCounterparties`)

거래처를 검색합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetB2bCounterpartiesBody](#getb2bcounterpartiesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 페이지 정보 |
| `filter` | [B2bCounterpartyFilter](#b2bcounterpartyfilter) |  | 검색 필터 |

### 응답

- **200** : [GetB2bCounterpartiesResponse](#getb2bcounterpartiesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/counterparties

**거래처 생성** (operationId: `createB2bCounterparty`)

거래처를 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [CreateB2bCounterpartyBody](#createb2bcounterpartybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterpartyId` | string |  | 거래처 아이디 — 입력하지 않으면 임의의 ID가 채번됩니다. |
| `counterparty` | [B2bCounterpartyInput](#b2bcounterpartyinput) | Y | 거래처 정보 |
| `options` | [B2bCounterpartyCreateOptions](#b2bcounterpartycreateoptions) |  | 거래처 생성 옵션 |

### 응답

- **200** 성공 응답: [CreateB2bCounterpartyResponse](#createb2bcounterpartyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bCounterpartyBrnInvalidError`: 사업자등록번호가 유효하지 않은 경우
  * `B2bCounterpartySelfOriginBrnMismatchError`: 자사 사업자등록번호와 동일한 거래처를 생성할 수 없는 경우
  * `B2bCounterpartyTooManyAdditionalContactsError`: 추가 담당자가 너무 많은 경우
  * `B2bCounterpartyMissingRequiredFieldsError`: 필수 입력 항목이 누락된 경우
  * `B2bCounterpartyVerificationBrnMismatchError`: 검증 결과의 사업자등록번호가 일치하지 않는 경우
  * `B2bCounterpartyVerificationTypeMismatchError`: 검증 유형이 일치하지 않는 경우
  * `B2bCounterpartyVerificationInvalidError`: 검증 결과가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
  * `B2bCounterpartyPartnerNotConnectableError`: 파트너 연동 거래처는 국세청 연동이 허용되지 않는 경우
- **404** * `B2bCounterpartyVerificationNotFoundError`: 검증 결과를 찾을 수 없는 경우
- **409** * `B2bCounterpartyIdAlreadyExistsError`: 거래처 ID가 이미 사용중인 경우<br>* `B2bCounterpartyIdAlreadyExistsByPartnerError`: 파트너 연동으로 생성된 거래처 ID가 이미 사용중인 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우<br>* `B2bCounterpartyNtsConnectionFailedError`: 국세청 연동에 실패한 경우

---

# 스키마

### B2bCertificate

B2bCertificate

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `registeredAt` | string(date-time) | Y | 등록일시 |
| `expiredAt` | string(date-time) | Y | 만료일시 |
| `issuerName` | string | Y | 발행자명 |
| `subjectName` | string | Y | 본인명 |
| `certificateType` | [B2bCertificateType](#b2bcertificatetype) | Y | 인증서 타입 |
| `oid` | string | Y | OID |
| `registrantContactName` | string | Y | 등록 담당자 성명 |
| `registrantContactId` | string | Y | 등록 담당자 ID |

### B2bCertificateType

인증서 타입

- `E_TAX` — 전자세금용 공동인증서
- `PORTONE` — 특수목적용 공동인증서
- `ETC` — 기타

### B2bCounterparty

거래처 — 거래처<br>B2B 거래처 정보입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 거래처 고유 아이디 |
| `graphqlId` | string | Y |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `brn` | string | Y | 사업자등록번호 — `-` 없이 숫자로만 구성됩니다. |
| `companyName` | string | Y | 상호명 |
| `representativeName` | string | Y | 대표자 성명 |
| `address` | string |  | 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `contact` | [B2bCounterpartyContact](#b2bcounterpartycontact) | Y | 담당자 정보 |
| `additionalContacts` | [B2bCounterpartyContact](#b2bcounterpartycontact)[] | Y | 추가 담당자 목록 — 최대 5명까지 등록할 수 있습니다. |
| `memo` | string |  | 메모 |
| `ntsConnectionStatus` | [B2bNtsConnectionStatus](#b2bntsconnectionstatus) | Y | 국세청 연동 상태 |
| `ntsConnectedAt` | string(date-time) |  | 국세청 연동 시각 |
| `ntsConnectionFailedReason` | string |  | 국세청 연동 실패 사유 |
| `partnerId` | string |  | 파트너 연동 ID — 파트너 연동 거래처인 경우에만 존재합니다. |
| `businessStatus` | [B2bCounterpartyBusinessStatus](#b2bcounterpartybusinessstatus) |  | 휴폐업 상태 |
| `businessStatusCheckedAt` | string(date-time) |  | 휴폐업 상태 확인 시각 |
| `businessStatusVerification` | [B2bCounterpartyVerification](#b2bcounterpartyverification) |  | 휴폐업 상태 검증 정보 |
| `businessInfoVerification` | [B2bCounterpartyVerification](#b2bcounterpartyverification) |  | 사업자 정보 검증 정보 |
| `appliedAt` | string(date-time) |  | 적용 시각 |

### B2bCounterpartyBusinessStatus

거래처 휴폐업 상태

- `UNKNOWN` — 미조회
- `IN_BUSINESS` — 영업중
- `CLOSED` — 폐업
- `SUSPENDED` — 휴업
- `NOT_FOUND` — 사업체 미등록
- `CHECK_PENDING` — 조회 대기
- `CHECK_FAILED` — 조회 실패

### B2bCounterpartyContact

거래처 담당자 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 담당자 성명 |
| `phoneNumber` | string |  | 담당자 전화번호 |
| `email` | string | Y | 담당자 이메일 |
| `memo` | string |  | 담당자 메모 |

### B2bCounterpartyContactInput

거래처 담당자 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 담당자 성명 |
| `phoneNumber` | string |  | 담당자 전화번호 |
| `email` | string | Y | 담당자 이메일 |
| `memo` | string |  | 담당자 메모 |

### B2bCounterpartyCreateOptions

거래처 생성 옵션

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `checkBusinessInfo` | boolean |  | 사업자 정보 조회 여부 — true인 경우 사업자 정보를 조회하여 거래처에 반영합니다. |
| `checkBusinessStatus` | boolean |  | 휴폐업 상태 조회 여부 — true인 경우 휴폐업 상태를 조회하여 거래처에 반영합니다. |
| `businessInfoVerificationId` | string |  | 사업자 정보 조회 결과 ID — 이전에 조회한 사업자 정보 조회 결과의 ID를 입력하면 재조회 없이 해당 결과를 사용합니다. |
| `businessStatusVerificationId` | string |  | 휴폐업 상태 조회 결과 ID — 이전에 조회한 휴폐업 상태 조회 결과의 ID를 입력하면 재조회 없이 해당 결과를 사용합니다. |

### B2bCounterpartyFilter

거래처 검색 필터

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 거래처 ID — prefix 검색 |
| `brn` | string |  | 사업자등록번호 |
| `companyName` | string |  | 거래처명 — 포함 검색 |
| `representativeName` | string |  | 대표자명 |
| `contactName` | string |  | 담당자 이름 |
| `contactPhone` | string |  | 담당자 전화번호 |
| `contactEmail` | string |  | 담당자 이메일 |
| `businessStatuses` | [B2bCounterpartyBusinessStatus](#b2bcounterpartybusinessstatus)[] |  | 휴폐업 상태 |
| `ntsConnectionStatuses` | [B2bNtsConnectionStatus](#b2bntsconnectionstatus)[] |  | 국세청 연동 상태 |
| `counterpartyIds` | string[] |  | 거래처 ID 목록 — 특정 ID 목록으로 필터링 |

### B2bCounterpartyInput

거래처 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string | Y | 사업자등록번호 — `-` 없이 숫자로만 구성됩니다. |
| `name` | string |  | 거래처명 |
| `representativeName` | string |  | 대표자 성명 |
| `address` | string |  | 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `contact` | [B2bCounterpartyContactInput](#b2bcounterpartycontactinput) |  | 담당자 정보 |
| `additionalContacts` | [B2bCounterpartyContactInput](#b2bcounterpartycontactinput)[] |  | 추가 담당자 목록 — 최대 5명까지 등록할 수 있습니다. |
| `memo` | string |  | 메모 |

### B2bCounterpartyVerification

거래처 검증 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 외부 API 사용 ID |
| `checkedAt` | string(date-time) | Y | 검증 시각 |

### B2bNtsConnectionStatus

국세청 연동 상태

- `NOT_CONNECTED` — 연동 안 됨
- `PENDING_CONNECT` — 연동 대기
- `CONNECTED` — 연동 됨
- `PENDING_DISCONNECT` — 연동 해제 대기
- `ERROR` — 연동 오류

### CreateB2bCounterpartyBody

거래처 생성 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterpartyId` | string |  | 거래처 아이디 — 입력하지 않으면 임의의 ID가 채번됩니다. |
| `counterparty` | [B2bCounterpartyInput](#b2bcounterpartyinput) | Y | 거래처 정보 |
| `options` | [B2bCounterpartyCreateOptions](#b2bcounterpartycreateoptions) |  | 거래처 생성 옵션 |

### CreateB2bCounterpartyResponse

거래처 생성 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterparty` | [B2bCounterparty](#b2bcounterparty) | Y | 거래처 정보 |

### DeleteB2bCounterpartyBody

거래처 삭제 요청

type: object

### DeleteB2bCounterpartyResponse

거래처 삭제 응답

type: object

### GetB2bCounterpartiesBody

거래처 검색 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 페이지 정보 |
| `filter` | [B2bCounterpartyFilter](#b2bcounterpartyfilter) |  | 검색 필터 |

### GetB2bCounterpartiesResponse

거래처 검색 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 페이지 정보 |
| `items` | [B2bCounterparty](#b2bcounterparty)[] | Y | 거래처 목록 |

### GetB2bCounterpartyCertificateRegistrationUrlResponse

인증서 등록 URL 조회 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `url` | string | Y | 인증서 등록 URL |

### UpdateB2bCounterpartyBody

거래처 정보 수정 요청

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterparty` | [B2bCounterpartyInput](#b2bcounterpartyinput) | Y | 거래처 정보 |
| `options` | [B2bCounterpartyCreateOptions](#b2bcounterpartycreateoptions) |  | 확인 옵션 — 사업자 정보 및 휴폐업 상태 조회 옵션입니다. |

### UpdateB2bCounterpartyResponse

거래처 정보 수정 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `counterparty` | [B2bCounterparty](#b2bcounterparty) | Y | 거래처 정보 |

### ValidateB2bCounterpartyCertificateResponse

인증서 유효성 검증 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isValid` | boolean | Y | 인증서 유효 여부 |
