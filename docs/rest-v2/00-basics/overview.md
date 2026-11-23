# REST API V2 개요

> 출처: https://developers.portone.io/api/rest-v2/overview

# PortOne REST API - V2

API 결제, 결제 정보 조회, 결제 취소 등의 기능을 제공하는 REST API입니다.
V2 API hostname: `api.portone.io`
[OpenAPI JSON 내려받기](https://developers.portone.io/api/schema/v2/openapi.json)[Postman 등에서 import하여 활용](https://learning.postman.com/docs/integrations/available-integrations/working-with-openAPI/)할 수 있습니다.

### 요청 및 응답 형식

요청과 응답의 본문은 JSON 형식입니다.
API 매개 변수 중 URL 경로 또는 query에 들어가는 문자열 값이 있는 경우, 그 자리에 들어갈 수 없는 문자는 이스케이프하여야 합니다. 자바스크립트의 `encodeURIComponent` 함수 등을 사용할 수 있습니다.

### 인증 방식

V2 API를 사용하기 위해서는 V2 API Secret이 필요하며, 포트원 관리자콘솔 내 결제연동 탭에서 발급받으실 수 있습니다.
인증 관련 API를 제외한 모든 API는 HTTP `Authorization` 헤더로 아래 형식의 인증 정보를 전달해주셔야 합니다.

- `Authorization: PortOne MY_API_SECRET`
`MY_API_SECRET` 자리에는 포트원 관리자콘솔 내 결제연동 탭에서 발급한 V2 API Secret을 넣어주세요.
콘솔 채널 설정에 입력하는 PG사 시크릿(Secret Key, OTP 등)은 API 인증 용도로 사용할 수 없습니다. PG사 시크릿을 이 헤더에 넣으면 `401 Unauthorized` 오류가 발생하므로, 포트원 관리자콘솔에서 발급한 V2 API Secret 값이 맞는지 확인해 주세요. 발급 방법은 [V2 API Secret 발급하기](https://developers.portone.io/opi/ko/integration/ready/readme?v=v2#4-2-v2-api-secret-발급하기)를 참고해 주세요.

### GET 요청 시 body 대신 query 사용하기

GET 요청 시에 body(content)를 전달하는 것은 HTTP 표준에 부합하지 않아, 클라이언트에 따라 사용 불가능한 경우가 있습니다.
이 경우, body 문자열을 `requestBody` query 필드에 넣어 주시면 됩니다. query 필드에 들어가는 문자열은 URL 인코드하셔야 합니다.

### 멱등 키

`Idempotency-Key` 헤더는 네트워크 장애나 타임아웃 시 동일한 요청이 중복 처리되는 것을 방지합니다. 자세한 표준은 [IETF 문서](https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/)를 참고하세요.

#### 사용 방법

요청 헤더에 고유한 키를 포함하세요. UUID 등 고유한 문자열이어야 합니다. 16~256자의 ASCII 문자열(권장: 영문 대소문자, 숫자, -, _) 을 사용 가능합니다. RFC 8941에 따라 문자열을 쌍따옴표로 감싸세요.

- `Idempotency-Key: "unique-request-id-123"`
요청 타임아웃이 발생한 경우, 동일한 멱등 키를 사용하여 재시도할 수 있습니다. 이 경우, 서버는 요청을 중복 처리하지 않고, 기존 응답을 반환합니다.
재시도에서 `IDEMPOTENCY_OUTSTANDING_REQUEST` 에러가 발생한 경우, 시간이 조금 지난 후 다시 재시도해 주세요.

- 지원 메서드: POST, PUT, PATCH, DELETE (GET은 무시됨)
- 처리 중인 요청을 재시도하는 경우: 409 에러 (`{"type":"IDEMPOTENCY_OUTSTANDING_REQUEST"}...}`)
- 완료된 요청을 재시도하는 경우: 기존 응답을 그대로 반환
- 멱등성 보장 기간: 3시간 (추후 변경 가능)
- 서로 다른 요청을 같은 멱등 키로 요청해서는 안 됩니다.

### 요청 타임아웃 정책

API 호출시 PG 및 결제 원천사의 응답 지연을 고려하여 최소 60초의 읽기 타임아웃 시간을 설정하도록 권장합니다.
서버에서 요청 처리 중에 클라이언트 측에서 연결을 끊은 경우에도 요청이 취소되지 않습니다. 이 경우 같은 멱등 키로 같은 요청을 재시도하면 결과를 확인할 수 있습니다.

### 하위호환성

포트원이 제공하는 모든 Stable API에 대해 아래와 같은 하위호환성이 보장됩니다.

-
현재 사용 가능한 입력 형식은 앞으로도 사용할 수 있습니다.

-
입력 형식 내 필드 정의가 삭제되지 않습니다.

-
필수 입력 정보가 추가되거나, 선택 입력 정보가 필수로 변경되지 않습니다.

- 오로지 선택 입력 정보만 추가될 수 있습니다.

-
하위 필드의 형식(타입) 또한 위 규칙을 지키며 변경됩니다.

-
enum 타입의 값이 삭제되지 않습니다.

-
출력 형식이 확장될 수 있지만, 축소되지 않습니다.

-
출력 형식 내 필드 정의가 삭제되지 않습니다.

-
사용 중인 필수 출력 정보가 선택사항으로 변경되거나 출력 시 누락되지 않습니다.

- 이미 존재하는 용례 내에서는 필수 출력 정보가 언제나 유지됩니다.

- 단, 기능이 추가 및 확장되는 등 새로운 용례로 사용될 때의 출력 정보에 한하여 선택사항으로 변경될 수 있습니다.

-
하위 필드의 형식(타입) 또한 위 규칙을 지키며 변경됩니다.

-
단, 새로운 필드 또는 enum 값, oneOf 케이스가 추가될 수 있습니다.

- 알지 못하는 필드 및 값이 주어지더라도 crash가 발생하지 않도록 유의하여 개발해주세요.

`UNSTABLE`이 표기된 일부 API의 경우, 위 하위호환성 정책과 무관하게 변경 및 지원 종료될 수 있으니 이용에 유의하세요.

## API 카테고리

[결제 관련 API](https://developers.portone.io/api/rest-v2/payment)결제와 관련된 API 기능을 제공합니다.

18개의 엔드포인트

[결제 예약 관련 API](https://developers.portone.io/api/rest-v2/payment.paymentSchedule)결제 예약과 관련된 API 기능을 제공합니다.

4개의 엔드포인트

[빌링키 관련 API](https://developers.portone.io/api/rest-v2/payment.billingKey)빌링키와 관련된 API 기능을 제공합니다.

6개의 엔드포인트

[현금 영수증 관련 API](https://developers.portone.io/api/rest-v2/payment.cashReceipt)현금 영수증과 관련된 API 기능을 제공합니다.

4개의 엔드포인트

[프로모션 관련 API](https://developers.portone.io/api/rest-v2/payment.promotion)프로모션과 관련된 API 기능을 제공합니다.

1개의 엔드포인트

[결제 부가기능 관련 API](https://developers.portone.io/api/rest-v2/payment.additionalFeature)결제 부가기능과 관련된 API 기능을 제공합니다.

1개의 엔드포인트

[상품 API](https://developers.portone.io/api/rest-v2/subscription.product)구독 결제에 사용할 상품에 관한 API 입니다.

7개의 엔드포인트

[할인 API](https://developers.portone.io/api/rest-v2/subscription.discount)구독에 붙여 인보이스 금액을 깎는 할인에 관한 API 입니다.

6개의 엔드포인트

[고객 API](https://developers.portone.io/api/rest-v2/subscription.customer)구독 결제의 대상이 되는 고객과 그 결제수단에 관한 API 입니다.

7개의 엔드포인트

[구독 API](https://developers.portone.io/api/rest-v2/subscription.subscription)고객이 상품을 정기적으로 청구받는 구독에 관한 API 입니다.

14개의 엔드포인트

[인보이스 API](https://developers.portone.io/api/rest-v2/subscription.invoice)구독의 회차별 청구 건인 인보이스에 관한 API 입니다.

8개의 엔드포인트

[인보이스 메일 템플릿 API](https://developers.portone.io/api/rest-v2/subscription.invoiceEmailTemplate)고객에게 보내는 인보이스 메일의 발신자 정보와 로고, 보낼 메일 유형에 관한 API 입니다.

7개의 엔드포인트

[본인인증 관련 API](https://developers.portone.io/api/rest-v2/identityVerification)본인인증과 관련된 API 기능을 제공합니다.

5개의 엔드포인트

[특정 PG사 관련 API](https://developers.portone.io/api/rest-v2/pgSpecific)특정 PG사에 국한된 API 기능을 제공합니다.

2개의 엔드포인트

[대사 서비스 API](https://developers.portone.io/api/rest-v2/reconciliation)거래 대사 및 정산 대사 관련 API 기능을 제공합니다.

2개의 엔드포인트

[거래처 관련 API](https://developers.portone.io/api/rest-v2/b2b.counterparty)거래처 관련 API 기능을 제공합니다.

8개의 엔드포인트

[세금계산서 발행 관련 API](https://developers.portone.io/api/rest-v2/b2b.taxInvoice)세금계산서 발행 관련 API 기능을 제공합니다.

22개의 엔드포인트

[정책 관련 API](https://developers.portone.io/api/rest-v2/platform.policy)파트너 정산에 적용할 정책에 관한 API 입니다.

18개의 엔드포인트

[파트너 관련 API](https://developers.portone.io/api/rest-v2/platform.partner)파트너 정산에 적용할 파트너에 관한 API 입니다.

11개의 엔드포인트

[정산 상세내역 관련 API](https://developers.portone.io/api/rest-v2/platform.transfer)파트너 정산 서비스의 정산 상세내역과 관련된 API 입니다.

7개의 엔드포인트

[계좌 관련 API](https://developers.portone.io/api/rest-v2/platform.account)파트너 정산 서비스의 계좌와 관련된 API 입니다.

1개의 엔드포인트

[정산 내역 관련 API](https://developers.portone.io/api/rest-v2/platform.partnerSettlement)파트너 정산 서비스의 정산 내역과 관련된 API 입니다.

2개의 엔드포인트

[지급 내역 관련 API](https://developers.portone.io/api/rest-v2/platform.payout)파트너 정산 서비스의 지급 내역과 관련된 API 입니다.

2개의 엔드포인트

[일괄 지급 내역 관련 API](https://developers.portone.io/api/rest-v2/platform.bulkPayout)파트너 정산 서비스의 일괄 지급 내역과 관련된 API 입니다.

1개의 엔드포인트

[이체 내역 관련 API](https://developers.portone.io/api/rest-v2/platform.accountTransfer)파트너 정산 서비스의 이체 내역과 관련된 API 입니다.

1개의 엔드포인트

[일괄 이체 내역 관련 API](https://developers.portone.io/api/rest-v2/platform.bulkAccountTransfer)파트너 정산 서비스의 일괄 이체 내역과 관련된 API 입니다.

1개의 엔드포인트

[사업자 관련 API](https://developers.portone.io/api/rest-v2/platform.company)파트너 정산 서비스의 사업자와 관련된 API 입니다.

2개의 엔드포인트

[인증 관련 API](https://developers.portone.io/api/rest-v2/auth)인증과 관련된 API 기능을 제공합니다.
접근 토큰 방식으로 인증하기를 원하는 경우, API 시크릿을 통해 토큰을 발급받은 후 Authorization 헤더에 `Bearer ACCESS_TOKEN` 형식으로 전달합니다.

2개의 엔드포인트

[결제 세션 API](https://developers.portone.io/api/rest-v2/paymentSession)결제 세션 생성 및 관리 API. 호스티드 체크아웃에 사용됩니다.

3개의 엔드포인트

[체크아웃 프로필 API](https://developers.portone.io/api/rest-v2/checkoutProfile)체크아웃 프로필에서 결제 수단 목록을 조회하기 위한 API.

1개의 엔드포인트

[공통 API](https://developers.portone.io/api/rest-v2/common)공통 API 기능을 제공합니다.

1개의 엔드포인트
