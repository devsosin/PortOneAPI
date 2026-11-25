# 현금영수증·세금계산서 조사 기록 (PORT-003)

## 출처

- PortOne REST V2 문서 스냅샷: `docs/rest-v2/cash-receipt-tax-invoice/cashReceipt.md`, `taxInvoice.md`, `counterparty.md`, `docs/rest-v2/payment/payment.md`, `docs/rest-v2/guides/platform-webhook-tax-invoice.md`
- 세무 검토: 이슈 #3 소유자 코멘트(아래 "세무 검토 결과")

- 공통 정책: [`docs/rest-v2/policy.md`](../rest-v2/policy.md) (API 작업 전 필독)

## 문서로 확인한 엔드포인트

| 엔드포인트 | 용도 | 크레이트 |
|---|---|---|
| `POST /cash-receipts` | 수동 발급 (필수: `paymentId`, `channelKey`, `type`, `orderName`, `currency`, `amount`, `customer`) | `issue_cash_receipt` |
| `POST /payments/{paymentId}/cash-receipt/cancel` | 취소 (`storeId` 쿼리 선택) | `cancel_cash_receipt` |
| `GET /payments/{paymentId}/cash-receipt` | 결제 건별 조회 | `get_cash_receipt` |
| `GET /cash-receipts` | 목록 (`page`/`sort`/`filter`) | `get_cash_receipts` |
| `/b2b/tax-invoices` 계열, `/b2b/counterparties` | B2B 세금계산서 | 조회만(`features::tax_invoice`), 발급 등은 미지원 |

매출전표 URL은 결제 응답 `receiptUrl`로 제공된다(`PaymentDetailData::get_receipt_url`).

## 세무 검토 결과 (소유자 확인)

- 현금영수증 대상 결제수단: 실시간 계좌이체(`trans`), 가상계좌(`vbank`, 입금 완료 시점 기준), 간편결제 내 현금성 수단(머니·계좌·포인트 등 충전금·계좌 결제분). 간편결제 카드 결제분은 아니다.
- 현금영수증 의무발행업종은 건당 10만 원 이상(부가세 포함) 거래 시 현금영수증을 반드시 발급해야 한다.
- PG 결제창 자동 발급: 가상계좌·계좌이체 호출 시 결제창에서 구매자가 유형(소득공제/지출증빙)과 식별번호를 입력하면 자동 발급된다. PG 상점 관리자에서 '의무발행 가맹점(미입력 시 자진발급)' 옵션을 켜야 자동 처리된다.
- 중복 주의: 법인·개인사업자 고객에게 해당 거래로 이미 세금계산서를 발행했다면 현금영수증을 중복 발행하지 않도록 예외 처리해야 한다.

## 크레이트 사용 지침

- PG 자동 발급이 켜져 있으면 `issue_cash_receipt`(수동 발급)를 호출하기 전에 `get_cash_receipt` 로 이미 발급됐는지 확인한다. 이미 발급된 경우 서버가 409 `CashReceiptAlreadyIssuedError` 로 거절한다.
- 세금계산서를 이미 발행한 거래는 호출 측에서 현금영수증 발급을 건너뛰어야 한다. 크레이트는 이를 판단하지 못한다.
- 수동 발급 대상은 위 대상 결제수단이며, 10만 원 이상 의무 여부 판단은 호출 측 책임이다.

## 크레이트 지원 범위

- 발급·단건 조회·취소·목록 조회, 유형(`CashReceiptType`)·식별값 유형(`IdentityNumberType`)·상태·정렬 enum, `identityNumberType` 입력.
- 목록 조회: `GetCashReceiptsBody`(page/sort/filter)를 JSON 으로 직렬화해 URL 인코딩한 `requestBody` 쿼리로 보낸다(공식 정책 "GET 요청 시 body 대신 query", 소유자 확인한 스키마와 일치). 필터에 `pgProviders`/`pgCompanies`/`version` 포함, 모든 필드 선택.
- 미지원: 세금계산서 발급 계열(#12).
- 공식 정책 점검 결과는 `docs/rest-v2/policy.md` 맨 아래 표 참고. 이 PR 에서 경로 인코딩(`get_payment_detail`)과 읽기 타임아웃 60초를 고쳤다.
- 테스트는 로컬 JSON 직렬화·역직렬화 단위 테스트뿐이며 실제 PortOne 호출은 없다.

## 후속 이슈

| 이슈 | 내용 |
|---|---|
| #12 | B2B 세금계산서·거래처 API와 웹훅 추가 |
| #13 | 현금영수증 수동 발급 전 중복 발급 방지 |
| #14 | 현금영수증 응답 `channel` 타입화 |
| #15 | 현금영수증 발급·취소에 `Idempotency-Key` 지원 |

(`GET /cash-receipts` 실연동 확인 이슈는 요청 스키마가 소유자 확인으로 확정돼 만들지 않았다.)
