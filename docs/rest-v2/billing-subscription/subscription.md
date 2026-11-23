# 구독 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /customers` — 구독 고객 다건 조회 (subscription.customer)
- `POST /customers` — 구독 고객 등록 (subscription.customer)
- `GET /customers/{customerId}` — 구독 고객 조회 (subscription.customer)
- `PATCH /customers/{customerId}` — 구독 고객 정보 수정 (subscription.customer)
- `GET /customers/{customerId}/invoices` — 고객의 인보이스 다건 조회 (subscription.customer)
- `POST /customers/{customerId}/payment-method/revoke` — 구독 고객 결제수단 해지 (subscription.customer)
- `GET /customers/{customerId}/subscriptions` — 고객의 구독 다건 조회 (subscription.customer)
- `GET /discounts` — 구독 할인 다건 조회 (subscription.discount)
- `POST /discounts` — 구독 할인 생성 (subscription.discount)
- `GET /discounts/{discountId}` — 구독 할인 조회 (subscription.discount)
- `PATCH /discounts/{discountId}` — 구독 할인 수정 (subscription.discount)
- `POST /discounts/{discountId}/archive` — 구독 할인 보관 (subscription.discount)
- `POST /discounts/{discountId}/unarchive` — 구독 할인 보관 해제 (subscription.discount)
- `GET /invoice-email-templates` — 인보이스 메일 템플릿 다건 조회 (subscription.invoiceEmailTemplate)
- `POST /invoice-email-templates` — 인보이스 메일 템플릿 생성 (subscription.invoiceEmailTemplate)
- `GET /invoice-email-templates/{invoiceEmailTemplateId}` — 인보이스 메일 템플릿 조회 (subscription.invoiceEmailTemplate)
- `DELETE /invoice-email-templates/{invoiceEmailTemplateId}` — 인보이스 메일 템플릿 삭제 (subscription.invoiceEmailTemplate)
- `PATCH /invoice-email-templates/{invoiceEmailTemplateId}` — 인보이스 메일 템플릿 수정 (subscription.invoiceEmailTemplate)
- `POST /invoice-email-templates/{invoiceEmailTemplateId}/activate` — 인보이스 메일 템플릿 활성화 (subscription.invoiceEmailTemplate)
- `POST /invoice-email-templates/{invoiceEmailTemplateId}/deactivate` — 인보이스 메일 템플릿 비활성화 (subscription.invoiceEmailTemplate)
- `GET /invoices` — 구독 인보이스 다건 조회 (subscription.invoice)
- `POST /invoices` — 구독 인보이스 수동 청구 (subscription.invoice)
- `GET /invoices-stats` — 구독 인보이스 집계 조회 (subscription.invoice)
- `POST /invoices/{invoiceId}/cancel` — 구독 인보이스 취소 (subscription.invoice)
- `POST /invoices/{invoiceId}/regenerate` — 구독 인보이스 재생성 (subscription.invoice)
- `POST /invoices/{invoiceId}/retry` — 구독 인보이스 결제 재시도 (subscription.invoice)
- `POST /invoices/{invoiceId}/settle` — 구독 인보이스 수동 완납 (subscription.invoice)
- `GET /invoices/{invoiceId}` — 구독 인보이스 조회 (subscription.invoice)
- `GET /products` — 구독 상품 다건 조회 (subscription.product)
- `POST /products` — 구독 상품 생성 (subscription.product)
- `GET /products/{productId}` — 구독 상품 조회 (subscription.product)
- `PATCH /products/{productId}` — 구독 상품 수정 (subscription.product)
- `POST /products/{productId}/archive` — 구독 상품 보관 (subscription.product)
- `POST /products/{productId}/pause-billing` — 구독 상품 청구 중지 (subscription.product)
- `POST /products/{productId}/resume-billing` — 구독 상품 청구 재개 (subscription.product)
- `GET /subscriptions` — 구독 다건 조회 (subscription.subscription)
- `POST /subscriptions` — 구독 생성 (subscription.subscription)
- `POST /subscriptions/pre-register` — 구독 사전 등록 (subscription.subscription)
- `GET /subscriptions/{subscriptionId}` — 구독 조회 (subscription.subscription)
- `PATCH /subscriptions/{subscriptionId}` — 구독 수정 (subscription.subscription)
- `POST /subscriptions/{subscriptionId}/cancel` — 구독 해지 (subscription.subscription)
- `POST /subscriptions/{subscriptionId}/charge-test` — 테스트 구독 즉시 청구 (subscription.subscription)
- `GET /subscriptions/{subscriptionId}/discounts` — 구독에 붙은 할인 조회 (subscription.subscription)
- `POST /subscriptions/{subscriptionId}/discounts` — 구독에 할인 추가 (subscription.subscription)
- `DELETE /subscriptions/{subscriptionId}/discounts/{discountId}` — 구독에서 할인 제거 (subscription.subscription)
- `GET /subscriptions/{subscriptionId}/invoice-preview` — 다음 인보이스 미리보기 (subscription.subscription)
- `GET /subscriptions/{subscriptionId}/invoices` — 구독의 인보이스 다건 조회 (subscription.subscription)
- `POST /subscriptions/{subscriptionId}/pause` — 구독 일시중지 (subscription.subscription)
- `POST /subscriptions/{subscriptionId}/resume` — 구독 재개 (subscription.subscription)

## GET /customers

**구독 고객 다건 조회** (operationId: `getSubscriptionCustomers`)

