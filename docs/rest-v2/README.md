# PortOne REST API V2 문서 (오프라인 사본)

- 원본: https://developers.portone.io/api/rest-v2?v=v2
- 수집일: 2026-11-22
- 엔드포인트 수: 196, OpenAPI 버전 소스: https://developers.portone.io/schema/v2.openapi.json
- 이 폴더는 `scripts/fetch-rest-v2-docs.mjs` 가 생성한다. 손으로 고치지 말고 스크립트를 다시 실행한다.

## 다시 만들기

```sh
# 저장소 루트에서. Node 18 이상, 별도 의존성 없음
FETCH_DATE=YYYY-MM-DD node scripts/fetch-rest-v2-docs.mjs
```

## 수집 방식

원본 페이지는 JS 렌더링(SolidStart)이라 HTML 은 엔드포인트별 본문을 그대로 담지 않는다. 같은 사이트의 OpenAPI JSON 이 렌더링 원본이므로 엔드포인트·파라미터·스키마·에러 코드는 OpenAPI 에서, 개요는 개요 페이지 HTML 에서, 가이드는 사이트가 제공하는 `.md` 에서 가져온다. 링크는 절대 URL 로 바꿨다.

## 파일

| 파일 | 내용 |
|---|---|
| [overview.md](overview.md) | 호스트, 인증 헤더, 멱등 키, 타임아웃, 하위호환성 |
| [errors.md](errors.md) | 전체 에러 코드(이름, HTTP 상태, 설명) |
| [schemas-common.md](schemas-common.md) | 여러 문서가 공유하는 스키마 |

| 카테고리 문서 | 제목 | 엔드포인트 |
|---|---|---|
| [b2b.taxInvoice.md](b2b.taxInvoice.md) | 세금계산서 발행 관련 API | 22 |
| [platform.md](platform.md) | 파트너 정산 관련 API | 67 |
| [b2b.counterparty.md](b2b.counterparty.md) | 거래처 관련 API | 8 |
| [common.md](common.md) | 공통 API | 1 |
| [payment.billingKey.md](payment.billingKey.md) | 빌링키 관련 API | 6 |
| [payment.cashReceipt.md](payment.cashReceipt.md) | 현금 영수증 관련 API | 4 |
| [checkoutProfile.md](checkoutProfile.md) | 체크아웃 프로필 API | 1 |
| [subscription.md](subscription.md) | 구독 API | 49 |
| [identityVerification.md](identityVerification.md) | 본인인증 관련 API | 5 |
| [pgSpecific.md](pgSpecific.md) | 특정 PG사 관련 API | 2 |
| [auth.md](auth.md) | 인증 관련 API | 2 |
| [payment.md](payment.md) | 결제 관련 API | 18 |
| [payment.additionalFeature.md](payment.additionalFeature.md) | 결제 부가기능 관련 API | 1 |
| [reconciliation.md](reconciliation.md) | 대사 서비스 API | 2 |
| [payment.paymentSchedule.md](payment.paymentSchedule.md) | 결제 예약 관련 API | 4 |
| [paymentSession.md](paymentSession.md) | 결제 세션 API | 3 |
| [payment.promotion.md](payment.promotion.md) | 프로모션 관련 API | 1 |

| 가이드 | 제목 | 원본 |
|---|---|---|
| [guides/webhook.md](guides/webhook.md) | 웹훅 연동하기 | https://developers.portone.io/opi/ko/integration/webhook/readme-v2.md |
| [guides/cancel.md](guides/cancel.md) | 결제취소(환불) 연동하기 | https://developers.portone.io/opi/ko/integration/cancel/v2/readme.md |
| [guides/promotion-refund.md](guides/promotion-refund.md) | 프로모션 결제 취소하기 | https://developers.portone.io/opi/ko/extra/promotion/refund.md |
| [guides/checkout-integration.md](guides/checkout-integration.md) | 인증 결제 연동하기 | https://developers.portone.io/opi/ko/integration/start/v2/checkout.md |
| [guides/subscription-webhook.md](guides/subscription-webhook.md) | 구독 결제 웹훅 | https://developers.portone.io/opi/ko/extra/subscription/webhook.md |
| [guides/platform-webhook-tax-invoice.md](guides/platform-webhook-tax-invoice.md) | 세금계산서 웹훅 가이드 | https://developers.portone.io/platform/ko/guides/webhook.md |
| [guides/server-sdk.md](guides/server-sdk.md) | 서버 SDK 레퍼런스 | https://developers.portone.io/sdk/ko/v2-server-sdk/readme.md |
| [guides/backward-compatibility.md](guides/backward-compatibility.md) | API 하위호환성 지원 | https://developers.portone.io/api/backward-compatibility.md |
