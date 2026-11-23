# 결제

결제 조회·승인·확정·가상계좌/에스크로·웹훅 재발송, 결제 세션, 체크아웃 프로필, PG사별 API, 인증 결제 연동 가이드. 취소 API 는 payment.md 에 있고 cancel-refund 에서 링크한다.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [payment.md](payment.md) | 결제 관련 API | 18 |
| [paymentSession.md](paymentSession.md) | 결제 세션 API | 3 |
| [checkoutProfile.md](checkoutProfile.md) | 체크아웃 프로필 API | 1 |
| [additionalFeature.md](additionalFeature.md) | 결제 부가기능 관련 API | 1 |
| [pgSpecific.md](pgSpecific.md) | 특정 PG사 관련 API | 2 |
| [schemas.md](schemas.md) | 이 폴더의 문서들이 공유하는 스키마 | |

## 가이드

| 파일 | 제목 | 원본 |
|---|---|---|
| [checkout-integration.md](checkout-integration.md) | 인증 결제 연동하기 | https://developers.portone.io/opi/ko/integration/start/v2/checkout.md |

## 주요 엔드포인트

- [`GET /payments/{paymentId}`](payment.md#get-paymentspaymentid) — 결제 단건 조회
- [`GET /payments`](payment.md#get-payments) — 결제 다건 조회(페이지 기반)
- [`POST /payments/{paymentId}/instant`](payment.md#post-paymentspaymentidinstant) — 수기 결제
- [`POST /payments/{paymentId}/confirm`](payment.md#post-paymentspaymentidconfirm) — 인증 결제 수동 승인
- [`POST /payments/{paymentId}/capture`](payment.md#post-paymentspaymentidcapture) — 수동 매입
- [`POST /payments/{paymentId}/pre-register`](payment.md#post-paymentspaymentidpre-register) — 결제 정보 사전 등록
- [`POST /payments/{paymentId}/resend-webhook`](payment.md#post-paymentspaymentidresend-webhook) — 웹훅 재발송
