# 빌링키·결제 예약·구독

정기결제: 빌링키 발급·조회·삭제, 빌링키 결제, 결제 예약, 구독 상품·고객·할인·인보이스.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [billingKey.md](billingKey.md) | 빌링키 관련 API | 6 |
| [paymentSchedule.md](paymentSchedule.md) | 결제 예약 관련 API | 4 |
| [subscription.md](subscription.md) | 구독 API | 49 |

## 주요 엔드포인트

- [`POST /billing-keys`](billingKey.md#post-billing-keys) — 빌링키 발급
- [`GET /billing-keys/{billingKey}`](billingKey.md#get-billing-keysbillingkey) — 빌링키 단건 조회
- [`DELETE /billing-keys/{billingKey}`](billingKey.md#delete-billing-keysbillingkey) — 빌링키 삭제
- [`POST /payments/{paymentId}/billing-key`](../payment/payment.md#post-paymentspaymentidbilling-key) — 빌링키 결제
- [`POST /payments/{paymentId}/schedule`](paymentSchedule.md#post-paymentspaymentidschedule) — 결제 예약
- [`GET /payment-schedules`](paymentSchedule.md#get-payment-schedules) — 결제 예약 다건 조회