조건에 맞는 구독 고객 리스트를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionCustomersBody](#getsubscriptioncustomersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 고객만 조회합니다. |

### 응답

- **200** 성공 응답으로 고객 리스트를 반환합니다.: [GetSubscriptionCustomersResponse](#getsubscriptioncustomersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /customers

**구독 고객 등록** (operationId: `createSubscriptionCustomer`)

빌링키와 함께 구독 고객을 등록합니다. 이미 있는 고객이면 입력한 필드만 갱신합니다. 이미 빌링키가 있는 고객이면 새 빌링키로 교체하며, 이전 빌링키는 해지하지 않습니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionCustomerBody](#createsubscriptioncustomerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 고객 아이디 — 미입력 시 새 고객으로 등록하고 아이디를 자동 생성합니다. 입력한 아이디의 고객이 있으면 그 고객을 갱신합니다. |
| `name` | string |  | 고객명 — 신규 고객이면 필수입니다. |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `billingKey` | string | Y | 등록할 빌링키 — 해당 상점의 카드 빌링키여야 합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |
| `subscription` | [SubscriptionCustomerCreateSubscriptionInput](#subscriptioncustomercreatesubscriptioninput) |  | 함께 만들 구독 — 입력하면 고객 등록과 구독 생성을 한 번에 처리합니다. 구독 생성이 실패하면 고객도 만들지 않습니다. |

### 응답

- **200** 성공 응답으로 등록된 고객과 함께 만들어진 구독을 반환합니다.: [CreateSubscriptionCustomerResponse](#createsubscriptioncustomerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionBillingKeyNotFoundError`](../00-basics/errors.md#subscriptionbillingkeynotfounderror): 구독에 사용할 수 있는 빌링키가 아닌 경우
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우
- **409**
  * [`SubscriptionBillingKeyAlreadyRegisteredError`](../00-basics/errors.md#subscriptionbillingkeyalreadyregisterederror): 다른 구독 고객이 이미 사용 중인 빌링키인 경우
  * [`SubscriptionBillingKeyModeMismatchError`](../00-basics/errors.md#subscriptionbillingkeymodemismatcherror): 구독 고객 모드와 다른 모드의 빌링키인 경우
  * [`SubscriptionAlreadyExistsError`](../00-basics/errors.md#subscriptionalreadyexistserror): 같은 아이디의 구독이 이미 존재하는 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우
  * [`SubscriptionCustomerEmailRequiredError`](../00-basics/errors.md#subscriptioncustomeremailrequirederror): 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우
  * [`SubscriptionProductArchivedError`](../00-basics/errors.md#subscriptionproductarchivederror): 구독 상품이 보관되어 수정할 수 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우
  * [`SubscriptionIntervalMismatchError`](../00-basics/errors.md#subscriptionintervalmismatcherror): 담긴 정기 상품들의 청구 주기가 서로 다르거나, 시작한 구독의 청구 주기와 다른 경우
  * [`SubscriptionCurrencyMismatchError`](../00-basics/errors.md#subscriptioncurrencymismatcherror): 담긴 상품들의 통화가 다른 경우
  * [`SubscriptionDiscountArchivedError`](../00-basics/errors.md#subscriptiondiscountarchivederror): 구독 할인이 보관된 경우
  * [`SubscriptionDiscountRedeemByPassedError`](../00-basics/errors.md#subscriptiondiscountredeembypassederror): 구독 할인의 적용 마감일이 지난 경우
  * [`SubscriptionDiscountAlreadyAppliedError`](../00-basics/errors.md#subscriptiondiscountalreadyappliederror): 같은 구독 할인이 이미 붙어 있는 경우
  * [`SubscriptionDiscountProductsNotInSubscriptionError`](../00-basics/errors.md#subscriptiondiscountproductsnotinsubscriptionerror): 구독 할인의 적용 대상 상품이 구독에 담겨 있지 않은 경우
  * [`SubscriptionDiscountCurrencyMismatchError`](../00-basics/errors.md#subscriptiondiscountcurrencymismatcherror): 구독 할인의 통화가 구독의 통화와 다른 경우

## GET /customers/{customerId}

**구독 고객 조회** (operationId: `getSubscriptionCustomer`)

주어진 아이디로 구독 고객을 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `customerId` | path | string | Y | 고객 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 고객을 반환합니다.: [GetSubscriptionCustomerResponse](#getsubscriptioncustomerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우

## PATCH /customers/{customerId}

**구독 고객 정보 수정** (operationId: `updateSubscriptionCustomer`)

구독 고객의 정보를 수정합니다. 입력하지 않은 필드는 기존 값을 유지하며, 아이디와 빌링키는 바꿀 수 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `customerId` | path | string | Y | 고객 아이디 |

### 요청 본문 (application/json, 필수)

타입: [UpdateSubscriptionCustomerBody](#updatesubscriptioncustomerbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 고객명 |
| `email` | string |  | 이메일 — 변경은 다음 인보이스 메일부터 반영됩니다. |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### 응답

- **200** 성공 응답으로 수정된 고객을 반환합니다.: [UpdateSubscriptionCustomerResponse](#updatesubscriptioncustomerresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우
- **409**
  * [`SubscriptionCustomerEmailRequiredError`](../00-basics/errors.md#subscriptioncustomeremailrequirederror): 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우

## GET /customers/{customerId}/invoices

**고객의 인보이스 다건 조회** (operationId: `getSubscriptionCustomerInvoices`)

한 고객의 모든 구독 인보이스를 청구 일시 역순으로 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `customerId` | path | string | Y | 고객 아이디 |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionCustomerInvoicesBody](#getsubscriptioncustomerinvoicesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `from` | string(date-time) |  | 조회 시작 일시 — 청구 일시 기준입니다. 미입력 시 90일 전입니다. |
| `to` | string(date-time) |  | 조회 종료 일시 — 청구 일시 기준입니다. 미입력 시 현재입니다. |

### 응답

- **200** 성공 응답으로 인보이스 리스트를 반환합니다.: [GetSubscriptionCustomerInvoicesResponse](#getsubscriptioncustomerinvoicesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우

## POST /customers/{customerId}/payment-method/revoke

**구독 고객 결제수단 해지** (operationId: `revokeSubscriptionCustomerPaymentMethod`)

고객의 현재 빌링키를 해지합니다. 구독은 유지되지만 빌링키가 없는 동안에는 청구일이 와도 인보이스를 만들지 않습니다. 빌링키를 다시 등록하면 지난 회차는 청구하지 않고 다음 청구일부터 청구합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `customerId` | path | string | Y | 고객 아이디 |

### 요청 본문 (application/json, 필수)

타입: [RevokeSubscriptionCustomerPaymentMethodBody](#revokesubscriptioncustomerpaymentmethodbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `reason` | string |  | 해지 사유 — 일부 결제사가 필수로 요구하므로 채워 보내는 것을 권장합니다. |

### 응답

- **200** 성공 응답으로 반영된 고객을 반환합니다.: [RevokeSubscriptionCustomerPaymentMethodResponse](#revokesubscriptioncustomerpaymentmethodresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우
- **409**
  * [`SubscriptionBillingKeyHasSchedulesError`](../00-basics/errors.md#subscriptionbillingkeyhasscheduleserror): 해지하려는 빌링키에 예약 결제가 걸려 있는 경우
- **502**
  * [`SubscriptionBillingKeyRevokeFailedError`](../00-basics/errors.md#subscriptionbillingkeyrevokefailederror): 빌링키 해지가 실패한 경우

## GET /customers/{customerId}/subscriptions

**고객의 구독 다건 조회** (operationId: `getSubscriptionCustomerSubscriptions`)

한 고객의 구독을 생성일 역순으로 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `customerId` | path | string | Y | 고객 아이디 |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionCustomerSubscriptionsBody](#getsubscriptioncustomersubscriptionsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `status` | [SubscriptionStatus](#subscriptionstatus) |  | 구독 상태 |

### 응답

- **200** 성공 응답으로 구독 리스트를 반환합니다.: [GetSubscriptionCustomerSubscriptionsResponse](#getsubscriptioncustomersubscriptionsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우

## GET /discounts

**구독 할인 다건 조회** (operationId: `getSubscriptionDiscounts`)

조건에 맞는 구독 할인 리스트를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionDiscountsBody](#getsubscriptiondiscountsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `archived` | boolean |  | 보관한 할인 포함 여부 — 미입력이거나 true면 보관한 할인도 포함해 조회하고, false면 보관하지 않은 할인만 조회합니다. |

### 응답

- **200** 성공 응답으로 구독 할인 리스트를 반환합니다.: [GetSubscriptionDiscountsResponse](#getsubscriptiondiscountsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /discounts

**구독 할인 생성** (operationId: `createSubscriptionDiscount`)

구독에 붙여 인보이스 금액을 깎는 할인을 생성합니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionDiscountBody](#createsubscriptiondiscountbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 할인 아이디 — 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 할인명 |
| `type` | [SubscriptionDiscountType](#subscriptiondiscounttype) | Y | 할인 유형 |
| `value` | integer(int64) | Y | 할인 값 — 정률은 1~100, 정액은 금액입니다. |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 — 정액 할인에만 입력합니다. KRW, USD, JPY만 지원합니다. |
| `redeemBy` | string(date) |  | 적용 마감일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### 응답

- **200** 성공 응답으로 생성된 구독 할인을 반환합니다.: [CreateSubscriptionDiscountResponse](#createsubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`SubscriptionDiscountAlreadyExistsError`](../00-basics/errors.md#subscriptiondiscountalreadyexistserror): 같은 아이디의 구독 할인이 이미 존재하는 경우

## GET /discounts/{discountId}

**구독 할인 조회** (operationId: `getSubscriptionDiscount`)

주어진 아이디로 구독 할인을 조회합니다. 보관된 할인도 조회됩니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `discountId` | path | string | Y | 구독 할인 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 구독 할인을 반환합니다.: [GetSubscriptionDiscountResponse](#getsubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우

## PATCH /discounts/{discountId}

**구독 할인 수정** (operationId: `updateSubscriptionDiscount`)

구독 할인의 이름을 수정합니다. 적용 마감일이 지난 할인도 이름은 바꿀 수 있습니다. 할인 조건과 적용 마감일은 생성 후 바꿀 수 없으므로 바꾸려면 새 할인을 만들어야 합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `discountId` | path | string | Y | 구독 할인 아이디 |

### 요청 본문 (application/json, 필수)

타입: [UpdateSubscriptionDiscountBody](#updatesubscriptiondiscountbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 할인명 |

### 응답

- **200** 성공 응답으로 수정된 구독 할인을 반환합니다.: [UpdateSubscriptionDiscountResponse](#updatesubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우

## POST /discounts/{discountId}/archive

**구독 할인 보관** (operationId: `archiveSubscriptionDiscount`)

구독 할인을 보관합니다. 보관된 할인은 새로 붙일 수 없고, 이미 붙은 구독에서는 잔여 회차까지 적용됩니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `discountId` | path | string | Y | 구독 할인 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독 할인을 반환합니다.: [ArchiveSubscriptionDiscountResponse](#archivesubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우

## POST /discounts/{discountId}/unarchive

**구독 할인 보관 해제** (operationId: `unarchiveSubscriptionDiscount`)

보관된 구독 할인의 보관을 해제해 다시 구독에 붙일 수 있게 합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `discountId` | path | string | Y | 구독 할인 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독 할인을 반환합니다.: [UnarchiveSubscriptionDiscountResponse](#unarchivesubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우

## GET /invoice-email-templates

**인보이스 메일 템플릿 다건 조회** (operationId: `getSubscriptionInvoiceEmailTemplates`)

상점의 인보이스 메일 템플릿 리스트를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionInvoiceEmailTemplatesBody](#getsubscriptioninvoiceemailtemplatesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |

### 응답

- **200** 성공 응답으로 인보이스 메일 템플릿 리스트를 반환합니다.: [GetSubscriptionInvoiceEmailTemplatesResponse](#getsubscriptioninvoiceemailtemplatesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /invoice-email-templates

**인보이스 메일 템플릿 생성** (operationId: `createSubscriptionInvoiceEmailTemplate`)

인보이스 메일 템플릿을 생성합니다. 생성한 템플릿은 비활성 상태입니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionInvoiceEmailTemplateBody](#createsubscriptioninvoiceemailtemplatebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string | Y | 템플릿 이름 |
| `logoImageUrl` | string |  | 로고 이미지 주소 — 메일 상단에 표시할 로고 이미지 주소입니다. 인증 없이 열리는 https 주소를 입력해 주세요. 가로 100px, 세로 24px 영역에 비율을 유지해 표시됩니다. |
| `senders` | [SubscriptionInvoiceEmailSenders](#subscriptioninvoiceemailsenders) | Y | 발신자 정보 |
| `events` | [SubscriptionInvoiceEmailEventType](#subscriptioninvoiceemaileventtype)[] | Y | 보낼 메일 유형 |

### 응답

- **200** 성공 응답으로 생성된 인보이스 메일 템플릿을 반환합니다.: [CreateSubscriptionInvoiceEmailTemplateResponse](#createsubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /invoice-email-templates/{invoiceEmailTemplateId}

**인보이스 메일 템플릿 조회** (operationId: `getSubscriptionInvoiceEmailTemplate`)

주어진 아이디로 인보이스 메일 템플릿을 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceEmailTemplateId` | path | string | Y | 인보이스 메일 템플릿 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 인보이스 메일 템플릿을 반환합니다.: [GetSubscriptionInvoiceEmailTemplateResponse](#getsubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceEmailTemplateNotFoundError`](../00-basics/errors.md#subscriptioninvoiceemailtemplatenotfounderror): 요청된 인보이스 메일 템플릿이 존재하지 않는 경우

## DELETE /invoice-email-templates/{invoiceEmailTemplateId}

**인보이스 메일 템플릿 삭제** (operationId: `deleteSubscriptionInvoiceEmailTemplate`)

인보이스 메일 템플릿을 삭제합니다. 활성 템플릿은 비활성화한 뒤 삭제할 수 있습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceEmailTemplateId` | path | string | Y | 인보이스 메일 템플릿 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 삭제된 템플릿 아이디를 반환합니다.: [DeleteSubscriptionInvoiceEmailTemplateResponse](#deletesubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceEmailTemplateNotFoundError`](../00-basics/errors.md#subscriptioninvoiceemailtemplatenotfounderror): 요청된 인보이스 메일 템플릿이 존재하지 않는 경우
- **409**
  * [`SubscriptionInvoiceEmailTemplateActiveError`](../00-basics/errors.md#subscriptioninvoiceemailtemplateactiveerror): 활성 인보이스 메일 템플릿이라 삭제할 수 없는 경우

## PATCH /invoice-email-templates/{invoiceEmailTemplateId}

**인보이스 메일 템플릿 수정** (operationId: `updateSubscriptionInvoiceEmailTemplate`)

인보이스 메일 템플릿을 수정합니다. 입력하지 않은 필드는 기존 값을 유지합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceEmailTemplateId` | path | string | Y | 인보이스 메일 템플릿 아이디 |

### 요청 본문 (application/json, 필수)

타입: [UpdateSubscriptionInvoiceEmailTemplateBody](#updatesubscriptioninvoiceemailtemplatebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 템플릿 이름 |
| `logoImageUrl` | string |  | 로고 이미지 주소 — 메일 상단에 표시할 로고 이미지 주소입니다. 인증 없이 열리는 https 주소를 입력해 주세요. 가로 100px, 세로 24px 영역에 비율을 유지해 표시됩니다. |
| `senders` | [UpdateSubscriptionInvoiceEmailTemplateBodySenders](#updatesubscriptioninvoiceemailtemplatebodysenders) |  | 발신자 정보 |
| `events` | [SubscriptionInvoiceEmailEventType](#subscriptioninvoiceemaileventtype)[] |  | 보낼 메일 유형 — 입력한 값으로 전체를 교체합니다. |

### 응답

- **200** 성공 응답으로 수정된 인보이스 메일 템플릿을 반환합니다.: [UpdateSubscriptionInvoiceEmailTemplateResponse](#updatesubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceEmailTemplateNotFoundError`](../00-basics/errors.md#subscriptioninvoiceemailtemplatenotfounderror): 요청된 인보이스 메일 템플릿이 존재하지 않는 경우

## POST /invoice-email-templates/{invoiceEmailTemplateId}/activate

**인보이스 메일 템플릿 활성화** (operationId: `activateSubscriptionInvoiceEmailTemplate`)

템플릿을 활성화합니다. 상점의 다른 활성 템플릿은 비활성화됩니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceEmailTemplateId` | path | string | Y | 인보이스 메일 템플릿 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 인보이스 메일 템플릿을 반환합니다.: [ActivateSubscriptionInvoiceEmailTemplateResponse](#activatesubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceEmailTemplateNotFoundError`](../00-basics/errors.md#subscriptioninvoiceemailtemplatenotfounderror): 요청된 인보이스 메일 템플릿이 존재하지 않는 경우

## POST /invoice-email-templates/{invoiceEmailTemplateId}/deactivate

**인보이스 메일 템플릿 비활성화** (operationId: `deactivateSubscriptionInvoiceEmailTemplate`)

템플릿을 비활성화합니다. 활성 템플릿이 없으면 인보이스 메일을 보내지 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceEmailTemplateId` | path | string | Y | 인보이스 메일 템플릿 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 인보이스 메일 템플릿을 반환합니다.: [DeactivateSubscriptionInvoiceEmailTemplateResponse](#deactivatesubscriptioninvoiceemailtemplateresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceEmailTemplateNotFoundError`](../00-basics/errors.md#subscriptioninvoiceemailtemplatenotfounderror): 요청된 인보이스 메일 템플릿이 존재하지 않는 경우

## GET /invoices

**구독 인보이스 다건 조회** (operationId: `getSubscriptionInvoicesAll`)

상점의 구독 인보이스를 조건에 맞춰 조회합니다. 기본은 생성 일시 역순입니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionInvoicesAllBody](#getsubscriptioninvoicesallbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `statuses` | [SubscriptionInvoiceStatus](#subscriptioninvoicestatus)[] |  | 인보이스 상태 리스트 — 미입력 시 모든 상태를 조회합니다. |
| `filter` | [SubscriptionInvoiceFilterInput](#subscriptioninvoicefilterinput) |  | 조회 조건 |
| `sortBy` | [SubscriptionInvoiceSortBy](#subscriptioninvoicesortby) |  | 정렬 기준 — 미입력 시 조회 기준 시점입니다. |
| `sortOrder` | [SortOrder](../00-basics/schemas-common.md#sortorder) |  | 정렬 방향 — 미입력 시 내림차순입니다. |

### 응답

- **200** 성공 응답으로 인보이스 리스트를 반환합니다.: [GetSubscriptionInvoicesAllResponse](#getsubscriptioninvoicesallresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /invoices

**구독 인보이스 수동 청구** (operationId: `createSubscriptionInvoice`)

정기 청구와 별개로 인보이스를 만들어 즉시 결제합니다. 결제 결과는 반환된 인보이스의 상태로 확인합니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionInvoiceBody](#createsubscriptioninvoicebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `subscriptionId` | string | Y | 구독 아이디 |
| `items` | [SubscriptionInvoiceItemForm](#subscriptioninvoiceitemform)[] | Y | 청구 항목 — 등록된 상품이나 임의 항목을 담을 수 있습니다. |
| `memo` | string |  | 메모 |

### 응답

- **200** 성공 응답으로 생성된 인보이스를 반환합니다.: [CreateSubscriptionInvoiceResponse](#createsubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우

## GET /invoices-stats

**구독 인보이스 집계 조회** (operationId: `getSubscriptionInvoiceStats`)

목록과 같은 조건으로 인보이스의 상태별 건수와 통화별 금액을 집계합니다. 상태별로 세므로 상태 조건은 받지 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionInvoiceStatsBody](#getsubscriptioninvoicestatsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `filter` | [SubscriptionInvoiceFilterInput](#subscriptioninvoicefilterinput) |  | 조회 조건 |

### 응답

- **200** 성공 응답으로 상태별 건수와 통화별 집계를 반환합니다.: [GetSubscriptionInvoiceStatsResponse](#getsubscriptioninvoicestatsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /invoices/{invoiceId}/cancel

**구독 인보이스 취소** (operationId: `cancelSubscriptionInvoice`)

청구를 취소합니다. 이미 결제된 인보이스는 취소할 수 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceId` | path | string | Y | 인보이스 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 인보이스를 반환합니다.: [CancelSubscriptionInvoiceResponse](#cancelsubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceNotFoundError`](../00-basics/errors.md#subscriptioninvoicenotfounderror): 요청된 구독 인보이스가 존재하지 않는 경우
- **409**
  * [`SubscriptionInvoiceStatusNotAllowedError`](../00-basics/errors.md#subscriptioninvoicestatusnotallowederror): 현재 인보이스 상태에서 허용되지 않는 요청인 경우

## POST /invoices/{invoiceId}/regenerate

**구독 인보이스 재생성** (operationId: `regenerateSubscriptionInvoice`)

취소한 정기 청구 회차를 지금 시점의 상품·할인으로 다시 계산해 새 인보이스로 만들고 결제합니다. 원래 인보이스는 취소된 채로 남습니다. 금액을 그대로 다시 결제하려면 결제 재시도를 사용합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceId` | path | string | Y | 인보이스 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 새로 만들어진 인보이스를 반환합니다.: [RegenerateSubscriptionInvoiceResponse](#regeneratesubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceNotFoundError`](../00-basics/errors.md#subscriptioninvoicenotfounderror): 요청된 구독 인보이스가 존재하지 않는 경우
- **409**
  * [`SubscriptionInvoiceStatusNotAllowedError`](../00-basics/errors.md#subscriptioninvoicestatusnotallowederror): 현재 인보이스 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우
  * [`SubscriptionInvoiceAlreadyRegeneratedError`](../00-basics/errors.md#subscriptioninvoicealreadyregeneratederror): 이미 재생성된 구독 인보이스인 경우

## POST /invoices/{invoiceId}/retry

**구독 인보이스 결제 재시도** (operationId: `retrySubscriptionInvoice`)

결제에 실패한 인보이스를 같은 금액으로 다시 결제합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceId` | path | string | Y | 인보이스 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 인보이스를 반환합니다.: [RetrySubscriptionInvoiceResponse](#retrysubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceNotFoundError`](../00-basics/errors.md#subscriptioninvoicenotfounderror): 요청된 구독 인보이스가 존재하지 않는 경우
- **409**
  * [`SubscriptionInvoiceStatusNotAllowedError`](../00-basics/errors.md#subscriptioninvoicestatusnotallowederror): 현재 인보이스 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우

## POST /invoices/{invoiceId}/settle

**구독 인보이스 수동 완납** (operationId: `settleSubscriptionInvoice`)

계좌 입금 등으로 대금을 따로 받은 인보이스를 완납 처리합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceId` | path | string | Y | 인보이스 아이디 |

### 요청 본문 (application/json, 필수)

타입: [SettleSubscriptionInvoiceBody](#settlesubscriptioninvoicebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `method` | [SubscriptionInvoiceSettlementMethod](#subscriptioninvoicesettlementmethod) | Y | 완납 방법 |
| `paidAt` | string(date-time) | Y | 입금 시각 — 실제로 돈을 받은 시각입니다. 지금보다 뒤일 수 없습니다. |
| `memo` | string |  | 메모 — 완납 처리한 경위를 남겨둘 수 있습니다. |

### 응답

- **200** 성공 응답으로 반영된 인보이스를 반환합니다.: [SettleSubscriptionInvoiceResponse](#settlesubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceNotFoundError`](../00-basics/errors.md#subscriptioninvoicenotfounderror): 요청된 구독 인보이스가 존재하지 않는 경우
- **409**
  * [`SubscriptionInvoiceStatusNotAllowedError`](../00-basics/errors.md#subscriptioninvoicestatusnotallowederror): 현재 인보이스 상태에서 허용되지 않는 요청인 경우

## GET /invoices/{invoiceId}

**구독 인보이스 조회** (operationId: `getSubscriptionInvoice`)

주어진 아이디로 구독 인보이스를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `invoiceId` | path | string | Y | 인보이스 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 인보이스를 반환합니다.: [GetSubscriptionInvoiceResponse](#getsubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionInvoiceNotFoundError`](../00-basics/errors.md#subscriptioninvoicenotfounderror): 요청된 구독 인보이스가 존재하지 않는 경우

## GET /products

**구독 상품 다건 조회** (operationId: `getSubscriptionProducts`)

조건에 맞는 구독 상품 리스트를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionProductsBody](#getsubscriptionproductsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `archived` | boolean |  | 보관한 상품 포함 여부 — 미입력이거나 true면 보관한 상품도 포함해 조회하고, false면 보관하지 않은 상품만 조회합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 상품만 조회합니다. |

### 응답

- **200** 성공 응답으로 구독 상품 리스트를 반환합니다.: [GetSubscriptionProductsResponse](#getsubscriptionproductsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /products

**구독 상품 생성** (operationId: `createSubscriptionProduct`)

구독 결제에 사용할 상품을 생성합니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionProductBody](#createsubscriptionproductbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 상품 아이디 — 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 상품명 |
| `description` | string |  | 상품 설명 |
| `pricingType` | [SubscriptionProductPricingType](#subscriptionproductpricingtype) | Y | 과금 유형 |
| `interval` | [SubscriptionInterval](#subscriptioninterval) |  | 청구 주기 — 정기 과금 상품에만 입력합니다. |
| `amount` | integer(int64) | Y | 상품 가격 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 — KRW, USD, JPY만 지원합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |

### 응답

- **200** 성공 응답으로 생성된 구독 상품을 반환합니다.: [CreateSubscriptionProductResponse](#createsubscriptionproductresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **409**
  * [`SubscriptionProductAlreadyExistsError`](../00-basics/errors.md#subscriptionproductalreadyexistserror): 같은 아이디의 구독 상품이 이미 존재하는 경우

## GET /products/{productId}

**구독 상품 조회** (operationId: `getSubscriptionProduct`)

주어진 아이디로 구독 상품을 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `productId` | path | string | Y | 구독 상품 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 구독 상품을 반환합니다.: [GetSubscriptionProductResponse](#getsubscriptionproductresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우

## PATCH /products/{productId}

**구독 상품 수정** (operationId: `updateSubscriptionProduct`)

구독 상품을 수정합니다. 입력하지 않은 필드는 기존 값을 유지하며, 보관된 상품은 수정할 수 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `productId` | path | string | Y | 구독 상품 아이디 |

### 요청 본문 (application/json, 필수)

타입: [UpdateSubscriptionProductBody](#updatesubscriptionproductbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 상품명 |
| `description` | string |  | 상품 설명 |
| `amount` | integer(int64) |  | 상품 가격 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |

### 응답

- **200** 성공 응답으로 수정된 구독 상품을 반환합니다.: [UpdateSubscriptionProductResponse](#updatesubscriptionproductresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
- **409**
  * [`SubscriptionProductArchivedError`](../00-basics/errors.md#subscriptionproductarchivederror): 구독 상품이 보관되어 수정할 수 없는 경우

## POST /products/{productId}/archive

**구독 상품 보관** (operationId: `archiveSubscriptionProduct`)

구독 상품을 보관합니다. 보관된 상품은 새 구독에 담을 수 없고, 기존 구독의 청구는 유지됩니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `productId` | path | string | Y | 구독 상품 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독 상품을 반환합니다.: [ArchiveSubscriptionProductResponse](#archivesubscriptionproductresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우

## POST /products/{productId}/pause-billing

**구독 상품 청구 중지** (operationId: `pauseSubscriptionProductBilling`)

구독 상품의 청구를 중지합니다. 이 상품이 담긴 구독은 해당 회차를 청구하지 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `productId` | path | string | Y | 구독 상품 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독 상품을 반환합니다.: [PauseSubscriptionProductBillingResponse](#pausesubscriptionproductbillingresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우

## POST /products/{productId}/resume-billing

**구독 상품 청구 재개** (operationId: `resumeSubscriptionProductBilling`)

중지된 구독 상품의 청구를 다시 시작합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `productId` | path | string | Y | 구독 상품 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독 상품을 반환합니다.: [ResumeSubscriptionProductBillingResponse](#resumesubscriptionproductbillingresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우

## GET /subscriptions

**구독 다건 조회** (operationId: `getSubscriptions`)

조건에 맞는 구독 리스트를 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionsBody](#getsubscriptionsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `status` | [SubscriptionStatus](#subscriptionstatus) |  | 구독 상태 |
| `customerId` | string |  | 고객 아이디 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 구독만 조회합니다. |

### 응답

- **200** 성공 응답으로 구독 리스트를 반환합니다.: [GetSubscriptionsResponse](#getsubscriptionsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /subscriptions

**구독 생성** (operationId: `createSubscription`)

이미 등록된 고객에게 구독을 추가합니다. 청구 시작일이 오늘이면 첫 인보이스를 즉시 결제합니다.

### 요청 본문 (application/json, 필수)

타입: [CreateSubscriptionBody](#createsubscriptionbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 구독 아이디 — 미입력 시 자동 생성됩니다. |
| `customerId` | string | Y | 고객 아이디 |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 — 정기 과금 상품이 하나 이상 있어야 합니다. |
| `discounts` | [SubscriptionDiscountApplication](#subscriptiondiscountapplication)[] |  | 붙일 할인 — 할인마다 적용 범위와 지속 방식을 정합니다. |
| `billStartDate` | string(date) |  | 청구 시작일 — 미입력 시 오늘입니다. 지난 날짜는 입력할 수 없습니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 청구 시작일보다 뒤여야 합니다. 이날과 그 뒤로는 청구하지 않습니다. 미입력 시 무기한입니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 — 미입력 시 고객 이메일이 있으면 발송합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) |  | 결제 옵션 — 매 회차 결제 요청에 그대로 실립니다. 미입력 시 모두 비어 있습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### 응답

- **200** 성공 응답으로 생성된 구독과 첫 인보이스를 반환합니다.: [CreateSubscriptionResponse](#createsubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionCustomerNotFoundError`](../00-basics/errors.md#subscriptioncustomernotfounderror): 요청된 구독 고객이 존재하지 않는 경우
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우
- **409**
  * [`SubscriptionAlreadyExistsError`](../00-basics/errors.md#subscriptionalreadyexistserror): 같은 아이디의 구독이 이미 존재하는 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우
  * [`SubscriptionCustomerEmailRequiredError`](../00-basics/errors.md#subscriptioncustomeremailrequirederror): 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우
  * [`SubscriptionProductArchivedError`](../00-basics/errors.md#subscriptionproductarchivederror): 구독 상품이 보관되어 수정할 수 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우
  * [`SubscriptionIntervalMismatchError`](../00-basics/errors.md#subscriptionintervalmismatcherror): 담긴 정기 상품들의 청구 주기가 서로 다르거나, 시작한 구독의 청구 주기와 다른 경우
  * [`SubscriptionCurrencyMismatchError`](../00-basics/errors.md#subscriptioncurrencymismatcherror): 담긴 상품들의 통화가 다른 경우
  * [`SubscriptionDiscountArchivedError`](../00-basics/errors.md#subscriptiondiscountarchivederror): 구독 할인이 보관된 경우
  * [`SubscriptionDiscountRedeemByPassedError`](../00-basics/errors.md#subscriptiondiscountredeembypassederror): 구독 할인의 적용 마감일이 지난 경우
  * [`SubscriptionDiscountAlreadyAppliedError`](../00-basics/errors.md#subscriptiondiscountalreadyappliederror): 같은 구독 할인이 이미 붙어 있는 경우
  * [`SubscriptionDiscountProductsNotInSubscriptionError`](../00-basics/errors.md#subscriptiondiscountproductsnotinsubscriptionerror): 구독 할인의 적용 대상 상품이 구독에 담겨 있지 않은 경우
  * [`SubscriptionDiscountCurrencyMismatchError`](../00-basics/errors.md#subscriptiondiscountcurrencymismatcherror): 구독 할인의 통화가 구독의 통화와 다른 경우

## POST /subscriptions/pre-register

**구독 사전 등록** (operationId: `preRegisterSubscription`)

결제수단 없이 구독을 미리 만듭니다. 구독 등록 SDK로 결제수단이 등록되면 그 시점에 구독이 시작되고 첫 회차를 결제합니다. 그 전까지 구독은 INCOMPLETE 상태이며 시작일이 없습니다. 고객이 없으면 함께 만들고, 있으면 입력한 필드만 갱신합니다.

### 요청 본문 (application/json, 필수)

타입: [PreRegisterSubscriptionBody](#preregistersubscriptionbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 구독 아이디 — 미입력 시 자동 생성됩니다. |
| `customer` | [SubscriptionPreRegisterCustomerInput](#subscriptionpreregistercustomerinput) | Y | 구독 고객 |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 — 정기 과금 상품이 하나 이상 있어야 합니다. |
| `discounts` | [SubscriptionDiscountApplication](#subscriptiondiscountapplication)[] |  | 붙일 할인 — 할인마다 적용 범위와 지속 방식을 정합니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 오늘보다 뒤여야 합니다. 이날이 되면 구독 등록을 할 수 없습니다. 미입력 시 무기한입니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 — 미입력 시 고객 이메일이 있으면 발송합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) |  | 결제 옵션 — 매 회차 결제 요청에 그대로 실립니다. 미입력 시 모두 비어 있습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### 응답

- **200** 성공 응답으로 사전 등록된 구독과 구독 고객을 반환합니다.: [PreRegisterSubscriptionResponse](#preregistersubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우
- **409**
  * [`SubscriptionAlreadyExistsError`](../00-basics/errors.md#subscriptionalreadyexistserror): 같은 아이디의 구독이 이미 존재하는 경우
  * [`SubscriptionCustomerEmailRequiredError`](../00-basics/errors.md#subscriptioncustomeremailrequirederror): 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우
  * [`SubscriptionProductArchivedError`](../00-basics/errors.md#subscriptionproductarchivederror): 구독 상품이 보관되어 수정할 수 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우
  * [`SubscriptionIntervalMismatchError`](../00-basics/errors.md#subscriptionintervalmismatcherror): 담긴 정기 상품들의 청구 주기가 서로 다르거나, 시작한 구독의 청구 주기와 다른 경우
  * [`SubscriptionCurrencyMismatchError`](../00-basics/errors.md#subscriptioncurrencymismatcherror): 담긴 상품들의 통화가 다른 경우
  * [`SubscriptionDiscountArchivedError`](../00-basics/errors.md#subscriptiondiscountarchivederror): 구독 할인이 보관된 경우
  * [`SubscriptionDiscountRedeemByPassedError`](../00-basics/errors.md#subscriptiondiscountredeembypassederror): 구독 할인의 적용 마감일이 지난 경우
  * [`SubscriptionDiscountAlreadyAppliedError`](../00-basics/errors.md#subscriptiondiscountalreadyappliederror): 같은 구독 할인이 이미 붙어 있는 경우
  * [`SubscriptionDiscountProductsNotInSubscriptionError`](../00-basics/errors.md#subscriptiondiscountproductsnotinsubscriptionerror): 구독 할인의 적용 대상 상품이 구독에 담겨 있지 않은 경우
  * [`SubscriptionDiscountCurrencyMismatchError`](../00-basics/errors.md#subscriptiondiscountcurrencymismatcherror): 구독 할인의 통화가 구독의 통화와 다른 경우

## GET /subscriptions/{subscriptionId}

**구독 조회** (operationId: `getSubscription`)

주어진 아이디로 구독을 조회합니다. 붙어 있는 할인과 다음 청구 예정 시각이 함께 내려갑니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 구독을 반환합니다.: [GetSubscriptionResponse](#getsubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우

## PATCH /subscriptions/{subscriptionId}

**구독 수정** (operationId: `updateSubscription`)

구독을 수정합니다. 입력하지 않은 필드는 기존 값을 유지하며, 변경은 다음 결제부터 반영됩니다. 일할 계산은 하지 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |

### 요청 본문 (application/json, 필수)

타입: [UpdateSubscriptionBody](#updatesubscriptionbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `items` | [SubscriptionItem](#subscriptionitem)[] |  | 구독 항목 — 전체 교체입니다. 추가·삭제·수량 변경을 한 번에 보냅니다. 정기 과금 상품이 하나 이상 있어야 하고, 담긴 정기 상품들의 청구 주기가 같아야 합니다. 시작한 구독은 청구 주기가 다른 정기 상품으로 바꿀 수 없고, 사전 등록 상태(구독 등록 전)에서는 새 상품의 청구 주기를 따릅니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 오늘보다 뒤여야 합니다. 이날과 그 뒤로는 청구하지 않습니다. null을 보내면 무기한이 됩니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 |
| `nextBillingDate` | string(date) |  | 다음 청구일 — 정기결제일을 옮깁니다. 오늘보다 뒤이고 종료일보다 앞이어야 하며, 지정한 날짜가 새 청구 기준일이 됩니다. 기준 시각과 회차 번호는 유지됩니다. 사전 등록한 구독은 구독 등록 전에 지정할 수 없습니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `paymentOptions` | [UpdateSubscriptionBodyPaymentOptions](#updatesubscriptionbodypaymentoptions) |  | 결제 옵션 — 입력한 필드만 고치고, null을 보낸 필드는 지웁니다. 다음 결제와 실패한 인보이스의 재시도부터 반영됩니다. 고객에게 등록된 결제수단이 있으면 고친 값으로 결제할 수 있는지 미리 확인하고, 결제할 수 없으면 수정하지 않습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### 응답

- **200** 성공 응답으로 수정된 구독을 반환합니다.: [UpdateSubscriptionResponse](#updatesubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
  * [`SubscriptionProductNotFoundError`](../00-basics/errors.md#subscriptionproductnotfounderror): 요청된 구독 상품이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionProductArchivedError`](../00-basics/errors.md#subscriptionproductarchivederror): 구독 상품이 보관되어 수정할 수 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우
  * [`SubscriptionIntervalMismatchError`](../00-basics/errors.md#subscriptionintervalmismatcherror): 담긴 정기 상품들의 청구 주기가 서로 다르거나, 시작한 구독의 청구 주기와 다른 경우
  * [`SubscriptionCustomerEmailRequiredError`](../00-basics/errors.md#subscriptioncustomeremailrequirederror): 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우
  * [`SubscriptionCurrencyMismatchError`](../00-basics/errors.md#subscriptioncurrencymismatcherror): 담긴 상품들의 통화가 다른 경우

## POST /subscriptions/{subscriptionId}/cancel

**구독 해지** (operationId: `cancelSubscription`)

구독을 해지합니다. 해지한 구독은 다시 살릴 수 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독을 반환합니다.: [CancelSubscriptionResponse](#cancelsubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우

## POST /subscriptions/{subscriptionId}/charge-test

**테스트 구독 즉시 청구** (operationId: `chargeTestSubscription`)

테스트 고객의 구독을 지금 한 회차 청구합니다. 다음 청구 예정 회차를 앞당겨 청구하며, 결제 실패를 시험하려면 실패 시뮬레이션을 켭니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |

### 요청 본문 (application/json, 필수)

타입: [ChargeTestSubscriptionBody](#chargetestsubscriptionbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `simulateFailure` | boolean |  | 실패 시뮬레이션 여부 — true면 결제사에 요청하지 않고 결제 실패로 처리합니다. 실패 웹훅과 미완료 상태를 시험할 때 씁니다. |

### 응답

- **200** 성공 응답으로 청구 뒤의 구독과 청구한 인보이스를 반환합니다.: [ChargeTestSubscriptionResponse](#chargetestsubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
- **409**
  * [`SubscriptionCustomerNotInTestModeError`](../00-basics/errors.md#subscriptioncustomernotintestmodeerror): 구독 고객이 테스트 고객이 아닌 경우
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionCustomerHasNoBillingKeyError`](../00-basics/errors.md#subscriptioncustomerhasnobillingkeyerror): 구독 고객에게 사용 가능한 빌링키가 없는 경우
  * [`SubscriptionProductBillingPausedError`](../00-basics/errors.md#subscriptionproductbillingpausederror): 구독 상품의 청구가 중지된 경우

## GET /subscriptions/{subscriptionId}/discounts

**구독에 붙은 할인 조회** (operationId: `getSubscriptionDiscountUsages`)

구독에 붙어 있는 할인을 남은 회차와 상태와 함께 조회합니다. 제거된 할인은 빠집니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 붙어 있는 할인 리스트를 반환합니다.: [GetSubscriptionDiscountUsagesResponse](#getsubscriptiondiscountusagesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우

## POST /subscriptions/{subscriptionId}/discounts

**구독에 할인 추가** (operationId: `addSubscriptionDiscount`)

구독에 할인을 붙입니다. 다음 결제부터 적용되며 결제 직전까지 추가할 수 있습니다. 개수 제한은 없고 같은 할인을 중복해서 붙일 수는 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |

### 요청 본문 (application/json, 필수)

타입: [AddSubscriptionDiscountBody](#addsubscriptiondiscountbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `discountId` | string | Y | 붙일 할인 아이디 |
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) | Y | 적용 범위 — 같은 할인을 상품 범위로 여러 번 붙일 수 있으나 상품이 겹치면 안 되고, 구독 전체 범위는 하나만 둘 수 있습니다. |
| `duration` | [SubscriptionDiscountDuration](#subscriptiondiscountduration) | Y | 지속 방식 |

### 응답

- **200** 성공 응답으로 반영된 구독을 반환합니다.: [AddSubscriptionDiscountResponse](#addsubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
  * [`SubscriptionDiscountNotFoundError`](../00-basics/errors.md#subscriptiondiscountnotfounderror): 요청된 구독 할인이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우
  * [`SubscriptionDiscountArchivedError`](../00-basics/errors.md#subscriptiondiscountarchivederror): 구독 할인이 보관된 경우
  * [`SubscriptionDiscountRedeemByPassedError`](../00-basics/errors.md#subscriptiondiscountredeembypassederror): 구독 할인의 적용 마감일이 지난 경우
  * [`SubscriptionDiscountAlreadyAppliedError`](../00-basics/errors.md#subscriptiondiscountalreadyappliederror): 같은 구독 할인이 이미 붙어 있는 경우
  * [`SubscriptionDiscountProductsNotInSubscriptionError`](../00-basics/errors.md#subscriptiondiscountproductsnotinsubscriptionerror): 구독 할인의 적용 대상 상품이 구독에 담겨 있지 않은 경우
  * [`SubscriptionDiscountCurrencyMismatchError`](../00-basics/errors.md#subscriptiondiscountcurrencymismatcherror): 구독 할인의 통화가 구독의 통화와 다른 경우

## DELETE /subscriptions/{subscriptionId}/discounts/{discountId}

**구독에서 할인 제거** (operationId: `removeSubscriptionDiscount`)

구독에서 할인을 제거합니다. 적용 범위를 지정하면 그 범위로 붙은 것만, 지정하지 않으면 그 할인이 붙은 모든 범위를 제거합니다. 다음 결제부터 그 할인이 적용되지 않습니다. 붙어 있지 않으면 아무 일도 하지 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `discountId` | path | string | Y | 제거할 할인 아이디 |
| `storeId` | query | string |  | 상점 아이디 |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [RemoveSubscriptionDiscountBody](#removesubscriptiondiscountbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) |  | 제거할 적용 범위 — 미입력 시 그 할인이 붙은 모든 범위를 제거합니다. |

### 응답

- **200** 성공 응답으로 반영된 구독을 반환합니다.: [RemoveSubscriptionDiscountResponse](#removesubscriptiondiscountresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우

## GET /subscriptions/{subscriptionId}/invoice-preview

**다음 인보이스 미리보기** (operationId: `previewSubscriptionInvoice`)

구독의 다음 회차가 어떻게 청구될지 미리 계산해 보여줍니다. 실제 인보이스를 만들지는 않습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 미리보기 결과를 반환합니다.: [PreviewSubscriptionInvoiceResponse](#previewsubscriptioninvoiceresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우

## GET /subscriptions/{subscriptionId}/invoices

**구독의 인보이스 다건 조회** (operationId: `getSubscriptionInvoices`)

한 구독의 인보이스를 청구 일시 역순으로 조회합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetSubscriptionInvoicesBody](#getsubscriptioninvoicesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `from` | string(date-time) |  | 조회 시작 일시 — 청구 일시 기준입니다. 미입력 시 90일 전입니다. |
| `to` | string(date-time) |  | 조회 종료 일시 — 청구 일시 기준입니다. 미입력 시 현재입니다. |

### 응답

- **200** 성공 응답으로 인보이스 리스트를 반환합니다.: [GetSubscriptionInvoicesResponse](#getsubscriptioninvoicesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우

## POST /subscriptions/{subscriptionId}/pause

**구독 일시중지** (operationId: `pauseSubscription`)

구독을 일시중지합니다. 중지 중에는 인보이스 생성과 결제가 없습니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독을 반환합니다.: [PauseSubscriptionResponse](#pausesubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우

## POST /subscriptions/{subscriptionId}/resume

**구독 재개** (operationId: `resumeSubscription`)

일시중지한 구독을 재개합니다. 청구 기준일과 기준 시각을 유지하므로 원래 정기결제일에 다음 인보이스를 만듭니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `subscriptionId` | path | string | Y | 구독 아이디 |
| `storeId` | query | string |  | 상점 아이디 |

### 응답

- **200** 성공 응답으로 반영된 구독을 반환합니다.: [ResumeSubscriptionResponse](#resumesubscriptionresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SubscriptionNotFoundError`](../00-basics/errors.md#subscriptionnotfounderror): 요청된 구독이 존재하지 않는 경우
- **409**
  * [`SubscriptionStatusNotAllowedError`](../00-basics/errors.md#subscriptionstatusnotallowederror): 현재 구독 상태에서 허용되지 않는 요청인 경우

---

# 스키마

### ActivateSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 활성화 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplate` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate) | Y | 반영된 인보이스 메일 템플릿 |

### AddSubscriptionDiscountBody

구독 할인 추가 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `discountId` | string | Y | 붙일 할인 아이디 |
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) | Y | 적용 범위 — 같은 할인을 상품 범위로 여러 번 붙일 수 있으나 상품이 겹치면 안 되고, 구독 전체 범위는 하나만 둘 수 있습니다. |
| `duration` | [SubscriptionDiscountDuration](#subscriptiondiscountduration) | Y | 지속 방식 |

### AddSubscriptionDiscountResponse

구독 할인 추가 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 반영된 구독 |

### ArchiveSubscriptionDiscountResponse

구독 할인 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discount` | [SubscriptionDiscount](#subscriptiondiscount) | Y | 반영된 구독 할인 |

### ArchiveSubscriptionProductResponse

구독 상품 보관 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 반영된 구독 상품 |

### CancelSubscriptionInvoiceResponse

구독 인보이스 취소 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 반영된 인보이스 |

### CancelSubscriptionResponse

구독 해지 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 반영된 구독 |

### ChargeTestSubscriptionBody

테스트 구독 즉시 청구 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `simulateFailure` | boolean |  | 실패 시뮬레이션 여부 — true면 결제사에 요청하지 않고 결제 실패로 처리합니다. 실패 웹훅과 미완료 상태를 시험할 때 씁니다. |

### ChargeTestSubscriptionResponse

테스트 구독 즉시 청구 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 청구 뒤의 구독 |
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 이번에 청구한 인보이스 — 청구 일시는 그 회차의 원래 청구 예정 일시라 결제 일시보다 늦을 수 있습니다. |

### CreateSubscriptionBody

구독 생성 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 구독 아이디 — 미입력 시 자동 생성됩니다. |
| `customerId` | string | Y | 고객 아이디 |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 — 정기 과금 상품이 하나 이상 있어야 합니다. |
| `discounts` | [SubscriptionDiscountApplication](#subscriptiondiscountapplication)[] |  | 붙일 할인 — 할인마다 적용 범위와 지속 방식을 정합니다. |
| `billStartDate` | string(date) |  | 청구 시작일 — 미입력 시 오늘입니다. 지난 날짜는 입력할 수 없습니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 청구 시작일보다 뒤여야 합니다. 이날과 그 뒤로는 청구하지 않습니다. 미입력 시 무기한입니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 — 미입력 시 고객 이메일이 있으면 발송합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) |  | 결제 옵션 — 매 회차 결제 요청에 그대로 실립니다. 미입력 시 모두 비어 있습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### CreateSubscriptionCustomerBody

구독 고객 등록 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 고객 아이디 — 미입력 시 새 고객으로 등록하고 아이디를 자동 생성합니다. 입력한 아이디의 고객이 있으면 그 고객을 갱신합니다. |
| `name` | string |  | 고객명 — 신규 고객이면 필수입니다. |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `billingKey` | string | Y | 등록할 빌링키 — 해당 상점의 카드 빌링키여야 합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |
| `subscription` | [SubscriptionCustomerCreateSubscriptionInput](#subscriptioncustomercreatesubscriptioninput) |  | 함께 만들 구독 — 입력하면 고객 등록과 구독 생성을 한 번에 처리합니다. 구독 생성이 실패하면 고객도 만들지 않습니다. |

### CreateSubscriptionCustomerResponse

구독 고객 등록 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `customer` | [SubscriptionCustomer](#subscriptioncustomer) | Y | 등록된 고객 |
| `subscription` | [Subscription](#subscription) |  | 함께 만들어진 구독 — 구독 정보를 함께 보낸 경우에만 존재합니다. |

### CreateSubscriptionDiscountBody

구독 할인 생성 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 할인 아이디 — 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 할인명 |
| `type` | [SubscriptionDiscountType](#subscriptiondiscounttype) | Y | 할인 유형 |
| `value` | integer(int64) | Y | 할인 값 — 정률은 1~100, 정액은 금액입니다. |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 — 정액 할인에만 입력합니다. KRW, USD, JPY만 지원합니다. |
| `redeemBy` | string(date) |  | 적용 마감일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### CreateSubscriptionDiscountResponse

구독 할인 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discount` | [SubscriptionDiscount](#subscriptiondiscount) | Y | 생성된 구독 할인 |

### CreateSubscriptionInvoiceBody

구독 인보이스 수동 청구 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `subscriptionId` | string | Y | 구독 아이디 |
| `items` | [SubscriptionInvoiceItemForm](#subscriptioninvoiceitemform)[] | Y | 청구 항목 — 등록된 상품이나 임의 항목을 담을 수 있습니다. |
| `memo` | string |  | 메모 |

### CreateSubscriptionInvoiceEmailTemplateBody

인보이스 메일 템플릿 생성 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string | Y | 템플릿 이름 |
| `logoImageUrl` | string |  | 로고 이미지 주소 — 메일 상단에 표시할 로고 이미지 주소입니다. 인증 없이 열리는 https 주소를 입력해 주세요. 가로 100px, 세로 24px 영역에 비율을 유지해 표시됩니다. |
| `senders` | [SubscriptionInvoiceEmailSenders](#subscriptioninvoiceemailsenders) | Y | 발신자 정보 |
| `events` | [SubscriptionInvoiceEmailEventType](#subscriptioninvoiceemaileventtype)[] | Y | 보낼 메일 유형 |

### CreateSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplate` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate) | Y | 생성된 인보이스 메일 템플릿 |

### CreateSubscriptionInvoiceResponse

구독 인보이스 수동 청구 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 생성된 인보이스 |

### CreateSubscriptionProductBody

구독 상품 생성 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 상품 아이디 — 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 상품명 |
| `description` | string |  | 상품 설명 |
| `pricingType` | [SubscriptionProductPricingType](#subscriptionproductpricingtype) | Y | 과금 유형 |
| `interval` | [SubscriptionInterval](#subscriptioninterval) |  | 청구 주기 — 정기 과금 상품에만 입력합니다. |
| `amount` | integer(int64) | Y | 상품 가격 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 — KRW, USD, JPY만 지원합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |

### CreateSubscriptionProductResponse

구독 상품 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 생성된 구독 상품 |

### CreateSubscriptionResponse

구독 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 생성된 구독 |
| `firstInvoice` | [SubscriptionInvoice](#subscriptioninvoice) |  | 첫 인보이스 — 청구 시작일이 오늘이면 즉시 결제된 첫 인보이스가 함께 내려갑니다. 결제가 실패해도 요청은 성공하고 구독은 첫 회차 미수납 상태가 됩니다. |

### DeactivateSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 비활성화 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplate` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate) | Y | 반영된 인보이스 메일 템플릿 |

### DeleteSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 삭제 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplateId` | string | Y | 삭제된 인보이스 메일 템플릿 아이디 |

### GetSubscriptionCustomerInvoicesBody

고객의 인보이스 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `from` | string(date-time) |  | 조회 시작 일시 — 청구 일시 기준입니다. 미입력 시 90일 전입니다. |
| `to` | string(date-time) |  | 조회 종료 일시 — 청구 일시 기준입니다. 미입력 시 현재입니다. |

### GetSubscriptionCustomerInvoicesResponse

고객의 인보이스 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionInvoice](#subscriptioninvoice)[] | Y | 조회된 인보이스 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionCustomerResponse

구독 고객 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `customer` | [SubscriptionCustomer](#subscriptioncustomer) | Y | 조회된 고객 |

### GetSubscriptionCustomerSubscriptionsBody

고객의 구독 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `status` | [SubscriptionStatus](#subscriptionstatus) |  | 구독 상태 |

### GetSubscriptionCustomerSubscriptionsResponse

고객의 구독 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [Subscription](#subscription)[] | Y | 조회된 구독 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionCustomersBody

구독 고객 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 고객만 조회합니다. |

### GetSubscriptionCustomersResponse

구독 고객 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionCustomer](#subscriptioncustomer)[] | Y | 조회된 고객 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionDiscountResponse

구독 할인 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discount` | [SubscriptionDiscount](#subscriptiondiscount) | Y | 조회된 구독 할인 |

### GetSubscriptionDiscountUsagesResponse

구독에 붙은 할인 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionDiscountUsage](#subscriptiondiscountusage)[] | Y | 붙어 있는 할인 |

### GetSubscriptionDiscountsBody

구독 할인 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `archived` | boolean |  | 보관한 할인 포함 여부 — 미입력이거나 true면 보관한 할인도 포함해 조회하고, false면 보관하지 않은 할인만 조회합니다. |

### GetSubscriptionDiscountsResponse

구독 할인 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionDiscount](#subscriptiondiscount)[] | Y | 조회된 구독 할인 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplate` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate) | Y | 조회된 인보이스 메일 템플릿 |

### GetSubscriptionInvoiceEmailTemplatesBody

인보이스 메일 템플릿 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |

### GetSubscriptionInvoiceEmailTemplatesResponse

인보이스 메일 템플릿 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate)[] | Y | 조회된 인보이스 메일 템플릿 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionInvoiceResponse

구독 인보이스 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 조회된 인보이스 |
| `events` | [SubscriptionInvoiceEvent](#subscriptioninvoiceevent)[] | Y | 처리내역 — 일어난 순서입니다. |

### GetSubscriptionInvoiceStatsBody

구독 인보이스 집계 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `filter` | [SubscriptionInvoiceFilterInput](#subscriptioninvoicefilterinput) |  | 조회 조건 |

### GetSubscriptionInvoiceStatsResponse

구독 인보이스 집계 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statusCounts` | [SubscriptionInvoiceStatusCount](#subscriptioninvoicestatuscount)[] | Y | 상태별 건수 |
| `currencySummaries` | [SubscriptionInvoiceCurrencySummary](#subscriptioninvoicecurrencysummary)[] | Y | 통화별 집계 |
| `availableCurrencies` | [Currency](../00-basics/schemas-common.md#currency)[] | Y | 상점 구독의 통화 리스트 — 통화 조건의 선택지로 씁니다. |

### GetSubscriptionInvoicesAllBody

구독 인보이스 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `statuses` | [SubscriptionInvoiceStatus](#subscriptioninvoicestatus)[] |  | 인보이스 상태 리스트 — 미입력 시 모든 상태를 조회합니다. |
| `filter` | [SubscriptionInvoiceFilterInput](#subscriptioninvoicefilterinput) |  | 조회 조건 |
| `sortBy` | [SubscriptionInvoiceSortBy](#subscriptioninvoicesortby) |  | 정렬 기준 — 미입력 시 조회 기준 시점입니다. |
| `sortOrder` | [SortOrder](../00-basics/schemas-common.md#sortorder) |  | 정렬 방향 — 미입력 시 내림차순입니다. |

### GetSubscriptionInvoicesAllResponse

구독 인보이스 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionInvoice](#subscriptioninvoice)[] | Y | 조회된 인보이스 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionInvoicesBody

구독 인보이스 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `from` | string(date-time) |  | 조회 시작 일시 — 청구 일시 기준입니다. 미입력 시 90일 전입니다. |
| `to` | string(date-time) |  | 조회 종료 일시 — 청구 일시 기준입니다. 미입력 시 현재입니다. |

### GetSubscriptionInvoicesResponse

구독 인보이스 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionInvoice](#subscriptioninvoice)[] | Y | 조회된 인보이스 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionProductResponse

구독 상품 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 조회된 구독 상품 |

### GetSubscriptionProductsBody

구독 상품 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `archived` | boolean |  | 보관한 상품 포함 여부 — 미입력이거나 true면 보관한 상품도 포함해 조회하고, false면 보관하지 않은 상품만 조회합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 상품만 조회합니다. |

### GetSubscriptionProductsResponse

구독 상품 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [SubscriptionProduct](#subscriptionproduct)[] | Y | 조회된 구독 상품 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### GetSubscriptionResponse

구독 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 조회된 구독 |

### GetSubscriptionsBody

구독 다건 조회 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 페이지 정보 |
| `status` | [SubscriptionStatus](#subscriptionstatus) |  | 구독 상태 |
| `customerId` | string |  | 고객 아이디 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 key-value가 모두 일치하는 구독만 조회합니다. |

### GetSubscriptionsResponse

구독 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [Subscription](#subscription)[] | Y | 조회된 구독 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 페이지 정보 |

### Map_String

Map_String

type: map<string, string>

### PauseSubscriptionProductBillingResponse

구독 상품 청구 중지 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 반영된 구독 상품 |

### PauseSubscriptionResponse

구독 일시중지 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 반영된 구독 |

### PreRegisterSubscriptionBody

구독 사전 등록 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `id` | string |  | 구독 아이디 — 미입력 시 자동 생성됩니다. |
| `customer` | [SubscriptionPreRegisterCustomerInput](#subscriptionpreregistercustomerinput) | Y | 구독 고객 |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 — 정기 과금 상품이 하나 이상 있어야 합니다. |
| `discounts` | [SubscriptionDiscountApplication](#subscriptiondiscountapplication)[] |  | 붙일 할인 — 할인마다 적용 범위와 지속 방식을 정합니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 오늘보다 뒤여야 합니다. 이날이 되면 구독 등록을 할 수 없습니다. 미입력 시 무기한입니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 — 미입력 시 고객 이메일이 있으면 발송합니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) |  | 결제 옵션 — 매 회차 결제 요청에 그대로 실립니다. 미입력 시 모두 비어 있습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### PreRegisterSubscriptionResponse

구독 사전 등록 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 사전 등록된 구독 |
| `customer` | [SubscriptionCustomer](#subscriptioncustomer) | Y | 구독 고객 |
| `registrationSecret` | string | Y | 구독 등록 비밀값 — 구독 등록 결제창을 열 때 subscriptionId와 함께 넘깁니다. 이 응답에서만 받을 수 있고 다시 조회할 수 없으니, 결제 페이지 응답 본문으로 브라우저에 전달해 주세요. URL에 넣지 마세요. |

### PreviewSubscriptionInvoiceResponse

다음 인보이스 미리보기 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `preview` | [SubscriptionInvoicePreview](#subscriptioninvoicepreview) |  | 미리보기 결과 — 일시중지·종료·첫 회차 미수납 상태이거나 청구가 중지된 상품이 담겼거나 종료일을 넘어 청구할 회차가 없으면 없습니다. |

### RegenerateSubscriptionInvoiceResponse

구독 인보이스 재생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 새로 만들어진 인보이스 |

### RemoveSubscriptionDiscountBody

구독 할인 제거 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) |  | 제거할 적용 범위 — 미입력 시 그 할인이 붙은 모든 범위를 제거합니다. |

### RemoveSubscriptionDiscountResponse

구독 할인 제거 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 반영된 구독 |

### ResumeSubscriptionProductBillingResponse

구독 상품 청구 재개 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 반영된 구독 상품 |

### ResumeSubscriptionResponse

구독 재개 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 반영된 구독 |

### RetrySubscriptionInvoiceResponse

구독 인보이스 결제 재시도 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 반영된 인보이스 |

### RevokeSubscriptionCustomerPaymentMethodBody

구독 고객 결제수단 해지 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `reason` | string |  | 해지 사유 — 일부 결제사가 필수로 요구하므로 채워 보내는 것을 권장합니다. |

### RevokeSubscriptionCustomerPaymentMethodResponse

구독 고객 결제수단 해지 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `customer` | [SubscriptionCustomer](#subscriptioncustomer) | Y | 반영된 고객 |

### SettleSubscriptionInvoiceBody

구독 인보이스 수동 완납 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `method` | [SubscriptionInvoiceSettlementMethod](#subscriptioninvoicesettlementmethod) | Y | 완납 방법 |
| `paidAt` | string(date-time) | Y | 입금 시각 — 실제로 돈을 받은 시각입니다. 지금보다 뒤일 수 없습니다. |
| `memo` | string |  | 메모 — 완납 처리한 경위를 남겨둘 수 있습니다. |

### SettleSubscriptionInvoiceResponse

구독 인보이스 수동 완납 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoice` | [SubscriptionInvoice](#subscriptioninvoice) | Y | 반영된 인보이스 |

### Subscription

구독

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 구독 아이디 — 고객사가 직접 부여하거나 미입력 시 자동 생성됩니다. |
| `customerId` | string | Y | 고객 아이디 — 청구에는 이 고객의 현재 빌링키를 사용합니다. |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 |
| `interval` | [SubscriptionInterval](#subscriptioninterval) |  | 청구 주기 — 담긴 정기 상품들의 공통 주기입니다. 일회성 상품만 담긴 구독에는 없습니다. |
| `billStartDate` | string(date) |  | 청구 시작일 — 구독 시작일이자 청구 기준일의 최초 출처입니다. 사전 등록한 구독은 구독 등록이 끝난 날로 정해지며, 그 전에는 없습니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날입니다. 이날과 그 뒤로는 청구하지 않고, 이날이 되면 구독을 종료합니다. 없으면 무기한입니다. |
| `anchorDate` | string(date) |  | 청구 기준일 — 이후 회차를 계산하는 기준입니다. 정기결제일을 옮기면 이 값이 바뀝니다. 사전 등록한 구독은 구독 등록 전에는 없습니다. |
| `anchorTime` | string |  | 청구 기준 시각 — 구독을 만든 시각(사전 등록한 구독은 구독 등록이 끝난 시각)으로 정해지며 고객사가 입력하지 않습니다. 구독 등록 전에는 없습니다. |
| `lastBilledAt` | string(date-time) |  | 마지막 청구 시각 |
| `nextBillingAt` | string(date-time) |  | 다음 청구 예정 시각 — 일시중지·종료 상태이거나 사전 등록 후 구독 등록 전이면 없습니다. |
| `discounts` | [SubscriptionDiscountUsage](#subscriptiondiscountusage)[] | Y | 붙어 있는 할인 |
| `sendInvoiceEmail` | boolean | Y | 인보이스 메일 발송 여부 |
| `status` | [SubscriptionStatus](#subscriptionstatus) | Y | 구독 상태 |
| `endedAt` | string(date-time) |  | 종료 시각 |
| `endReason` | [SubscriptionEndReason](#subscriptionendreason) |  | 종료 사유 |
| `metadata` | [Map_String](#map_string) | Y | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) | Y | 결제 옵션 |
| `isTest` | boolean |  | 테스트 여부 — 고객이 테스트 채널 빌링키를 쓰면 true입니다. 고객이 빌링키를 등록한 적이 없으면 없습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionCustomer

구독 고객 — 구독 고객<br>구독 결제의 대상이 되는 고객입니다. 고객당 빌링키는 하나입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 고객 아이디 — 고객사가 직접 부여합니다. |
| `name` | string | Y | 고객명 |
| `email` | string |  | 이메일 — 결제 결과 인보이스를 받는 주소입니다. |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `status` | [SubscriptionCustomerStatus](#subscriptioncustomerstatus) | Y | 고객 상태 |
| `billingKey` | string |  | 현재 빌링키 — 해지하면 없어집니다. |
| `billingKeyMethod` | [SubscriptionCustomerBillingKeyMethod](#subscriptioncustomerbillingkeymethod) |  | 현재 빌링키의 결제수단 — 빌링키가 없으면 없습니다. |
| `card` | [SubscriptionCustomerCard](#subscriptioncustomercard) |  | 현재 빌링키의 카드 — 빌링키가 없거나 결제사가 카드 정보를 주지 않았으면 없습니다. |
| `isTest` | boolean |  | 테스트 여부 — 테스트 채널 빌링키를 쓰는 고객이면 true입니다. 처음 등록한 빌링키로 정해지며 바뀌지 않습니다. 빌링키를 해지해도 유지되고, 빌링키를 등록한 적이 없으면 없습니다. |
| `metadata` | [Map_String](#map_string) | Y | 사용자 정의 데이터 |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionCustomerBillingKeyMethod

구독 고객 빌링키 결제수단

- `CARD` — 카드
- `PAYPAL` — 페이팔

### SubscriptionCustomerCard

구독 고객 카드 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `issuerName` | string |  | 카드사 이름 |
| `number` | string |  | 카드 번호 — 결제사가 준 마스킹된 번호입니다. |

### SubscriptionCustomerCreateSubscriptionInput

함께 만들 구독 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 구독 아이디 — 미입력 시 자동 생성됩니다. |
| `items` | [SubscriptionItem](#subscriptionitem)[] | Y | 구독 항목 — 정기 과금 상품이 하나 이상 있어야 합니다. |
| `discounts` | [SubscriptionDiscountApplication](#subscriptiondiscountapplication)[] |  | 붙일 할인 — 할인마다 적용 범위와 지속 방식을 정합니다. |
| `billStartDate` | string(date) |  | 청구 시작일 — 미입력 시 오늘입니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 청구 시작일보다 뒤여야 합니다. 이날과 그 뒤로는 청구하지 않습니다. 미입력 시 무기한입니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 |
| `paymentOptions` | [SubscriptionPaymentOptions](#subscriptionpaymentoptions) |  | 결제 옵션 — 매 회차 결제 요청에 그대로 실립니다. 미입력 시 모두 비어 있습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### SubscriptionCustomerStatus

구독 고객 상태

- `ACTIVE` — 활성

### SubscriptionDiscount

구독 할인 — 구독 할인<br>얼마를 깎는지만 정합니다. 무엇에 몇 회차 적용할지는 구독에 붙일 때 정합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 할인 아이디 — 고객사가 직접 부여하거나 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 할인명 — 인보이스와 화면에 표시됩니다. |
| `type` | [SubscriptionDiscountType](#subscriptiondiscounttype) | Y | 할인 유형 |
| `value` | integer(int64) | Y | 할인 값 — 정률은 1~100, 정액은 금액입니다. |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) |  | 통화 — 정액 할인에만 존재합니다. |
| `redeemBy` | string(date) |  | 적용 마감일 — 이 날이 지나면 구독에 새로 붙일 수 없습니다. |
| `archived` | boolean | Y | 보관 여부 — 보관된 할인은 새로 붙일 수 없고, 이미 붙은 구독에서는 잔여 회차까지 적용됩니다. |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionDiscountApplication

구독에 붙일 할인

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountId` | string | Y | 할인 아이디 |
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) | Y | 적용 범위 |
| `duration` | [SubscriptionDiscountDuration](#subscriptiondiscountduration) | Y | 지속 방식 |

### SubscriptionDiscountAppliesTo

SubscriptionDiscountAppliesTo

oneOf, discriminator `type`:

- `PRODUCTS` → [SubscriptionDiscountAppliesToProducts](#subscriptiondiscountappliestoproducts)
- `SUBSCRIPTION` → [SubscriptionDiscountAppliesToSubscription](#subscriptiondiscountappliestosubscription)

### SubscriptionDiscountAppliesToProducts

지정한 상품 항목에만 적용

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 할인 적용 범위 |
| `productIds` | string[] | Y | 적용 대상 상품 아이디 |

### SubscriptionDiscountAppliesToSubscription

인보이스 전체에 적용

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 할인 적용 범위 |

### SubscriptionDiscountDuration

SubscriptionDiscountDuration

oneOf, discriminator `type`:

- `CYCLES` → [SubscriptionDiscountDurationCycles](#subscriptiondiscountdurationcycles)
- `FOREVER` → [SubscriptionDiscountDurationForever](#subscriptiondiscountdurationforever)
- `ONCE` → [SubscriptionDiscountDurationOnce](#subscriptiondiscountdurationonce)

### SubscriptionDiscountDurationCycles

지정한 회차만큼 적용

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 할인 지속 방식 |
| `count` | integer(int32) | Y | 적용 회차 수 |

### SubscriptionDiscountDurationForever

소진 없이 계속 적용

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 할인 지속 방식 |

### SubscriptionDiscountDurationOnce

한 회차만 적용

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 할인 지속 방식 |

### SubscriptionDiscountType

구독 할인 유형

- `PERCENT` — 정률 할인
- `AMOUNT` — 정액 할인

### SubscriptionDiscountUsage

구독 할인 사용 현황 — 구독 할인 사용 현황<br>구독에 붙어 있는 할인의 적용 범위·지속 방식과 남은 회차, 상태입니다. 할인 금액 조건은 할인 아이디로 조회합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountId` | string | Y | 할인 아이디 |
| `appliedAt` | string(date-time) | Y | 구독에 연결한 시각 |
| `remainingCount` | integer(int32) |  | 남은 적용 회차 — 소진 없이 계속 적용되는 할인은 없습니다. |
| `status` | [SubscriptionDiscountUsageStatus](#subscriptiondiscountusagestatus) | Y | 사용 상태 |
| `appliesTo` | [SubscriptionDiscountAppliesTo](#subscriptiondiscountappliesto) | Y | 적용 범위 |
| `duration` | [SubscriptionDiscountDuration](#subscriptiondiscountduration) | Y | 지속 방식 |

### SubscriptionDiscountUsageStatus

구독 할인 사용 상태

- `ACTIVE` — 적용 중
- `EXHAUSTED` — 회차 소진
- `REMOVED` — 제거됨

### SubscriptionEndReason

구독 종료 사유

- `CANCELED` — 해지
- `ONE_TIME_EXHAUSTED` — 일회성 상품 소진
- `END_DATE_REACHED` — 종료일 도달

### SubscriptionInterval

구독 주기

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `unit` | [SubscriptionIntervalUnit](#subscriptionintervalunit) | Y | 주기 단위 |
| `count` | integer(int32) | Y | 주기 값 — 단위가 월이고 값이 1이면 한 달에 한 번 청구합니다. |

### SubscriptionIntervalUnit

구독 주기 단위

- `DAY` — 일
- `WEEK` — 주
- `MONTH` — 월
- `YEAR` — 년

### SubscriptionInvoice

구독 인보이스

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 인보이스 아이디 |
| `subscriptionId` | string | Y | 구독 아이디 |
| `customerId` | string | Y | 고객 아이디 |
| `sequence` | integer(int32) |  | 청구 회차 — 수동 청구로 만든 인보이스에는 없습니다. |
| `regenerationSeq` | integer(int32) | Y | 재생성 차수 — 같은 회차를 다시 생성할 때마다 1씩 늘어납니다. |
| `status` | [SubscriptionInvoiceStatus](#subscriptioninvoicestatus) | Y | 인보이스 상태 |
| `source` | [SubscriptionInvoiceSource](#subscriptioninvoicesource) | Y | 생성 경로 |
| `billingAt` | string(date-time) | Y | 청구 일시 |
| `items` | [SubscriptionInvoiceItem](#subscriptioninvoiceitem)[] | Y | 청구 항목 |
| `discounts` | [SubscriptionInvoiceDiscountLine](#subscriptioninvoicediscountline)[] | Y | 적용된 할인 내역 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `supplyAmount` | integer(int64) | Y | 공급가액 |
| `vatAmount` | integer(int64) | Y | 부가세 — KRW에서만 계산하며 다른 통화는 0입니다. |
| `totalAmount` | integer(int64) | Y | 청구 총액 |
| `method` | [SubscriptionInvoiceMethod](#subscriptioninvoicemethod) | Y | 결제 수단 |
| `payment` | [SubscriptionInvoicePayment](#subscriptioninvoicepayment) |  | 결제 정보 — 결제를 시도하기 전이거나 0원 청구, 수동 완납이면 없습니다. |
| `isTest` | boolean | Y | 테스트 여부 — 인보이스를 발행한 시점 고객의 테스트 여부입니다. |
| `settlement` | [SubscriptionInvoiceSettlement](#subscriptioninvoicesettlement) |  | 수동 완납 정보 — 수동 완납한 인보이스에만 존재합니다. |
| `customerName` | string |  | 고객 이름 — 목록·단건 조회에서만 존재합니다. |
| `memo` | string |  | 메모 |
| `paidAt` | string(date-time) |  | 결제 완료 시각 |
| `canceledAt` | string(date-time) |  | 취소 시각 |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionInvoiceCurrencySummary

구독 인보이스 통화별 집계

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `count` | integer(int64) | Y | 건수 |
| `totalAmount` | integer(int64) | Y | 청구 총액 합 |
| `paidAmount` | integer(int64) | Y | 결제 금액 합 — 결제 완료·수동 완납 인보이스의 금액 합입니다. |

### SubscriptionInvoiceDiscountLine

구독 인보이스 할인 내역 — 구독 인보이스 할인 내역<br>인보이스 발행 시점의 할인 조건을 그대로 담습니다. 원본 할인이 이후에 바뀌어도 변하지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discountId` | string | Y | 할인 아이디 |
| `name` | string | Y | 할인명 |
| `type` | [SubscriptionDiscountType](#subscriptiondiscounttype) | Y | 할인 유형 |
| `value` | integer(int64) | Y | 할인 값 |
| `productIds` | string[] | Y | 적용 대상 상품 아이디 — 인보이스 전체에 적용된 할인이면 비어 있습니다. |
| `discountAmount` | integer(int64) | Y | 실제 깎인 금액 |

### SubscriptionInvoiceEmailEventType

인보이스 메일 유형

- `PAID` — 결제 완료
- `REFUNDED` — 환불

### SubscriptionInvoiceEmailSender

인보이스 메일 발신자 정보 — 인보이스 메일 발신자 정보<br>메일 본문 하단에 표시됩니다. 이메일은 회신 주소로도 씁니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyName` | string |  | 상호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `address` | string |  | 주소 |
| `email` | string |  | 이메일 |
| `websiteUrl` | string |  | 웹사이트 주소 |
| `phoneNumber` | string |  | 전화번호 |

### SubscriptionInvoiceEmailSenders

메일 언어별 발신자 정보 — 메일 언어별 발신자 정보<br>영어·일본어 정보가 없거나 필드가 비어 있으면 한국어 값을 씁니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `koKr` | [SubscriptionInvoiceEmailSender](#subscriptioninvoiceemailsender) | Y | 한국어 — 상호(companyName)는 필수입니다. 나머지 필드는 선택입니다. |
| `enUs` | [SubscriptionInvoiceEmailSender](#subscriptioninvoiceemailsender) |  | 영어 |
| `jaJp` | [SubscriptionInvoiceEmailSender](#subscriptioninvoiceemailsender) |  | 일본어 |

### SubscriptionInvoiceEmailTemplate

인보이스 메일 템플릿 — 인보이스 메일 템플릿<br>상점에 여러 개 둘 수 있고 활성 템플릿은 상점당 하나입니다. 활성 템플릿이 없으면 인보이스 메일을 보내지 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 템플릿 아이디 |
| `name` | string | Y | 템플릿 이름 |
| `logoImageUrl` | string |  | 로고 이미지 주소 — 메일 상단에 표시하는 로고 이미지 주소입니다. |
| `senders` | [SubscriptionInvoiceEmailSenders](#subscriptioninvoiceemailsenders) | Y | 발신자 정보 |
| `events` | [SubscriptionInvoiceEmailEventType](#subscriptioninvoiceemaileventtype)[] | Y | 보낼 메일 유형 |
| `isActive` | boolean | Y | 활성 여부 |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionInvoiceEvent

구독 인보이스 처리내역 — 구독 인보이스 처리내역<br>인보이스 상태가 바뀐 순간의 기록입니다. 쌓이기만 합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 처리내역 아이디 |
| `occurredAt` | string(date-time) | Y | 일어난 시각 |
| `detail` | [SubscriptionInvoiceEventDetail](#subscriptioninvoiceeventdetail) | Y | 내용 |

### SubscriptionInvoiceEventCanceled

청구 취소

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |

### SubscriptionInvoiceEventDetail

SubscriptionInvoiceEventDetail

oneOf, discriminator `type`:

- `CANCELED` → [SubscriptionInvoiceEventCanceled](#subscriptioninvoiceeventcanceled)
- `PAYMENT_FAILED` → [SubscriptionInvoiceEventPaymentFailed](#subscriptioninvoiceeventpaymentfailed)
- `PAYMENT_METHOD_CHANGED` → [SubscriptionInvoiceEventPaymentMethodChanged](#subscriptioninvoiceeventpaymentmethodchanged)
- `PAYMENT_REFUNDED` → [SubscriptionInvoiceEventPaymentRefunded](#subscriptioninvoiceeventpaymentrefunded)
- `PAYMENT_SUCCEEDED` → [SubscriptionInvoiceEventPaymentSucceeded](#subscriptioninvoiceeventpaymentsucceeded)
- `SETTLED_MANUALLY` → [SubscriptionInvoiceEventSettledManually](#subscriptioninvoiceeventsettledmanually)
- `ZERO_AMOUNT_PAID` → [SubscriptionInvoiceEventZeroAmountPaid](#subscriptioninvoiceeventzeroamountpaid)

### SubscriptionInvoiceEventPaymentFailed

결제 실패

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |
| `attempt` | [SubscriptionInvoicePaymentAttempt](#subscriptioninvoicepaymentattempt) | Y | 결제 시도 |
| `failure` | [SubscriptionInvoicePaymentFailure](#subscriptioninvoicepaymentfailure) | Y | 실패 정보 |

### SubscriptionInvoiceEventPaymentMethodChanged

결제 수단 변경 — 결제 수단 변경<br>실패한 채로 있는 동안 고객 빌링키가 바뀌었습니다. 다음 재시도는 이 수단으로 나갑니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |
| `method` | [SubscriptionInvoiceMethod](#subscriptioninvoicemethod) | Y | 바뀐 결제 수단 |

### SubscriptionInvoiceEventPaymentRefunded

결제 환불 — 결제 환불<br>결제 건이 취소됐습니다. 부분 취소마다 한 줄입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |
| `refund` | [SubscriptionInvoiceRefund](#subscriptioninvoicerefund) | Y | 환불 |

### SubscriptionInvoiceEventPaymentSucceeded

결제 승인

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |
| `attempt` | [SubscriptionInvoicePaymentAttempt](#subscriptioninvoicepaymentattempt) | Y | 결제 시도 |

### SubscriptionInvoiceEventSettledManually

수동 완납 — 수동 완납<br>처리내역 시각은 처리한 시각이고, 실제 입금 시각은 paidAt입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |
| `method` | [SubscriptionInvoiceSettlementMethod](#subscriptioninvoicesettlementmethod) | Y | 완납 방법 |
| `paidAt` | string(date-time) | Y | 입금 시각 |

### SubscriptionInvoiceEventZeroAmountPaid

0원 완납 — 0원 완납<br>청구 금액이 0원이라 결제 없이 완납했습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 구독 인보이스 처리내역 내용 |

### SubscriptionInvoiceFilterInput

구독 인보이스 조회 조건 — 구독 인보이스 조회 조건<br>번호·고객명 검색도 기간 안에서 찾습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampType` | [SubscriptionInvoiceTimestampType](#subscriptioninvoicetimestamptype) |  | 조회 기준 시점 — 미입력 시 생성 일시 기준입니다. |
| `from` | string(date-time) |  | 조회 시작 일시 — 미입력 시 조회 종료 일시가 속한 달의 1일(KST)입니다. |
| `to` | string(date-time) |  | 조회 종료 일시 — 미입력 시 현재입니다. |
| `currencies` | [Currency](../00-basics/schemas-common.md#currency)[] |  | 통화 리스트 |
| `invoiceId` | string |  | 인보이스 아이디 |
| `customerId` | string |  | 고객 아이디 |
| `subscriptionId` | string |  | 구독 아이디 |
| `customerName` | string |  | 고객 이름 — 부분 일치이며 2글자 이상 입력합니다. |
| `isTest` | boolean |  | 테스트 인보이스 여부 — true면 테스트 인보이스만, 미입력이거나 false면 라이브 인보이스만 조회합니다. |

### SubscriptionInvoiceItem

구독 인보이스 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `productId` | string |  | 상품 아이디 — 수동 청구로 추가한 임의 항목에는 없습니다. |
| `name` | string | Y | 항목명 |
| `unitPrice` | integer(int64) | Y | 단가 |
| `quantity` | integer(int32) | Y | 수량 |
| `amount` | integer(int64) | Y | 항목 금액 — 단가와 수량을 곱한 금액입니다. |

### SubscriptionInvoiceItemForm

SubscriptionInvoiceItemForm

oneOf, discriminator `type`:

- `CUSTOM` → [SubscriptionInvoiceItemFormCustom](#subscriptioninvoiceitemformcustom)
- `PRODUCT` → [SubscriptionInvoiceItemFormProduct](#subscriptioninvoiceitemformproduct)

### SubscriptionInvoiceItemFormCustom

임의 항목으로 청구

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 수동 청구 항목 입력 |
| `name` | string | Y | 항목명 |
| `unitPrice` | integer(int64) | Y | 단가 |
| `quantity` | integer(int32) | Y | 수량 |

### SubscriptionInvoiceItemFormProduct

등록된 상품으로 청구

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 수동 청구 항목 입력 |
| `productId` | string | Y | 상품 아이디 |
| `quantity` | integer(int32) | Y | 수량 |

### SubscriptionInvoiceMethod

구독 인보이스 결제 수단

- `CARD_BILLING` — 카드 빌링키
- `PAYPAL_BILLING` — 페이팔 빌링키

### SubscriptionInvoicePayment

구독 인보이스 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `paymentId` | string | Y | 결제 건 아이디 |
| `billingKey` | string | Y | 사용한 빌링키 |
| `failure` | [SubscriptionInvoicePaymentFailure](#subscriptioninvoicepaymentfailure) |  | 실패 정보 — 결제에 실패한 경우에만 존재합니다. |
| `attemptCount` | integer(int32) | Y | 결제 시도 수 — 결과를 받은 결제 시도 수입니다. 시도 내역은 인보이스 단건 조회의 처리내역에 있습니다. |
| `lastAttemptedAt` | string(date-time) |  | 마지막 결제 시도 시각 |
| `refunds` | [SubscriptionInvoiceRefund](#subscriptioninvoicerefund)[] | Y | 환불 내역 — 환불돼도 인보이스 상태는 결제 완료 그대로입니다. |

### SubscriptionInvoicePaymentAttempt

구독 인보이스 결제 시도

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sequence` | integer(int32) | Y | 시도 순번 — 인보이스 안에서 1부터 셉니다. |
| `trigger` | [SubscriptionInvoicePaymentTrigger](#subscriptioninvoicepaymenttrigger) | Y | 시도 계기 |
| `method` | [SubscriptionInvoiceMethod](#subscriptioninvoicemethod) |  | 결제 수단 |
| `txId` | string |  | 포트원 거래 아이디 — 조회에 실패했거나 거래가 만들어지기 전에 거절된 실패면 없습니다. |
| `pgTxId` | string |  | 결제사 거래 아이디 |

### SubscriptionInvoicePaymentFailure

구독 인보이스 결제 실패 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [SubscriptionInvoicePaymentFailureType](#subscriptioninvoicepaymentfailuretype) | Y | 실패 유형 |
| `pgCode` | string |  | 결제사 오류 코드 — 결제사가 거절한 경우에만 존재합니다. |
| `pgMessage` | string |  | 결제사 오류 메시지 — 결제사가 거절한 경우에만 존재합니다. |
| `message` | string |  | 실패 사유 — 결제 요청이 검증에서 거절되는 등 사유 문장이 있는 경우에만 존재합니다. |
| `reason` | [SubscriptionInvoicePaymentFailureReason](#subscriptioninvoicepaymentfailurereason) |  | 실패 사유 분류 — 결제사가 거절한 경우에만 존재합니다. |

### SubscriptionInvoicePaymentFailureReason

구독 인보이스 결제 실패 사유 분류 — 구독 인보이스 결제 실패 사유 분류<br>결제사 실패 메시지를 단어로 묶은 것입니다.

- `INSUFFICIENT_FUNDS_OR_LIMIT_EXCEEDED` — 잔액 부족·한도 초과
- `CARD_UNAVAILABLE` — 이용 불가 카드
- `CARD_EXPIRED` — 카드 유효기간 오류
- `DECLINED` — 카드사 거절
- `OTHER` — 기타

### SubscriptionInvoicePaymentFailureType

구독 인보이스 결제 실패 유형

- `PG_PROVIDER` — 결제사 거절
- `BILLING_KEY_NOT_FOUND` — 빌링키 없음
- `BILLING_KEY_ALREADY_DELETED` — 삭제된 빌링키
- `CHANNEL_NOT_FOUND` — 채널 없음
- `MAX_TRANSACTION_COUNT_REACHED` — 결제 시도 횟수 초과
- `INVALID_REQUEST` — 잘못된 결제 요청
- `PORTONE_ERROR` — 포트원 내부 오류

### SubscriptionInvoicePaymentTrigger

구독 인보이스 결제 시도 계기

- `ISSUE` — 발행 직후 자동 결제
- `RETRY` — 재시도
- `REGISTRATION` — 구독 등록 결제창 결제

### SubscriptionInvoicePreview

다음 인보이스 미리보기 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `billingAt` | string(date-time) | Y | 청구 예정 일시 |
| `sequence` | integer(int32) | Y | 청구 회차 |
| `items` | [SubscriptionInvoiceItem](#subscriptioninvoiceitem)[] | Y | 청구 항목 |
| `discounts` | [SubscriptionInvoiceDiscountLine](#subscriptioninvoicediscountline)[] | Y | 적용될 할인 내역 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `supplyAmount` | integer(int64) | Y | 공급가액 |
| `vatAmount` | integer(int64) | Y | 부가세 |
| `totalAmount` | integer(int64) | Y | 청구 총액 |

### SubscriptionInvoiceRefund

구독 인보이스 환불 — 구독 인보이스 환불<br>결제 건이 결제취소 API·콘솔로 취소된 한 건입니다. 부분 취소마다 한 건씩 쌓입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cancellationId` | string | Y | 취소 아이디 |
| `amount` | integer(int64) | Y | 환불 금액 |
| `refundedAt` | string(date-time) | Y | 환불 시각 |
| `reason` | string |  | 취소 사유 |

### SubscriptionInvoiceSettlement

구독 인보이스 수동 완납 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `method` | [SubscriptionInvoiceSettlementMethod](#subscriptioninvoicesettlementmethod) | Y | 완납 방법 |

### SubscriptionInvoiceSettlementMethod

구독 인보이스 수동 완납 방법

- `BANK_TRANSFER` — 계좌 입금 확인
- `CARD` — 현장 카드 결제 확인
- `OTHER` — 기타

### SubscriptionInvoiceSortBy

구독 인보이스 정렬 기준

- `BILLING_AT` — 청구 일시
- `CREATED_AT` — 생성 일시
- `PAID_AT` — 결제 일시
- `TOTAL_AMOUNT` — 청구 총액
- `LAST_ATTEMPTED_AT` — 마지막 결제 시도 일시

### SubscriptionInvoiceSource

구독 인보이스 생성 경로

- `AUTO` — 청구 주기에 따라 자동 생성
- `MANUAL` — 수동 청구로 생성

### SubscriptionInvoiceStatus

구독 인보이스 상태

- `PENDING` — 결제 결과 대기 중
- `PAID` — 결제 완료
- `FAILED` — 결제 실패
- `CANCELED` — 취소됨
- `PAID_MANUALLY` — 수동 완납 처리됨

### SubscriptionInvoiceStatusCount

구독 인보이스 상태별 건수

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | [SubscriptionInvoiceStatus](#subscriptioninvoicestatus) | Y | 인보이스 상태 |
| `count` | integer(int64) | Y | 건수 |

### SubscriptionInvoiceTimestampType

구독 인보이스 조회 기준 시점

- `BILLING_AT` — 청구 일시
- `CREATED_AT` — 생성 일시
- `PAID_AT` — 결제 일시

### SubscriptionItem

구독 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `productId` | string | Y | 상품 아이디 |
| `quantity` | integer(int32) | Y | 수량 |

### SubscriptionPaymentOptions

구독 결제 옵션 — 구독 결제 옵션<br>매 회차 결제 요청에 그대로 실리는 값입니다. 빌링키 결제 예약의 같은 이름 필드와 의미가 같습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `locale` | [Locale](../00-basics/schemas-common.md#locale) |  | 결제 시 사용할 언어 — 엑심베이의 경우 필수 입력입니다. |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `customData` | string |  | 사용자 지정 데이터 |

### SubscriptionPreRegisterCustomerInput

함께 등록할 구독 고객 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 고객 아이디 — 미입력 시 새 고객으로 등록하고 아이디를 자동 생성합니다. 입력한 아이디의 고객이 있으면 그 고객을 갱신합니다. |
| `name` | string |  | 고객명 — 신규 고객이면 필수입니다. |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### SubscriptionProduct

구독 상품

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 아이디 — 고객사가 직접 부여하거나 미입력 시 자동 생성됩니다. |
| `name` | string | Y | 상품명 |
| `description` | string |  | 상품 설명 |
| `pricingType` | [SubscriptionProductPricingType](#subscriptionproductpricingtype) | Y | 과금 유형 |
| `interval` | [SubscriptionInterval](#subscriptioninterval) |  | 청구 주기 — 정기 과금 상품에만 존재합니다. |
| `amount` | integer(int64) | Y | 상품 가격 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `billingPaused` | boolean | Y | 청구 중지 여부 — 청구가 중지된 상품이 담긴 구독은 해당 회차를 청구하지 않습니다. |
| `archived` | boolean | Y | 보관 여부 — 보관된 상품은 새 구독에 담을 수 없고, 기존 구독의 청구는 유지됩니다. |
| `metadata` | [Map_String](#map_string) | Y | 사용자 정의 데이터 |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `updatedAt` | string(date-time) | Y | 수정 시각 |

### SubscriptionProductPricingType

구독 상품 과금 유형

- `RECURRING` — 정기 과금
- `ONE_TIME` — 일회성 과금

### SubscriptionStatus

구독 상태

- `ACTIVE` — 청구 중
- `INCOMPLETE` — 첫 회차 미수납
- `PAUSED` — 일시중지
- `ENDED` — 종료

### UnarchiveSubscriptionDiscountResponse

구독 할인 보관 해제 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discount` | [SubscriptionDiscount](#subscriptiondiscount) | Y | 반영된 구독 할인 |

### UpdateSubscriptionBody

구독 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `items` | [SubscriptionItem](#subscriptionitem)[] |  | 구독 항목 — 전체 교체입니다. 추가·삭제·수량 변경을 한 번에 보냅니다. 정기 과금 상품이 하나 이상 있어야 하고, 담긴 정기 상품들의 청구 주기가 같아야 합니다. 시작한 구독은 청구 주기가 다른 정기 상품으로 바꿀 수 없고, 사전 등록 상태(구독 등록 전)에서는 새 상품의 청구 주기를 따릅니다. |
| `endDate` | string(date) |  | 구독 종료일 — 구독이 끝나는 날로, 오늘보다 뒤여야 합니다. 이날과 그 뒤로는 청구하지 않습니다. null을 보내면 무기한이 됩니다. |
| `sendInvoiceEmail` | boolean |  | 인보이스 메일 발송 여부 |
| `nextBillingDate` | string(date) |  | 다음 청구일 — 정기결제일을 옮깁니다. 오늘보다 뒤이고 종료일보다 앞이어야 하며, 지정한 날짜가 새 청구 기준일이 됩니다. 기준 시각과 회차 번호는 유지됩니다. 사전 등록한 구독은 구독 등록 전에 지정할 수 없습니다. |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `paymentOptions` | [UpdateSubscriptionBodyPaymentOptions](#updatesubscriptionbodypaymentoptions) |  | 결제 옵션 — 입력한 필드만 고치고, null을 보낸 필드는 지웁니다. 다음 결제와 실패한 인보이스의 재시도부터 반영됩니다. 고객에게 등록된 결제수단이 있으면 고친 값으로 결제할 수 있는지 미리 확인하고, 결제할 수 없으면 수정하지 않습니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### UpdateSubscriptionBodyPaymentOptions

구독 결제 옵션 수정 입력 — 구독 결제 옵션 수정 입력<br>입력한 필드만 고칩니다. null을 보내면 그 필드를 지웁니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `locale` | [Locale](../00-basics/schemas-common.md#locale) |  | 결제 시 사용할 언어 — 엑심베이의 경우 필수 입력입니다. |
| `productType` | [PaymentProductType](../00-basics/schemas-common.md#paymentproducttype) |  | 상품 유형 |
| `installmentMonth` | integer(int32) |  | 할부 개월 수 |
| `useFreeInterestFromMerchant` | boolean |  | 무이자 할부 이자를 고객사가 부담할지 여부 |
| `useCardPoint` | boolean |  | 카드 포인트 사용 여부 |
| `country` | [Country](../00-basics/schemas-common.md#country) |  | 결제 국가 |
| `bypass` | object |  | PG사별 추가 파라미터 ("PG사별 연동 가이드" 참고) |
| `customData` | string |  | 사용자 지정 데이터 |

### UpdateSubscriptionCustomerBody

구독 고객 정보 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 고객명 |
| `email` | string |  | 이메일 — 변경은 다음 인보이스 메일부터 반영됩니다. |
| `phoneNumber` | string |  | 휴대폰 번호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |
| `memo` | string |  | 메모 — 고객사 내부 메모입니다. |

### UpdateSubscriptionCustomerResponse

구독 고객 정보 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `customer` | [SubscriptionCustomer](#subscriptioncustomer) | Y | 수정된 고객 |

### UpdateSubscriptionDiscountBody

구독 할인 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 할인명 |

### UpdateSubscriptionDiscountResponse

구독 할인 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `discount` | [SubscriptionDiscount](#subscriptiondiscount) | Y | 수정된 구독 할인 |

### UpdateSubscriptionInvoiceEmailTemplateBody

인보이스 메일 템플릿 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 템플릿 이름 |
| `logoImageUrl` | string |  | 로고 이미지 주소 — 메일 상단에 표시할 로고 이미지 주소입니다. 인증 없이 열리는 https 주소를 입력해 주세요. 가로 100px, 세로 24px 영역에 비율을 유지해 표시됩니다. |
| `senders` | [UpdateSubscriptionInvoiceEmailTemplateBodySenders](#updatesubscriptioninvoiceemailtemplatebodysenders) |  | 발신자 정보 |
| `events` | [SubscriptionInvoiceEmailEventType](#subscriptioninvoiceemaileventtype)[] |  | 보낼 메일 유형 — 입력한 값으로 전체를 교체합니다. |

### UpdateSubscriptionInvoiceEmailTemplateBodySender

인보이스 메일 발신자 정보 수정 입력 — 인보이스 메일 발신자 정보 수정 입력<br>입력한 필드만 고칩니다. null을 보내면 그 필드를 지웁니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `companyName` | string |  | 상호 |
| `businessRegistrationNumber` | string |  | 사업자등록번호 |
| `address` | string |  | 주소 |
| `email` | string |  | 이메일 |
| `websiteUrl` | string |  | 웹사이트 주소 |
| `phoneNumber` | string |  | 전화번호 |

### UpdateSubscriptionInvoiceEmailTemplateBodySenders

메일 언어별 발신자 정보 수정 입력 — 메일 언어별 발신자 정보 수정 입력<br>언어마다 입력한 필드만 고칩니다. 정보가 없던 언어는 입력한 값으로 새로 만듭니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `koKr` | [UpdateSubscriptionInvoiceEmailTemplateBodySender](#updatesubscriptioninvoiceemailtemplatebodysender) |  | 한국어 — null로 지울 수 없습니다. 상호(companyName)는 필수입니다. |
| `enUs` | [UpdateSubscriptionInvoiceEmailTemplateBodySender](#updatesubscriptioninvoiceemailtemplatebodysender) |  | 영어 |
| `jaJp` | [UpdateSubscriptionInvoiceEmailTemplateBodySender](#updatesubscriptioninvoiceemailtemplatebodysender) |  | 일본어 |

### UpdateSubscriptionInvoiceEmailTemplateResponse

인보이스 메일 템플릿 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `invoiceEmailTemplate` | [SubscriptionInvoiceEmailTemplate](#subscriptioninvoiceemailtemplate) | Y | 수정된 인보이스 메일 템플릿 |

### UpdateSubscriptionProductBody

구독 상품 수정 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string |  | 상점 아이디 — 접근 권한이 있는 상점 아이디만 입력 가능하며, 미입력시 인증 정보의 상점 아이디를 사용합니다. |
| `name` | string |  | 상품명 |
| `description` | string |  | 상품 설명 |
| `amount` | integer(int64) |  | 상품 가격 |
| `metadata` | [Map_String](#map_string) |  | 사용자 정의 데이터 — 입력한 값으로 전체를 교체합니다. |

### UpdateSubscriptionProductResponse

구독 상품 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [SubscriptionProduct](#subscriptionproduct) | Y | 수정된 구독 상품 |

### UpdateSubscriptionResponse

구독 수정 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `subscription` | [Subscription](#subscription) | Y | 수정된 구독 |
