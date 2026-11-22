# 현금영수증·세금계산서 API 조사 노트

출처: PortOne REST V2 문서 (https://developers.portone.io/api/rest-v2?v=v2, OpenAPI `https://developers.portone.io/schema/v2.openapi.json`). 문서 스냅샷은 별도 PR(`docs/rest-v2/`)로 관리된다.

## 현금영수증 (`payment.cashReceipt`)

| 엔드포인트 | 크레이트 | 비고 |
|---|---|---|
| `POST /cash-receipts` | `PortOneCashReceiptService::issue_cash_receipt` | 수동 발급. `paymentId`·`channelKey`·`type`·`orderName`·`currency`·`amount`·`customer` 필수 |
| `GET /payments/{paymentId}/cash-receipt` | `get_cash_receipt` | `status` 로 `ISSUED`/`CANCELLED`/`ISSUE_FAILED` 구분 |
| `POST /payments/{paymentId}/cash-receipt/cancel` | `cancel_cash_receipt` | `storeId` 쿼리 선택 |
| `GET /cash-receipts` | 미구현 | 문서상 필터를 요청 본문(GET)으로 받는 형태라 동작이 불명확함. 필요해지면 별도 이슈 |

- 수동 발급 시 `paymentMethod`(`TRANSFER`/`VIRTUAL_ACCOUNT`)와 `businessRegistrationNumber` 는 일부 PG(문서 기준 웰컴페이먼츠)에서만 입력한다.
- 결제 취소 응답에 현금영수증 취소 정보가 함께 오는지는 결제 취소 API 쪽 문서(`payment.md`)를 따른다.

## 세금계산서 (`b2b.taxInvoice`)

`/b2b/tax-invoices` 계열은 사업자 간(B2B) 세금계산서 발행·역발행·국세청 전송 API다. 이 크레이트는 **읽기 전용** 조회만 제공한다.

| 엔드포인트 | 크레이트 |
|---|---|
| `GET /b2b/tax-invoices/{taxInvoiceKey}` | `PortOneTaxInvoiceService::get_tax_invoice` (원본 JSON) |
| `GET /b2b/tax-invoices/{taxInvoiceKey}/pdf-download-url` | `get_tax_invoice_pdf_url` |
| `GET /b2b/tax-invoices/{taxInvoiceKey}/print-url` | `get_tax_invoice_print_url` |

## 구현하지 않은 것

- 세금계산서 발행(`issue-immediately`, `draft`, `issue`, `request`, `send-to-nts`, 취소·삭제 등): 요청 스키마가 크고 실제 세금계산서가 만들어지므로, 수집 항목과 발행 대상을 확정한 뒤 별도 이슈로 진행한다.
- 팝업 URL, 첨부파일, 일괄 발행, 시트 다운로드.
- 의무 발행 대상 결제수단과 법적 요건: 문서에 없으며 세무 확인이 필요하다.
- 매출전표 URL: 결제 응답의 `receiptUrl`(`payment.md`)로 제공된다. 기존 결제 조회 DTO에는 아직 해당 필드가 없다.

## 에러 처리

비 2xx 응답은 기존과 같이 상태 코드만 `PortOneError` 로 변환한다(400/401/403/404, 그 외 `UnknownError`). 409(`CashReceiptAlreadyIssuedError`)와 502(`PgProviderError`)는 `UnknownError` 로 나오며, 에러 본문 파싱은 후속 과제다.
