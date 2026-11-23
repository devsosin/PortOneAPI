# 프로모션 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /promotions/{promotionId}` — 프로모션 단건 조회

## GET /promotions/{promotionId}

**프로모션 단건 조회** (operationId: `getPromotion`)

주어진 아이디에 대응되는 프로모션을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `promotionId` | path | string | Y | 조회할 프로모션 아이디 |

### 응답

- **200** 성공 응답으로 프로모션 객체를 반환합니다.: [Promotion](#promotion)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PromotionNotFoundError`](../00-basics/errors.md#promotionnotfounderror): 프로모션이 존재하지 않는 경우

---

# 스키마

### CardPromotion

카드 프로모션

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 프로모션 유형 |
| `id` | string | Y | 프로모션 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `name` | string | Y | 프로모션 이름 |
| `discountPolicy` | [PromotionDiscountPolicy](#promotiondiscountpolicy) | Y | 할인 정책 |
| `totalBudget` | integer(int64) | Y | 총 예산 |
| `maxDiscountAmount` | integer(int64) |  | 최대 할인 금액 |
| `spentAmount` | integer(int64) | Y | 소진 금액 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 금액 화폐 |
| `startAt` | string(date-time) | Y | 프로모션 시작 시각 |
| `endAt` | string(date-time) | Y | 프로모션 종료 시각 |
| `terminatedAt` | string(date-time) |  | 프로모션 중단 시각 |
| `cardCompany` | [PromotionCardCompany](#promotioncardcompany) | Y | 프로모션 카드사 |
| `status` | [PromotionStatus](#promotionstatus) | Y | 프로모션 상태 |
| `createdAt` | string(date-time) | Y | 프로모션 생성 시각 |
| `recoverOption` | [PromotionRecoverOption](#promotionrecoveroption) | Y | 결제 취소 시 프로모션 예산 복구 옵션 |

### Promotion

프로모션

oneOf, discriminator `type`:

- `CARD` → [CardPromotion](#cardpromotion)

### PromotionAmountDiscountScheme

PromotionAmountDiscountScheme

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 프로모션 할인 유형 |
| `amount` | integer(int64) | Y |  |

### PromotionCardCompany

프로모션 적용 가능한 카드사

- `WOORI_CARD` — 우리카드
- `BC_CARD` — BC카드
- `SAMSUNG_CARD` — 삼성카드
- `SHINHAN_CARD` — 신한카드
- `HYUNDAI_CARD` — 현대카드
- `LOTTE_CARD` — 롯데카드
- `NH_CARD` — NH카드
- `HANA_CARD` — 하나카드
- `KOOKMIN_CARD` — 국민카드

### PromotionDiscountPartition

금액 구간별 프로모션 할인 정책

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `amountFrom` | integer(int64) | Y |  |
| `scheme` | [PromotionDiscountScheme](#promotiondiscountscheme) | Y |  |

### PromotionDiscountPolicy

프로모션 할인 정책

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partitions` | [PromotionDiscountPartition](#promotiondiscountpartition)[] | Y | 금액 구간별 프로모션 할인 정책 |

### PromotionDiscountScheme

PromotionDiscountScheme

oneOf, discriminator `type`:

- `AMOUNT` → [PromotionAmountDiscountScheme](#promotionamountdiscountscheme)
- `PERCENT` → [PromotionPercentDiscountScheme](#promotionpercentdiscountscheme)

### PromotionPercentDiscountScheme

PromotionPercentDiscountScheme

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 프로모션 할인 유형 |
| `percent` | integer(int32) | Y |  |

### PromotionRecoverOption

PromotionRecoverOption

oneOf, discriminator `type`:

- `NO_RECOVER` → [PromotionRecoverOptionNoRecover](#promotionrecoveroptionnorecover)
- `RECOVER` → [PromotionRecoverOptionRecover](#promotionrecoveroptionrecover)

### PromotionRecoverOptionNoRecover

결제 취소 시 프로모션 예산 미복구

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 결제 취소 시 프로모션 예산 복구 옵션 |
| `spareBudget` | [PromotionSpareBudget](#promotionsparebudget) |  |  |

### PromotionRecoverOptionRecover

결제 취소 시 프로모션 예산 복구

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 결제 취소 시 프로모션 예산 복구 옵션 |

### PromotionSpareBudget

PromotionSpareBudget

oneOf, discriminator `type`:

- `AMOUNT` → [PromotionSpareBudgetAmount](#promotionsparebudgetamount)
- `PERCENT` → [PromotionSpareBudgetPercent](#promotionsparebudgetpercent)

### PromotionSpareBudgetAmount

PromotionSpareBudgetAmount

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 환불 대비용 프로모션 추가 예산 |
| `amount` | integer(int64) | Y |  |

### PromotionSpareBudgetPercent

PromotionSpareBudgetPercent

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 환불 대비용 프로모션 추가 예산 |
| `percent` | integer(int32) | Y |  |

### PromotionStatus

PromotionStatus

- `SCHEDULED` — 예정됨
- `IN_PROGRESS` — 진행중
- `PAUSED` — 일시 중지됨
- `BUDGET_EXHAUSTED` — 예산 소진됨
- `TERMINATED` — 중단됨
- `COMPLETED` — 완료됨
