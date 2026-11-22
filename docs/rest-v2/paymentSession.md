# 결제 세션 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `POST /payment-sessions` — 결제 세션 생성
- `GET /payment-sessions/{sessionId}` — 결제 세션 조회
- `POST /payment-sessions/{sessionId}/close` — 결제 세션 종료

## POST /payment-sessions

**결제 세션 생성** (operationId: `createPaymentSession`)

결제 세션을 생성합니다. 호스티드 체크아웃 페이지가 생성되어 URL이 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [CreatePaymentSessionBody](#createpaymentsessionbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `profileKey` | string | Y | 프로필 키 |
| `paymentMethod` | [CheckoutPaymentMethod](schemas-common.md#checkoutpaymentmethod) |  | 결제 수단 지정 — 지정한 경우, 정보 추가 입력이 필요하지 않은 경우에 주문서를 건너뛰고 결제로 바로 이동합니다. |
| `country` | [Country](schemas-common.md#country) | Y | 국가 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `totalAmount` | integer(int64) | Y | 전체 결제 금액 |
| `orderName` | string | Y | 주문명 |
| `redirectUrl` | string |  | 결제 완료 후 리다이렉트 URL — 지정하지 않으면 기본 결과 페이지가 표시됩니다. |
| `products` | [PaymentSessionProduct](#paymentsessionproduct)[] |  | 주문 항목 목록 |
| `customerName` | string |  | 구매자 이름 |
| `customerEmail` | string |  | 구매자 이메일 |
| `storeName` | string |  | 상점 이름 — 페이지 헤더 및 결제사 UI에 표시됩니다. |
| `agreements` | [PaymentSessionAgreement](#paymentsessionagreement)[] |  | 사용자 지정 약관 목록 — 구매자가 모든 약관에 동의해야 결제 버튼이 활성화됩니다. |
| `orderImageUrl` | string |  | 주문 대표 이미지 URL |
| `customData` | string |  | 사용자 지정 데이터 — 결제 완료 후 결제 건 조회에서도 확인할 수 있습니다. |
| `colors` | [PaymentSessionColors](#paymentsessioncolors) |  | 체크아웃 페이지 색 설정 |
| `ttlSeconds` | integer(int64) |  | 세션 TTL (초) |

### 응답

- **200** 성공 응답: [CreatePaymentSessionResponse](#createpaymentsessionresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`MaxTtlExceededError`](errors.md#maxttlexceedederror): 요청된 TTL이 정책 상한을 초과한 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우

## GET /payment-sessions/{sessionId}

**결제 세션 조회** (operationId: `getPaymentSession`)

결제 세션을 조회합니다. 인증 헤더 없이 웹 페이지에서도 접근 가능합니다.

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `sessionId` | path | string | Y | 결제 세션 아이디 |

### 응답

- **200** 결제 세션 조회 성공 응답: [PaymentSession](#paymentsession)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **404**
  * [`SessionNotFoundError`](errors.md#sessionnotfounderror): 결제 세션이 존재하지 않는 경우
- **410**
  * [`SessionExpiredError`](errors.md#sessionexpirederror): 결제 세션이 만료된 경우

## POST /payment-sessions/{sessionId}/close

**결제 세션 종료** (operationId: `closePaymentSession`)

결제 세션을 즉시 만료시킵니다. 이후 해당 세션으로는 결제 페이지에 접근할 수 없습니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `sessionId` | path | string | Y | 결제 세션 아이디 |

### 응답

- **200** 성공 응답: [ClosePaymentSessionResponse](#closepaymentsessionresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`SessionNotFoundError`](errors.md#sessionnotfounderror): 결제 세션이 존재하지 않는 경우

---

# 스키마

### ClosePaymentSessionResponse

결제 세션 종료 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `closedAt` | string(date-time) | Y | 결제 세션 종료 시각 |

### CreatePaymentSessionBody

결제 세션 생성 요청

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `profileKey` | string | Y | 프로필 키 |
| `paymentMethod` | [CheckoutPaymentMethod](schemas-common.md#checkoutpaymentmethod) |  | 결제 수단 지정 — 지정한 경우, 정보 추가 입력이 필요하지 않은 경우에 주문서를 건너뛰고 결제로 바로 이동합니다. |
| `country` | [Country](schemas-common.md#country) | Y | 국가 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `totalAmount` | integer(int64) | Y | 전체 결제 금액 |
| `orderName` | string | Y | 주문명 |
| `redirectUrl` | string |  | 결제 완료 후 리다이렉트 URL — 지정하지 않으면 기본 결과 페이지가 표시됩니다. |
| `products` | [PaymentSessionProduct](#paymentsessionproduct)[] |  | 주문 항목 목록 |
| `customerName` | string |  | 구매자 이름 |
| `customerEmail` | string |  | 구매자 이메일 |
| `storeName` | string |  | 상점 이름 — 페이지 헤더 및 결제사 UI에 표시됩니다. |
| `agreements` | [PaymentSessionAgreement](#paymentsessionagreement)[] |  | 사용자 지정 약관 목록 — 구매자가 모든 약관에 동의해야 결제 버튼이 활성화됩니다. |
| `orderImageUrl` | string |  | 주문 대표 이미지 URL |
| `customData` | string |  | 사용자 지정 데이터 — 결제 완료 후 결제 건 조회에서도 확인할 수 있습니다. |
| `colors` | [PaymentSessionColors](#paymentsessioncolors) |  | 체크아웃 페이지 색 설정 |
| `ttlSeconds` | integer(int64) |  | 세션 TTL (초) |

### CreatePaymentSessionResponse

결제 세션 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sessionId` | string | Y | 세션 아이디 |
| `url` | string | Y | 호스티드 체크아웃 페이지 URL |
| `expiresAt` | string(date-time) | Y | 만료 시각 |

### PaymentSession

결제 세션

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 결제 세션 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `paymentId` | string | Y | 결제 건 아이디 |
| `profileKey` | string | Y | 프로필 키 |
| `paymentMethod` | [CheckoutPaymentMethod](schemas-common.md#checkoutpaymentmethod) |  | 결제 수단 지정 — 지정한 경우, 정보 추가 입력이 필요하지 않은 경우에 주문서를 건너뛰고 결제로 바로 이동합니다. |
| `country` | [Country](schemas-common.md#country) | Y | 국가 |
| `currency` | [Currency](schemas-common.md#currency) | Y | 통화 |
| `totalAmount` | integer(int64) | Y | 전체 결제 금액 |
| `orderName` | string | Y | 주문명 |
| `redirectUrl` | string |  | 결제 완료 후 리다이렉트 URL — 지정하지 않으면 기본 결과 페이지가 표시됩니다. |
| `products` | [PaymentSessionProduct](#paymentsessionproduct)[] |  | 주문 항목 목록 |
| `customerName` | string |  | 구매자 이름 |
| `customerEmail` | string |  | 구매자 이메일 |
| `storeName` | string |  | 상점 이름 — 페이지 헤더 및 결제사 UI에 표시됩니다. |
| `agreements` | [PaymentSessionAgreement](#paymentsessionagreement)[] |  | 사용자 지정 약관 목록 — 구매자가 모든 약관에 동의해야 결제 버튼이 활성화됩니다. |
| `orderImageUrl` | string |  | 주문 대표 이미지 URL |
| `customData` | string |  | 사용자 지정 데이터 — 결제 완료 후 결제 건 조회에서도 확인할 수 있습니다. |
| `colors` | [PaymentSessionColors](#paymentsessioncolors) |  | 체크아웃 페이지 색 설정 |
| `createdAt` | string(date-time) | Y | 생성 시각 |
| `expiresAt` | string(date-time) | Y | 만료 시각 |

### PaymentSessionAgreement

결제 세션 약관

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 약관 이름 |
| `url` | string | Y | 약관 URL |

### PaymentSessionColors

체크아웃 페이지 색 설정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `primary` | string |  | 주 색상 — CSS 색 문자열 형식 |
| `primaryHover` | string |  | 호버 주 색상 — CSS 색 문자열 형식 |
| `primaryLight` | string |  | 밝은 주 색상 — CSS 색 문자열 형식 |

### PaymentSessionProduct

결제 세션 주문 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 항목 아이디 |
| `name` | string | Y | 상품 이름 |
| `code` | string |  | 상품 코드 |
| `unitPrice` | integer(int64) | Y | 상품 단가 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `startDate` | string(date) |  | 제공 시작일 — 구독 등 제공 기간이 있는 상품의 경우 입력하세요. |
| `endDate` | string(date) |  | 제공 종료일 — 구독 등 제공 기간이 있는 상품의 경우 입력하세요. |
| `url` | string |  | 판매 링크 |
| `nPayCategoryType` | string |  | 네이버페이 카테고리 타입 |
| `nPayCategoryId` | string |  | 네이버페이 카테고리 아이디 |
| `nPayUid` | string |  | 네이버페이 상품 식별자 |
