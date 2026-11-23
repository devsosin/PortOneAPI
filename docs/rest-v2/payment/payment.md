# 결제 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /payment-events-by-cursor` — 결제 이벤트 대용량 다건 조회(커서 기반)
- `GET /payments-by-cursor` — 결제 대용량 다건 조회(커서 기반)
- `POST /payments/{paymentId}/billing-key` — 빌링키 결제
- `POST /payments/{paymentId}/cancel` — 결제 취소
- `POST /payments/{paymentId}/cancellations/{cancellationId}/stop` — 결제 취소 요청 취소
- `POST /payments/{paymentId}/capture` — 수동 매입
- `POST /payments/{paymentId}/confirm` — 인증 결제 수동 승인
- `POST /payments/{paymentId}/escrow/complete` — 에스크로 구매 확정
- `POST /payments/{paymentId}/escrow/logistics` — 에스크로 배송 정보 등록
- `PATCH /payments/{paymentId}/escrow/logistics` — 에스크로 배송 정보 수정
- `POST /payments/{paymentId}/instant` — 수기 결제
- `POST /payments/{paymentId}/pre-register` — 결제 정보 사전 등록
- `POST /payments/{paymentId}/register-store-receipt` — 영수증 내 하위 상점 거래 등록
- `POST /payments/{paymentId}/resend-webhook` — 웹훅 재발송
- `GET /payments/{paymentId}/transactions` — 결제 시도 내역 조회
- `POST /payments/{paymentId}/virtual-account/close` — 가상계좌 말소
- `GET /payments/{paymentId}` — 결제 단건 조회
- `GET /payments` — 결제 다건 조회(페이지 기반)

## GET /payment-events-by-cursor

**결제 이벤트 대용량 다건 조회(커서 기반)** (operationId: `getAllPaymentEventsByCursor`)

