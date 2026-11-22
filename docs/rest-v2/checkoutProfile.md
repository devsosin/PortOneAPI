# 체크아웃 프로필 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /checkout-profiles/evaluate` — 체크아웃 프로필에서 결제 수단 목록 조회

## GET /checkout-profiles/evaluate

**체크아웃 프로필에서 결제 수단 목록 조회** (operationId: `evaluateCheckoutProfile`)

주어진 금액 및 국가에서 사용 가능한 결제 수단 목록을 반환

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `profileKey` | query | string | Y | 프로필 키 |
| `country` | query | [Country](schemas-common.md#country) | Y | 국가 |
| `currency` | query | [Currency](schemas-common.md#currency) | Y | 통화 |
| `amount` | query | integer(int64) | Y | 결제 금액 |

### 응답

- **200** 성공 응답: [EvaluateCheckoutProfileResponse](#evaluatecheckoutprofileresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **404**
  * [`ProfileSettingsNotFoundError`](errors.md#profilesettingsnotfounderror): 프로필 설정이 존재하지 않는 경우

---

# 스키마

### EvaluateCheckoutProfileResponse

체크아웃 프로필 평가 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `methods` | [EvaluatedCheckoutMethod](#evaluatedcheckoutmethod)[] |  | 사용 가능한 결제수단 목록 |

### EvaluatedCheckoutMethod

결제 수단

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `paymentMethod` | [CheckoutPaymentMethod](schemas-common.md#checkoutpaymentmethod) | Y | 결제수단 |
| `channelKey` | string | Y | 채널 키 |
