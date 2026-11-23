# 웹훅

결제·구독·정산 웹훅 수신과 검증 가이드. 웹훅 재발송 API 는 payment 의 resend-webhook.

## 가이드

| 파일 | 제목 | 원본 |
|---|---|---|
| [webhook.md](webhook.md) | 웹훅 연동하기 | https://developers.portone.io/opi/ko/integration/webhook/readme-v2.md |
| [subscription-webhook.md](subscription-webhook.md) | 구독 결제 웹훅 | https://developers.portone.io/opi/ko/extra/subscription/webhook.md |
| [platform-webhook-tax-invoice.md](platform-webhook-tax-invoice.md) | 세금계산서 웹훅 가이드 | https://developers.portone.io/platform/ko/guides/webhook.md |

## 주요 엔드포인트

- [`POST /payments/{paymentId}/resend-webhook`](../payment/payment.md#post-paymentspaymentidresend-webhook) — 웹훅 재발송
