# 현금 영수증 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /cash-receipts` — 현금영수증 다건 조회
- `POST /cash-receipts` — 현금 영수증 수동 발급
- `POST /payments/{paymentId}/cash-receipt/cancel` — 현금 영수증 취소
- `GET /payments/{paymentId}/cash-receipt` — 현금 영수증 단건 조회

## GET /cash-receipts

**현금영수증 다건 조회** (operationId: `getCashReceipts`)

주어진 조건에 맞는 현금영수증들을 페이지 기반으로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetCashReceiptsBody](#getcashreceiptsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [CashReceiptSortInput](#cashreceiptsortinput) |  | 정렬 조건 — 미 입력 시 sortBy: ISSUED_AT, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [CashReceiptFilterInput](#cashreceiptfilterinput) |  | 조회할 현금영수증 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 현금영수증 리스트와 페이지 정보가 반환됩니다.: [GetCashReceiptsResponse](#getcashreceiptsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /cash-receipts

**현금 영수증 수동 발급** (operationId: `issueCashReceipt`)

현금 영수증 발급을 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [IssueCashReceiptBody](#issuecashreceiptbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `paymentId` | string | Y | 결제 건 아이디 — 외부 결제 건에 대한 수동 발급의 경우, 아이디를 직접 채번하여 입력합니다. |
| `channelKey` | string | Y | 채널 키 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) | Y | 현금 영수증 유형 |
| `orderName` | string | Y | 주문명 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 금액 세부 입력 정보 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `customer` | [IssueCashReceiptCustomerInput](#issuecashreceiptcustomerinput) | Y | 고객 정보 |
| `paidAt` | string(date-time) |  | 결제 일자 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 — 웰컴페이먼츠의 경우에만 입력합니다. |
| `paymentMethod` | [IssueCashReceiptPaymentMethodType](#issuecashreceiptpaymentmethodtype) |  | 결제 수단 — 웰컴페이먼츠의 경우에만 입력합니다. |

### 응답

- **200** 성공 응답: [IssueCashReceiptResponse](#issuecashreceiptresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`ChannelNotFoundError`](../00-basics/errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **409**
  * [`CashReceiptAlreadyIssuedError`](../00-basics/errors.md#cashreceiptalreadyissuederror): 현금영수증이 이미 발급된 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/cash-receipt/cancel

**현금 영수증 취소** (operationId: `cancelCashReceiptByPaymentId`)

현금 영수증 취소를 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답: [CancelCashReceiptResponse](#cancelcashreceiptresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`CashReceiptNotIssuedError`](../00-basics/errors.md#cashreceiptnotissuederror): 현금영수증이 발급되지 않은 경우
  * [`CashReceiptNotFoundError`](../00-basics/errors.md#cashreceiptnotfounderror): 현금영수증이 존재하지 않는 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## GET /payments/{paymentId}/cash-receipt

**현금 영수증 단건 조회** (operationId: `getCashReceiptByPaymentId`)

주어진 결제 아이디에 대응되는 현금 영수증 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답으로 현금 영수증 객체를 반환합니다.: [CashReceipt](#cashreceipt)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`CashReceiptNotFoundError`](../00-basics/errors.md#cashreceiptnotfounderror): 현금영수증이 존재하지 않는 경우

---

# 스키마

### CancelCashReceiptResponse

현금 영수증 취소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cancelledAmount` | integer(int64) | Y | 취소 금액 |
| `cancelledAt` | string(date-time) | Y | 현금 영수증 취소 완료 시점 |

### CancelledCashReceipt

발급 취소

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 현금영수증 상태 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 현금영수증 발급에 사용된 채널 |
| `amount` | integer(int64) | Y | 결제 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `orderName` | string | Y | 주문명 |
| `isManual` | boolean | Y | 수동 발급 여부 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) |  | 현금영수증 유형 |
| `pgReceiptId` | string |  | PG사 현금영수증 아이디 |
| `issueNumber` | string | Y | 승인번호 |
| `url` | string |  | 현금영수증 URL |
| `issuedAt` | string(date-time) | Y | 발급 시점 |
| `cancelledAt` | string(date-time) | Y | 취소 시점 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 시점 |

### CashReceipt

현금영수증 내역

oneOf, discriminator `status`:

- `CANCELLED` → [CancelledCashReceipt](#cancelledcashreceipt)
- `ISSUE_FAILED` → [IssueFailedCashReceipt](#issuefailedcashreceipt)
- `ISSUED` → [IssuedCashReceipt](#issuedcashreceipt)

### CashReceiptFilterInput

현금영수증 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 현금영수증을 조회합니다. |
| `timeRangeField` | [CashReceiptTimeRangeField](#cashreceipttimerangefield) |  | 조회 기준 시점 유형 — 값을 입력하지 않으면 ISSUED_AT으로 설정됩니다. |
| `from` | string(date-time) |  | 조회 기준 시점 범위의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 조회 기준 시점 범위의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `paymentId` | string |  | 고객사 결제 아이디 |
| `isTest` | boolean |  | 테스트 결제 필터링 |
| `orderName` | string |  | 주문명 |
| `statuses` | [CashReceiptStatus](#cashreceiptstatus)[] |  | 현금영수증 발급 상태 리스트 — 값을 입력하지 않으면 필터링이 적용되지 않습니다. |
| `isManual` | boolean |  | 수동 발급 여부 |
| `pgReceiptId` | string |  | PG사 현금영수증 발급 번호 |
| `pgMerchantId` | string |  | PG 상점아이디 |
| `pgProviders` | [PgProvider](../00-basics/schemas-common.md#pgprovider)[] |  | PG사 결제 모듈 리스트 — 값을 입력하지 않으면 PG사 결제 모듈 필터링이 적용되지 않습니다. |
| `pgCompanies` | [PgCompany](../00-basics/schemas-common.md#pgcompany)[] |  | PG사 리스트 — 값을 입력하지 않으면 PG사 필터링이 적용되지 않습니다. |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) |  | 포트원 버전 |
| `types` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype)[] |  | 현금영수증 유형 리스트 — 값을 입력하지 않으면 필터링이 적용되지 않습니다. |

### CashReceiptSortBy

현금영수증 정렬 기준

- `ISSUED_AT` — 발급 시각
- `CANCELLED_AT` — 취소 시각
- `STATUS_UPDATED_AT` — 상태 변경 시각

### CashReceiptSortInput

현금영수증 다건 조회 시 정렬 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `by` | [CashReceiptSortBy](#cashreceiptsortby) |  | 정렬 기준 필드 — 어떤 필드를 기준으로 정렬할 지 결정합니다. 비워서 보낼 경우, ISSUED_AT이 기본값으로 설정됩니다. |
| `order` | [SortOrder](../00-basics/schemas-common.md#sortorder) |  | 정렬 순서 — 어떤 순서로 정렬할 지 결정합니다. 비워서 보낼 경우, DESC(내림차순)가 기본값으로 설정됩니다. |

### CashReceiptStatus

현금영수증 발급 건 상태

- `ISSUED`
- `CANCELLED`
- `FAILED`

### CashReceiptSummary

현금영수증 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `issueNumber` | string | Y | 발행 번호 |
| `url` | string | Y | 현금 영수증 URL |
| `pgReceiptId` | string | Y | PG사 현금영수증 아이디 |

### CashReceiptTimeRangeField

현금영수증 다건 조회 시, 시각 범위를 적용할 필드

- `ISSUED_AT` — 발급 시각
- `CANCELLED_AT` — 취소 시각
- `STATUS_UPDATED_AT` — 상태 변경 시각

### GetCashReceiptsBody

GetCashReceiptsBody — 현금영수증 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [CashReceiptSortInput](#cashreceiptsortinput) |  | 정렬 조건 — 미 입력 시 sortBy: ISSUED_AT, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [CashReceiptFilterInput](#cashreceiptfilterinput) |  | 조회할 현금영수증 조건 필터 |

### GetCashReceiptsResponse

현금영수증 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [CashReceipt](#cashreceipt)[] | Y | 조회된 현금영수증 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### IssueCashReceiptBody

현금영수증 발급 요청 양식

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `paymentId` | string | Y | 결제 건 아이디 — 외부 결제 건에 대한 수동 발급의 경우, 아이디를 직접 채번하여 입력합니다. |
| `channelKey` | string | Y | 채널 키 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) | Y | 현금 영수증 유형 |
| `orderName` | string | Y | 주문명 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 금액 세부 입력 정보 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `customer` | [IssueCashReceiptCustomerInput](#issuecashreceiptcustomerinput) | Y | 고객 정보 |
| `paidAt` | string(date-time) |  | 결제 일자 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 — 웰컴페이먼츠의 경우에만 입력합니다. |
| `paymentMethod` | [IssueCashReceiptPaymentMethodType](#issuecashreceiptpaymentmethodtype) |  | 결제 수단 — 웰컴페이먼츠의 경우에만 입력합니다. |

### IssueCashReceiptCustomerInput

현금영수증 발급 시 고객 관련 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `identityNumber` | string | Y | 고객 식별값 |
| `identityNumberType` | [IssueCashReceiptCustomerInputIdentityNumberType](../00-basics/schemas-common.md#issuecashreceiptcustomerinputidentitynumbertype) |  | 고객 식별값 유형 — 갤럭시아머니트리의 경우 필요합니다 |
| `name` | string |  | 이름 |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 전화번호 |

### IssueCashReceiptPaymentMethodType

현금영수증 발급 가능 결제 수단

- `TRANSFER` — 계좌이체
- `VIRTUAL_ACCOUNT` — 가상계좌

### IssueCashReceiptResponse

현금 영수증 발급 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cashReceipt` | [CashReceiptSummary](#cashreceiptsummary) | Y |  |

### IssueFailedCashReceipt

발급 실패

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 현금영수증 상태 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) |  | 현금영수증 발급에 사용된 채널 |
| `orderName` | string | Y | 주문명 |
| `isManual` | boolean | Y | 수동 발급 여부 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 시점 |

### IssuedCashReceipt

발급 완료

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 현금영수증 상태 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 현금영수증 발급에 사용된 채널 |
| `amount` | integer(int64) | Y | 결제 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `orderName` | string | Y | 주문명 |
| `isManual` | boolean | Y | 수동 발급 여부 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) |  | 현금영수증 유형 |
| `pgReceiptId` | string |  | PG사 현금영수증 아이디 |
| `issueNumber` | string | Y | 승인 번호 |
| `url` | string |  | 현금영수증 URL |
| `issuedAt` | string(date-time) | Y | 발급 시점 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 시점 |
