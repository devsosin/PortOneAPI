# 취소·환불

결제 취소(환불)와 프로모션 결제 취소 가이드. 취소 엔드포인트 자체는 payment/payment.md 에 있어 앵커로 연결한다.

## 가이드

| 파일 | 제목 | 원본 |
|---|---|---|
| [cancel.md](cancel.md) | 결제취소(환불) 연동하기 | https://developers.portone.io/opi/ko/integration/cancel/v2/readme.md |
| [promotion-refund.md](promotion-refund.md) | 프로모션 결제 취소하기 | https://developers.portone.io/opi/ko/extra/promotion/refund.md |

## 주요 엔드포인트

- [`POST /payments/{paymentId}/cancel`](../payment/payment.md#post-paymentspaymentidcancel) — 결제 취소
- [`POST /payments/{paymentId}/cancellations/{cancellationId}/stop`](../payment/payment.md#post-paymentspaymentidcancellationscancellationidstop) — 결제 취소 요청 취소