기간 내 모든 결제 이벤트를 커서 기반으로 조회합니다. 결제 이벤트의 생성일시를 기준으로 주어진 기간 내 존재하는 모든 결제 이벤트가 조회됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetAllPaymentEventsByCursorBody](#getallpaymenteventsbycursorbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 결제 이벤트를 조회합니다. |
| `from` | string(date-time) |  | 결제 이벤트 생성시점 범위 조건의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 이벤트 생성시점 범위 조건의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `cursor` | string |  | 커서 — 결제 이벤트 리스트 중 어디서부터 읽어야 할지 가리키는 값입니다. 최초 요청일 경우 값을 입력하지 마시되, 두번째 요청 부터는 이전 요청 응답값의 cursor를 입력해주시면 됩니다. |
| `size` | integer(int32) |  | 페이지 크기 — 미입력 시 기본값은 10 이며 최대 1000까지 허용 |

### 응답

- **200** 성공 응답으로 조회된 결제 이벤트 리스트와 커서 정보가 반환됩니다.: [GetAllPaymentEventsByCursorResponse](#getallpaymenteventsbycursorresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /payments-by-cursor

**결제 대용량 다건 조회(커서 기반)** (operationId: `getAllPaymentsByCursor`)

기간 내 모든 결제 건을 커서 기반으로 조회합니다. 결제 건의 생성일시를 기준으로 주어진 기간 내 존재하는 모든 결제 건이 조회됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetAllPaymentsByCursorBody](#getallpaymentsbycursorbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `from` | string(date-time) |  | 결제 건 생성시점 범위 조건의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 건 생성시점 범위 조건의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `cursor` | string |  | 커서 — 결제 건 리스트 중 어디서부터 읽어야 할지 가리키는 값입니다. 최초 요청일 경우 값을 입력하지 마시되, 두번째 요청 부터는 이전 요청 응답값의 cursor를 입력해주시면 됩니다. |
| `size` | integer(int32) |  | 페이지 크기 — 미입력 시 기본값은 10 이며 최대 1000까지 허용 |

### 응답

- **200** 성공 응답으로 조회된 결제 건 리스트와 커서 정보가 반환됩니다.: [GetAllPaymentsByCursorResponse](#getallpaymentsbycursorresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /payments/{paymentId}/billing-key

**빌링키 결제** (operationId: `payWithBillingKey`)

빌링키로 결제를 진행합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [BillingKeyPaymentInput](#billingkeypaymentinput)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string | Y | 빌링키 결제에 사용할 빌링키 |
| `channelKey` | string |  | 채널 키 — 다수 채널에 대해 발급된 빌링키에 대해, 결제 채널을 특정하고 싶을 때 명시 |
| `orderName` | string | Y | 주문명 |
| `customer` | [CustomerInput](../00-basics/schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 결제 금액 세부 입력 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |
| `cashReceipt` | [CashReceiptInput](../00-basics/schemas-common.md#cashreceiptinput) |  | 현금영수증 정보 — 나이스페이먼츠를 통해 네이버페이 포인트 빌링결제 시, 현금영수증 발급을 위해 입력 가능 (신청 필요) |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `noticeUrls` | string[] |  | 웹훅 주소 — 결제 승인/실패 시 요청을 받을 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>빈 배열은 무시됩니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 — 입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `productCount` | integer(int32) |  | 상품 개수 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `shippingAddress` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 배송지 주소 |
| `promotionId` | string |  | 해당 결제에 적용할 프로모션 아이디 |
| `locale` | [Locale](../00-basics/schemas-common.md#locale) |  | 결제 시 사용할 언어 — 엑심베이의 경우 필수 입력입니다. |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### 응답

- **200** 성공 응답: [PayWithBillingKeyResponse](#paywithbillingkeyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PromotionPayMethodDoesNotMatchError`](../00-basics/errors.md#promotionpaymethoddoesnotmatcherror): 결제수단이 프로모션에 지정된 것과 일치하지 않는 경우
  * [`DiscountAmountExceedsTotalAmountError`](../00-basics/errors.md#discountamountexceedstotalamounterror): 프로모션 할인 금액이 결제 시도 금액 이상인 경우
  * [`MaxTransactionCountReachedError`](../00-basics/errors.md#maxtransactioncountreachederror): 결제 혹은 본인인증 시도 횟수가 최대에 도달한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`BillingKeyNotFoundError`](../00-basics/errors.md#billingkeynotfounderror): 빌링키가 존재하지 않는 경우
  * [`ChannelNotFoundError`](../00-basics/errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **409**
  * [`AlreadyPaidError`](../00-basics/errors.md#alreadypaiderror): 결제가 이미 완료된 경우
  * [`SumOfPartsExceedsTotalAmountError`](../00-basics/errors.md#sumofpartsexceedstotalamounterror): 면세 금액 등 하위 항목들의 합이 전체 결제 금액을 초과한 경우
  * [`BillingKeyAlreadyDeletedError`](../00-basics/errors.md#billingkeyalreadydeletederror): 빌링키가 이미 삭제된 경우
  * [`PaymentScheduleAlreadyExistsError`](../00-basics/errors.md#paymentschedulealreadyexistserror): 결제 예약건이 이미 존재하는 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/cancel

**결제 취소** (operationId: `cancelPayment`)

결제 취소를 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [CancelPaymentBody](#cancelpaymentbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `amount` | integer(int64) |  | 취소 총 금액 — 값을 입력하지 않으면 전액 취소됩니다. |
| `taxFreeAmount` | integer(int64) |  | 취소 금액 중 면세 금액 — 값을 입력하지 않으면 전액 과세 취소됩니다. |
| `vatAmount` | integer(int64) |  | 취소 금액 중 부가세액 — 값을 입력하지 않으면 자동 계산됩니다. |
| `reason` | string | Y | 취소 사유 |
| `requester` | [CancelRequester](#cancelrequester) |  | 취소 요청자 — 고객에 의한 취소일 경우 Customer, 관리자에 의한 취소일 경우 Admin으로 입력합니다. |
| `promotionDiscountRetainOption` | [PromotionDiscountRetainOption](#promotiondiscountretainoption) |  | 프로모션 할인율 유지 옵션 — 프로모션이 적용된 결제를 부분 취소하는 경우, 최초 할인율을 유지할지 여부를 선택할 수 있습니다.<br>RETAIN 으로 설정 시, 최초 할인율을 유지할 수 있도록 취소 금액이 조정됩니다.<br>RELEASE 으로 설정 시, 취소 후 남은 금액이 속한 구간에 맞게 프로모션 할인이 새롭게 적용됩니다.<br>값을 입력하지 않으면 RELEASE 로 취급합니다. |
| `currentCancellableAmount` | integer(int64) |  | 결제 건의 취소 가능 잔액 — 본 취소 요청 이전의 취소 가능 잔액으로써, 값을 입력하면 잔액이 일치하는 경우에만 취소가 진행됩니다. 값을 입력하지 않으면 별도의 검증 처리를 수행하지 않습니다. |
| `refundAccount` | [CancelPaymentBodyRefundAccount](#cancelpaymentbodyrefundaccount) |  | 환불 계좌 — 계좌 환불일 경우 입력합니다. 계좌 환불이 필요한 경우는 가상계좌 환불, 휴대폰 익월 환불 등이 있습니다. |
| `refundEmail` | string |  | 환불 이메일 — Triple-A 결제 환불에 필요합니다. 해당 이메일로 환불 안내가 발송됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 취소가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### 응답

- **200** 성공 응답: [CancelPaymentResponse](#cancelpaymentresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`NegativePromotionAdjustedCancelAmountError`](../00-basics/errors.md#negativepromotionadjustedcancelamounterror): 프로모션에 의해 조정된 취소 금액이 음수인 경우
  * [`PromotionDiscountRetainOptionShouldNotBeChangedError`](../00-basics/errors.md#promotiondiscountretainoptionshouldnotbechangederror): 프로모션 혜택 유지 옵션을 이전 부분 취소와 다른 것으로 입력한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
  * [`PaymentAlreadyCancelledError`](../00-basics/errors.md#paymentalreadycancellederror): 결제가 이미 취소된 경우
  * [`CancellableAmountConsistencyBrokenError`](../00-basics/errors.md#cancellableamountconsistencybrokenerror): 취소 가능 잔액 검증에 실패한 경우
  * [`CancelAmountExceedsCancellableAmountError`](../00-basics/errors.md#cancelamountexceedscancellableamounterror): 결제 취소 금액이 취소 가능 금액을 초과한 경우
  * [`SumOfPartsExceedsCancelAmountError`](../00-basics/errors.md#sumofpartsexceedscancelamounterror): 면세 금액 등 하위 항목들의 합이 전체 취소 금액을 초과한 경우
  * [`CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError`](../00-basics/errors.md#canceltaxfreeamountexceedscancellabletaxfreeamounterror): 취소 면세 금액이 취소 가능한 면세 금액을 초과한 경우
  * [`CancelTaxAmountExceedsCancellableTaxAmountError`](../00-basics/errors.md#canceltaxamountexceedscancellabletaxamounterror): 취소 과세 금액이 취소 가능한 과세 금액을 초과한 경우
  * [`MaxCancelCountReachedError`](../00-basics/errors.md#maxcancelcountreachederror): 취소 시도 횟수가 초과된 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/cancellations/{cancellationId}/stop

**결제 취소 요청 취소** (operationId: `stopPaymentCancellation`)

비동기적으로 수행되는 결제 취소 요청을 취소합니다.
Triple-A에서만 사용됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |
| `cancellationId` | path | string | Y | 취소 요청 아이디 |

### 요청 본문 (application/json, 필수)

타입: [StopPaymentCancellationBody](#stoppaymentcancellationbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답: [StopPaymentCancellationResponse](#stoppaymentcancellationresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentCancellationNotFoundError`](../00-basics/errors.md#paymentcancellationnotfounderror): 결제 취소 내역이 존재하지 않는 경우
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentCancellationNotPendingError`](../00-basics/errors.md#paymentcancellationnotpendingerror): 결제 취소 내역이 취소 요청 상태가 아닌 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/capture

**수동 매입** (operationId: `capturePayment`)

수동 매입을 요청합니다. PG 및 포트원과의 사전 협의가 필요합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [CapturePaymentBody](#capturepaymentbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답: [CapturePaymentResponse](#capturepaymentresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/confirm

**인증 결제 수동 승인** (operationId: `confirmPayment`)

수동 승인으로 설정된 인증 결제에 대해, 결제를 완료 처리합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ConfirmPaymentBody](#confirmpaymentbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `paymentToken` | string | Y | 결제 토큰 — 인증 완료 시 발급된 토큰입니다. |
| `txId` | string |  | 결제 시도 아이디 — 검증용 파라미터로, 결제 시도 아이디와 일치하지 않을 경우 오류가 반환됩니다. |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 — 검증용 파라미터로, 결제 건 화폐와 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `totalAmount` | integer(int64) |  | 결제 금액 — 검증용 파라미터로, 결제 건 총 금액과 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `taxFreeAmount` | integer(int64) |  | 면세 금액 — 검증용 파라미터로, 결제 건 면세 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### 응답

- **200** 성공 응답으로 결제 승인 결과를 반환합니다.: [ConfirmedPaymentSummary](#confirmedpaymentsummary)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`InvalidPaymentTokenError`](../00-basics/errors.md#invalidpaymenttokenerror): 유효하지 않은 결제 토큰인 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`AlreadyPaidError`](../00-basics/errors.md#alreadypaiderror): 결제가 이미 완료된 경우
  * [`InformationMismatchError`](../00-basics/errors.md#informationmismatcherror): 정보가 일치하지 않는 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/escrow/complete

**에스크로 구매 확정** (operationId: `confirmEscrow`)

에스크로 결제를 구매 확정 처리합니다

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ConfirmEscrowBody](#confirmescrowbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `fromStore` | boolean |  | 확인 주체가 상점인지 여부 — 구매확정요청 주체가 고객사 관리자인지 구매자인지 구분하기 위한 필드입니다.<br>네이버페이 전용 파라미터이며, 구분이 모호한 경우 고객사 관리자(true)로 입력합니다. |

### 응답

- **200** 성공 응답: [ConfirmEscrowResponse](#confirmescrowresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/escrow/logistics

**에스크로 배송 정보 등록** (operationId: `applyEscrowLogistics`)

에스크로 배송 정보를 등록합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [RegisterEscrowLogisticsBody](#registerescrowlogisticsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `sender` | [PaymentEscrowSenderInput](#paymentescrowsenderinput) |  | 에스크로 발송자 정보 |
| `receiver` | [PaymentEscrowReceiverInput](#paymentescrowreceiverinput) |  | 에스크로 수취인 정보 |
| `logistics` | [PaymentLogistics](#paymentlogistics) | Y | 에스크로 물류 정보 |
| `sendEmail` | boolean |  | 이메일 알림 전송 여부 — 에스크로 구매 확정 시 이메일로 알림을 보낼지 여부입니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |

### 응답

- **200** 성공 응답: [ApplyEscrowLogisticsResponse](#applyescrowlogisticsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## PATCH /payments/{paymentId}/escrow/logistics

**에스크로 배송 정보 수정** (operationId: `modifyEscrowLogistics`)

에스크로 배송 정보를 수정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ModifyEscrowLogisticsBody](#modifyescrowlogisticsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `sender` | [PaymentEscrowSenderInput](#paymentescrowsenderinput) |  | 에스크로 발송자 정보 |
| `receiver` | [PaymentEscrowReceiverInput](#paymentescrowreceiverinput) |  | 에스크로 수취인 정보 |
| `logistics` | [PaymentLogistics](#paymentlogistics) | Y | 에스크로 물류 정보 |
| `sendEmail` | boolean |  | 이메일 알림 전송 여부 — 에스크로 구매 확정 시 이메일로 알림을 보낼지 여부입니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |

### 응답

- **200** 성공 응답: [ModifyEscrowLogisticsResponse](#modifyescrowlogisticsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/instant

**수기 결제** (operationId: `payInstantly`)

카드 비인증 결제 또는 가상 계좌 발급을 API로 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [InstantPaymentInput](#instantpaymentinput)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `channelKey` | string |  | 채널 키 — 채널 키 또는 채널 그룹 ID 필수 |
| `channelGroupId` | string |  | 채널 그룹 ID — 채널 키 또는 채널 그룹 ID 필수 |
| `method` | [InstantPaymentMethodInput](#instantpaymentmethodinput) | Y | 결제수단 정보 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 — 기본값은 false 입니다. |
| `isEscrow` | boolean |  | 에스크로 결제 여부 — 기본값은 false 입니다. |
| `customer` | [CustomerInput](../00-basics/schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 결제 금액 세부 입력 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `noticeUrls` | string[] |  | 웹훅 주소 — 결제 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>빈 배열은 무시됩니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 — 입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `productCount` | integer(int32) |  | 상품 개수 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `shippingAddress` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 배송지 주소 |
| `promotionId` | string |  | 해당 결제에 적용할 프로모션 아이디 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### 응답

- **200** 성공 응답: [PayInstantlyResponse](#payinstantlyresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PromotionPayMethodDoesNotMatchError`](../00-basics/errors.md#promotionpaymethoddoesnotmatcherror): 결제수단이 프로모션에 지정된 것과 일치하지 않는 경우
  * [`DiscountAmountExceedsTotalAmountError`](../00-basics/errors.md#discountamountexceedstotalamounterror): 프로모션 할인 금액이 결제 시도 금액 이상인 경우
  * [`MaxTransactionCountReachedError`](../00-basics/errors.md#maxtransactioncountreachederror): 결제 혹은 본인인증 시도 횟수가 최대에 도달한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`ChannelNotFoundError`](../00-basics/errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **409**
  * [`AlreadyPaidError`](../00-basics/errors.md#alreadypaiderror): 결제가 이미 완료된 경우
  * [`SumOfPartsExceedsTotalAmountError`](../00-basics/errors.md#sumofpartsexceedstotalamounterror): 면세 금액 등 하위 항목들의 합이 전체 결제 금액을 초과한 경우
  * [`PaymentScheduleAlreadyExistsError`](../00-basics/errors.md#paymentschedulealreadyexistserror): 결제 예약건이 이미 존재하는 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/pre-register

**결제 정보 사전 등록** (operationId: `preRegisterPayment`)

결제 정보를 사전 등록합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [PreRegisterPaymentBody](#preregisterpaymentbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `totalAmount` | integer(int64) |  | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 결제 면세 금액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 |

### 응답

- **200** 성공 응답: [PreRegisterPaymentResponse](#preregisterpaymentresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`AlreadyPaidError`](../00-basics/errors.md#alreadypaiderror): 결제가 이미 완료된 경우

## POST /payments/{paymentId}/register-store-receipt

**영수증 내 하위 상점 거래 등록** (operationId: `registerStoreReceipt`)

결제 내역 매출전표에 하위 상점의 거래를 등록합니다.
지원되는 PG사:
KG이니시스(이용 전 콘솔 -> 결제연동 탭에서 INIApi Key 등록 필요)

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 등록할 하위 상점 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [RegisterStoreReceiptBody](#registerstorereceiptbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [RegisterStoreReceiptBodyItem](#registerstorereceiptbodyitem)[] | Y | 하위 상점 거래 목록 |
| `storeId` | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답: [RegisterStoreReceiptResponse](#registerstorereceiptresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotPaidError`](../00-basics/errors.md#paymentnotpaiderror): 결제가 완료되지 않은 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## POST /payments/{paymentId}/resend-webhook

**웹훅 재발송** (operationId: `resendWebhook`)

웹훅을 재발송합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ResendWebhookBody](#resendwebhookbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `webhookId` | string |  | 웹훅 아이디 — 입력하지 않으면 결제 건의 가장 최근 웹훅 아이디가 기본 적용됩니다 |

### 응답

- **200** 성공 응답: [ResendWebhookResponse](#resendwebhookresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`MaxWebhookRetryCountReachedError`](../00-basics/errors.md#maxwebhookretrycountreachederror): 동일한 webhook id에 대한 수동 재시도 횟수가 최대에 도달한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
  * [`WebhookNotFoundError`](../00-basics/errors.md#webhooknotfounderror): 웹훅 내역이 존재하지 않는 경우

## GET /payments/{paymentId}/transactions

**결제 시도 내역 조회** (operationId: `getPaymentTransactions`)

주어진 아이디에 대응되는 결제 건의 결제 시도 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 조회할 결제 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 결제 시도 이력을 반환합니다.: [GetPaymentTransactionsResponse](#getpaymenttransactionsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우

## POST /payments/{paymentId}/virtual-account/close

**가상계좌 말소** (operationId: `closeVirtualAccount`)

발급된 가상계좌를 말소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 결제 건 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답: [CloseVirtualAccountResponse](#closevirtualaccountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우
- **409**
  * [`PaymentNotWaitingForDepositError`](../00-basics/errors.md#paymentnotwaitingfordepositerror): 결제 건이 입금 대기 상태가 아닌 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

## GET /payments/{paymentId}

**결제 단건 조회** (operationId: `getPayment`)

주어진 아이디에 대응되는 결제 건을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `paymentId` | path | string | Y | 조회할 결제 아이디 |
| `storeId` | query | string |  | 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### 응답

- **200** 성공 응답으로 결제 건 객체를 반환합니다.: [Payment](#payment)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PaymentNotFoundError`](../00-basics/errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우

## GET /payments

**결제 다건 조회(페이지 기반)** (operationId: `getPayments`)

주어진 조건에 맞는 결제 건들을 페이지 기반으로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPaymentsBody](#getpaymentsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number = 0, size = 10 으로 기본값이 적용됩니다. (number + 1) * size가 60,000을 초과할 수 없습니다. |
| `filter` | [PaymentFilterInput](#paymentfilterinput) |  | 조회할 결제 건 조건 필터 — V1 결제 건의 경우 일부 필드에 대해 필터가 적용되지 않을 수 있습니다. |

### 응답

- **200** 성공 응답으로 조회된 결제 건 리스트와 페이지 정보가 반환됩니다.: [GetPaymentsResponse](#getpaymentsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

---

# 스키마

### ApplyEscrowLogisticsResponse

에스크로 배송 정보 등록 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceNumber` | string | Y | 송장 번호 |
| `sentAt` | string(date-time) | Y | 발송 시점 |
| `appliedAt` | string(date-time) | Y | 에스크로 정보 등록 시점 |

### BeforeRegisteredPaymentEscrow

배송 정보 등록 전

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |

### BillingKeyPaymentInput

빌링키 결제 요청 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `billingKey` | string | Y | 빌링키 결제에 사용할 빌링키 |
| `channelKey` | string |  | 채널 키 — 다수 채널에 대해 발급된 빌링키에 대해, 결제 채널을 특정하고 싶을 때 명시 |
| `orderName` | string | Y | 주문명 |
| `customer` | [CustomerInput](../00-basics/schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 결제 금액 세부 입력 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |
| `cashReceipt` | [CashReceiptInput](../00-basics/schemas-common.md#cashreceiptinput) |  | 현금영수증 정보 — 나이스페이먼츠를 통해 네이버페이 포인트 빌링결제 시, 현금영수증 발급을 위해 입력 가능 (신청 필요) |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `noticeUrls` | string[] |  | 웹훅 주소 — 결제 승인/실패 시 요청을 받을 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>빈 배열은 무시됩니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 — 입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `productCount` | integer(int32) |  | 상품 개수 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `shippingAddress` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 배송지 주소 |
| `promotionId` | string |  | 해당 결제에 적용할 프로모션 아이디 |
| `locale` | [Locale](../00-basics/schemas-common.md#locale) |  | 결제 시 사용할 언어 — 엑심베이의 경우 필수 입력입니다. |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### BillingKeyPaymentSummary

빌링키 결제 완료된 결제 건 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgTxId` | string | Y | PG사 결제 아이디 |
| `paidAt` | string(date-time) | Y | 결제 완료 시점 |

### CancelPaymentBody

결제 취소 요청 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `amount` | integer(int64) |  | 취소 총 금액 — 값을 입력하지 않으면 전액 취소됩니다. |
| `taxFreeAmount` | integer(int64) |  | 취소 금액 중 면세 금액 — 값을 입력하지 않으면 전액 과세 취소됩니다. |
| `vatAmount` | integer(int64) |  | 취소 금액 중 부가세액 — 값을 입력하지 않으면 자동 계산됩니다. |
| `reason` | string | Y | 취소 사유 |
| `requester` | [CancelRequester](#cancelrequester) |  | 취소 요청자 — 고객에 의한 취소일 경우 Customer, 관리자에 의한 취소일 경우 Admin으로 입력합니다. |
| `promotionDiscountRetainOption` | [PromotionDiscountRetainOption](#promotiondiscountretainoption) |  | 프로모션 할인율 유지 옵션 — 프로모션이 적용된 결제를 부분 취소하는 경우, 최초 할인율을 유지할지 여부를 선택할 수 있습니다.<br>RETAIN 으로 설정 시, 최초 할인율을 유지할 수 있도록 취소 금액이 조정됩니다.<br>RELEASE 으로 설정 시, 취소 후 남은 금액이 속한 구간에 맞게 프로모션 할인이 새롭게 적용됩니다.<br>값을 입력하지 않으면 RELEASE 로 취급합니다. |
| `currentCancellableAmount` | integer(int64) |  | 결제 건의 취소 가능 잔액 — 본 취소 요청 이전의 취소 가능 잔액으로써, 값을 입력하면 잔액이 일치하는 경우에만 취소가 진행됩니다. 값을 입력하지 않으면 별도의 검증 처리를 수행하지 않습니다. |
| `refundAccount` | [CancelPaymentBodyRefundAccount](#cancelpaymentbodyrefundaccount) |  | 환불 계좌 — 계좌 환불일 경우 입력합니다. 계좌 환불이 필요한 경우는 가상계좌 환불, 휴대폰 익월 환불 등이 있습니다. |
| `refundEmail` | string |  | 환불 이메일 — Triple-A 결제 환불에 필요합니다. 해당 이메일로 환불 안내가 발송됩니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 취소가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. |

### CancelPaymentBodyRefundAccount

고객 정보 입력 형식

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `number` | string | Y | 계좌번호 |
| `holderName` | string | Y | 예금주 |
| `holderPhoneNumber` | string |  | 예금주 연락처 - 스마트로 가상계좌 결제인 경우에 필요합니다. |

### CancelPaymentResponse

결제 취소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cancellation` | [PaymentCancellation](#paymentcancellation) | Y | 결제 취소 내역 |

### CancelRequester

결제 취소 요청 주체

- `CUSTOMER` — 구매자
- `ADMIN` — 관리자

### CancelledPayment

결제 취소 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) |  | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellations` | [PaymentCancellation](#paymentcancellation)[] | Y | 결제 취소 내역 |
| `cancelledAt` | string(date-time) | Y | 결제 취소 시점 |

### CancelledPaymentCashReceipt

취소된 현금영수증

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 내 현금영수증 상태 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) |  | 현금영수증 유형 |
| `pgReceiptId` | string |  | PG사 영수증 발급 아이디 |
| `issueNumber` | string | Y | 승인 번호 |
| `totalAmount` | integer(int64) | Y | 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `url` | string |  | 현금영수증 URL |
| `issuedAt` | string(date-time) | Y | 발급 시점 |
| `cancelledAt` | string(date-time) | Y | 취소 시점 |

### CancelledPaymentEscrow

거래 취소

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |

### CancelledPaymentEvent

결제 취소 상태 이벤트

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 결제 이벤트 종류 |
| `id` | string | Y | 결제 이벤트 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 시도 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `createdAt` | string(date-time) | Y | 이벤트 생성 시점 |
| `orderName` | string | Y | 주문명 |
| `totalAmount` | [PaymentAmount](#paymentamount) | Y | 총 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellation` | [PaymentCancellation](#paymentcancellation) |  | 결제 취소 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `eventAmount` | integer(int64) | Y | 처리 금액 — 해당 이벤트에서 처리된 금액으로, 취소 이벤트인 경우 음수로 표기됩니다. |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |

### CancelledPaymentTransaction

결제 취소 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) |  | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellations` | [PaymentCancellation](#paymentcancellation)[] | Y | 결제 취소 내역 |
| `cancelledAt` | string(date-time) | Y | 결제 취소 시점 |

### CapturePaymentBody

수동 매입 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### CapturePaymentResponse

수동 매입 성공 응답

type: object

### CashReceiptIssuanceStatus

현금영수증 발행여부

- `ISSUED` — 발행 완료
- `NOT_ISSUED` — 미발행

### CloseVirtualAccountResponse

가상계좌 말소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `closedAt` | string(date-time) | Y | 가상계좌 말소 시점 |

### ConfirmEscrowBody

에스크로 구매 확정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `fromStore` | boolean |  | 확인 주체가 상점인지 여부 — 구매확정요청 주체가 고객사 관리자인지 구매자인지 구분하기 위한 필드입니다.<br>네이버페이 전용 파라미터이며, 구분이 모호한 경우 고객사 관리자(true)로 입력합니다. |

### ConfirmEscrowResponse

에스크로 구매 확정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `completedAt` | string(date-time) | Y | 에스크로 구매 확정 시점 |

### ConfirmPaymentBody

결제 승인 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `paymentToken` | string | Y | 결제 토큰 — 인증 완료 시 발급된 토큰입니다. |
| `txId` | string |  | 결제 시도 아이디 — 검증용 파라미터로, 결제 시도 아이디와 일치하지 않을 경우 오류가 반환됩니다. |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 — 검증용 파라미터로, 결제 건 화폐와 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `totalAmount` | integer(int64) |  | 결제 금액 — 검증용 파라미터로, 결제 건 총 금액과 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `taxFreeAmount` | integer(int64) |  | 면세 금액 — 검증용 파라미터로, 결제 건 면세 금액과 일치하지 않을 경우 오류가 반환됩니다. |
| `isTest` | boolean |  | 테스트 결제 여부 — 검증용 파라미터로, 결제 건 테스트 여부와 일치하지 않을 경우 오류가 반환됩니다. 값 전달을 권장합니다. |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### ConfirmedPaymentEscrow

구매 확정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |
| `isAutomaticallyConfirmed` | boolean | Y | 자동 구매 확정 처리 여부 |

### ConfirmedPaymentSummary

결제 수동 승인 완료된 결제 건 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgTxId` | string | Y | PG사 결제 아이디 |
| `paidAt` | string(date-time) | Y | 결제 완료 시점 |

### ConvenienceStoreBrand

편의점 브랜드

- `LAWSON` — 로손
- `FAMILY_MART` — 패밀리마트
- `MINI_STOP` — 미니스톱
- `SEVEN_ELEVEN` — 세븐일레븐
- `SEICOMART` — 세이코마트

### DeliveredPaymentEscrow

배송 완료

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |

### Dispute

분쟁 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | [DisputeStatus](#disputestatus) | Y | 분쟁 상태 |
| `pgDisputeId` | string |  | PG사 분쟁 아이디 |
| `reason` | string | Y | 분쟁 사유 |
| `createdAt` | string(date-time) | Y | 분쟁 발생 시각 |
| `resolvedAt` | string(date-time) |  | 분쟁 해소 시각 |

### DisputeStatus

분쟁 상태

- `UNRESOLVED` — 분쟁 발생 상태
- `RESOLVED` — 분쟁 해소 상태

### FailedPayment

결제 실패 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) |  | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `failedAt` | string(date-time) | Y | 결제 실패 시점 |
| `failure` | [PaymentFailure](#paymentfailure) | Y | 결제 실패 정보 |

### FailedPaymentCancellation

취소 실패 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 취소 내역 상태 |
| `id` | string | Y | 취소 내역 아이디 |
| `pgCancellationId` | string |  | PG사 결제 취소 내역 아이디 |
| `totalAmount` | integer(int64) | Y | 취소 금액 |
| `taxFreeAmount` | integer(int64) | Y | 취소 금액 중 면세 금액 |
| `vatAmount` | integer(int64) | Y | 취소 금액 중 부가세액 |
| `easyPayDiscountAmount` | integer(int64) |  | 적립형 포인트의 환불 금액 |
| `reason` | string | Y | 취소 사유 |
| `cancelledAt` | string(date-time) |  | 취소 시점 |
| `requestedAt` | string(date-time) | Y | 취소 요청 시점 |
| `trigger` | [Trigger](#trigger) |  | 취소 요청 경로 |

### FailedPaymentTransaction

결제 실패 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) |  | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `failedAt` | string(date-time) | Y | 결제 실패 시점 |
| `failure` | [PaymentFailure](#paymentfailure) | Y | 결제 실패 정보 |

### GetAllPaymentEventsByCursorBody

GetAllPaymentEventsByCursorBody — 결제 이벤트 커서 기반 대용량 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 결제 이벤트를 조회합니다. |
| `from` | string(date-time) |  | 결제 이벤트 생성시점 범위 조건의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 이벤트 생성시점 범위 조건의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `cursor` | string |  | 커서 — 결제 이벤트 리스트 중 어디서부터 읽어야 할지 가리키는 값입니다. 최초 요청일 경우 값을 입력하지 마시되, 두번째 요청 부터는 이전 요청 응답값의 cursor를 입력해주시면 됩니다. |
| `size` | integer(int32) |  | 페이지 크기 — 미입력 시 기본값은 10 이며 최대 1000까지 허용 |

### GetAllPaymentEventsByCursorResponse

결제 이벤트 커서 기반 대용량 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentEventWithCursor](#paymenteventwithcursor)[] | Y | 조회된 결제 이벤트 및 커서 정보 리스트 |

### GetAllPaymentsByCursorBody

GetAllPaymentsByCursorBody — 결제 건 커서 기반 대용량 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `from` | string(date-time) |  | 결제 건 생성시점 범위 조건의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 건 생성시점 범위 조건의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `cursor` | string |  | 커서 — 결제 건 리스트 중 어디서부터 읽어야 할지 가리키는 값입니다. 최초 요청일 경우 값을 입력하지 마시되, 두번째 요청 부터는 이전 요청 응답값의 cursor를 입력해주시면 됩니다. |
| `size` | integer(int32) |  | 페이지 크기 — 미입력 시 기본값은 10 이며 최대 1000까지 허용 |

### GetAllPaymentsByCursorResponse

결제 건 커서 기반 대용량 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentWithCursor](#paymentwithcursor)[] | Y | 조회된 결제 건 및 커서 정보 리스트 |

### GetPaymentTransactionsResponse

결제 시도 내역 조회 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentTransaction](#paymenttransaction)[] | Y | 결제 시도 내역 |

### GetPaymentsBody

결제 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 — 미 입력 시 number = 0, size = 10 으로 기본값이 적용됩니다. (number + 1) * size가 60,000을 초과할 수 없습니다. |
| `filter` | [PaymentFilterInput](#paymentfilterinput) |  | 조회할 결제 건 조건 필터 — V1 결제 건의 경우 일부 필드에 대해 필터가 적용되지 않을 수 있습니다. |

### GetPaymentsResponse

결제 건 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [Payment](#payment)[] | Y | 조회된 결제 건 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### InstantPaymentInput

수기 결제 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `channelKey` | string |  | 채널 키 — 채널 키 또는 채널 그룹 ID 필수 |
| `channelGroupId` | string |  | 채널 그룹 ID — 채널 키 또는 채널 그룹 ID 필수 |
| `method` | [InstantPaymentMethodInput](#instantpaymentmethodinput) | Y | 결제수단 정보 |
| `orderName` | string | Y | 주문명 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 — 기본값은 false 입니다. |
| `isEscrow` | boolean |  | 에스크로 결제 여부 — 기본값은 false 입니다. |
| `customer` | [CustomerInput](../00-basics/schemas-common.md#customerinput) |  | 고객 정보 |
| `customData` | string |  | 사용자 지정 데이터 |
| `amount` | [PaymentAmountInput](../00-basics/schemas-common.md#paymentamountinput) | Y | 결제 금액 세부 입력 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `noticeUrls` | string[] |  | 웹훅 주소 — 결제 웹훅 주소입니다.<br>상점에 설정되어 있는 값보다 우선적으로 적용됩니다.<br>빈 배열은 무시됩니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 — 입력된 값이 없을 경우에는 빈 배열로 해석됩니다. |
| `productCount` | integer(int32) |  | 상품 개수 |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `shippingAddress` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 배송지 주소 |
| `promotionId` | string |  | 해당 결제에 적용할 프로모션 아이디 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `skipWebhook` | boolean |  | 웹훅 생략 여부 — 결제가 성공했을 때 웹훅을 전송하지 않으려면 true로 설정합니다. 가상계좌 입금 완료 등 외부 이벤트로 결제가 완료되는 경우 발생하는 웹훅은 스킵되지 않습니다. |

### InstantPaymentMethodInput

수기 결제 수단 입력 정보 — 수기 결제 수단 입력 정보<br>하나의 필드만 입력합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `card` | [InstantPaymentMethodInputCard](#instantpaymentmethodinputcard) |  | 카드 |
| `virtualAccount` | [InstantPaymentMethodInputVirtualAccount](#instantpaymentmethodinputvirtualaccount) |  | 가상계좌 |

### InstantPaymentMethodInputCard

카드 수단 정보 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `credential` | [CardCredential](../00-basics/schemas-common.md#cardcredential) | Y | 카드 인증 관련 정보 |
| `installmentMonth` | integer(int32) |  | 카드 할부 개월 수 |
| `useFreeInstallmentPlan` | boolean |  | 무이자 할부 적용 여부 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |

### InstantPaymentMethodInputVirtualAccount

가상계좌 수단 정보 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `expiry` | [InstantPaymentMethodInputVirtualAccountExpiry](#instantpaymentmethodinputvirtualaccountexpiry) | Y | 입금 만료 기한 |
| `option` | [InstantPaymentMethodInputVirtualAccountOption](#instantpaymentmethodinputvirtualaccountoption) | Y | 가상계좌 유형 |
| `cashReceipt` | [InstantPaymentMethodInputVirtualAccountCashReceiptInfo](#instantpaymentmethodinputvirtualaccountcashreceiptinfo) |  | 현금영수증 정보 |
| `remitteeName` | string |  | 예금주명 |

### InstantPaymentMethodInputVirtualAccountCashReceiptInfo

가상계좌 결제 시 현금영수증 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [CashReceiptInputType](../00-basics/schemas-common.md#cashreceiptinputtype) | Y | 현금영수증 유형 |
| `customerIdentityNumber` | string | Y | 사용자 식별 번호 |

### InstantPaymentMethodInputVirtualAccountExpiry

입금 만료 기한 — 입금 만료 기한<br>validHours와 dueDate 둘 중 하나의 필드만 입력합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `validHours` | integer(int32) |  | 유효 시간 — 시간 단위로 입력합니다. |
| `dueDate` | string(date-time) |  | 만료 시점 |

### InstantPaymentMethodInputVirtualAccountOption

가상계좌 발급 방식

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [InstantPaymentMethodInputVirtualAccountOptionType](#instantpaymentmethodinputvirtualaccountoptiontype) | Y | 발급 유형 |
| `fixed` | [InstantPaymentMethodInputVirtualAccountOptionFixed](#instantpaymentmethodinputvirtualaccountoptionfixed) |  | 고정식 가상계좌 발급 방식 — 발급 유형을 FIXED 로 선택했을 시에만 입력합니다. |

### InstantPaymentMethodInputVirtualAccountOptionFixed

고정식 가상계좌 발급 유형 — 고정식 가상계좌 발급 유형<br>pgAccountId, accountNumber 유형 중 한 개의 필드만 입력합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgAccountId` | string |  | Account ID 고정식 가상계좌 — 고객사가 가상계좌번호를 직접 관리하지 않고 PG사가 pgAccountId에 매핑되는 가상계좌번호를 내려주는 방식입니다.<br>동일한 pgAccountId로 가상계좌 발급 요청시에는 항상 같은 가상계좌번호가 내려옵니다. |
| `accountNumber` | string |  | Account Number 고정식 가상계좌 — PG사가 일정 개수만큼의 가상계좌번호를 발급하여 고객사에게 미리 전달하고 고객사가 그 중 하나를 선택하여 사용하는 방식입니다. |

### InstantPaymentMethodInputVirtualAccountOptionType

가상계좌 발급 유형

- `NORMAL` — 회전식 가상계좌
- `FIXED` — 고정식 가상계좌

### InstantPaymentSummary

수기 결제가 완료된 결제 건 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgTxId` | string | Y | PG 결제 아이디 |
| `paidAt` | string(date-time) | Y | 결제 완료 시점 (가상 계좌는 발급 완료 시점) |

### IssuedPaymentCashReceipt

발급 완료된 현금영수증

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 내 현금영수증 상태 |
| `type` | [CashReceiptType](../00-basics/schemas-common.md#cashreceipttype) |  | 현금영수증 유형 |
| `pgReceiptId` | string |  | PG사 영수증 발급 아이디 |
| `issueNumber` | string | Y | 승인 번호 |
| `totalAmount` | integer(int64) | Y | 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `url` | string |  | 현금영수증 URL |
| `issuedAt` | string(date-time) | Y | 발급 시점 |

### ModifyEscrowLogisticsBody

에스크로 배송 정보 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `sender` | [PaymentEscrowSenderInput](#paymentescrowsenderinput) |  | 에스크로 발송자 정보 |
| `receiver` | [PaymentEscrowReceiverInput](#paymentescrowreceiverinput) |  | 에스크로 수취인 정보 |
| `logistics` | [PaymentLogistics](#paymentlogistics) | Y | 에스크로 물류 정보 |
| `sendEmail` | boolean |  | 이메일 알림 전송 여부 — 에스크로 구매 확정 시 이메일로 알림을 보낼지 여부입니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |

### ModifyEscrowLogisticsResponse

에스크로 배송 정보 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceNumber` | string | Y | 송장 번호 |
| `sentAt` | string(date-time) | Y | 발송 시점 |
| `modifiedAt` | string(date-time) | Y | 에스크로 정보 수정 시점 |

### PaidPayment

결제 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) | Y | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `pgResponse` | string |  | PG사 거래 응답 본문 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `disputes` | [Dispute](#dispute)[] | Y | 분쟁 목록 |
| `cancellations` | [PaymentCancellation](#paymentcancellation)[] |  | 결제 취소 요청 내역 |

### PaidPaymentEvent

결제 완료 이벤트

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 결제 이벤트 종류 |
| `id` | string | Y | 결제 이벤트 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 시도 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `createdAt` | string(date-time) | Y | 이벤트 생성 시점 |
| `orderName` | string | Y | 주문명 |
| `totalAmount` | [PaymentAmount](#paymentamount) | Y | 총 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `pgResponse` | string |  | PG사 거래 응답 본문 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `disputes` | [Dispute](#dispute)[] | Y | 분쟁 목록 |
| `promotionId` | string |  | 프로모션 아이디 |
| `eventAmount` | integer(int64) | Y | 처리 금액 — 해당 이벤트에서 처리된 금액으로, 취소 이벤트인 경우 음수로 표기됩니다. |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |

### PaidPaymentTransaction

결제 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) | Y | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `pgResponse` | string |  | PG사 거래 응답 본문 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |

### PartialCancelledPayment

결제 부분 취소 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) |  | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellations` | [PaymentCancellation](#paymentcancellation)[] | Y | 결제 취소 내역 |
| `cancelledAt` | string(date-time) | Y | 결제 취소 시점 |

### PartialCancelledPaymentEvent

결제 부분 취소 상태 이벤트

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 결제 이벤트 종류 |
| `id` | string | Y | 결제 이벤트 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 시도 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `createdAt` | string(date-time) | Y | 이벤트 생성 시점 |
| `orderName` | string | Y | 주문명 |
| `totalAmount` | [PaymentAmount](#paymentamount) | Y | 총 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellation` | [PaymentCancellation](#paymentcancellation) |  | 결제 취소 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `eventAmount` | integer(int64) | Y | 처리 금액 — 해당 이벤트에서 처리된 금액으로, 취소 이벤트인 경우 음수로 표기됩니다. |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |

### PartialCancelledPaymentTransaction

결제 부분 취소 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `paidAt` | string(date-time) |  | 결제 완료 시점 |
| `pgTxId` | string |  | PG사 거래 아이디 |
| `cashReceipt` | [PaymentCashReceipt](#paymentcashreceipt) |  | 현금영수증 |
| `cashReceiptIssuanceStatus` | [CashReceiptIssuanceStatus](#cashreceiptissuancestatus) |  | 현금영수증 발행여부 — 승인 시점에 확인된 발행여부입니다.<br>발행번호 없이 발행여부만 제공 가능한 경우, ISSUED이면서 cashReceipt가 존재하지 않을 수 있습니다. |
| `receiptUrl` | string |  | 거래 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 거래 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `cancellations` | [PaymentCancellation](#paymentcancellation)[] | Y | 결제 취소 내역 |
| `cancelledAt` | string(date-time) | Y | 결제 취소 시점 |

### PayInstantlyResponse

수기 결제 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payment` | [InstantPaymentSummary](#instantpaymentsummary) | Y | 결제 건 요약 정보 |

### PayPendingPayment

결제 완료 대기 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |

### PayPendingPaymentTransaction

결제 완료 대기 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |

### PayWithBillingKeyResponse

빌링키 결제 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payment` | [BillingKeyPaymentSummary](#billingkeypaymentsummary) | Y | 결제 건 요약 정보 |

### Payment

결제 건

oneOf, discriminator `status`:

- `CANCELLED` → [CancelledPayment](#cancelledpayment)
- `FAILED` → [FailedPayment](#failedpayment)
- `PAID` → [PaidPayment](#paidpayment)
- `PARTIAL_CANCELLED` → [PartialCancelledPayment](#partialcancelledpayment)
- `PAY_PENDING` → [PayPendingPayment](#paypendingpayment)
- `READY` → [ReadyPayment](#readypayment)
- `VIRTUAL_ACCOUNT_ISSUED` → [VirtualAccountIssuedPayment](#virtualaccountissuedpayment)

### PaymentAmount

결제 금액 세부 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `total` | integer(int64) | Y | 총 결제금액 |
| `taxFree` | integer(int64) | Y | 면세액 |
| `vat` | integer(int64) |  | 부가세액 |
| `supply` | integer(int64) |  | 공급가액 |
| `discount` | integer(int64) | Y | 총 할인금액 — 카드사 할인금액, 포트원 프로모션 할인금액, 간편결제 할인금액(적립형 포인트 결제, 쿠폰 할인) 등을 포함합니다. |
| `paid` | integer(int64) | Y | 실제 결제금액 |
| `cancelled` | integer(int64) | Y | 총 취소금액 |
| `cancelledTaxFree` | integer(int64) | Y | 총 취소금액 중 면세액 |

### PaymentCancellation

결제 취소 내역

oneOf, discriminator `status`:

- `FAILED` → [FailedPaymentCancellation](#failedpaymentcancellation)
- `REQUESTED` → [RequestedPaymentCancellation](#requestedpaymentcancellation)
- `SUCCEEDED` → [SucceededPaymentCancellation](#succeededpaymentcancellation)

### PaymentCashReceipt

결제 건 내 현금영수증 정보

oneOf, discriminator `status`:

- `CANCELLED` → [CancelledPaymentCashReceipt](#cancelledpaymentcashreceipt)
- `ISSUED` → [IssuedPaymentCashReceipt](#issuedpaymentcashreceipt)

### PaymentCashReceiptStatus

결제건 내 현금영수증 상태

- `ISSUED`
- `CANCELLED`

### PaymentEscrow

에스크로 정보 — 에스크로 정보<br>V1 결제 건의 경우 타입이 REGISTERED 로 고정됩니다.

oneOf, discriminator `status`:

- `BEFORE_REGISTERED` → [BeforeRegisteredPaymentEscrow](#beforeregisteredpaymentescrow)
- `CANCELLED` → [CancelledPaymentEscrow](#cancelledpaymentescrow)
- `CONFIRMED` → [ConfirmedPaymentEscrow](#confirmedpaymentescrow)
- `DELIVERED` → [DeliveredPaymentEscrow](#deliveredpaymentescrow)
- `REGISTERED` → [RegisteredPaymentEscrow](#registeredpaymentescrow)
- `REJECT_CONFIRMED` → [RejectConfirmedPaymentEscrow](#rejectconfirmedpaymentescrow)
- `REJECTED` → [RejectedPaymentEscrow](#rejectedpaymentescrow)

### PaymentEscrowReceiverInput

에스크로 수취인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 이름 |
| `phoneNumber` | string |  | 전화번호 |
| `zipcode` | string |  | 우편번호 |
| `address` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 주소 |

### PaymentEscrowSenderInput

에스크로 발송자 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 이름 |
| `phoneNumber` | string |  | 전화번호 |
| `zipcode` | string |  | 우편번호 |
| `relationship` | string |  | 수취인과의 관계 |
| `address` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 주소 |

### PaymentEvent

결제 이벤트

oneOf, discriminator `type`:

- `CANCELLED` → [CancelledPaymentEvent](#cancelledpaymentevent)
- `PAID` → [PaidPaymentEvent](#paidpaymentevent)
- `PARTIAL_CANCELLED` → [PartialCancelledPaymentEvent](#partialcancelledpaymentevent)

### PaymentEventWithCursor

결제 이벤트 및 커서 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `paymentEvent` | [PaymentEvent](#paymentevent) | Y | 결제 이벤트 정보 |
| `cursor` | string | Y | 해당 결제 이벤트의 커서 정보 |

### PaymentFailure

결제 실패 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `reason` | string |  | 실패 사유 |
| `pgCode` | string |  | PG사 실패 코드 |
| `pgMessage` | string |  | PG사 실패 메시지 |

### PaymentFilterInput

결제 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `merchantId` | string |  | 고객사 아이디 |
| `storeId` | string |  | 상점 아이디 — Merchant 사용자만 사용가능하며, 지정되지 않은 경우 고객사 전체 결제 건을 조회합니다. |
| `timestampType` | [PaymentTimestampType](#paymenttimestamptype) |  | 조회 기준 시점 유형 |
| `from` | string(date-time) |  | 결제 요청/상태 승인 시점 범위의 시작 — 값을 입력하지 않으면 end의 90일 전으로 설정됩니다. |
| `until` | string(date-time) |  | 결제 요청/상태 승인 시점 범위의 끝 — 값을 입력하지 않으면 현재 시점으로 설정됩니다. |
| `status` | [PaymentStatus](#paymentstatus)[] |  | 결제 상태 리스트 — 값을 입력하지 않으면 결제상태 필터링이 적용되지 않습니다. |
| `methods` | [PaymentMethodType](../00-basics/schemas-common.md#paymentmethodtype)[] |  | 결제수단 리스트 — 값을 입력하지 않으면 결제수단 필터링이 적용되지 않습니다. |
| `pgProvider` | [PgProvider](../00-basics/schemas-common.md#pgprovider)[] |  | PG사 결제 모듈 리스트 — 값을 입력하지 않으면 PG사 결제 모듈 필터링이 적용되지 않습니다. |
| `isTest` | boolean |  | 테스트 결제 여부 |
| `isScheduled` | boolean |  | 결제 예약 건 필터링 |
| `sortBy` | [PaymentSortBy](#paymentsortby) |  | 결제 건 정렬 기준 |
| `sortOrder` | [SortOrder](../00-basics/schemas-common.md#sortorder) |  | 결제 건 정렬 방식 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) |  | 포트원 버전 |
| `webhookStatus` | [PaymentWebhookStatus](#paymentwebhookstatus) |  | 최종 결제 상태의 웹훅 상태 |
| `platformType` | [PaymentClientType](../00-basics/schemas-common.md#paymentclienttype) |  | 플랫폼 유형 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 |
| `isEscrow` | boolean |  | 에스크로 결제 여부 |
| `escrowStatus` | [PaymentFilterInputEscrowStatus](#paymentfilterinputescrowstatus) |  | 에스크로 결제의 배송 정보 상태 |
| `cardBrand` | [CardBrand](../00-basics/schemas-common.md#cardbrand) |  | 카드 브랜드 |
| `cardType` | [CardType](../00-basics/schemas-common.md#cardtype) |  | 카드 유형 |
| `cardOwnerType` | [CardOwnerType](../00-basics/schemas-common.md#cardownertype) |  | 카드 소유주 유형 |
| `giftCertificateType` | [PaymentMethodGiftCertificateType](#paymentmethodgiftcertificatetype) |  | 상품권 종류 |
| `cashReceiptType` | [CashReceiptInputType](../00-basics/schemas-common.md#cashreceiptinputtype) |  | 현금영수증 유형 |
| `cashReceiptStatus` | [PaymentCashReceiptStatus](#paymentcashreceiptstatus) |  | 현금영수증 상태 |
| `cashReceiptIssuedAtRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 현금영수증 발급 시간 범위 |
| `cashReceiptCancelledAtRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 현금영수증 취소 시간 범위 |
| `textSearch` | [PaymentTextSearch](#paymenttextsearch)[] |  | 통합 검색 리스트 필터 |

### PaymentFilterInputEscrowStatus

에스크로 상태

- `REGISTERED`
- `DELIVERED`
- `CONFIRMED`
- `REJECTED`
- `CANCELLED`
- `REJECT_CONFIRMED`

### PaymentInstallment

할부 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `month` | integer(int32) | Y | 할부 개월 수 |
| `isInterestFree` | boolean | Y | 무이자할부 여부 |
| `isInterestFreeFromMerchant` | boolean |  | 상점 부담 무이자할부 여부 — 정보 필요시 포트원과 협의해 주세요. |

### PaymentLogistics

배송정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `company` | [PaymentLogisticsCompany](#paymentlogisticscompany) | Y | 물류회사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) | Y | 발송시점 |
| `receivedAt` | string(date-time) |  | 수령시점 |
| `address` | [SeparatedAddressInput](../00-basics/schemas-common.md#separatedaddressinput) |  | 주소 |

### PaymentLogisticsCompany

물류 회사

- `LOTTE` — 롯데글로벌로지스
- `LOGEN` — 로젠택배
- `DONGWON` — 동원로엑스
- `POST` — 우체국택배
- `CJ` — 대한통운
- `HANJIN` — 한진택배
- `DAESIN` — 대신택배
- `ILYANG` — 일양로지스
- `KYUNGDONG` — 경동택배
- `CHUNIL` — 천일택배
- `POST_REGISTERED` — 등기우편
- `GS` — GS네트웍스
- `WOORI` — 우리택배
- `HAPDONG` — 합동택배
- `FEDEX` — FedEx
- `UPS` — UPS
- `GSM_NTON` — GSM NtoN
- `SUNGWON` — 성원글로벌카고
- `LX_PANTOS` — LX판토스
- `ACI` — ACI
- `CJ_INTL` — CJ대한통운 국제특송
- `USPS` — USPS
- `EMS` — EMS
- `DHL` — DHL
- `KGL` — KGL네트웍스
- `GOODSTOLUCK` — 굿투럭
- `KUNYOUNG` — 건영택배
- `SLX` — SLX
- `SF` — SF Express
- `ETC` — 기타

### PaymentMethod

결제수단 정보

oneOf, discriminator `type`:

- `PaymentMethodCard` → [PaymentMethodCard](#paymentmethodcard)
- `PaymentMethodConvenienceStore` → [PaymentMethodConvenienceStore](#paymentmethodconveniencestore)
- `PaymentMethodCrypto` → [PaymentMethodCrypto](#paymentmethodcrypto)
- `PaymentMethodEasyPay` → [PaymentMethodEasyPay](#paymentmethodeasypay)
- `PaymentMethodGiftCertificate` → [PaymentMethodGiftCertificate](#paymentmethodgiftcertificate)
- `PaymentMethodMobile` → [PaymentMethodMobile](#paymentmethodmobile)
- `PaymentMethodTransfer` → [PaymentMethodTransfer](#paymentmethodtransfer)
- `PaymentMethodVirtualAccount` → [PaymentMethodVirtualAccount](#paymentmethodvirtualaccount)

### PaymentMethodCard

결제수단 카드 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `card` | [Card](../00-basics/schemas-common.md#card) |  | 카드 상세 정보 |
| `approvalNumber` | string |  | 승인 번호 |
| `installment` | [PaymentInstallment](#paymentinstallment) |  | 할부 정보 |
| `pointUsed` | boolean |  | 카드 포인트 사용여부 |

### PaymentMethodConvenienceStore

편의점 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `convenienceStoreBrand` | [ConvenienceStoreBrand](#conveniencestorebrand) |  | 편의점 브랜드 |
| `confirmationNumber` | string |  | 결제 확인 번호 |
| `receiptNumber` | string |  | 결제 접수 번호 |
| `paymentDeadline` | string(date-time) |  | 결제 마감 시간 |

### PaymentMethodCrypto

암호화폐 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PaymentMethodEasyPay

간편 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `provider` | [EasyPayProvider](../00-basics/schemas-common.md#easypayprovider) |  | 간편 결제 PG사 |
| `easyPayMethod` | [PaymentMethodEasyPayMethod](#paymentmethodeasypaymethod) |  | 간편 결제 수단 |

### PaymentMethodEasyPayMethod

간편 결제 수단

oneOf, discriminator `type`:

- `PaymentMethodCard` → [PaymentMethodCard](#paymentmethodcard)
- `PaymentMethodEasyPayMethodCharge` → [PaymentMethodEasyPayMethodCharge](#paymentmethodeasypaymethodcharge)
- `PaymentMethodTransfer` → [PaymentMethodTransfer](#paymentmethodtransfer)

### PaymentMethodEasyPayMethodCharge

충전식 포인트 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `bank` | [Bank](../00-basics/schemas-common.md#bank) |  | 표준 은행 코드 |

### PaymentMethodGiftCertificate

상품권 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `giftCertificateType` | [PaymentMethodGiftCertificateType](#paymentmethodgiftcertificatetype) |  | 상품권 종류 |
| `approvalNumber` | string | Y | 상품권 승인 번호 |

### PaymentMethodGiftCertificateType

상품권 종류

- `BOOKNLIFE`
- `SMART_MUNSANG`
- `CULTURELAND`
- `HAPPYMONEY`
- `CULTUREGIFT`

### PaymentMethodMobile

모바일 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `phoneNumber` | string |  | 전화번호 |

### PaymentMethodTransfer

계좌 이체 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `bank` | [Bank](../00-basics/schemas-common.md#bank) |  | 표준 은행 코드 |
| `accountNumber` | string |  | 계좌번호 |

### PaymentMethodVirtualAccount

가상계좌 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `bank` | [Bank](../00-basics/schemas-common.md#bank) |  | 표준 은행 코드 |
| `accountNumber` | string | Y | 계좌번호 |
| `accountType` | [PaymentMethodVirtualAccountType](#paymentmethodvirtualaccounttype) |  | 계좌 유형 |
| `remitteeName` | string |  | 계좌주 |
| `remitterName` | string |  | 송금인(입금자) |
| `expiredAt` | string(date-time) |  | 입금만료시점 |
| `issuedAt` | string(date-time) |  | 계좌발급시점 |
| `refundStatus` | [PaymentMethodVirtualAccountRefundStatus](#paymentmethodvirtualaccountrefundstatus) |  | 가상계좌 결제가 환불 단계일 때의 환불 상태 |

### PaymentMethodVirtualAccountRefundStatus

가상계좌 환불 상태

- `PENDING` — 처리중
- `PARTIAL_REFUND_FAILED` — 부분 환불 실패
- `FAILED` — 환불 실패
- `COMPLETED` — 환불 완료

### PaymentMethodVirtualAccountType

가상계좌 유형

- `FIXED` — 고정식
- `NORMAL` — 회전식

### PaymentOrigin

결제 출처 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `platformType` | [PaymentOriginPlatformType](#paymentoriginplatformtype) | Y | 결제를 요청한 플랫폼 타입 |
| `userAgent` | string |  | 결제를 요청한 user agent 문자열 |
| `url` | string |  | 결제를 요청한 페이지 URL |
| `ipAddress` | string | Y | 결제를 요청한 IP 주소 |

### PaymentOriginPlatformType

플랫폼 타입

- `PC`
- `MOBILE`
- `API`

### PaymentSortBy

결제 건 정렬 기준

- `REQUESTED_AT` — 결제 요청 시점
- `STATUS_CHANGED_AT` — 상태 변경 시점

### PaymentStatus

결제 건 상태

- `READY`
- `PENDING`
- `VIRTUAL_ACCOUNT_ISSUED`
- `PAID`
- `FAILED`
- `PARTIAL_CANCELLED`
- `CANCELLED`

### PaymentTextSearch

통합검색 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `field` | [PaymentTextSearchField](#paymenttextsearchfield) | Y |  |
| `value` | string | Y |  |

### PaymentTextSearchField

통합검색 항목

- `ALL`
- `PAYMENT_ID`
- `TX_ID`
- `SCHEDULE_ID`
- `FAIL_REASON`
- `CARD_ISSUER`
- `CARD_ACQUIRER`
- `CARD_BIN`
- `CARD_NUMBER`
- `CARD_APPROVAL_NUMBER`
- `CARD_RECEIPT_NAME`
- `CARD_INSTALLMENT`
- `TRANS_BANK`
- `VIRTUAL_ACCOUNT_HOLDER_NAME`
- `VIRTUAL_ACCOUNT_BANK`
- `VIRTUAL_ACCOUNT_NUMBER`
- `PG_MERCHANT_ID`
- `PG_TX_ID`
- `PG_RECEIPT_ID`
- `RECEIPT_APPROVAL_NUMBER`
- `PG_CANCELLATION_ID`
- `CANCEL_REASON`
- `ORDER_NAME`
- `CUSTOMER_NAME`
- `CUSTOMER_EMAIL`
- `CUSTOMER_PHONE_NUMBER`
- `CUSTOMER_ADDRESS`
- `CUSTOMER_ZIPCODE`
- `USER_AGENT`
- `BILLING_KEY`
- `PROMOTION_ID`
- `GIFT_CERTIFICATION_APPROVAL_NUMBER`

### PaymentTimestampType

조회 시점 기준 — 조회 시점 기준<br>어떤 시점을 기준으로 조회를 할 것인지 선택합니다.<br>CREATED_AT: 결제 건 생성 시점을 기준으로 조회합니다.<br>STATUS_CHANGED_AT: 상태 승인 시점을 기준으로 조회합니다. 결제 건의 최종 상태에 따라 검색 기준이 다르게 적용됩니다.<br>ready -> 결제 요청 시점 기준<br>paid -> 결제 완료 시점 기준<br>cancelled -> 결제 취소 시점 기준<br>failed -> 결제 실패 시점 기준<br>값을 입력하지 않으면 STATUS_CHANGED_AT 으로 자동 적용됩니다.

- `CREATED_AT` — 결제 건 생성 시점
- `STATUS_CHANGED_AT` — 상태 변경 시점

### PaymentTransaction

결제 시도

oneOf, discriminator `status`:

- `CANCELLED` → [CancelledPaymentTransaction](#cancelledpaymenttransaction)
- `FAILED` → [FailedPaymentTransaction](#failedpaymenttransaction)
- `PAID` → [PaidPaymentTransaction](#paidpaymenttransaction)
- `PARTIAL_CANCELLED` → [PartialCancelledPaymentTransaction](#partialcancelledpaymenttransaction)
- `PAY_PENDING` → [PayPendingPaymentTransaction](#paypendingpaymenttransaction)
- `READY` → [ReadyPaymentTransaction](#readypaymenttransaction)
- `VIRTUAL_ACCOUNT_ISSUED` → [VirtualAccountIssuedPaymentTransaction](#virtualaccountissuedpaymenttransaction)

### PaymentWebhook

성공 웹훅 내역

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `paymentStatus` | [PaymentWebhookPaymentStatus](#paymentwebhookpaymentstatus) |  | 웹훅 발송 시 결제 건 상태 — V1 결제 건인 경우, 값이 존재하지 않습니다. |
| `id` | string | Y | 웹훅 아이디 |
| `status` | [PaymentWebhookStatus](#paymentwebhookstatus) |  | 웹훅 상태 |
| `url` | string | Y | 웹훅이 발송된 url — V1 결제 건인 경우, 값이 존재하지 않습니다. |
| `isAsync` | boolean |  | 비동기 웹훅 여부 — V1 결제 건인 경우, 값이 존재하지 않습니다. |
| `currentExecutionCount` | integer(int32) |  | 현재 발송 횟수 |
| `maxExecutionCount` | integer(int32) |  | 최대 발송 횟수 |
| `trigger` | [PaymentWebhookTrigger](#paymentwebhooktrigger) |  | 웹훅 실행 맥락 |
| `request` | [PaymentWebhookRequest](#paymentwebhookrequest) |  | 웹훅 요청 정보 |
| `response` | [PaymentWebhookResponse](#paymentwebhookresponse) |  | 웹훅 응답 정보 |
| `triggeredAt` | string(date-time) |  | 웹훅 처리 시작 시점 |

### PaymentWebhookPaymentStatus

웹훅 발송 시 결제 건 상태

- `READY`
- `VIRTUAL_ACCOUNT_ISSUED`
- `PAID`
- `FAILED`
- `PARTIAL_CANCELLED`
- `CANCELLED`
- `PAY_PENDING`

### PaymentWebhookRequest

웹훅 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `header` | string |  | 요청 헤더 |
| `body` | string | Y | 요청 본문 |
| `requestedAt` | string(date-time) |  | 요청 시점 |

### PaymentWebhookResponse

웹훅 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `code` | string | Y | 응답 HTTP 코드 |
| `header` | string | Y | 응답 헤더 |
| `body` | string | Y | 응답 본문 |
| `respondedAt` | string(date-time) | Y | 응답 시점 |

### PaymentWebhookStatus

웹훅 전송 상태

- `SUCCEEDED`
- `FAILED_NOT_OK_RESPONSE`
- `FAILED_UNEXPECTED_ERROR`

### PaymentWebhookTrigger

웹훅 실행 트리거 — 웹훅 실행 트리거<br>수동 웹훅 재발송, 가상계좌 입금, 비동기 취소 승인 시 발생한 웹훅일 때 필드의 값이 존재합니다.

- `MANUAL`
- `VIRTUAL_ACCOUNT_DEPOSIT`
- `ASYNC_CANCEL_APPROVED`
- `ASYNC_CANCEL_FAILED`
- `ASYNC_PAY_APPROVED`
- `ASYNC_PAY_FAILED`
- `ASYNC_PAY_PENDING`
- `DISPUTE_CREATED`
- `DISPUTE_RESOLVED`

### PaymentWithCursor

결제 건 및 커서 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payment` | [Payment](#payment) | Y | 결제 건 정보 |
| `cursor` | string | Y | 해당 결제 건의 커서 정보 |

### PreRegisterPaymentBody

결제 정보 사전 등록 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `totalAmount` | integer(int64) |  | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 결제 면세 금액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 |

### PreRegisterPaymentResponse

결제 사전 등록 성공 응답

type: object

### PromotionDiscountRetainOption

PromotionDiscountRetainOption

- `RETAIN`
- `RELEASE`

### ReadyPayment

준비 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) |  | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제의 배송 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |

### ReadyPaymentTransaction

준비 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) |  | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제의 배송 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |

### RegisterEscrowLogisticsBody

에스크로 배송 정보 등록 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `sender` | [PaymentEscrowSenderInput](#paymentescrowsenderinput) |  | 에스크로 발송자 정보 |
| `receiver` | [PaymentEscrowReceiverInput](#paymentescrowreceiverinput) |  | 에스크로 수취인 정보 |
| `logistics` | [PaymentLogistics](#paymentlogistics) | Y | 에스크로 물류 정보 |
| `sendEmail` | boolean |  | 이메일 알림 전송 여부 — 에스크로 구매 확정 시 이메일로 알림을 보낼지 여부입니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |

### RegisterStoreReceiptBody

영수증 내 하위 상점 거래 등록 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [RegisterStoreReceiptBodyItem](#registerstorereceiptbodyitem)[] | Y | 하위 상점 거래 목록 |
| `storeId` | string |  | 상점 아이디 |

### RegisterStoreReceiptBodyItem

하위 상점 거래 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeBusinessRegistrationNumber` | string | Y | 하위 상점 사업자등록번호 |
| `storeName` | string | Y | 하위 상점명 |
| `totalAmount` | integer(int64) | Y | 결제 총 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세액 |
| `vatAmount` | integer(int64) |  | 부가세액 |
| `supplyAmount` | integer(int64) |  | 공급가액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |

### RegisterStoreReceiptResponse

영수증 내 하위 상점 거래 등록 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `receiptUrl` | string |  | 결제 영수증 URL |

### RegisteredPaymentEscrow

배송 정보 등록 완료

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |

### RejectConfirmedPaymentEscrow

구매 거절 확정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |

### RejectedPaymentEscrow

구매 거절

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 에스크로 상태 |
| `company` | string | Y | 택배사 |
| `invoiceNumber` | string | Y | 송장번호 |
| `sentAt` | string(date-time) |  | 발송 일시 |
| `appliedAt` | string(date-time) |  | 배송등록 처리 일자 |

### RequestedPaymentCancellation

취소 요청 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 취소 내역 상태 |
| `id` | string | Y | 취소 내역 아이디 |
| `pgCancellationId` | string |  | PG사 결제 취소 내역 아이디 |
| `totalAmount` | integer(int64) | Y | 취소 금액 |
| `taxFreeAmount` | integer(int64) | Y | 취소 금액 중 면세 금액 |
| `vatAmount` | integer(int64) | Y | 취소 금액 중 부가세액 |
| `easyPayDiscountAmount` | integer(int64) |  | 적립형 포인트의 환불 금액 |
| `reason` | string | Y | 취소 사유 |
| `cancelledAt` | string(date-time) |  | 취소 시점 |
| `requestedAt` | string(date-time) | Y | 취소 요청 시점 |
| `trigger` | [Trigger](#trigger) |  | 취소 요청 경로 |

### ResendWebhookBody

ResendWebhookBody — 웹훅 재발송을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `webhookId` | string |  | 웹훅 아이디 — 입력하지 않으면 결제 건의 가장 최근 웹훅 아이디가 기본 적용됩니다 |

### ResendWebhookResponse

웹훅 재발송 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `webhook` | [PaymentWebhook](#paymentwebhook) | Y | 재발송 웹훅 정보 |

### StopPaymentCancellationBody

결제 취소 요청 취소 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |

### StopPaymentCancellationResponse

결제 취소 요청 취소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `stoppedAt` | string(date-time) | Y | 결제 취소 요청 취소 완료 시각 |

### SucceededPaymentCancellation

취소 완료 상태

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 취소 내역 상태 |
| `id` | string | Y | 취소 내역 아이디 |
| `pgCancellationId` | string |  | PG사 결제 취소 내역 아이디 |
| `totalAmount` | integer(int64) | Y | 취소 금액 |
| `taxFreeAmount` | integer(int64) | Y | 취소 금액 중 면세 금액 |
| `vatAmount` | integer(int64) | Y | 취소 금액 중 부가세액 |
| `easyPayDiscountAmount` | integer(int64) |  | 적립형 포인트의 환불 금액 |
| `reason` | string | Y | 취소 사유 |
| `cancelledAt` | string(date-time) |  | 취소 시점 |
| `requestedAt` | string(date-time) | Y | 취소 요청 시점 |
| `receiptUrl` | string |  | 취소 영수증 URL |
| `receiptUrlWithoutAuthentication` | string |  | 본인인증 없이 열람 가능한 취소 영수증 URL — 나이스정보통신에서 제공합니다. 이용시 사전 협의가 필요합니다. |
| `trigger` | [Trigger](#trigger) |  | 취소 요청 경로 |

### Trigger

Trigger

- `CONSOLE`
- `API`
- `PORTONE_ADMIN`
- `CHARGEBACK`

### VirtualAccountIssuedPayment

가상계좌 발급 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 건 상태 |
| `id` | string | Y | 결제 건 아이디 |
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `origin` | [PaymentOrigin](#paymentorigin) | Y | 결제 출처 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |

### VirtualAccountIssuedPaymentTransaction

가상계좌 발급 완료 상태 건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | string | Y | 결제 시도 상태 |
| `id` | string | Y | 결제 시도 아이디 (transactionId) — V1 결제 건의 경우 imp_uid에 해당합니다. |
| `paymentId` | string | Y | 결제 건 아이디 |
| `merchantId` | string | Y | 고객사 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `method` | [PaymentMethod](#paymentmethod) |  | 결제수단 정보 |
| `channel` | [SelectedChannel](../00-basics/schemas-common.md#selectedchannel) | Y | 결제 채널 |
| `channelGroup` | [ChannelGroupSummary](../00-basics/schemas-common.md#channelgroupsummary) |  | 결제 채널 그룹 정보 |
| `version` | [PortOneVersion](../00-basics/schemas-common.md#portoneversion) | Y | 포트원 버전 |
| `scheduleId` | string |  | 결제 예약 건 아이디 — 결제 예약을 이용한 경우에만 존재 |
| `billingKey` | string |  | 결제 시 사용된 빌링키 — 빌링키 결제인 경우에만 존재 |
| `webhooks` | [PaymentWebhook](#paymentwebhook)[] |  | 웹훅 발송 내역 |
| `requestedAt` | string(date-time) | Y | 결제 요청 시점 |
| `updatedAt` | string(date-time) | Y | 업데이트 시점 |
| `statusChangedAt` | string(date-time) | Y | 상태 업데이트 시점 |
| `orderName` | string | Y | 주문명 |
| `amount` | [PaymentAmount](#paymentamount) | Y | 결제 금액 관련 세부 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `customer` | [Customer](../00-basics/schemas-common.md#customer) | Y | 구매자 정보 |
| `promotionId` | string |  | 프로모션 아이디 |
| `isCulturalExpense` | boolean |  | 문화비 지출 여부 |
| `escrow` | [PaymentEscrow](#paymentescrow) |  | 에스크로 결제 정보 — 에스크로 결제인 경우 존재합니다. |
| `products` | [PaymentProduct](../00-basics/schemas-common.md#paymentproduct)[] |  | 상품 정보 |
| `productCount` | integer(int32) |  | 상품 갯수 |
| `customData` | string |  | 사용자 지정 데이터 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 국가 코드 |
| `pgTxId` | string |  | PG사 거래 아이디 |
