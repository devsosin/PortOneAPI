# 현금영수증·세금계산서 조사 기록 (PORT-003)

## 출처

- PortOne REST V2 문서 스냅샷: `docs/rest-v2/cash-receipt-tax-invoice/cashReceipt.md`, `taxInvoice.md`, `counterparty.md`, `docs/rest-v2/payment/payment.md`, `docs/rest-v2/guides/platform-webhook-tax-invoice.md`
- 세무 검토: 이슈 #3 소유자 코멘트(아래 "세무 검토 결과")

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
- 목록 조회는 조건을 JSON 으로 직렬화해 `requestBody` 쿼리로 보낸다. 문서 스냅샷은 이 파라미터의 타입·인코딩을 적지 않았고(`requestBody | query`), 요청 본문 스키마만 있다. 전송 방식이 달라도 공개 API(`GetCashReceiptsBody`)는 바뀌지 않으므로, 실제 연동에서 확인이 필요하다(후속 이슈 3).
- 미지원: 목록 필터의 `pgProviders`/`pgCompanies`/`version`, 세금계산서 발급 계열.
- 테스트는 로컬 JSON 직렬화·역직렬화 단위 테스트뿐이며 실제 PortOne 호출은 없다.

## 후속 이슈 초안 (생성하지 않음)

### 1. [PORT-0xx] B2B 세금계산서 API 추가

- 개요: `/b2b/tax-invoices` 계열(즉시 발행 `issue-immediately`, 임시저장 `draft`, `issue`/`request`/`send-to-nts`/`cancel-issuance`/`cancel-request`/`refuse-request`/`request-reverse-issuance`, 조회, `pdf-download-url`/`print-url`/`popup-url`, 삭제)과 거래처(`/b2b/counterparties`), 상태 변경 웹훅(`platform-webhook-tax-invoice.md`)을 크레이트에 추가한다.
- 사용자 검토 필요: 지원 범위(발행만 vs 전체), 웹훅 검증을 기존 `features::webhook` 에 합칠지, 수집할 거래처 정보.
- 작업: 발행·취소 등 상태 변경 호출에 멱등키 적용, DTO·enum, 모의 서버 테스트.
- 완료 조건: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
- 선행: 없음(이 PR 병합 후 진행 권장).

### 2. [PORT-0xx] 현금영수증 발급 전 중복 방지 헬퍼

- 개요: 수동 발급 전 기존 발급 여부(`get_cash_receipt`) 확인과 `CashReceiptAlreadyIssuedError`(409) 전용 처리를 크레이트에서 제공할지 정한다. `CancelError` 처럼 기능별 오류 enum 이 필요한지 포함.
- 사용자 검토 필요: 크레이트 책임 범위(호출 측에 맡김 vs 헬퍼 제공).

### 3. [PORT-0xx] 현금영수증 목록 조회 실연동 확인

- 개요: `GET /cash-receipts` 의 `requestBody` 쿼리 인코딩(JSON 문자열)을 테스트 채널로 확인하고, 필터 `pgProviders`/`pgCompanies`/`version` 을 추가한다. 실연동 테스트는 `#[ignore]` 로 두고 사용자 승인 후 실행한다.

### 4. [PORT-0xx] 현금영수증 응답 `channel` 타입화

- 개요: `channel` 이 `serde_json::Value` 이다. 문서의 `SelectedChannel`(`type`, `id`, `key`, `name`, `pgProvider`, `pgMerchantId`)로 타입화하되 PG사 enum 은 알 수 없는 값을 허용해야 한다(공개 타입 변경이라 마이너 버전 항목).
