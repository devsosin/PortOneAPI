# 대사 서비스 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /payment-reconciliations/settlements/vat-report` — 정산일 기준 부가세 내역 조회
- `GET /payment-reconciliations/transactions/vat-report` — 거래일 기준 부가세 내역 조회

## GET /payment-reconciliations/settlements/vat-report

**정산일 기준 부가세 내역 조회** (operationId: `getPaymentReconciliationSettlementVatReport`)

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  | Y |  |

### 응답

- **200** : [GetPaymentReconciliationSettlementVatReportResponse](#getpaymentreconciliationsettlementvatreportresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /payment-reconciliations/transactions/vat-report

**거래일 기준 부가세 내역 조회** (operationId: `getPaymentReconciliationTransactionVatReport`)

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  | Y |  |

### 응답

- **200** : [GetPaymentReconciliationTransactionVatReportResponse](#getpaymentreconciliationtransactionvatreportresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

---

# 스키마

### GetPaymentReconciliationSettlementVatReportBody

GetPaymentReconciliationSettlementVatReportBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dateRange` | [DateRange](../00-basics/schemas-common.md#daterange) | Y | 정산일 범위 |
| `filter` | [PaymentReconciliationSettlementSummaryFilterInput](#paymentreconciliationsettlementsummaryfilterinput) |  |  |

### GetPaymentReconciliationSettlementVatReportResponse

GetPaymentReconciliationSettlementVatReportResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentReconciliationVatReportItem](#paymentreconciliationvatreportitem)[] |  | 부가세 내역 항목 리스트 |
| `summary` | [PaymentReconciliationVatReportSummary](#paymentreconciliationvatreportsummary) | Y | 부가세 내역 요약 |
| `pgSpecifiers` | [ReconciliationPgSpecifier](#reconciliationpgspecifier)[] |  | PG사 식별자 리스트 |

### GetPaymentReconciliationTransactionVatReportBody

GetPaymentReconciliationTransactionVatReportBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dateRange` | [DateRange](../00-basics/schemas-common.md#daterange) | Y | 거래일 범위 |
| `filter` | [PaymentReconciliationTransactionSummaryFilterInput](#paymentreconciliationtransactionsummaryfilterinput) |  |  |

### GetPaymentReconciliationTransactionVatReportResponse

GetPaymentReconciliationTransactionVatReportResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PaymentReconciliationVatReportItem](#paymentreconciliationvatreportitem)[] |  | 부가세 내역 항목 리스트 |
| `summary` | [PaymentReconciliationVatReportSummary](#paymentreconciliationvatreportsummary) | Y | 부가세 내역 요약 |
| `pgSpecifiers` | [ReconciliationPgSpecifier](#reconciliationpgspecifier)[] |  | PG사 식별자 리스트 |

### PaymentReconciliationSettlementSummaryFilterInput

거래대사 정산 요약 내역 필터

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgSpecifiers` | [ReconciliationPgSpecifier](#reconciliationpgspecifier)[] |  | PG사 가맹점 식별자 필터 |
| `storeIds` | string[] |  | 하위 상점 아이디 필터 |

### PaymentReconciliationStatus

결제 건의 대사 상태

- `MATCHED` — 대사 매칭 성공 상태
- `NOT_MATCHED` — 대사 매칭 실패 상태
- `INCOMPARABLE` — 대사 불가능 상태
- `NOT_COLLECTED` — PG사 결제 정보가 수집되지 않은 상태

### PaymentReconciliationTransactionSummaryFilterInput

거래대사 거래내역 필터

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `reconciliationStatuses` | [PaymentReconciliationStatus](#paymentreconciliationstatus)[] |  | 대사 상태 필터 |
| `pgSpecifiers` | [ReconciliationPgSpecifier](#reconciliationpgspecifier)[] |  | 대사용 PG사 가맹점 식별자 필터 |
| `storeIds` | string[] |  | 하위 상점 아이디 필터 |

### PaymentReconciliationVatReportItem

PaymentReconciliationVatReportItem

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `paymentMethod` | [SimplifiedPaymentMethodType](#simplifiedpaymentmethodtype) | Y | 결제수단 |
| `supplyAmount` | integer(int64) | Y | 공급가액 |
| `vatAmount` | integer(int64) | Y | 부가세 금액 |
| `taxFreeAmount` | integer(int64) | Y | 면세 금액 |
| `totalAmount` | integer(int64) | Y | 총 금액 |

### PaymentReconciliationVatReportSummary

PaymentReconciliationVatReportSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `totalSupplyAmount` | integer(int64) | Y | 총 공급가액 |
| `totalVatAmount` | integer(int64) | Y | 총 부가세 금액 |
| `totalTaxFreeAmount` | integer(int64) | Y | 총 면세 금액 |
| `totalAmount` | integer(int64) | Y | 총 금액 |

### ReconciliationPgProvider

ReconciliationPgProvider

- `KAKAOPAY`
- `NICEPAY`
- `NAVERPAY`
- `UPLUS`
- `TOSSPAYMENTS`
- `TOSSPAY`
- `PAYCO`
- `KCP`
- `DANAL`
- `EXIMBAY`
- `INICIS`
- `HECTO`
- `KSNET`
- `KPN`
- `HYPHEN`
- `PAYPAL`
- `HECTO_EASY`
- `MOBILIANS`
- `PAYLETTER_GLOBAL`
- `KIWOOMPAY`
- `PAYMENTWALL`
- `INNOPAY`
- `GALAXIA`
- `KOMOJU`
- `GRAB`
- `ATOME`
- `PAYME`

### ReconciliationPgSpecifier

대사용 PG사 가맹점 식별자

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `pgMerchantId` | string | Y | PG사 가맹점 식별 아이디 |
| `pgProvider` | [ReconciliationPgProvider](#reconciliationpgprovider) | Y | PG사 |

### SimplifiedPaymentMethodType

간소화된 결제수단 목록

- `CARD` — 신용카드/체크카드
- `VIRTUAL_ACCOUNT` — 가상계좌
- `TRANSFER` — 계좌이체
- `CHARGE` — 머니
- `MOBILE` — 모바일
- `POINT` — 포인트
- `ETC` — 기타
