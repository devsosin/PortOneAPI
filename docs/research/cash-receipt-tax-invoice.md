# 현금영수증·세금계산서 조사 기록 (PORT-003)

## 출처

- PortOne REST V2 문서 스냅샷: `docs/rest-v2/cash-receipt-tax-invoice/cashReceipt.md`, `taxInvoice.md`, `counterparty.md`, `docs/rest-v2/payment/payment.md`, `docs/rest-v2/webhook/platform-webhook-tax-invoice.md`
- 세무 검토: 이슈 #3 소유자 코멘트(아래 "세무 검토 결과")

- 공통 정책: [`docs/rest-v2/policy.md`](../rest-v2/policy.md) (API 작업 전 필독)

## 문서로 확인한 엔드포인트

| 엔드포인트 | 용도 | 크레이트 |
|---|---|---|
| `POST /cash-receipts` | 수동 발급 (필수: `paymentId`, `channelKey`, `type`, `orderName`, `currency`, `amount`, `customer`) | `issue_cash_receipt` |
| `POST /payments/{paymentId}/cash-receipt/cancel` | 취소 (`storeId` 쿼리 선택) | `cancel_cash_receipt` |
| `GET /payments/{paymentId}/cash-receipt` | 결제 건별 조회 | `get_cash_receipt` |
| `GET /cash-receipts` | 목록 (`page`/`sort`/`filter`) | `get_cash_receipts` |
| `/b2b/tax-invoices`, `/b2b/counterparties` 계열 | B2B 세금계산서·거래처 | 아래 "B2B 세금계산서·거래처 (PORT-005)" 참고 |

매출전표 URL은 결제 응답 `receiptUrl`로 제공된다(`PaymentDetailData::get_receipt_url`).

## 세무 검토 결과 (소유자 확인)

- 현금영수증 대상 결제수단: 실시간 계좌이체(`trans`), 가상계좌(`vbank`, 입금 완료 시점 기준), 간편결제 내 현금성 수단(머니·계좌·포인트 등 충전금·계좌 결제분). 간편결제 카드 결제분은 아니다.
- 현금영수증 의무발행업종은 건당 10만 원 이상(부가세 포함) 거래 시 현금영수증을 반드시 발급해야 한다.
- PG 결제창 자동 발급: 가상계좌·계좌이체 호출 시 결제창에서 구매자가 유형(소득공제/지출증빙)과 식별번호를 입력하면 자동 발급된다. PG 상점 관리자에서 '의무발행 가맹점(미입력 시 자진발급)' 옵션을 켜야 자동 처리된다.
- 중복 주의: 법인·개인사업자 고객에게 해당 거래로 이미 세금계산서를 발행했다면 현금영수증을 중복 발행하지 않도록 예외 처리해야 한다.

## 크레이트 사용 지침

