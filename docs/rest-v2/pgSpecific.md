# 특정 PG사 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /kakaopay/payment/order` — 카카오페이 주문 조회 API
- `POST /paymentwall/delivery/confirm` — 페이먼트월 배송 정보 등록

## GET /kakaopay/payment/order

**카카오페이 주문 조회 API** (operationId: `getKakaopayPaymentOrder`)

주어진 아이디에 대응되는 카카오페이 주문 건을 조회합니다.
해당 API 사용이 필요한 경우 포트원 기술지원팀으로 문의 주시길 바랍니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `pgTxId` | query | string | Y | 카카오페이 주문 번호 (tid) |
| `channelKey` | query | string | Y | 채널 키 |

### 응답

- **200** 성공 응답으로 카카오페이 주문 조회 응답 객체를 반환합니다.: [GetKakaopayPaymentOrderResponse](#getkakaopaypaymentorderresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

## POST /paymentwall/delivery/confirm

**페이먼트월 배송 정보 등록** (operationId: `confirmPaymentwallDelivery`)

배송 정보를 페이먼트월에 등록합니다.
등록된 배송 정보는 차지백 발생 시 고객사의 상품 배송 완료 증빙 자료로 활용되므로, 반드시 연동해야 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 요청 본문 (application/json, 필수)

타입: [ConfirmPaymentwallDeliveryBody](#confirmpaymentwalldeliverybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 |
| `deliveryType` | [PaymentwallDeliveryType](#paymentwalldeliverytype) | Y | 배송 유형 |
| `deliveryStatus` | [PaymentwallDeliveryStatus](#paymentwalldeliverystatus) | Y | 배송 상태 |
| `estimatedDeliveryDatetime` | string(date-time) | Y | 배송 완료 예상 일시 — 배송 유형이 DIGITAL인 경우 현재 시각을 입력해도 무방합니다. |
| `estimatedUpdateDatetime` | string(date-time) | Y | 배송 상태 업데이트 예정 일시 — 배송 유형이 DIGITAL인 경우 현재 시각을 입력해도 무방합니다. |
| `reason` | string |  | 상태 변경 사유 |
| `refundable` | boolean | Y | 환불 가능 여부 |
| `details` | string | Y | 상세 설명 |
| `shippingAddressEmail` | string | Y | 고객 이메일 주소 |
| `carrierTrackingId` | string |  | 운송장 번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `carrierType` | string |  | 운송사 이름 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressCountry` | [Country](schemas-common.md#country) |  | 수신자 국가 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressCity` | string |  | 수신자 도시 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressZip` | string |  | 수신자 우편번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressState` | string |  | 수신자 주 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressStreet` | string |  | 수신자 도로명 주소 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressPhone` | string |  | 수신자 전화번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressFirstname` | string |  | 수신자 이름 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressLastname` | string |  | 수신자 성 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `attachments` | string[] |  | 배송 증빙 첨부 파일 URL 목록 — 배송 증빙 자료의 URL(이미지 등)을 입력합니다. 증빙 자료를 제공하기 어려운 경우 생략할 수 있습니다. |

### 응답

- **200** 성공 응답: [ConfirmPaymentwallDeliveryResponse](#confirmpaymentwalldeliveryresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **404**
  * [`PaymentNotFoundError`](errors.md#paymentnotfounderror): 결제 건이 존재하지 않는 경우

---

# 스키마

### ConfirmPaymentwallDeliveryBody

페이먼트월 배송 정보 등록 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transactionId` | string | Y | 결제 건 포트원 채번 아이디 |
| `deliveryType` | [PaymentwallDeliveryType](#paymentwalldeliverytype) | Y | 배송 유형 |
| `deliveryStatus` | [PaymentwallDeliveryStatus](#paymentwalldeliverystatus) | Y | 배송 상태 |
| `estimatedDeliveryDatetime` | string(date-time) | Y | 배송 완료 예상 일시 — 배송 유형이 DIGITAL인 경우 현재 시각을 입력해도 무방합니다. |
| `estimatedUpdateDatetime` | string(date-time) | Y | 배송 상태 업데이트 예정 일시 — 배송 유형이 DIGITAL인 경우 현재 시각을 입력해도 무방합니다. |
| `reason` | string |  | 상태 변경 사유 |
| `refundable` | boolean | Y | 환불 가능 여부 |
| `details` | string | Y | 상세 설명 |
| `shippingAddressEmail` | string | Y | 고객 이메일 주소 |
| `carrierTrackingId` | string |  | 운송장 번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `carrierType` | string |  | 운송사 이름 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressCountry` | [Country](schemas-common.md#country) |  | 수신자 국가 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressCity` | string |  | 수신자 도시 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressZip` | string |  | 수신자 우편번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressState` | string |  | 수신자 주 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressStreet` | string |  | 수신자 도로명 주소 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressPhone` | string |  | 수신자 전화번호 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressFirstname` | string |  | 수신자 이름 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `shippingAddressLastname` | string |  | 수신자 성 — 배송 유형이 PHYSICAL인 경우 필수입니다. |
| `attachments` | string[] |  | 배송 증빙 첨부 파일 URL 목록 — 배송 증빙 자료의 URL(이미지 등)을 입력합니다. 증빙 자료를 제공하기 어려운 경우 생략할 수 있습니다. |

### ConfirmPaymentwallDeliveryResponse

페이먼트월 배송 정보 등록 성공 응답

type: object

### GetKakaopayPaymentOrderResponse

카카오페이 주문 조회 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statusCode` | integer(int32) | Y | HTTP 상태 코드 |
| `body` | string | Y | HTTP 응답 본문 (JSON) |

### PaymentwallDeliveryStatus

페이먼트월 배송 상태

- `ORDER_PLACED` — 주문 접수
- `ORDER_SHIPPED` — 배송 중
- `DELIVERED` — 배송 완료

### PaymentwallDeliveryType

페이먼트월 배송 유형

- `DIGITAL` — 디지털
- `PHYSICAL` — 실물
