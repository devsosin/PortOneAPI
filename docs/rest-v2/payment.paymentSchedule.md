# 결제 예약 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /payment-schedules/{paymentScheduleId}` — 결제 예약 단건 조회
- `GET /payment-schedules` — 결제 예약 다건 조회
- `DELETE /payment-schedules` — 결제 예약 취소
- `POST /payments/{paymentId}/schedule` — 결제 예약

## GET /payment-schedules/{paymentScheduleId}

**결제 예약 단건 조회** (operationId: `getPaymentSchedule`)

주어진 아이디에 대응되는 결제 예약 건을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentScheduleId` | path | string | Y | 조회할 결제 예약 건 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답으로 결제 예약 건 객체를 반환합니다.: [PaymentSchedule](#paymentschedule)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentScheduleNotFoundError`](errors.md#paymentschedulenotfounderror): 결제 예약건이 존재하지 않는 경우

## GET /payment-schedules

**결제 예약 다건 조회** (operationId: `getPaymentSchedules`)

주어진 조건에 맞는 결제 예약 건들을 조회합니다.
`filter.from`, `filter.until` 파라미터의 기본값이 결제 시점 기준 지난 90일에 속하는 건을 조회하도록 되어 있으니, 미래 예약 상태의 건을 조회하기 위해서는 해당 파라미터를 직접 설정해 주셔야 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPaymentSchedulesBody](#getpaymentschedulesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [PaymentScheduleSortInput](#paymentschedulesortinput) |  | 정렬 조건 — 미 입력 시 sortBy: TIME_TO_PAY, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [PaymentScheduleFilterInput](#paymentschedulefilterinput) |  | 조회할 결제 예약 건의 조건 필터 |

### 응답

- **200** 성공 응답으로 조회된 예약 결제 건 리스트가 반환됩니다.: [GetPaymentSchedulesResponse](#getpaymentschedulesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## DELETE /payment-schedules

**결제 예약 취소** (operationId: `revokePaymentSchedules`)

결제 예약 건을 취소합니다.
billingKey, scheduleIds 중 하나 이상은 필수로 입력합니다.
billingKey 만 입력된 경우 -> 해당 빌링키로 예약된 모든 결제 예약 건들이 취소됩니다.
scheduleIds 만 입력된 경우 -> 입력된 결제 예약 건 아이디에 해당하는 예약 건들이 취소됩니다.
billingKey, scheduleIds 모두 입력된 경우 -> 입력된 결제 예약 건 아이디에 해당하는 예약 건들이 취소됩니다. 단, 예약한 빌링키가 입력된 빌링키와 일치하지 않으면 실패합니다.
위 정책에 따라 선택된 결제 예약 건들 중 하나라도 취소에 실패할 경우, 모든 취소 요청이 실패합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [RevokePaymentSchedulesBody](#revokepaymentschedulesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string |  | 빌링키 |
| `scheduleIds` | string[] |  | 결제 예약 건 아이디 목록 |

### 응답

- **200** 성공 응답: [RevokePaymentSchedulesResponse](#revokepaymentschedulesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentScheduleNotFoundError`](errors.md#paymentschedulenotfounderror): 결제 예약건이 존재하지 않는 경우
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
- **409**
  * [`PaymentScheduleAlreadyProcessedError`](errors.md#paymentschedulealreadyprocessederror): 결제 예약건이 이미 처리된 경우
  * [`PaymentScheduleAlreadyRevokedError`](errors.md#paymentschedulealreadyrevokederror): 결제 예약건이 이미 취소된 경우
  * [`BillingKeyAlreadyDeletedError`](errors.md#billingkeyalreadydeletederror): 빌링키가 이미 삭제된 경우

## POST /payments/{paymentId}/schedule

**결제 예약** (operationId: `createPaymentSchedule`)

결제를 예약합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [CreatePaymentScheduleBody](#createpaymentschedulebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payment` | [BillingKeyPaymentScheduleInput](#billingkeypaymentscheduleinput) | Y | 빌링키 결제 입력 정보 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |

### 응답

- **200** 성공 응답: [CreatePaymentScheduleResponse](#createpaymentscheduleresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotFoundError`](errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
- **409**
  * [`AlreadyPaidOrWaitingError`](errors.md#alreadypaidorwaitingerror): 결제가 이미 완료되었거나 대기중인 경우
  * [`SumOfPartsExceedsTotalAmountError`](errors.md#sumofpartsexceedstotalamounterror): 면세 금액 등 하위 항목들의 합이 전체 결제 금액을 초과한 경우
  * [`BillingKeyAlreadyDeletedError`](errors.md#billingkeyalreadydeletederror): 빌링키가 이미 삭제된 경우
  * [`PaymentScheduleAlreadyExistsError`](errors.md#paymentschedulealreadyexistserror): 결제 예약건이 이미 존재하는 경우

---

# 스키마

### BillingKeyPaymentScheduleInput

빌링키 결제 예약 요청 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string | Y | 빌링키 결제에 사용할 빌링키 |
| `channelKey` | string |  | 채널 키 — 다수 채널에 대해 발급된 빌링키에 대해, 결제 채널을 특정하고 싶을 때 명시 |
| `orderName` | string | Y | 주문명 |
| `customer` | [CustomerInput](schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `amount` | [PaymentAmountInput](schemas-common.md#paymentamountinput) | Y | 결제 금액 세부 입력 정보 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |
| `cashReceipt` | [CashReceiptInput](schemas-common.md#cashreceiptinput) |  | 현금영수증 정보 — 나이스페이먼츠를 통해 네이버페이 포인트 빌링결제 시, 현금영수증 발급을 위해 입력 가능 (신청 필요) |
| `country` | [Country](schemas-common.md#country) |  | 결제 국가 |
| `noticeUrls` | string[] |  | 웹훅 주소 — 결제 승인/실패 시 요청을 받을 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>빈 배열은 무시됩니다. |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 — 입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `productCount` | integer(int32) |  | 상품 개수 |
| `productType` | [PaymentProductType](schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `shippingAddress` | [SeparatedAddressInput](schemas-common.md#separatedaddressinput) |  | 배송지 주소 |
| `promotionId` | string |  | 해당 결제에 적용할 프로모션 아이디 |
| `locale` | [Locale](schemas-common.md#locale) |  | 결제 시 사용할 언어 — 엑심베이의 경우 필수 입력입니다. |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |

### CreatePaymentScheduleBody

결제 예약 요청 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payment` | [BillingKeyPaymentScheduleInput](#billingkeypaymentscheduleinput) | Y | 빌링키 결제 입력 정보 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |

### CreatePaymentScheduleResponse

결제 예약 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `schedule` | [PaymentScheduleSummary](#paymentschedulesummary) | Y | 결제 예약 건 |

### FailedPaymentSchedule

결제 실패 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |
| `startedAt` | string(date-time) | Y | 결제 시작 시점 |
| `completedAt` | string(date-time) | Y | 결제 완료 시점 |

### GetPaymentSchedulesBody

결제 예약 다건 조회를 위한 입력 정보 — 결제 예약 다건 조회를 위한 입력 정보<br>조회 결과는 결제 예정 시점(timeToPay) 기준 최신 순으로 정렬됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number: 0, size: 10 으로 기본값이 적용됩니다. |
| `sort` | [PaymentScheduleSortInput](#paymentschedulesortinput) |  | 정렬 조건 — 미 입력 시 sortBy: TIME_TO_PAY, sortOrder: DESC 으로 기본값이 적용됩니다. |
| `filter` | [PaymentScheduleFilterInput](#paymentschedulefilterinput) |  | 조회할 결제 예약 건의 조건 필터 |

### GetPaymentSchedulesResponse

결제 예약 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentSchedule](#paymentschedule)[] | Y | 조회된 결제 예약 건 리스트 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### PaymentSchedule

결제 예약 건

oneOf, discriminator `status`:

- `FAILED` → [FailedPaymentSchedule](#failedpaymentschedule)
- `PENDING` → [PendingPaymentSchedule](#pendingpaymentschedule)
- `REVOKED` → [RevokedPaymentSchedule](#revokedpaymentschedule)
- `SCHEDULED` → [ScheduledPaymentSchedule](#scheduledpaymentschedule)
- `STARTED` → [StartedPaymentSchedule](#startedpaymentschedule)
- `SUCCEEDED` → [SucceededPaymentSchedule](#succeededpaymentschedule)

### PaymentScheduleFilterInput

결제 예약 건 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string |  | 빌링키 |
| `from` | string(date-time) |  | 결제 예정 시점 조건 범위의 시작 — 값을 입력하지 않으면 파라미터 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 예정 시점 조건 범위의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `status` | [PaymentScheduleStatus](#paymentschedulestatus)[] |  | 결제 예약 건 상태 리스트 — 값을 입력하지 않으면 상태 필터링이 적용되지 않습니다. |
| `isForTest` | boolean |  | 테스트 결제 여부 |

### PaymentScheduleSortBy

결제 예약 건 정렬 기준

- `CREATED_AT` — 결제 예약 생성 시각
- `TIME_TO_PAY` — 결제 예정 시각
- `COMPLETED_AT` — 예약 결제 시도(성공 / 실패) 시각
- `STATUS_TIMESTAMP` — 결제 시도 또는 예정 시각. 해지 건은 해지 시각.

### PaymentScheduleSortInput

결제 예약 건 다건 조회 시 정렬 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `by` | [PaymentScheduleSortBy](#paymentschedulesortby) |  | 정렬 기준 필드 — 어떤 필드를 기준으로 정렬할 지 결정합니다. 비워서 보낼 경우, TIME_TO_PAY가 기본값으로 설정됩니다. |
| `order` | [SortOrder](schemas-common.md#sortorder) |  | 정렬 순서 — 어떤 순서로 정렬할 지 결정합니다. 비워서 보낼 경우, DESC(내림차순)가 기본값으로 설정됩니다. |

### PaymentScheduleStatus

결제 예약 건 상태

- `SCHEDULED` — 예약 완료
- `STARTED` — 결제 시작
- `SUCCEEDED` — 결제 성공
- `FAILED` — 결제 실패
- `REVOKED` — 취소된 결제 예약
- `PENDING` — 결제 승인 대기

### PaymentScheduleSummary

결제 예약 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 결제 예약 건 아이디 |

### PendingPaymentSchedule

결제 완료 대기 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |
| `startedAt` | string(date-time) | Y | 결제 시작 시점 |
| `completedAt` | string(date-time) | Y | 결제 완료 시점 |

### RevokePaymentSchedulesBody

결제 예약 건 취소 요청 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string |  | 빌링키 |
| `scheduleIds` | string[] |  | 결제 예약 건 아이디 목록 |

### RevokePaymentSchedulesResponse

결제 예약 건 취소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `revokedScheduleIds` | string[] | Y | 취소 완료된 결제 예약 건 아이디 목록 |
| `revokedAt` | string(date-time) |  | 결제 예약 건 취소 완료 시점 |

### RevokedPaymentSchedule

결제 예약 취소 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |
| `revokedAt` | string(date-time) | Y | 결제 취소 시점 |

### ScheduledPaymentSchedule

결제 예약 완료 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |

### StartedPaymentSchedule

결제 시작 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |
| `startedAt` | string(date-time) | Y | 결제 시작 시점 |

### SucceededPaymentSchedule

결제 성공 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 예약 건 상태 |
| `id` | string | Y | 결제 예약 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 빌링키 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean | Y | 문화비 지출 여부 |
| `isEscrow` | boolean | Y | 에스크로 결제 여부 |
| `customer` | [Customer](schemas-common.md#customer) | Y | 고객 정보 |
| `customData` | string | Y | 사용자 지정 데이터 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `noticeUrls` | string[] |  | 웹훅 주소 |
| `products` | [PaymentProduct](schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `createdAt` | string(date-time) | Y | 결제 예약 등록 시점 |
| `timeToPay` | string(date-time) | Y | 결제 예정 시점 |
| `startedAt` | string(date-time) | Y | 결제 시작 시점 |
| `completedAt` | string(date-time) | Y | 결제 완료 시점 |
