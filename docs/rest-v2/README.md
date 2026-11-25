# PortOne REST API V2 문서 (오프라인 사본)

## 먼저 읽을 것: 공통 정책

API 를 추가하거나 바꾸기 전에 [policy.md](policy.md) 를 먼저 읽는다(손으로 관리하는 파일이며 재생성 때 보존된다). 요약:

- 인코딩: 경로·query 값은 `encodeURIComponent` 규칙으로 이스케이프한다.
- 인증: `Authorization: PortOne <API Secret>` 헤더(인증 API 제외).
- GET 의 본문: body 대신 JSON 을 URL 인코딩해 `requestBody` query 로 보낸다.
- 멱등 키: 상태를 바꾸는 요청(POST·PUT·PATCH·DELETE)은 `Idempotency-Key` 헤더를 쓴다.
- 읽기 타임아웃: 최소 60초.

- 원본: https://developers.portone.io/api/rest-v2?v=v2
- 수집일: 2026-11-22
- 엔드포인트 수: 196, OpenAPI 소스: https://developers.portone.io/schema/v2.openapi.json
- 이 폴더는 `scripts/fetch-rest-v2-docs.mjs` 가 생성한다. 손으로 고치지 말고 스크립트를 다시 실행한다(카테고리 배치는 스크립트의 `UNIT_FILE`·`GUIDES`·`CATS`).

## 다시 만들기

```sh
# 저장소 루트에서. Node 18 이상, 별도 의존성 없음
FETCH_DATE=YYYY-MM-DD node scripts/fetch-rest-v2-docs.mjs
node scripts/check-rest-v2-links.mjs   # 상대 링크·앵커 검사
```

## 수집 방식

원본 페이지는 JS 렌더링(SolidStart)이라 HTML 은 엔드포인트별 본문을 그대로 담지 않는다. 같은 사이트의 OpenAPI JSON 이 렌더링 원본이므로 엔드포인트·파라미터·스키마·에러 코드는 OpenAPI 에서, 개요는 개요 페이지 HTML 에서, 가이드는 사이트가 제공하는 `.md` 에서 가져온다. 가이드의 링크는 절대 URL 로 바꿨다.

## 카테고리

한 주제의 API 레퍼런스와 관련 가이드를 한 폴더에 모았다. 각 폴더의 README.md 에 파일 목록과 주요 엔드포인트가 있다.

| 폴더 | 카테고리 | 내용 | 엔드포인트 |
|---|---|---|---|
| [00-basics/](00-basics/README.md) | 기본 | API 호출 전에 알아야 할 것: 호스트·인증 헤더·멱등 키·타임아웃(개요), 인증 토큰 API, 전체 에러 코드, 여러 문서가 공유하는 스키마, 하위호환성, 서버 SDK | 3 |
| [payment/](payment/README.md) | 결제 | 결제 조회·승인·확정·가상계좌/에스크로·웹훅 재발송, 결제 세션, 체크아웃 프로필, PG사별 API, 인증 결제 연동 가이드 | 25 |
| [cancel-refund/](cancel-refund/README.md) | 취소·환불 | 결제 취소(환불)와 프로모션 결제 취소 가이드 |  |
| [webhook/](webhook/README.md) | 웹훅 | 결제·구독·정산 웹훅 수신과 검증 가이드 |  |
| [billing-subscription/](billing-subscription/README.md) | 빌링키·결제 예약·구독 | 정기결제: 빌링키 발급·조회·삭제, 빌링키 결제, 결제 예약, 구독 상품·고객·할인·인보이스 | 59 |
| [cash-receipt-tax-invoice/](cash-receipt-tax-invoice/README.md) | 현금영수증·세금계산서 | 현금영수증 발급·조회·취소, B2B 세금계산서 발행과 거래처 관리 | 34 |
| [promotion/](promotion/README.md) | 프로모션 | 프로모션 조회 API | 1 |
| [identity-verification/](identity-verification/README.md) | 본인인증 | 본인인증 요청·확인·조회·SMS 재전송 | 5 |
| [platform/](platform/README.md) | 파트너 정산(플랫폼) | 파트너 정산 API(약 67개) | 67 |
| [reconciliation/](reconciliation/README.md) | 대사 | 대사(정산 대조) 서비스 API | 2 |

에러 코드는 [00-basics/errors.md](00-basics/errors.md), 공통 스키마는 [00-basics/schemas-common.md](00-basics/schemas-common.md).