- PG 자동 발급이 켜져 있으면 `issue_cash_receipt`(수동 발급)를 호출하기 전에 `get_cash_receipt` 로 이미 발급됐는지 확인한다. 이미 발급된 경우 서버가 409 `CashReceiptAlreadyIssuedError` 로 거절하며, 크레이트는 이를 `Error::Receipt(ReceiptError::CashReceiptAlreadyIssuedError)` 로 돌려준다.
- 중복 발급 방지는 호출 측 책임이다(소유자 결정, #13). 크레이트는 중복 확인 헬퍼를 제공하지 않는다. 호출 전에 `get_cash_receipt`(`GET /payments/{id}/cash-receipt`)나 자체 기록으로 확인하고, 세금계산서를 발행했다면 건너뛴다.
- 세금계산서를 이미 발행한 거래는 호출 측에서 현금영수증 발급을 건너뛰어야 한다. 크레이트는 이를 판단하지 못한다.
- 수동 발급 대상은 위 대상 결제수단이며, 10만 원 이상 의무 여부 판단은 호출 측 책임이다.

## 크레이트 지원 범위

- 발급·단건 조회·취소·목록 조회, 유형(`CashReceiptType`)·식별값 유형(`IdentityNumberType`)·상태·정렬 enum, `identityNumberType` 입력.
- 목록 조회: `GetCashReceiptsBody`(page/sort/filter)를 JSON 으로 직렬화해 URL 인코딩한 `requestBody` 쿼리로 보낸다(공식 정책 "GET 요청 시 body 대신 query", 소유자 확인한 스키마와 일치). 필터에 `pgProviders`/`pgCompanies`/`version` 포함, 모든 필드 선택.
- 세금계산서 발급 계열은 아래 PORT-005 절.
- 공식 정책 점검 결과는 `docs/rest-v2/policy.md` 맨 아래 표 참고. 이 PR 에서 경로 인코딩(`get_payment_detail`)과 읽기 타임아웃 60초를 고쳤다.
- 테스트는 로컬 JSON 직렬화·역직렬화 단위 테스트뿐이며 실제 PortOne 호출은 없다.

## 구현하지 않은 것·참고

- 수동 발급 시 `paymentMethod`(`TRANSFER`/`VIRTUAL_ACCOUNT`)와 `businessRegistrationNumber` 는 일부 PG(문서 기준 웰컴페이먼츠)에서만 입력한다.
- 세금계산서·거래처는 아래 PORT-005 절.

## 후속 이슈

| 이슈 | 내용 |
|---|---|
| #12 | B2B 세금계산서·거래처 API와 웹훅 추가 → 구현(PORT-005, 아래 절), 남은 작업은 아래 표 |
| #13 | 현금영수증 수동 발급 전 중복 발급 방지 (병합됨) |
| #14 | 현금영수증 응답 `channel` 타입화 → `SelectedChannel`, 알 수 없는 `pgProvider` 는 `Unknown`(성공 취급 안 함) |
| #15 | 현금영수증 발급·취소에 `Idempotency-Key` 지원 → `idempotency_key: Option<&str>` 인자, 409 outstanding 은 재시도 없이 전달 |

(`GET /cash-receipts` 실연동 확인 이슈는 요청 스키마가 소유자 확인으로 확정돼 만들지 않았다.)

## B2B 세금계산서·거래처 (PORT-005)

### 출처

- `docs/rest-v2/cash-receipt-tax-invoice/taxInvoice.md`, `counterparty.md`
- 웹훅: https://developers.portone.io/platform/ko/guides/webhook (오프라인 사본 `docs/rest-v2/webhook/platform-webhook-tax-invoice.md`), V2 웹훅 서명: https://developers.portone.io/opi/ko/integration/webhook/readme-v2?v=v2

### 구현 범위 (소유자 결정: 즉시 발행·취소·조회부터)

| 엔드포인트 | 크레이트 |
|---|---|
| `POST /b2b/tax-invoices/issue-immediately` | `issue_tax_invoice_immediately(body, test, idempotency_key)` |
| `POST /b2b/tax-invoices/{key}/cancel-issuance` | `cancel_tax_invoice_issuance(key, lookup, body, idempotency_key)` |
| `GET /b2b/tax-invoices/{key}` | `get_tax_invoice` (타입 `TaxInvoice`) |
| `GET /b2b/tax-invoices` | `get_tax_invoices` (`requestBody` 쿼리) |
| `GET …/{key}/pdf-download-url`, `print-url`, `popup-url` | `get_tax_invoice_pdf_url`, `_print_url`, `_popup_url` |
| `POST/GET/PATCH/DELETE /b2b/counterparties` 계열 | `create_counterparty`, `get_counterparty`, `get_counterparties`, `update_counterparty`, `delete_counterparty` |

구조: `features/b2b/`(공통: `B2bError`, `test` 쿼리, 페이지 타입), `features/tax_invoice/`, `features/counterparty/` (각각 `mod.rs` + `dto/`).

### 사용 시 주의

- `B2bTaxInvoiceInput.issuanceType` 서버 기본값은 **역발행(`REVERSE`)** 이다. 즉시 정발행에는 `Some(IssuanceType::Normal)` 을 넣는다(어기면 `B2bIssuanceTypeMismatchError`).
- 응답 enum(상태 등)의 알 수 없는 값은 `Unknown` 이며 성공으로 취급하지 않는다. 발행 성공 여부는 `status` 가 알려진 발행 이후 상태인지로 판단한다.
- 문서에 나온 `B2b*Error` 타입은 모두 `B2bError` 변종이다(예: 취소 시 `B2bTaxInvoiceNotIssuedStatusError`). 문서에 없는 `type` 은 `PortOneError::Unclassified`.
- 멱등 키: 발행·취소·거래처 생성·수정·삭제는 `idempotency_key` 를 받는다. 발행은 실제 세금계산서를 만들므로 재시도 때 같은 키를 쓴다.
- 국세청 전송 전(`ISSUED`)에만 취소할 수 있다(`cancel-issuance` 문서).
- 세금계산서를 발행한 거래는 현금영수증을 중복 발급하지 않는다(호출 측 책임, 위 "크레이트 사용 지침").

### 웹훅

- 위치: `features::webhook`(`webhook::TaxInvoiceEvent`, `TaxInvoiceEventType`, `TaxInvoiceEventData`). 서명 검증은 기존 `WebhookVerifier` 를 그대로 쓴다(코드 중복 없음).
- 주 경로: `WebhookVerifier::verify_and_parse_tax_invoice(body, headers)` — 서명 확인 뒤에만 본문을 파싱한다. `TaxInvoiceEvent::parse(body)` 는 이미 검증한 본문용 하위 수준 도우미다.
- **서명**: 세금계산서 웹훅은 V2 웹훅과 같은 검증을 쓴다(Standard Webhooks 헤더, 콘솔에서 발급한 웹훅 시크릿). 소유자가 포트원에 확인했다(2026-11-25). 별도 서명 방식·시크릿은 없다. 근거: https://developers.portone.io/opi/ko/integration/webhook/readme-v2?v=v2. 고정 IP `52.78.5.241` 허용은 선택적 추가 방어다.
- 본문: `{type, timestamp(RFC 3339), data}`.
- `type`: `TaxInvoice.Requested`(역발행 요청 완료), `Issued`(공급자 발행 승인), `RequestCancelled`(공급받는자 요청 취소), `IssuanceCancelled`(공급자 취소), `Refused`(발행 거부), `SendingCompleted`(국세청 전송 완료), `Manual`(수동 재발송, 수동 발송 API 는 추후 지원 예정). 모르는 값은 `Unknown(raw)`.
- `data`: `taxInvoiceId`, `supplierDocumentKey`, `recipientDocumentKey`, `status`, `supplierBrn`, `totalAmount`, `totalSupplyAmount`, `totalTaxAmount`(모두 필수). `status` 는 `TaxInvoiceStatus`(웹훅은 주요 상태 변경에만 발송).
- 설정: 포트원 기술지원(`tech.support@portone.io`)에 웹훅 URL(운영·테스트 별도)을 전달. 테스트 모드는 콘솔 웹훅 테스트 토글을 켜고 테스트 세금계산서를 발행하면 테스트 URL로 전송된다.
- 재시도: 실패 응답이면 최초 + 4회, 총 5회. 지수 백오프(기본 5·10·20·40분) + Equal Jitter. 수신 측은 `webhook-id` 또는 `taxInvoiceId`+`type` 으로 중복을 걸러야 한다.
- 발송 IP: `52.78.5.241` (변경 시 메일 안내).

### 남은 작업 (이번에 구현하지 않음)

| 범위 | 엔드포인트 |
|---|---|
| 임시저장 | `POST/PUT /b2b/tax-invoices/draft` |
| 역발행 | `POST /b2b/tax-invoices/request-reverse-issuance`, `…/{key}/request`, `…/{key}/cancel-request`, `…/{key}/refuse-request` |
| 발행 승인·전송 | `POST …/{key}/issue`, `…/{key}/send-to-nts` |
| 삭제 | `DELETE /b2b/tax-invoices/{key}` |
| 첨부 | `POST …/{key}/attach-file`, `GET/DELETE …/{key}/attachments[/{id}]`, `POST /b2b/file-upload-url` |
| 일괄·시트 | `GET /b2b/bulk-tax-invoices/{id}`, `GET /b2b/tax-invoices-sheet` |
| 거래처 인증서 | `GET …/{brn}/certificate/registration-url`, `POST …/{brn}/certificate/validate`, `GET …/{brn}/certificate` |
| 웹훅 | 수동 재발송 API(문서상 추후 지원 예정) |

### 테스트

로컬 단위 테스트와 모의 서버 테스트(`tests/tax_invoice.rs`, `tests/counterparty.rs`)만 있고 실제 PortOne 호출은 없다.
