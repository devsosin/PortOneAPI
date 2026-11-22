# 빌링키 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /billing-keys` — 빌링키 다건 조회
- `POST /billing-keys` — 빌링키 발급
- `POST /billing-keys/confirm` — 빌링키 발급 수동 승인
- `POST /billing-keys/confirm-issue-and-pay` — 빌링키 발급 및 초회 결제 수동 승인
- `GET /billing-keys/{billingKey}` — 빌링키 단건 조회
- `DELETE /billing-keys/{billingKey}` — 빌링키 삭제

## GET /billing-keys

**빌링키 다건 조회** (operationId: `getBillingKeyInfos`)

주어진 조건에 맞는 빌링키들을 페이지 기반으로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetBillingKeyInfosBody](#getbillingkeyinfosbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [BillingKeySortInput](#billingkeysortinput) |  | 정렬 조건 — 미 입력 시 sortBy: TIME_TO_PAY, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [BillingKeyFilterInput](#billingkeyfilterinput) |  | 조회할 빌링키 조건 필터 — V1 빌링키 건의 경우 일부 필드에 대해 필터가 적용되지 않을 수 있습니다. |

### 응답

- **200** 성공 응답으로 조회된 빌링키 리스트와 페이지 정보가 반환됩니다.: [GetBillingKeyInfosResponse](#getbillingkeyinfosresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## POST /billing-keys

**빌링키 발급** (operationId: `issueBillingKey`)

빌링키 발급을 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [IssueBillingKeyBody](#issuebillingkeybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `method` | [InstantBillingKeyPaymentMethodInput](#instantbillingkeypaymentmethodinput) | Y | 빌링키 결제 수단 정보 |
| `channelKey` | string |  | 채널 키 — 채널 키 또는 채널 그룹 ID 필수 |
| `channelGroupId` | string |  | 채널 그룹 ID — 채널 키 또는 채널 그룹 ID 필수 |
| `customer` | [CustomerInput](schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `noticeUrls` | string[] |  | 웹훅 주소 — 빌링키 발급 시 요청을 받을 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급이 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### 응답

- **200** 성공 응답: [IssueBillingKeyResponse](#issuebillingkeyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`ChannelNotFoundError`](errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우
  * [`ChannelSpecificError`](errors.md#channelspecificerror): 여러 채널을 지정한 요청에서, 채널 각각에서 오류가 발생한 경우

## POST /billing-keys/confirm

**빌링키 발급 수동 승인** (operationId: `confirmBillingKey`)

수동 승인으로 설정된 빌링키 발급에 대해, 빌링키 발급을 완료 처리합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [ConfirmBillingKeyBody](#confirmbillingkeybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingIssueToken` | string | Y | 빌링키 발급 토큰 — 빌링키 발급 요청 완료 시 발급된 토큰입니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급이 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### 응답

- **200** 성공 응답으로 빌링키 발급 승인 결과를 반환합니다.: [ConfirmedBillingKeySummary](#confirmedbillingkeysummary)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
- **409**
  * [`BillingKeyAlreadyIssuedError`](errors.md#billingkeyalreadyissuederror)
  * [`InformationMismatchError`](errors.md#informationmismatcherror): 정보가 일치하지 않는 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /billing-keys/confirm-issue-and-pay

**빌링키 발급 및 초회 결제 수동 승인** (operationId: `confirmBillingKeyIssueAndPay`)

수동 승인으로 설정된 빌링키 발급 및 초회 결제에 대해, 빌링키 발급과 결제를 완료 처리합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [ConfirmBillingKeyIssueAndPayBody](#confirmbillingkeyissueandpaybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingIssueToken` | string | Y | 빌링키 발급 토큰 — 빌링키 발급 및 초회 결제 요청 완료 시 발급된 토큰입니다. |
| `paymentId` | string |  | 결제 건 아이디 — 검증용 파라미터로, 결제 건 아이디와 일치하지 않을 경우 오류가 반환됩니다. |
| `currency` | [Currency](schemas-common.md#currency) |  | 통화 — 검증용 파라미터로, 결제 건 화폐와 일치하지 않을 경우 오류가 반환됩니다. |
| `totalAmount` | integer(int64) |  | 결제 금액 — 검증용 파라미터로, 결제 건 총 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `taxFreeAmount` | integer(int64) |  | 면세 금액 — 검증용 파라미터로, 결제 건 면세 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급 및 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### 응답

- **200** 성공 응답으로 빌링키 발급 및 초회 결제 승인 결과를 반환합니다.: [ConfirmedBillingKeyIssueAndPaySummary](#confirmedbillingkeyissueandpaysummary)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
- **409**
  * [`BillingKeyAlreadyIssuedError`](errors.md#billingkeyalreadyissuederror)
  * [`InformationMismatchError`](errors.md#informationmismatcherror): 정보가 일치하지 않는 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## GET /billing-keys/{billingKey}

**빌링키 단건 조회** (operationId: `getBillingKeyInfo`)

주어진 빌링키에 대응되는 빌링키 정보를 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `billingKey` | path | string | Y | 조회할 빌링키 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답으로 빌링키 정보를 반환합니다.: [BillingKeyInfo](#billingkeyinfo)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우

## DELETE /billing-keys/{billingKey}

**빌링키 삭제** (operationId: `deleteBillingKey`)

빌링키를 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `billingKey` | path | string | Y | 삭제할 빌링키 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `reason` | query | string |  | 네이버페이: 자동결제 해지 사유입니다. 명시가 필요합니다. |
| `requester` | query | [BillingKeyDeleteRequester](#billingkeydeleterequester) |  | 네이버페이: 자동결제 해지 요청 주체입니다. 명시가 필요합니다. |
| `skipWebhook` | query | boolean |  | 빌링키 삭제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### 응답

- **200** 성공 응답: [DeleteBillingKeyResponse](#deletebillingkeyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotIssuedError`](errors.md#billingkeynotissuederror)
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
- **409**
  * [`BillingKeyAlreadyDeletedError`](errors.md#billingkeyalreadydeletederror): 빌링키가 이미 삭제된 경우
  * [`PaymentScheduleAlreadyExistsError`](errors.md#paymentschedulealreadyexistserror): 결제 예약건이 이미 존재하는 경우
- **502**
  * [`PgProviderError`](errors.md#pgprovidererror): PG사에서 오류를 전달한 경우
  * [`ChannelSpecificError`](errors.md#channelspecificerror): 여러 채널을 지정한 요청에서, 채널 각각에서 오류가 발생한 경우

---

# 스키마

### BillingKeyDeleteRequester

빌링키 삭제 요청 주체

- `CUSTOMER` — 구매자
- `ADMIN` — 관리자

### BillingKeyFailure

발급 실패 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `message` | string |  | 실패 사유 |
| `pgCode` | string |  | PG사 실패 코드 |
| `pgMessage` | string |  | PG사 실패 사유 |
| `failedAt` | string(date-time) | Y | 실패 시점 |

### BillingKeyFilterInput

빌링키 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 빌링키를 조회합니다. |
| `timeRangeField` | [BillingKeyTimeRangeField](#billingkeytimerangefield) |  | 조회 기준 시점 유형 |
| `from` | string(date-time) |  | 조회 기준 시점 범위의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 조회 기준 시점 범위의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `status` | [BillingKeyStatus](#billingkeystatus)[] |  | 빌링키 상태 리스트 — 값을 입력하지 않으면 빌링키 상태 필터링이 적용되지 않습니다. |
| `channelGroupIds` | string[] |  | 채널 그룹 아이디 리스트 — 값을 입력하지 않으면 스마트 라우팅 그룹 아이디 필터링이 적용되지 않습니다. |
| `customerId` | string |  | 고객 ID |
| `platformType` | [PaymentClientType](schemas-common.md#paymentclienttype) |  | 플랫폼 유형 |
| `textSearch` | [BillingKeyTextSearch](#billingkeytextsearch) |  | 통합 검색 필터 |
| `pgProviders` | [PgProvider](schemas-common.md#pgprovider)[] |  | PG사 결제 모듈 리스트 — 값을 입력하지 않으면 PG사 결제 모듈 필터링이 적용되지 않습니다. |
| `pgCompanies` | [PgCompany](schemas-common.md#pgcompany)[] |  | PG사 리스트 — 값을 입력하지 않으면 PG사 필터링이 적용되지 않습니다. |
| `methods` | [BillingKeyPaymentMethodType](#billingkeypaymentmethodtype)[] |  | 결제수단 리스트 — 값을 입력하지 않으면 결제수단 필터링이 적용되지 않습니다. |
| `version` | [PortOneVersion](schemas-common.md#portoneversion) |  | 포트원 버전 |

### BillingKeyInfo

빌링키 정보

oneOf, discriminator `status`:

- `DELETED` → [DeletedBillingKeyInfo](#deletedbillingkeyinfo)
- `ISSUED` → [IssuedBillingKeyInfo](#issuedbillingkeyinfo)

### BillingKeyInfoSummary

BillingKeyInfoSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `billingKey` | string | Y | 발급된 빌링키 |
| `channels` | [SelectedChannel](schemas-common.md#selectedchannel)[] |  | 발급된 채널 |
| `issuedAt` | string(date-time) | Y | 빌링키 발급 완료 시점 |

### BillingKeyPaymentMethod

빌링키 발급 수단 정보

oneOf, discriminator `type`:

- `BillingKeyPaymentMethodCard` → [BillingKeyPaymentMethodCard](#billingkeypaymentmethodcard)
- `BillingKeyPaymentMethodEasyPay` → [BillingKeyPaymentMethodEasyPay](#billingkeypaymentmethodeasypay)
- `BillingKeyPaymentMethodMobile` → [BillingKeyPaymentMethodMobile](#billingkeypaymentmethodmobile)
- `BillingKeyPaymentMethodPaypal` → [BillingKeyPaymentMethodPaypal](#billingkeypaymentmethodpaypal)
- `BillingKeyPaymentMethodTransfer` → [BillingKeyPaymentMethodTransfer](#billingkeypaymentmethodtransfer)

### BillingKeyPaymentMethodCard

카드 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `card` | [Card](schemas-common.md#card) |  | 카드 상세 정보 |

### BillingKeyPaymentMethodEasyPay

간편 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `provider` | [EasyPayProvider](schemas-common.md#easypayprovider) |  | 간편 결제 PG사 |
| `method` | [BillingKeyPaymentMethodEasyPayMethod](#billingkeypaymentmethodeasypaymethod) |  | 간편 결제 수단 |

### BillingKeyPaymentMethodEasyPayCharge

충전식 포인트 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### BillingKeyPaymentMethodEasyPayMethod

간편 결제 수단

oneOf, discriminator `type`:

- `BillingKeyPaymentMethodCard` → [BillingKeyPaymentMethodCard](#billingkeypaymentmethodcard)
- `BillingKeyPaymentMethodEasyPayCharge` → [BillingKeyPaymentMethodEasyPayCharge](#billingkeypaymentmethodeasypaycharge)
- `BillingKeyPaymentMethodTransfer` → [BillingKeyPaymentMethodTransfer](#billingkeypaymentmethodtransfer)

### BillingKeyPaymentMethodMobile

모바일 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `phoneNumber` | string |  | 전화번호 |

### BillingKeyPaymentMethodPaypal

페이팔 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### BillingKeyPaymentMethodTransfer

계좌이체 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `bank` | [Bank](schemas-common.md#bank) |  | 표준 은행 코드 |
| `accountNumber` | string |  | 계좌번호 |

### BillingKeyPaymentMethodType

빌링키 결제 수단

- `CARD` — 카드
- `MOBILE` — 모바일
- `EASY_PAY` — 간편 결제
- `TRANSFER` — 계좌 이체

### BillingKeySortBy

빌링키 정렬 기준

- `REQUESTED_AT` — 발급 요청 시각
- `ISSUED_AT` — 발급 완료 시각
- `DELETED_AT` — 삭제 완료 시각
- `STATUS_TIMESTAMP` — 상태 변경 시각

### BillingKeySortInput

빌링키 다건 조회 시 정렬 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `by` | [BillingKeySortBy](#billingkeysortby) |  | 정렬 기준 필드 — 어떤 필드를 기준으로 정렬할 지 결정합니다. 비워서 보낼 경우, REQUESTED_AT이 기본값으로 설정됩니다. |
| `order` | [SortOrder](schemas-common.md#sortorder) |  | 정렬 순서 — 어떤 순서로 정렬할 지 결정합니다. 비워서 보낼 경우, DESC(내림차순)가 기본값으로 설정됩니다. |

### BillingKeyStatus

빌링키 상태

- `ISSUED`
- `DELETED`

### BillingKeyTextSearch

통합검색 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `field` | [BillingKeyTextSearchField](#billingkeytextsearchfield) | Y |  |
| `value` | string | Y |  |

### BillingKeyTextSearchField

통합검색 항목

- `CARD_BIN`
- `CARD_NUMBER`
- `PG_MERCHANT_ID`
- `CUSTOMER_NAME`
- `CUSTOMER_EMAIL`
- `CUSTOMER_PHONE_NUMBER`
- `CUSTOMER_ADDRESS`
- `CUSTOMER_ZIPCODE`
- `USER_AGENT`
- `BILLING_KEY`
- `CHANNEL_GROUP_NAME`

### BillingKeyTimeRangeField

빌링키 다건 조회 시, 시각 범위를 적용할 필드

- `REQUESTED_AT` — 발급 요청 시각
- `ISSUED_AT` — 발급 완료 시각
- `DELETED_AT` — 삭제 완료 시각
- `STATUS_TIMESTAMP` — 상태 변경 시각

### ConfirmBillingKeyBody

빌링키 발급 승인 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingIssueToken` | string | Y | 빌링키 발급 토큰 — 빌링키 발급 요청 완료 시 발급된 토큰입니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급이 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### ConfirmBillingKeyIssueAndPayBody

빌링키 발급 및 초회 결제 승인 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingIssueToken` | string | Y | 빌링키 발급 토큰 — 빌링키 발급 및 초회 결제 요청 완료 시 발급된 토큰입니다. |
| `paymentId` | string |  | 결제 건 아이디 — 검증용 파라미터로, 결제 건 아이디와 일치하지 않을 경우 오류가 반환됩니다. |
| `currency` | [Currency](schemas-common.md#currency) |  | 통화 — 검증용 파라미터로, 결제 건 화폐와 일치하지 않을 경우 오류가 반환됩니다. |
| `totalAmount` | integer(int64) |  | 결제 금액 — 검증용 파라미터로, 결제 건 총 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `taxFreeAmount` | integer(int64) |  | 면세 금액 — 검증용 파라미터로, 결제 건 면세 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급 및 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### ConfirmedBillingKeyIssueAndPaySummary

빌링키 발급 및 초회 결제 수동 승인 완료 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `billingKey` | string | Y | 빌링키 |
| `paymentId` | string | Y | 결제 건 아이디 |

### ConfirmedBillingKeySummary

빌링키 발급 수동 승인 완료 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `billingKey` | string | Y | 빌링키 |

### DeleteBillingKeyResponse

빌링키 삭제 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `deletedAt` | string(date-time) | Y | 빌링키 삭제 완료 시점 |

### DeletedBillingKeyInfo

빌링키 삭제 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 빌링키 상태 |
| `billingKey` | string | Y | 빌링키 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `methods` | [BillingKeyPaymentMethod](#billingkeypaymentmethod)[] |  | 빌링키 결제수단 상세 정보 — 추후 슈퍼빌링키 기능 제공 시 여러 결제수단 정보가 담길 수 있습니다. |
| `channels` | [SelectedChannel](schemas-common.md#selectedchannel)[] | Y | 빌링키 발급 시 사용된 채널 — 추후 슈퍼빌링키 기능 제공 시 여러 채널 정보가 담길 수 있습니다. |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `issueId` | string |  | 고객사가 채번하는 빌링키 발급 건 고유 아이디 |
| `issueName` | string |  | 빌링키 발급 건 이름 |
| `requestedAt` | string(date-time) |  | 발급 요청 시점 |
| `issuedAt` | string(date-time) | Y | 발급 시점 |
| `channelGroup` | [ChannelGroupSummary](schemas-common.md#channelgroupsummary) |  | 채널 그룹 |
| `pgBillingKeyIssueResponses` | [PgBillingKeyIssueResponse](#pgbillingkeyissueresponse)[] |  | 채널 별 빌링키 발급 응답 — 슈퍼빌링키의 경우, 빌링키 발급이 성공하더라도 일부 채널에 대한 발급은 실패할 수 있습니다. |
| `deletedAt` | string(date-time) | Y | 발급 삭제 시점 |

### FailedPgBillingKeyIssueResponse

빌링키 발급 실패 채널 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `channel` | [SelectedChannel](schemas-common.md#selectedchannel) | Y | 채널 — 빌링키 발급을 시도한 채널입니다. |
| `failure` | [BillingKeyFailure](#billingkeyfailure) | Y | 발급 실패 상세 정보 |

### GetBillingKeyInfosBody

GetBillingKeyInfosBody — 빌링키 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [BillingKeySortInput](#billingkeysortinput) |  | 정렬 조건 — 미 입력 시 sortBy: TIME_TO_PAY, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [BillingKeyFilterInput](#billingkeyfilterinput) |  | 조회할 빌링키 조건 필터 — V1 빌링키 건의 경우 일부 필드에 대해 필터가 적용되지 않을 수 있습니다. |

### GetBillingKeyInfosResponse

빌링키 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [BillingKeyInfo](#billingkeyinfo)[] | Y | 조회된 빌링키 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### InstantBillingKeyPaymentMethodInput

빌링키 발급 시 결제 수단 입력 양식 — 빌링키 발급 시 결제 수단 입력 양식<br>`card`를 반드시 입력해 주세요.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `card` | [InstantBillingKeyPaymentMethodInputCard](#instantbillingkeypaymentmethodinputcard) |  |  |

### InstantBillingKeyPaymentMethodInputCard

카드 수단 정보 입력 양식

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `credential` | [CardCredential](schemas-common.md#cardcredential) | Y |  |

### IssueBillingKeyBody

빌링키 발급 요청 양식

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `method` | [InstantBillingKeyPaymentMethodInput](#instantbillingkeypaymentmethodinput) | Y | 빌링키 결제 수단 정보 |
| `channelKey` | string |  | 채널 키 — 채널 키 또는 채널 그룹 ID 필수 |
| `channelGroupId` | string |  | 채널 그룹 ID — 채널 키 또는 채널 그룹 ID 필수 |
| `customer` | [CustomerInput](schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `noticeUrls` | string[] |  | 웹훅 주소 — 빌링키 발급 시 요청을 받을 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 빌링키 발급이 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### IssueBillingKeyResponse

빌링키 발급 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `billingKeyInfo` | [BillingKeyInfoSummary](#billingkeyinfosummary) | Y | 빌링키 정보 |
| `channelSpecificFailures` | [ChannelSpecificFailure](errors.md#channelspecificfailure)[] |  | 발급에 실패한 채널이 있을시 실패 정보 |

### IssuedBillingKeyInfo

빌링키 발급 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 빌링키 상태 |
| `billingKey` | string | Y | 빌링키 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `methods` | [BillingKeyPaymentMethod](#billingkeypaymentmethod)[] |  | 빌링키 결제수단 상세 정보 — 추후 슈퍼빌링키 기능 제공 시 여러 결제수단 정보가 담길 수 있습니다. |
| `channels` | [SelectedChannel](schemas-common.md#selectedchannel)[] | Y | 빌링키 발급 시 사용된 채널 — 추후 슈퍼빌링키 기능 제공 시 여러 채널 정보가 담길 수 있습니다. |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `issueId` | string |  | 고객사가 채번하는 빌링키 발급 건 고유 아이디 |
| `issueName` | string |  | 빌링키 발급 건 이름 |
| `requestedAt` | string(date-time) |  | 발급 요청 시점 |
| `issuedAt` | string(date-time) | Y | 발급 시점 |
| `channelGroup` | [ChannelGroupSummary](schemas-common.md#channelgroupsummary) |  | 채널 그룹 |
| `pgBillingKeyIssueResponses` | [PgBillingKeyIssueResponse](#pgbillingkeyissueresponse)[] |  | 채널 별 빌링키 발급 응답 — 슈퍼빌링키의 경우, 빌링키 발급이 성공하더라도 일부 채널에 대한 빌링키 발급은 실패할 수 있습니다. |

### IssuedPgBillingKeyIssueResponse

빌링키 발급 성공 채널 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `channel` | [SelectedChannel](schemas-common.md#selectedchannel) | Y | 채널 — 빌링키 발급을 시도한 채널입니다. |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `method` | [BillingKeyPaymentMethod](#billingkeypaymentmethod) |  | 빌링키 결제수단 상세 정보 — 채널에 대응되는 PG사에서 응답한 빌링키 발급 수단 정보입니다. |

### PgBillingKeyIssueResponse

채널 별 빌링키 발급 응답

oneOf, discriminator `type`:

- `FAILED` → [FailedPgBillingKeyIssueResponse](#failedpgbillingkeyissueresponse)
- `ISSUED` → [IssuedPgBillingKeyIssueResponse](#issuedpgbillingkeyissueresponse)
