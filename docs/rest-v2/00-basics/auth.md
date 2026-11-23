# 인증 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `POST /login/api-secret` — API secret 를 사용한 토큰 발급
- `POST /token/refresh` — 토큰 갱신

## POST /login/api-secret

**API secret 를 사용한 토큰 발급** (operationId: `loginViaApiSecret`)

API secret 를 통해 API 인증에 사용할 토큰을 가져옵니다.

### 요청 본문 (application/json, 필수)

타입: [LoginViaApiSecretBody](#loginviaapisecretbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `apiSecret` | string | Y | 발급받은 API secret |

### 응답

- **200** 성공 응답으로 토큰을 반환합니다.: [LoginViaApiSecretResponse](#loginviaapisecretresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

## POST /token/refresh

**토큰 갱신** (operationId: `refreshToken`)

리프레시 토큰을 사용해 유효기간이 연장된 새로운 토큰을 재발급합니다.

### 요청 본문 (application/json, 필수)

타입: [RefreshTokenBody](#refreshtokenbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `refreshToken` | string | Y | 리프레시 토큰 |

### 응답

- **200** : [RefreshTokenResponse](#refreshtokenresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

---

# 스키마

### LoginViaApiSecretBody

API Secret 로그인을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `apiSecret` | string | Y | 발급받은 API secret |

### LoginViaApiSecretResponse

API key 로그인 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `accessToken` | string | Y | 인증에 사용하는 엑세스 토큰 — 30분의 유효기간을 가지고 있습니다. |
| `refreshToken` | string | Y | 토큰 재발급 및 유효기간 연장을 위해 사용하는 리프레시 토큰 — 하루의 유효기간을 가지고 있으며, 리프레시 토큰을 통해 유효기간이 연장된 새로운 엑세스 토큰을 발급받을 수 있습니다.<br>동일한 유저가 로그인 또는 토큰 재발급을 진행할 때마다 기존에 발급된 리프레시 토큰은 즉시 만료되므로 API 사용에 유의해주세요. |

### RefreshTokenBody

토큰 재발급을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `refreshToken` | string | Y | 리프레시 토큰 |

### RefreshTokenResponse

토큰 재발급 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `accessToken` | string | Y | 인증에 사용하는 엑세스 토큰 — 30분의 유효기간을 가지고 있습니다. |
| `refreshToken` | string | Y | 토큰 재발급 및 유효기간 연장을 위해 사용하는 리프레시 토큰 — 하루의 유효기간을 가지고 있으며, 리프레시 토큰을 통해 유효기간이 연장된 새로운 엑세스 토큰을 발급받을 수 있습니다.<br>동일한 유저가 로그인 또는 토큰 재발급을 진행할 때마다 기존에 발급된 리프레시 토큰은 즉시 만료되므로 API 사용에 유의해주세요. |
