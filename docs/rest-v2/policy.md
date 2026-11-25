# PortOne REST API V2 공통 정책

> 출처: PortOne REST API V2 공식 문서 — 공통 정책 (https://developers.portone.io/api/rest-v2). 소유자가 확인해 옮긴 텍스트이며 `scripts/fetch-rest-v2-docs.mjs` 가 생성하는 파일이 아니다.
>
> **API 를 추가하거나 바꾸기 전에 반드시 읽는다.** 크레이트 반영 현황은 맨 아래 표.

## 요청 및 응답 형식

요청과 응답의 본문은 JSON 형식입니다.
API 매개 변수 중 URL 경로 또는 query에 들어가는 문자열 값이 있는 경우, 그 자리에 들어갈 수 없는 문자는 이스케이프하여야 합니다. 자바스크립트의 `encodeURIComponent` 함수 등을 사용할 수 있습니다.

## 인증 방식

V2 API를 사용하기 위해서는 V2 API Secret이 필요하며, 포트원 관리자콘솔 내 결제연동 탭에서 발급받으실 수 있습니다.
인증 관련 API를 제외한 모든 API는 HTTP `Authorization` 헤더로 아래 형식의 인증 정보를 전달해주셔야 합니다.
* `Authorization: PortOne MY_API_SECRET`

`MY_API_SECRET` 자리에는 포트원 관리자콘솔 내 결제연동 탭에서 발급한 V2 API Secret을 넣어주세요.

## GET 요청 시 body 대신 query 사용하기

GET 요청 시에 body(content)를 전달하는 것은 HTTP 표준에 부합하지 않아, 클라이언트에 따라 사용 불가능한 경우가 있습니다.
이 경우, body 문자열을 `requestBody` query 필드에 넣어 주시면 됩니다. query 필드에 들어가는 문자열은 URL 인코드하셔야 합니다.

## 멱등 키

`Idempotency-Key` 헤더는 네트워크 장애나 타임아웃 시 동일한 요청이 중복 처리되는 것을 방지합니다. 자세한 표준은 IETF 문서(https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/)를 참고하세요.

### 사용 방법

요청 헤더에 고유한 키를 포함하세요. UUID 등 고유한 문자열이어야 합니다. 16~256자의 ASCII 문자열(권장: 영문 대소문자, 숫자, -, _) 을 사용 가능합니다. RFC 8941에 따라 문자열을 쌍따옴표로 감싸세요.
* `Idempotency-Key: "unique-request-id-123"`

요청 타임아웃이 발생한 경우, 동일한 멱등 키를 사용하여 재시도할 수 있습니다. 이 경우, 서버는 요청을 중복 처리하지 않고, 기존 응답을 반환합니다.
재시도에서 `IDEMPOTENCY_OUTSTANDING_REQUEST` 에러가 발생한 경우, 시간이 조금 지난 후 다시 재시도해 주세요.
* 지원 메서드: POST, PUT, PATCH, DELETE (GET은 무시됨)
* 처리 중인 요청을 재시도하는 경우: 409 에러 (`{"type":"IDEMPOTENCY_OUTSTANDING_REQUEST"}...}`)
* 완료된 요청을 재시도하는 경우: 기존 응답을 그대로 반환
* 멱등성 보장 기간: 3시간 (추후 변경 가능)
* 서로 다른 요청을 같은 멱등 키로 요청해서는 안 됩니다.

## 요청 타임아웃 정책

API 호출시 PG 및 결제 원천사의 응답 지연을 고려하여 최소 60초의 읽기 타임아웃 시간을 설정하도록 권장합니다.
서버에서 요청 처리 중에 클라이언트 측에서 연결을 끊은 경우에도 요청이 취소되지 않습니다. 이 경우 같은 멱등 키로 같은 요청을 재시도하면 결과를 확인할 수 있습니다.

## 크레이트 반영 현황 (2026-11-25 점검)

| 정책 | 상태 |
|---|---|
| 경로·쿼리 값 이스케이프 | `AuthedPortOneAPI::build_url` 이 경로 조각과 쿼리 값을 `encodeURIComponent` 규칙(공백 `%20`)으로 인코딩. `get_payment_detail` 이 인코딩 없이 `paymentId` 를 붙이던 것을 수정 |
| `Authorization: PortOne <secret>` | `client::request` 한 곳에서 부착 |
| GET 의 `requestBody` | `get_cash_receipts` 가 JSON 직렬화 후 `requestBody` 쿼리로 전달(URL 인코딩, 단위 테스트로 확인) |
| 읽기 타임아웃 60초 | 수정: 읽기 60초, 연결 10초(이전 전체 10초) |
| `Idempotency-Key` | 취소 API 만 지원. 현금영수증 발급·취소는 #15 |
