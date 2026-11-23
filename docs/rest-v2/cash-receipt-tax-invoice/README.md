# 현금영수증·세금계산서

현금영수증 발급·조회·취소, B2B 세금계산서 발행과 거래처 관리.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [cashReceipt.md](cashReceipt.md) | 현금 영수증 관련 API | 4 |
| [taxInvoice.md](taxInvoice.md) | 세금계산서 발행 관련 API | 22 |
| [counterparty.md](counterparty.md) | 거래처 관련 API | 8 |

## 주요 엔드포인트

- [`POST /cash-receipts`](cashReceipt.md#post-cash-receipts) — 현금 영수증 수동 발급
- [`GET /payments/{paymentId}/cash-receipt`](cashReceipt.md#get-paymentspaymentidcash-receipt) — 현금 영수증 단건 조회
- [`POST /payments/{paymentId}/cash-receipt/cancel`](cashReceipt.md#post-paymentspaymentidcash-receiptcancel) — 현금 영수증 취소
- [`GET /cash-receipts`](cashReceipt.md#get-cash-receipts) — 현금영수증 다건 조회
