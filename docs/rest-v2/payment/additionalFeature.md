# 결제 부가기능 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /payment-gateways/card-promotion` — PG사 카드 프로모션 조회 API

## GET /payment-gateways/card-promotion

**PG사 카드 프로모션 조회 API** (operationId: `getPgCardPromotions`)

주어진 채널에 대해 PG사에서 제공하는 카드 프로모션 목록을 조회합니다.
해당 API는 현재 특정 PG사(KCP_V2)에 대해서만 지원되며, 지원 여부는 포트원 기술지원팀에 문의 부탁드립니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `channelKey` | query | string | Y | 조회하고자 하는 채널의 키 |
| `amount` | query | integer(int64) | Y | 결제 금액입니다. 해당 결제 금액 기준 이용 가능한 프로모션 목록이 조회됩니다. |
| `cardCompany` | query | [PgPromotionCardCompany](#pgpromotioncardcompany) |  | 조회할 카드사입니다. 값을 입력하지 않으면 카드사 필터링이 적용되지 않습니다. |

### 응답

- **200** 성공 응답으로 카드 프로모션 목록을 반환합니다.: [GetPgCardPromotionsResponse](#getpgcardpromotionsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **404**
  * [`ChannelNotFoundError`](../00-basics/errors.md#channelnotfounderror): 요청된 채널이 존재하지 않는 경우
- **502**
  * [`PgProviderError`](../00-basics/errors.md#pgprovidererror): PG사에서 오류를 전달한 경우

---

# 스키마

### GetPgCardPromotionsResponse

PG사 카드 프로모션 조회 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `promotions` | [PgCardPromotion](#pgcardpromotion)[] |  | 카드 프로모션 목록 |

### PgCardPromotion

PG사 카드 프로모션 — PG사 카드 프로모션<br>PG사에서 제공하는 카드 프로모션 정보입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `promotionId` | string | Y | 프로모션 아이디 — PG사에서 부여한 프로모션 식별자입니다. |
| `cardCompany` | [PgPromotionCardCompany](#pgpromotioncardcompany) | Y | 카드사 — 프로모션이 적용되는 카드사입니다. |
| `discountAmount` | integer(int64) | Y | 할인 금액 — 프로모션 적용 시 할인되는 금액입니다. |
| `minimumPaymentAmount` | integer(int64) | Y | 최소 결제 금액 — 프로모션이 적용되기 위한 최소 결제 금액입니다. |

### PgPromotionCardCompany

PG 프로모션 카드사 — PG 프로모션 카드사<br>PG사 프로모션 조회 시 필터링할 수 있는 카드사 목록입니다.

- `KOREA_DEVELOPMENT_BANK` — KDB산업은행
- `KFCC` — 새마을금고
- `SHINHYUP` — 신협
- `EPOST` — 우체국
- `SAVINGS_BANK_KOREA` — 저축은행
- `KAKAO_BANK` — 카카오뱅크
- `WOORI_CARD` — 우리카드
- `BC_CARD` — BC카드
- `GWANGJU_CARD` — 광주카드
- `SAMSUNG_CARD` — 삼성카드
- `SHINHAN_CARD` — 신한카드
- `HYUNDAI_CARD` — 현대카드
- `LOTTE_CARD` — 롯데카드
- `SUHYUP_CARD` — 수협카드
- `CITI_CARD` — 씨티카드
- `NH_CARD` — NH카드
- `JEONBUK_CARD` — 전북카드
- `JEJU_CARD` — 제주카드
- `HANA_CARD` — 하나카드
- `KOOKMIN_CARD` — 국민카드
- `K_BANK` — 케이뱅크
- `TOSS_BANK` — 토스뱅크
- `MIRAE_ASSET_SECURITIES` — 미래에셋증권
