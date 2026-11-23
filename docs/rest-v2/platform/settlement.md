# 파트너 정산: 정산·지급·이체 내역

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](../00-basics/errors.md). 공통 스키마: [schemas-common.md](../00-basics/schemas-common.md).

## 엔드포인트 목록

- `GET /platform/account-transfers` — 이체 내역 다건 조회 (platform.accountTransfer)
- `GET /platform/bulk-account-transfers` — 일괄 이체 내역 다건 조회 (platform.bulkAccountTransfer)
- `GET /platform/bulk-payouts` — 일괄 지급 내역 다건 조회 (platform.bulkPayout)
- `POST /platform/partner-settlements/complete-payout` — 일괄 지급 완료 처리 (platform.payout)
- `POST /platform/partner-settlements/delete` — 정산내역 삭제 (platform.partnerSettlement)
- `GET /platform/partner-settlements` — 정산 내역 다건 조회 (platform.partnerSettlement)
- `GET /platform/payouts` — 지급 내역 다건 조회 (platform.payout)
- `GET /platform/transfer-summaries/sheet-file` — 정산 상세 내역 다운로드 (platform.transfer)
- `GET /platform/transfer-summaries` — 정산건 다건 조회 (platform.transfer)
- `POST /platform/transfers/manual` — 수기 정산건 생성 (platform.transfer)
- `POST /platform/transfers/order-cancel` — 주문 취소 정산건 생성 (platform.transfer)
- `POST /platform/transfers/order` — 주문 정산건 생성 (platform.transfer)
- `GET /platform/transfers/{id}` — 정산건 조회 (platform.transfer)
- `DELETE /platform/transfers/{id}` — 정산건 삭제 (platform.transfer)

## GET /platform/account-transfers

**이체 내역 다건 조회** (operationId: `getPlatformAccountTransfers`)

여러 이체 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetAccountTransfersBody1](#getaccounttransfersbody1)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformAccountTransferFilter](#platformaccounttransferfilter) |  |  |

### 응답

- **200** 성공 응답으로 조회된 이체 내역과 페이지 정보를 반환합니다.: [GetPlatformAccountTransfersResponse](#getplatformaccounttransfersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/bulk-account-transfers

**일괄 이체 내역 다건 조회** (operationId: `getPlatformBulkAccountTransfers`)

성공 응답으로 조회된 일괄 이체 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformBulkAccountTransfersBody](#getplatformbulkaccounttransfersbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkAccountTransferFilterInput](#platformbulkaccounttransferfilterinput) |  |  |

### 응답

- **200** : [GetPlatformBulkAccountTransfersResponse](#getplatformbulkaccounttransfersresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/bulk-payouts

**일괄 지급 내역 다건 조회** (operationId: `getPlatformBulkPayouts`)

성공 응답으로 조회된 일괄 지급 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformBulkPayoutsBody](#getplatformbulkpayoutsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkPayoutFilterInput](#platformbulkpayoutfilterinput) |  |  |

### 응답

- **200** : [GetPlatformBulkPayoutsResponse](#getplatformbulkpayoutsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/partner-settlements/complete-payout

**일괄 지급 완료 처리** (operationId: `completePlatformPayoutByPartnerSettlementIds`)

선택한 정산내역 아이디들로 일괄 지급을 완료 처리 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CompletePlatformPayoutByPartnerSettlementIdsBody](#completeplatformpayoutbypartnersettlementidsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string | Y |  |
| `name` | string |  |  |
| `partnerSettlementIds` | string[] | Y |  |
| `completedAt` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [CompletePlatformPayoutByPartnerSettlementIdsResponse](#completeplatformpayoutbypartnersettlementidsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNoSelectedPartnerSettlementsError`](../00-basics/errors.md#platformnoselectedpartnersettlementserror): 선택된 정산건이 없는 경우
  * [`PlatformDuplicatedPartnerSettlementIdsError`](../00-basics/errors.md#platformduplicatedpartnersettlementidserror): 선택된 정산건 아이디에 중복이 있는 경우
  * [`PlatformNonPayablePartnerSettlementsError`](../00-basics/errors.md#platformnonpayablepartnersettlementserror): 지급할 수 없는 정산건이 포함된 경우
  * [`PlatformNegativePayoutAmountPartnersError`](../00-basics/errors.md#platformnegativepayoutamountpartnerserror): 지급 금액의 총합이 음수인 파트너가 존재하는 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerSettlementsNotFoundError`](../00-basics/errors.md#platformpartnersettlementsnotfounderror): 요청한 정산건 목록을 찾을 수 없는 경우
- **409**
  * [`PlatformBulkPayoutIdAlreadyExistsError`](../00-basics/errors.md#platformbulkpayoutidalreadyexistserror): 일괄 지급 아이디가 이미 존재하는 경우

## POST /platform/partner-settlements/delete

**정산내역 삭제** (operationId: `deletePlatformPartnerSettlements`)

선택한 정산내역들을 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [DeletePlatformPartnerSettlementsBody](#deleteplatformpartnersettlementsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementIds` | string[] | Y |  |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [DeletePlatformPartnerSettlementsResponse](#deleteplatformpartnersettlementsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformNonDeletablePartnerSettlementsError`](../00-basics/errors.md#platformnondeletablepartnersettlementserror): 삭제할 수 없는 정산건이 포함된 경우
  * [`PlatformReferencedCancelOrderTransfersExistError`](../00-basics/errors.md#platformreferencedcancelordertransfersexisterror): 취소 정산건이 참조 중인 정산건이 포함된 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerSettlementsNotFoundError`](../00-basics/errors.md#platformpartnersettlementsnotfounderror): 요청한 정산건 목록을 찾을 수 없는 경우

## GET /platform/partner-settlements

**정산 내역 다건 조회** (operationId: `getPlatformPartnerSettlements`)

여러 정산 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformPartnerSettlementsBody](#getplatformpartnersettlementsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerSettlementFilterInput](#platformpartnersettlementfilterinput) | Y | 조회할 정산내역 조건 필터 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 조회된 정산 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.: [GetPlatformPartnerSettlementsResponse](#getplatformpartnersettlementsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/payouts

**지급 내역 다건 조회** (operationId: `getPlatformPayouts`)

여러 지급 내역을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformPayoutsBody](#getplatformpayoutsbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformPayoutFilterInput](#platformpayoutfilterinput) |  |  |

### 응답

- **200** 성공 응답으로 조회된 지급 내역 리스트와 페이지 정보 및 상태 별 개수 정보를 반환합니다.: [GetPlatformPayoutsResponse](#getplatformpayoutsresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## GET /platform/transfer-summaries/sheet-file

**정산 상세 내역 다운로드** (operationId: `downloadPlatformTransferSheet`)

정산 상세 내역을 csv 파일로 다운로드 합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [DownloadPlatformTransferSheetBody](#downloadplatformtransfersheetbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 컬럼 키 목록 — - TRANSFER_MEMO:  메모 <br>- TRANSFER_TYPE: 정산 유형<br>- TRANSFER_STATUS:  상태 <br>- TRANSFER_ID: 정산 아이디<br>- TRANSFER_SETTLEMENT_DATE:  정산일 <br>- TRANSFER_SETTLEMENT_AMOUNT: 정산 금액<br>- TRANSFER_SETTLEMENT_TAX_FREE_AMOUNT: 정산 면세액<br>- TRANSFER_SETTLEMENT_CURRENCY: 정산 통화<br>- TRANSFER_SETTLEMENT_START_DATE: 정산 시작일<br>- TRANSFER_ORDER_NAME:  주문명 <br>- TRANSFER_ORDER_AMOUNT: 주문 금액<br>- TRANSFER_ORDER_TAX_FREE_AMOUNT: 주문 면세액<br>- TRANSFER_PAYMENT_ID: 주문 번호<br>- TRANSFER_PAYMENT_METHOD: 결제 수단<br>- TRANSFER_PAYMENT_AMOUNT: 결제 금액<br>- TRANSFER_PAYMENT_SUPPLY_AMOUNT: 결제 공급가액<br>- TRANSFER_PAYMENT_VAT_AMOUNT: 결제 부가세액<br>- TRANSFER_PAYMENT_TAX_FREE_AMOUNT: 결제 면세액<br>- TRANSFER_PAYMENT_VAT_BURDEN_AMOUNT: 결제 부가세 부담금<br>- TRANSFER_PLATFORM_FEE:  중개수수료 <br>- TRANSFER_PLATFORM_FEE_VAT: 중개수수료 부가세 부담금<br>- TRANSFER_CONTRACT_ID: 계약 아이디<br>- TRANSFER_CONTRACT_NAME: 계약 이름<br>- TRANSFER_DISCOUNT_AMOUNT: 할인 금액<br>- TRANSFER_DISCOUNT_TAX_FREE_AMOUNT: 할인 면세액<br>- TRANSFER_DISCOUNT_SHARE_AMOUNT: 할인 분담금<br>- TRANSFER_DISCOUNT_SHARE_TAX_FREE_AMOUNT: 할인 면세 분담금<br>- TRANSFER_ADDITIONAL_FEE:  추가수수료 <br>- TRANSFER_ADDITIONAL_FEE_VAT: 추가수수료 부가세 부담금<br>- TRANSFER_{UserDefinedProperty.Key}<br>- FORMULA_{UserDefinedFormula.Key}<br>- PARTNER_* : 파트너 컬럼 키 사용 가능(w/o PARTNER_STATUS_UPDATED_AT) |
| `fields` | string[] |  |  |

### 응답

- **200** 
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

## GET /platform/transfer-summaries

**정산건 다건 조회** (operationId: `getPlatformTransferSummaries`)

성공 응답으로 조회된 정산건 요약 리스트와 페이지 정보가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetPlatformTransferSummariesBody](#getplatformtransfersummariesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 조회할 정산건 조건 필터 |

### 응답

- **200** : [GetPlatformTransferSummariesResponse](#getplatformtransfersummariesresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우

## POST /platform/transfers/manual

**수기 정산건 생성** (operationId: `createPlatformManualTransfer`)

성공 응답으로 생성된 수기 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformManualTransferBody](#createplatformmanualtransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `memo` | string |  | 메모 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) |  | 정산 면세 금액 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) |  | 정산 통화 |

### 응답

- **200** : [CreateManualTransferResponse](#createmanualtransferresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformCurrencyNotSupportedError`](../00-basics/errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
  * [`PlatformNegativeAmountNotAllowedError`](../00-basics/errors.md#platformnegativeamountnotallowederror): 정산 건별 옵션이 켜진 플랫폼에서 음수 금액 수기 정산 생성을 시도한 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformTransferIdAlreadyUsedError`](../00-basics/errors.md#platformtransferidalreadyusederror)

## POST /platform/transfers/order-cancel

**주문 취소 정산건 생성** (operationId: `createPlatformOrderCancelTransfer`)

성공 응답으로 생성된 주문 취소 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformOrderCancelTransferBody](#createplatformordercanceltransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string |  | 파트너 아이디 |
| `paymentId` | string |  | 결제 아이디 |
| `transferId` | string |  | 정산건 아이디 |
| `cancellationId` | string | Y | 취소 내역 아이디 |
| `memo` | string |  | 메모 |
| `orderDetail` | [CreatePlatformOrderCancelTransferBodyOrderDetail](#createplatformordercanceltransferbodyorderdetail) |  | 주문 취소 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 취소 면세 금액 — 주문 취소 항목과 취소 면세 금액을 같이 전달하시면 최종 취소 면세 금액은 주문 취소 항목의 면세 금액이 아닌 전달해주신 취소 면세 금액으로 적용됩니다. |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 할인 정보 |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `externalCancellationDetail` | [CreatePlatformOrderCancelTransferBodyExternalCancellationDetail](#createplatformordercanceltransferbodyexternalcancellationdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 취소 정산건으로 등록되고, 존재하지않은 경우 포트원 결제 취소 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 취소 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### 응답

- **200** : [CreateOrderCancelTransferResponse](#createordercanceltransferresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformOrderDetailMismatchedError`](../00-basics/errors.md#platformorderdetailmismatchederror)
  * [`PlatformDiscountSharePolicyIdDuplicatedError`](../00-basics/errors.md#platformdiscountsharepolicyidduplicatederror)
  * [`PlatformCancellableAmountExceededError`](../00-basics/errors.md#platformcancellableamountexceedederror): 취소 가능한 금액이 초과한 경우
  * [`PlatformCancellableDiscountAmountExceededError`](../00-basics/errors.md#platformcancellablediscountamountexceedederror)
  * [`PlatformCancellableDiscountTaxFreeAmountExceededError`](../00-basics/errors.md#platformcancellablediscounttaxfreeamountexceedederror)
  * [`PlatformProductIdDuplicatedError`](../00-basics/errors.md#platformproductidduplicatederror)
  * [`PlatformCancellableProductQuantityExceededError`](../00-basics/errors.md#platformcancellableproductquantityexceedederror)
  * [`PlatformOrderTransferAlreadyCancelledError`](../00-basics/errors.md#platformordertransferalreadycancellederror)
  * [`PlatformSettlementAmountExceededError`](../00-basics/errors.md#platformsettlementamountexceedederror): 정산 가능한 금액을 초과한 경우
  * [`PlatformCancellationAndPaymentTypeMismatchedError`](../00-basics/errors.md#platformcancellationandpaymenttypemismatchederror)
  * [`PlatformSettlementCancelAmountExceededPortOneCancelError`](../00-basics/errors.md#platformsettlementcancelamountexceededportonecancelerror): 정산 취소 요청 금액이 포트원 결제 취소 내역의 취소 금액을 초과한 경우
  * [`PlatformCannotSpecifyTransferError`](../00-basics/errors.md#platformcannotspecifytransfererror): 정산 건 식별에 실패한 경우
  * [`PlatformSettlementDateEarlierThanSettlementStartDateError`](../00-basics/errors.md#platformsettlementdateearlierthansettlementstartdateerror): 정산일이 정산 시작일보다 빠른 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](../00-basics/errors.md#platformtransfernotfounderror)
  * [`PlatformCancellationNotFoundError`](../00-basics/errors.md#platformcancellationnotfounderror)
  * [`PlatformPaymentNotFoundError`](../00-basics/errors.md#platformpaymentnotfounderror)
  * [`PlatformProductIdNotFoundError`](../00-basics/errors.md#platformproductidnotfounderror)
  * [`PlatformTransferDiscountSharePolicyNotFoundError`](../00-basics/errors.md#platformtransferdiscountsharepolicynotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
- **409**
  * [`PlatformTransferAlreadyExistsError`](../00-basics/errors.md#platformtransferalreadyexistserror)
  * [`PlatformTransferIdAlreadyUsedError`](../00-basics/errors.md#platformtransferidalreadyusederror)

## POST /platform/transfers/order

**주문 정산건 생성** (operationId: `createPlatformOrderTransfer`)

성공 응답으로 생성된 주문 정산건 객체가 반환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 요청 본문 (application/json, 필수)

타입: [CreatePlatformOrderTransferBody](#createplatformordertransferbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `contractId` | string |  | 계약 아이디 — 기본값은 파트너의 기본 계약 아이디 입니다. |
| `memo` | string |  | 메모 |
| `paymentId` | string | Y | 결제 아이디 |
| `orderDetail` | [CreatePlatformOrderTransferBodyOrderDetail](#createplatformordertransferbodyorderdetail) | Y | 주문 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 면세 금액 — 주문 항목과 면세 금액을 같이 전달하시면 최종 면세 금액은 주문 항목의 면세 금액이 아닌 전달해주신 면세 금액으로 적용됩니다. |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 기본값은 결제 일시 입니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 추가 수수료 정보 |
| `externalPaymentDetail` | [CreatePlatformOrderTransferBodyExternalPaymentDetail](#createplatformordertransferbodyexternalpaymentdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 정산건 으로 등록되고, 존재하지않은 경우 포트원 결제 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `parameters` | [TransferParameters](#transferparameters) |  | 정산 파라미터 (실험기능) |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### 응답

- **200** : [CreateOrderTransferResponse](#createordertransferresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformProductIdDuplicatedError`](../00-basics/errors.md#platformproductidduplicatederror)
  * [`PlatformSettlementPaymentAmountExceededPortOnePaymentError`](../00-basics/errors.md#platformsettlementpaymentamountexceededportonepaymenterror): 정산 요청 결제 금액이 포트원 결제 내역의 결제 금액을 초과한 경우
  * [`PlatformSettlementTaxFreeAmountExceededPortOnePaymentError`](../00-basics/errors.md#platformsettlementtaxfreeamountexceededportonepaymenterror): 정산 요청 면세 금액이 포트원 결제 내역의 면세 금액을 초과한 경우
  * [`PlatformSettlementSupplyWithVatAmountExceededPortOnePaymentError`](../00-basics/errors.md#platformsettlementsupplywithvatamountexceededportonepaymenterror): 정산 요청 공급대가가 포트원 결제 내역의 공급대가를 초과한 경우
  * [`PlatformSettlementAmountExceededError`](../00-basics/errors.md#platformsettlementamountexceedederror): 정산 가능한 금액을 초과한 경우
  * [`PlatformContractPlatformFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`](../00-basics/errors.md#platformcontractplatformfixedamountfeecurrencyandsettlementcurrencymismatchederror)
  * [`PlatformAdditionalFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`](../00-basics/errors.md#platformadditionalfixedamountfeecurrencyandsettlementcurrencymismatchederror)
  * [`PlatformCurrencyNotSupportedError`](../00-basics/errors.md#platformcurrencynotsupportederror): 지원 되지 않는 통화를 선택한 경우
  * [`PlatformSettlementDateEarlierThanSettlementStartDateError`](../00-basics/errors.md#platformsettlementdateearlierthansettlementstartdateerror): 정산일이 정산 시작일보다 빠른 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformPartnerNotFoundError`](../00-basics/errors.md#platformpartnernotfounderror)
  * [`PlatformContractNotFoundError`](../00-basics/errors.md#platformcontractnotfounderror)
  * [`PlatformAdditionalFeePoliciesNotFoundError`](../00-basics/errors.md#platformadditionalfeepoliciesnotfounderror)
  * [`PlatformDiscountSharePoliciesNotFoundError`](../00-basics/errors.md#platformdiscountsharepoliciesnotfounderror)
  * [`PlatformPaymentNotFoundError`](../00-basics/errors.md#platformpaymentnotfounderror)
  * [`PlatformUserDefinedPropertyNotFoundError`](../00-basics/errors.md#platformuserdefinedpropertynotfounderror): 사용자 정의 속성이 존재 하지 않는 경우
  * [`PlatformSettlementParameterNotFoundError`](../00-basics/errors.md#platformsettlementparameternotfounderror): 정산 파라미터가 존재하지 않는 경우
- **409**
  * [`PlatformTransferAlreadyExistsError`](../00-basics/errors.md#platformtransferalreadyexistserror)
  * [`PlatformTransferIdAlreadyUsedError`](../00-basics/errors.md#platformtransferidalreadyusederror)

## GET /platform/transfers/{id}

**정산건 조회** (operationId: `getPlatformTransfer`)

정산건을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 조회하고 싶은 정산건 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** 성공 응답으로 정산건 객체가 반환됩니다.: [PlatformTransfer](#platformtransfer)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](../00-basics/errors.md#platformtransfernotfounderror)

## DELETE /platform/transfers/{id}

**정산건 삭제** (operationId: `deletePlatformTransfer`)

scheduled, in_process 상태의 정산건만 삭제가능합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `id` | path | string | Y | 정산건 아이디 |
| `test` | query | boolean |  | 테스트 모드 여부를 결정합니다. true 이면 테스트 모드로 실행됩니다. Request Body에도 isForTest가 있을 수 있으나, 둘 다 제공되면 Query Parameter의 test 값을 사용하고, Request Body의 isForTest는 무시됩니다. Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### 응답

- **200** : [DeletePlatformTransferResponse](#deleteplatformtransferresponse)
- **400**
  * [`InvalidRequestError`](../00-basics/errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * [`PlatformCancelOrderTransfersExistsError`](../00-basics/errors.md#platformcancelordertransfersexistserror)
  * [`PlatformTransferNonDeletableStatusError`](../00-basics/errors.md#platformtransfernondeletablestatuserror)
- **401**
  * [`UnauthorizedError`](../00-basics/errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * [`PlatformNotEnabledError`](../00-basics/errors.md#platformnotenablederror): 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우
  * [`ForbiddenError`](../00-basics/errors.md#forbiddenerror): 요청이 거절된 경우
- **404**
  * [`PlatformTransferNotFoundError`](../00-basics/errors.md#platformtransfernotfounderror)

---

# 스키마

### CompletePlatformPayoutByPartnerSettlementIdsBody

CompletePlatformPayoutByPartnerSettlementIdsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string | Y |  |
| `name` | string |  |  |
| `partnerSettlementIds` | string[] | Y |  |
| `completedAt` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### CompletePlatformPayoutByPartnerSettlementIdsResponse

일괄 지급 완료 처리 결과

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bulkPayoutId` | string |  |  |
| `bulkPayoutGraphqlId` | string |  |  |
| `payoutCount` | integer(int32) | Y |  |
| `partnerSettlementCount` | integer(int32) | Y |  |

### CreateManualTransferResponse

CreateManualTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformManualTransfer](#platformmanualtransfer) | Y |  |

### CreateOrderCancelTransferResponse

CreateOrderCancelTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformOrderCancelTransfer](#platformordercanceltransfer) | Y |  |

### CreateOrderTransferResponse

CreateOrderTransferResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transfer` | [PlatformOrderTransfer](#platformordertransfer) | Y |  |

### CreatePlatformManualTransferBody

수기 정산건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `memo` | string |  | 메모 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) |  | 정산 면세 금액 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) |  | 정산 통화 |

### CreatePlatformOrderCancelTransferBody

주문 취소 정산 등록을 위한 입력 정보 — 주문 취소 정산 등록을 위한 입력 정보<br>하나의 payment에 하나의 정산 건만 존재하는 경우에는 (partnerId, paymentId)로 취소 정산을 등록하실 수 있습니다.<br>하나의 payment에 여러 개의 정산 건이 존재하는 경우에는 transferId를 필수로 입력해야 합니다.<br>transferId를 입력한 경우 (partnerId, paymentId)는 생략 가능합니다.<br>id를 입력한 경우 생성될 취소 정산건 아이디로 사용됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string |  | 파트너 아이디 |
| `paymentId` | string |  | 결제 아이디 |
| `transferId` | string |  | 정산건 아이디 |
| `cancellationId` | string | Y | 취소 내역 아이디 |
| `memo` | string |  | 메모 |
| `orderDetail` | [CreatePlatformOrderCancelTransferBodyOrderDetail](#createplatformordercanceltransferbodyorderdetail) |  | 주문 취소 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 취소 면세 금액 — 주문 취소 항목과 취소 면세 금액을 같이 전달하시면 최종 취소 면세 금액은 주문 취소 항목의 면세 금액이 아닌 전달해주신 취소 면세 금액으로 적용됩니다. |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 할인 정보 |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `externalCancellationDetail` | [CreatePlatformOrderCancelTransferBodyExternalCancellationDetail](#createplatformordercanceltransferbodyexternalcancellationdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 취소 정산건으로 등록되고, 존재하지않은 경우 포트원 결제 취소 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 취소 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### CreatePlatformOrderCancelTransferBodyDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicyId` | string | Y | 할인 분담 정책 아이디 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세 할인 금액 |

### CreatePlatformOrderCancelTransferBodyExternalCancellationDetail

외부 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `cancelledAt` | string(date-time) |  | 취소 일시 |

### CreatePlatformOrderCancelTransferBodyOrderDetail

주문 취소 정보 — 주문 취소 정보<br>orderAmount, orderLines, all 중에서 하나만 입력하여야 합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `orderAmount` | integer(int64) |  | 주문 취소 금액 |
| `orderLines` | [CreatePlatformOrderCancelTransferBodyOrderLine](#createplatformordercanceltransferbodyorderline)[] |  | 주문 취소 항목 리스트 |
| `all` | [CreatePlatformOrderCancelTransferBodyOrderDetailAll](#createplatformordercanceltransferbodyorderdetailall) |  | 전체 금액 취소 |

### CreatePlatformOrderCancelTransferBodyOrderDetailAll

전체 금액 취소

type: object

### CreatePlatformOrderCancelTransferBodyOrderLine

주문 취소 항목 리스트

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `productId` | string | Y | 상품 아이디 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [CreatePlatformOrderCancelTransferBodyDiscount](#createplatformordercanceltransferbodydiscount)[] | Y | 상품 할인 정보 |

### CreatePlatformOrderTransferBody

주문 정산건 생성을 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerId` | string | Y | 파트너 아이디 |
| `contractId` | string |  | 계약 아이디 — 기본값은 파트너의 기본 계약 아이디 입니다. |
| `memo` | string |  | 메모 |
| `paymentId` | string | Y | 결제 아이디 |
| `orderDetail` | [CreatePlatformOrderTransferBodyOrderDetail](#createplatformordertransferbodyorderdetail) | Y | 주문 정보 |
| `taxFreeAmount` | integer(int64) |  | 주문 면세 금액 — 주문 항목과 면세 금액을 같이 전달하시면 최종 면세 금액은 주문 항목의 면세 금액이 아닌 전달해주신 면세 금액으로 적용됩니다. |
| `settlementStartDate` | string(date) |  | 정산 시작일 — 기본값은 결제 일시 입니다. |
| `settlementDate` | string(date) |  | 정산일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 추가 수수료 정보 |
| `externalPaymentDetail` | [CreatePlatformOrderTransferBodyExternalPaymentDetail](#createplatformordertransferbodyexternalpaymentdetail) |  | 외부 결제 상세 정보 — 해당 정보가 존재하는 경우 외부 결제 정산건 으로 등록되고, 존재하지않은 경우 포트원 결제 정산건으로 등록됩니다. |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `parameters` | [TransferParameters](#transferparameters) |  | 정산 파라미터 (실험기능) |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] |  | 사용자 정의 속성 |
| `id` | string |  | 생성할 정산건 아이디 — 명시하지 않으면 id 가 임의로 생성됩니다. |

### CreatePlatformOrderTransferBodyAdditionalFee

추가 수수료 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `policyId` | string | Y | 추가 수수료 정책 아이디 |

### CreatePlatformOrderTransferBodyDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicyId` | string | Y | 할인 분담 정책 아이디 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) |  | 면세 할인 금액 |

### CreatePlatformOrderTransferBodyExternalPaymentDetail

외부 결제 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `orderName` | string |  | 주문 명 |
| `paidAt` | string(date-time) |  | 결제 일시 |
| `method` | [PlatformPaymentMethodInput](#platformpaymentmethodinput) |  | 결제 수단 |

### CreatePlatformOrderTransferBodyOrderDetail

주문 정보 — 주문 정보<br>주문 금액 또는 주문 항목 하나만 입력 가능합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `orderAmount` | integer(int64) |  | 주문 금액 |
| `orderLines` | [CreatePlatformOrderTransferBodyOrderLine](#createplatformordertransferbodyorderline)[] |  | 주문 항목 리스트 |

### CreatePlatformOrderTransferBodyOrderLine

주문 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [CreatePlatformOrderTransferBodyProduct](#createplatformordertransferbodyproduct) | Y | 상품 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [CreatePlatformOrderTransferBodyDiscount](#createplatformordertransferbodydiscount)[] | Y | 상품 할인 정보 |
| `additionalFees` | [CreatePlatformOrderTransferBodyAdditionalFee](#createplatformordertransferbodyadditionalfee)[] | Y | 상품 추가 수수료 정보 |

### CreatePlatformOrderTransferBodyProduct

상품

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 아이디 |
| `name` | string | Y | 상품 이름 |
| `amount` | integer(int64) | Y | 상품 금액 |
| `taxFreeAmount` | integer(int64) |  | 상품 면세 금액 |
| `tag` | string |  | 태그 |

### DeletePlatformPartnerSettlementsBody

DeletePlatformPartnerSettlementsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementIds` | string[] | Y |  |
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### DeletePlatformPartnerSettlementsResponse

정산내역 삭제 결과

type: object

### DeletePlatformTransferResponse

DeletePlatformTransferResponse

type: object

### DownloadPlatformTransferSheetBody

DownloadPlatformTransferSheetBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 컬럼 키 목록 — - TRANSFER_MEMO:  메모 <br>- TRANSFER_TYPE: 정산 유형<br>- TRANSFER_STATUS:  상태 <br>- TRANSFER_ID: 정산 아이디<br>- TRANSFER_SETTLEMENT_DATE:  정산일 <br>- TRANSFER_SETTLEMENT_AMOUNT: 정산 금액<br>- TRANSFER_SETTLEMENT_TAX_FREE_AMOUNT: 정산 면세액<br>- TRANSFER_SETTLEMENT_CURRENCY: 정산 통화<br>- TRANSFER_SETTLEMENT_START_DATE: 정산 시작일<br>- TRANSFER_ORDER_NAME:  주문명 <br>- TRANSFER_ORDER_AMOUNT: 주문 금액<br>- TRANSFER_ORDER_TAX_FREE_AMOUNT: 주문 면세액<br>- TRANSFER_PAYMENT_ID: 주문 번호<br>- TRANSFER_PAYMENT_METHOD: 결제 수단<br>- TRANSFER_PAYMENT_AMOUNT: 결제 금액<br>- TRANSFER_PAYMENT_SUPPLY_AMOUNT: 결제 공급가액<br>- TRANSFER_PAYMENT_VAT_AMOUNT: 결제 부가세액<br>- TRANSFER_PAYMENT_TAX_FREE_AMOUNT: 결제 면세액<br>- TRANSFER_PAYMENT_VAT_BURDEN_AMOUNT: 결제 부가세 부담금<br>- TRANSFER_PLATFORM_FEE:  중개수수료 <br>- TRANSFER_PLATFORM_FEE_VAT: 중개수수료 부가세 부담금<br>- TRANSFER_CONTRACT_ID: 계약 아이디<br>- TRANSFER_CONTRACT_NAME: 계약 이름<br>- TRANSFER_DISCOUNT_AMOUNT: 할인 금액<br>- TRANSFER_DISCOUNT_TAX_FREE_AMOUNT: 할인 면세액<br>- TRANSFER_DISCOUNT_SHARE_AMOUNT: 할인 분담금<br>- TRANSFER_DISCOUNT_SHARE_TAX_FREE_AMOUNT: 할인 면세 분담금<br>- TRANSFER_ADDITIONAL_FEE:  추가수수료 <br>- TRANSFER_ADDITIONAL_FEE_VAT: 추가수수료 부가세 부담금<br>- TRANSFER_{UserDefinedProperty.Key}<br>- FORMULA_{UserDefinedFormula.Key}<br>- PARTNER_* : 파트너 컬럼 키 사용 가능(w/o PARTNER_STATUS_UPDATED_AT) |
| `fields` | string[] |  |  |

### EasyPayMethodType

간편 결제 수단

- `CARD`
- `TRANSFER`
- `CHARGE`

### GetAccountTransfersBody1

GetAccountTransfersBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformAccountTransferFilter](#platformaccounttransferfilter) |  |  |

### GetPlatformAccountTransfersResponse

이체내역 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformAccountTransfer](#platformaccounttransfer)[] | Y | 조회된 이체내역 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |
| `counts` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y | 이체 내역 상태별 건 수 |

### GetPlatformBulkAccountTransfersBody

GetPlatformBulkAccountTransfersBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkAccountTransferFilterInput](#platformbulkaccounttransferfilterinput) |  |  |

### GetPlatformBulkAccountTransfersResponse

GetPlatformBulkAccountTransfersResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformBulkAccountTransfer](#platformbulkaccounttransfer)[] | Y |  |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformBulkAccountTransferStatusStats](#platformbulkaccounttransferstatusstats) | Y |  |

### GetPlatformBulkPayoutsBody

GetPlatformBulkPayoutsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformBulkPayoutFilterInput](#platformbulkpayoutfilterinput) |  |  |

### GetPlatformBulkPayoutsResponse

GetPlatformBulkPayoutsResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformBulkPayout](#platformbulkpayout)[] | Y |  |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformBulkPayoutStatusStats](#platformbulkpayoutstatusstats) | Y |  |

### GetPlatformPartnerSettlementsBody

정산내역 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformPartnerSettlementFilterInput](#platformpartnersettlementfilterinput) | Y | 조회할 정산내역 조건 필터 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### GetPlatformPartnerSettlementsResponse

정산내역 다건 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPartnerSettlement](#platformpartnersettlement)[] | Y | 조회된 정산내역 리스트 |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |
| `counts` | [PlatformPartnerSettlementStatusStats](#platformpartnersettlementstatusstats) | Y | 정산내역 상태 별 갯수 |

### GetPlatformPayoutsBody

GetPlatformPayoutsBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `isForTest` | boolean |  | Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Request Body의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  |  |
| `filter` | [PlatformPayoutFilterInput](#platformpayoutfilterinput) |  |  |

### GetPlatformPayoutsResponse

GetPlatformPayoutsResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [PlatformPayout](#platformpayout)[] | Y |  |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y |  |
| `counts` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |

### GetPlatformTransferSummariesBody

정산건 요약 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `page` | [PageInput](../00-basics/schemas-common.md#pageinput) |  | 요청할 페이지 정보 |
| `filter` | [PlatformTransferFilterInput](#platformtransferfilterinput) |  | 조회할 정산건 조건 필터 |

### GetPlatformTransferSummariesResponse

GetPlatformTransferSummariesResponse

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `transferSummaries` | [PlatformTransferSummary](#platformtransfersummary)[] | Y |  |
| `page` | [PageInfo](../00-basics/schemas-common.md#pageinfo) | Y |  |

### PlatformAccountTransfer

계좌 이체 — 계좌 이체<br>송금 대행을 통해 일어난 정산 금액 지급, 인출 목적의 계좌 이체 결과 정보입니다.

oneOf, discriminator `type`:

- `DEPOSIT` → [PlatformDepositAccountTransfer](#platformdepositaccounttransfer)
- `WITHDRAWAL` → [PlatformWithdrawalAccountTransfer](#platformwithdrawalaccounttransfer)

### PlatformAccountTransferFilter

PlatformAccountTransferFilter

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `tradeTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 이체 일시 범위 |
| `createdTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `scheduledTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 이체 예정 일시 범위 |
| `accountTransferId` | string |  | 이체 아이디 |
| `bankAccountId` | string |  | 계좌 아이디 |
| `bulkAccountTransferId` | string |  | 일괄 이체 아이디 |
| `payoutId` | string |  | 지급 아이디 |
| `accountTransferIds` | string[] |  | 이체 아이디 리스트 |
| `types` | [PlatformAccountTransferType](#platformaccounttransfertype)[] |  | 구분 |
| `statuses` | [PlatformAccountTransferStatus](#platformaccounttransferstatus)[] |  | 상태 |
| `depositorName` | string |  | 입금자명 |
| `depositAccountHolder` | string |  | 예금주 |
| `depositMemo` | string |  | 받는 이 통장 메모 |
| `withdrawalMemo` | string |  | 보내는 이 통장 메모 |

### PlatformAccountTransferStatus

계좌 이체 상태

- `PREPARED` — 대기
- `SCHEDULED` — 예약
- `CANCELLED` — 취소
- `STOPPED` — 중단
- `PROCESSING` — 처리 중
- `ASYNC_PROCESSING` — 비동기 처리 중
- `SUCCEEDED` — 성공
- `FAILED` — 실패

### PlatformAccountTransferStatusStats

PlatformAccountTransferStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `prepared` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |
| `stopped` | integer(int64) | Y |  |
| `processing` | integer(int64) | Y |  |
| `asyncProcessing` | integer(int64) | Y |  |
| `succeeded` | integer(int64) | Y |  |
| `failed` | integer(int64) | Y |  |

### PlatformAccountTransferType

계좌 이체 유형

- `DEPOSIT` — 입금
- `WITHDRAWAL` — 출금

### PlatformBankAccountDetail

계좌 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `accountNumber` | string | Y | 계좌번호 |
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `provider` | [PlatformBankAccountProvider](#platformbankaccountprovider) | Y | 제공자 |
| `holder` | string |  | 예금주명 |

### PlatformBankAccountProvider

제공자

- `HYPHEN_DATA` — 하이픈 데이터
- `HYPHEN_FIRM` — 하이픈 펌뱅킹
- `DOZN` — 더즌
- `MOCK` — 모의

### PlatformBulkAccountTransfer

PlatformBulkAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 일괄 이체 고유 아이디 |
| `graphqlId` | string | Y |  |
| `creatorId` | string | Y |  |
| `bankAccountId` | string | Y | 출금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `totalAmount` | integer(int64) | Y |  |
| `status` | [PlatformBulkAccountTransferStatus](#platformbulkaccounttransferstatus) | Y |  |
| `stats` | [PlatformBulkAccountTransferStats](#platformbulkaccounttransferstats) | Y |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `createdAt` | string(date-time) | Y |  |
| `updatedAt` | string(date-time) | Y |  |
| `scheduledAt` | string(date-time) |  |  |

### PlatformBulkAccountTransferFilterInput

PlatformBulkAccountTransferFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformBulkAccountTransferStatus](#platformbulkaccounttransferstatus)[] |  |  |
| `criteria` | [PlatformBulkAccountTransferFilterInputCriteria](#platformbulkaccounttransferfilterinputcriteria) |  |  |

### PlatformBulkAccountTransferFilterInputCriteria

PlatformBulkAccountTransferFilterInputCriteria

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `bulkAccountTransferId` | string |  | 일괄 이체 아이디 |

### PlatformBulkAccountTransferStats

PlatformBulkAccountTransferStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `amount` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y |  |
| `count` | [PlatformAccountTransferStatusStats](#platformaccounttransferstatusstats) | Y |  |

### PlatformBulkAccountTransferStatus

PlatformBulkAccountTransferStatus

- `PREPARED`
- `SCHEDULED`
- `ONGOING`
- `COMPLETED`

### PlatformBulkAccountTransferStatusStats

PlatformBulkAccountTransferStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `prepared` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |
| `ongoing` | integer(int64) | Y |  |
| `completed` | integer(int64) | Y |  |

### PlatformBulkPayout

PlatformBulkPayout

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 일괄 지급 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 이름 |
| `creatorId` | string | Y | 생성자 아이디 |
| `method` | [PlatformPayoutMethod](#platformpayoutmethod) | Y | 지급 유형 |
| `totalPayoutAmount` | integer(int64) | Y | 총 지급 금액 |
| `totalSettlementAmount` | integer(int64) | Y | 총 정산 금액 |
| `status` | [PlatformBulkPayoutStatus](#platformbulkpayoutstatus) | Y | 상태 |
| `payoutStats` | [PlatformBulkPayoutStats](#platformbulkpayoutstats) | Y | 지급 통계 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 업데이트 일시 |

### PlatformBulkPayoutFilterInput

PlatformBulkPayoutFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformBulkPayoutStatus](#platformbulkpayoutstatus)[] |  |  |
| `methods` | [PlatformPayoutMethod](#platformpayoutmethod)[] |  |  |
| `criteria` | [PlatformBulkPayoutFilterInputCriteria](#platformbulkpayoutfilterinputcriteria) |  |  |

### PlatformBulkPayoutFilterInputCriteria

PlatformBulkPayoutFilterInputCriteria

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `bulkPayoutId` | string |  | 일괄 지급 아이디 |

### PlatformBulkPayoutStats

PlatformBulkPayoutStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `amount` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |
| `count` | [PlatformPayoutStatusStats](#platformpayoutstatusstats) | Y |  |

### PlatformBulkPayoutStatus

PlatformBulkPayoutStatus

- `PREPARING`
- `PREPARED`
- `ONGOING`
- `POST_PROCESS_PENDING`
- `CANCELLED`
- `COMPLETED`

### PlatformBulkPayoutStatusStats

PlatformBulkPayoutStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `preparing` | integer(int64) | Y |  |
| `prepared` | integer(int64) | Y |  |
| `ongoing` | integer(int64) | Y |  |
| `postProcessPending` | integer(int64) | Y |  |
| `completed` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |

### PlatformDepositAccountTransfer

PlatformDepositAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 계좌 이체 유형 |
| `id` | string | Y | 계좌 이체 아이디 |
| `bankAccountId` | string | Y | 입금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `bankAccount` | [PlatformBankAccountDetail](#platformbankaccountdetail) | Y | 입금 계좌 정보 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `amount` | integer(int64) | Y | 금액 |
| `depositMemo` | string |  | 입금 계좌 적요 |
| `tradedAt` | string(date-time) |  | 이체 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 수정 일시 |
| `depositorName` | string | Y | 입금자명 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `status` | [PlatformAccountTransferStatus](#platformaccounttransferstatus) | Y | 상태 |

### PlatformExternalPayment

외부 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 결제 아이디 |
| `orderName` | string |  | 주문 명 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `method` | [PlatformPaymentMethod](#platformpaymentmethod) |  | 결제 수단 |
| `paidAt` | string(date-time) |  | 결제 일시 |

### PlatformManualTransfer

수기 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |

### PlatformManualTransferSummary

PlatformManualTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `settlementAmount` | integer(int64) | Y |  |
| `settlementTaxFreeAmount` | integer(int64) | Y |  |

### PlatformOrderCancelTransfer

주문 취소 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 정산 금액 정보 |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 계약 |
| `payment` | [PlatformPayment](#platformpayment) | Y | 결제 정보 |
| `settlementStartDate` | string(date) | Y | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `orderLines` | [PlatformOrderTransferOrderLine](#platformordertransferorderline)[] | Y | 주문 항목 리스트 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 정산 금액 계산 시 사용된 추가 수수료 정보 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 정산 금액 계산 시 사용된 할인 정보 |
| `cancellation` | [PlatformOrderTransferCancellation](#platformordertransfercancellation) | Y | 주문 취소 정보 |
| `parameters` | [TransferParameters](#transferparameters) | Y | 정산 파라미터 (실험기능) |

### PlatformOrderCancelTransferSummary

PlatformOrderCancelTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `storeId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y |  |
| `payment` | [PlatformTransferSummaryPayment](#platformtransfersummarypayment) | Y |  |
| `settlementStartDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformOrderSettlementAmount

정산 금액 정보 — 정산 금액 정보<br>정산 금액과 정산 금액 계산에 사용된 금액 정보들 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlement` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFree` | integer(int64) | Y | 정산 면세 금액 |
| `payment` | integer(int64) | Y | 결제 금액 |
| `paymentVat` | integer(int64) | Y | 결제 금액 부가세 |
| `paymentVatBurden` | integer(int64) | Y | 결제 금액 부가세 부담금액 — 참조된 계약의 결제 금액 부가세 감액 여부에 따라 false인 경우 0원, true인 경우 결제 금액 부가세입니다. |
| `paymentTaxFree` | integer(int64) | Y | 결제 면세 금액 |
| `paymentSupply` | integer(int64) | Y | 결제 공급가액 |
| `order` | integer(int64) | Y | 주문 금액 |
| `orderTaxFree` | integer(int64) | Y | 면세 주문 금액 |
| `platformFee` | integer(int64) | Y | 중개 수수료 |
| `platformFeeVat` | integer(int64) | Y | 중개 수수료 부가세 |
| `additionalFee` | integer(int64) | Y | 추가 수수료 |
| `additionalFeeVat` | integer(int64) | Y | 추가 수수료 부가세 |
| `discount` | integer(int64) | Y | 할인 금액 |
| `discountTaxFree` | integer(int64) | Y | 면세 할인 금액 |
| `discountShare` | integer(int64) | Y | 할인 분담 금액 |
| `discountShareTaxFree` | integer(int64) | Y | 면세 할인 분담 금액 |
| `userDefinedFormulas` | [PlatformUserDefinedFormulaResults](#platformuserdefinedformularesults) | Y | 사용자 정의 수식 결과 |

### PlatformOrderTransfer

주문 정산건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산건 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y | 정산 상태 |
| `memo` | string |  | 메모 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `payoutId` | string |  | 더 이상 사용되지 않는 값입니다. |
| `payoutGraphqlId` | string |  |  |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 정산 금액 정보 |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 계약 |
| `payment` | [PlatformPayment](#platformpayment) | Y | 결제 정보 |
| `settlementStartDate` | string(date) | Y | 정산 시작일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `orderLines` | [PlatformOrderTransferOrderLine](#platformordertransferorderline)[] | Y | 주문 항목 리스트 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 정산 금액 계산 시 사용된 추가 수수료 정보 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 정산 금액 계산 시 사용된 할인 정보 |
| `parameters` | [TransferParameters](#transferparameters) | Y | 정산 파라미터 (실험기능) |

### PlatformOrderTransferAdditionalFee

추가 수수료 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `policy` | [PlatformAdditionalFeePolicy](schemas.md#platformadditionalfeepolicy) | Y | 추가 수수료 정책 |
| `amount` | integer(int64) | Y | 추가 수수료 금액 |
| `vat` | integer(int64) | Y | 추가 수수료 부가세 금액 |

### PlatformOrderTransferCancellation

주문 취소 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 주문 취소 아이디 |
| `cancelledAt` | string(date-time) | Y | 취소 일시 |

### PlatformOrderTransferDiscount

할인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `sharePolicy` | [PlatformDiscountSharePolicy](schemas.md#platformdiscountsharepolicy) | Y | 할인 분담 정책 |
| `amount` | integer(int64) | Y | 할인 금액 |
| `taxFreeAmount` | integer(int64) | Y | 면세 할인 금액 |
| `shareAmount` | integer(int64) | Y | 할인 분담 금액 |
| `shareTaxFreeAmount` | integer(int64) | Y | 면세 할인 분담 금액 |

### PlatformOrderTransferOrderLine

주문 항목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `product` | [PlatformOrderTransferProduct](#platformordertransferproduct) | Y | 상품 |
| `quantity` | integer(int32) | Y | 상품 수량 |
| `discounts` | [PlatformOrderTransferDiscount](#platformordertransferdiscount)[] | Y | 상품 할인 정보 |
| `additionalFees` | [PlatformOrderTransferAdditionalFee](#platformordertransferadditionalfee)[] | Y | 상품 추가 수수료 정보 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 상품 정산 금액 정보 |

### PlatformOrderTransferProduct

상품

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 아이디 |
| `name` | string | Y | 상품 이름 |
| `amount` | integer(int64) | Y | 상품 금액 |
| `taxFreeAmount` | integer(int64) | Y | 상품 면세 금액 |
| `tag` | string |  | 태그 |

### PlatformOrderTransferSummary

PlatformOrderTransferSummary

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `storeId` | string | Y |  |
| `partner` | [PlatformTransferSummaryPartner](#platformtransfersummarypartner) | Y |  |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y |  |
| `status` | [PlatformTransferStatus](#platformtransferstatus) | Y |  |
| `memo` | string |  |  |
| `settlementDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `isForTest` | boolean | Y |  |
| `partnerUserDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 — 5월 삭제 예정 필드입니다. partner.userDefinedProperties를 사용해주시길 바랍니다. |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y |  |
| `payment` | [PlatformTransferSummaryPayment](#platformtransfersummarypayment) | Y |  |
| `settlementStartDate` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPartnerManualSettlement

PlatformPartnerManualSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `amount` | integer(int64) | Y | 정산 금액 |
| `taxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerOrderCancelSettlement

PlatformPartnerOrderCancelSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 계약 |
| `settlementStartDateRange` | [DateRange](../00-basics/schemas-common.md#daterange) | Y | 정산 시작 일 범위 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 금액 정보 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerOrderSettlement

PlatformPartnerOrderSettlement

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 정산내역 아이디 |
| `graphqlId` | string | Y |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y | 파트너 |
| `settlementDate` | string(date) | Y | 정산 일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `settlementCurrency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산 통화 |
| `status` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus) | Y | 정산 상태 |
| `statusUpdatedAt` | string(date-time) |  | 상태 업데이트 일시 |
| `memo` | string |  | 메모 |
| `contract` | [PlatformContract](schemas.md#platformcontract) | Y | 계약 |
| `settlementStartDateRange` | [DateRange](../00-basics/schemas-common.md#daterange) | Y | 정산 시작 일 범위 |
| `amount` | [PlatformOrderSettlementAmount](#platformordersettlementamount) | Y | 금액 정보 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |

### PlatformPartnerSettlement

PlatformPartnerSettlement

oneOf, discriminator `type`:

- `MANUAL` → [PlatformPartnerManualSettlement](#platformpartnermanualsettlement)
- `ORDER_CANCEL` → [PlatformPartnerOrderCancelSettlement](#platformpartnerordercancelsettlement)
- `ORDER` → [PlatformPartnerOrderSettlement](#platformpartnerordersettlement)

### PlatformPartnerSettlementFilterInput

PlatformPartnerSettlementFilterInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlementDates` | string(date)[] |  |  |
| `contractIds` | string[] |  |  |
| `partnerTags` | string[] |  |  |
| `settlementCurrencies` | [Currency](../00-basics/schemas-common.md#currency)[] |  | 통화 |
| `statuses` | [PlatformPartnerSettlementStatus](#platformpartnersettlementstatus)[] |  | 정산 상태 |
| `partnerIds` | string[] |  |  |
| `settlementTypes` | [PlatformPartnerSettlementType](#platformpartnersettlementtype)[] |  | 정산 유형 |
| `keyword` | [PlatformPartnerSettlementFilterKeywordInput](#platformpartnersettlementfilterkeywordinput) |  |  |

### PlatformPartnerSettlementFilterKeywordInput

PlatformPartnerSettlementFilterKeywordInput

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `partnerSettlementId` | string |  |  |
| `payoutId` | string |  |  |
| `bulkPayoutId` | string |  |  |

### PlatformPartnerSettlementStatus

정산 상태

- `PAYOUT_SCHEDULED` — 지급 예약
- `PAYOUT_PREPARED` — 지급 예정
- `PAYOUT_WITHHELD` — 지급 보류
- `PAYOUT_EXCLUDED` — 지급 제외
- `PAYOUT_FAILED` — 지급 실패
- `IN_PAYOUT` — 지급 중
- `PAID_OUT` — 지급 완료
- `CANCELLED` — 지급 취소
- `CONFIRMED` — 지급 확정

### PlatformPartnerSettlementStatusStats

PlatformPartnerSettlementStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `payoutScheduled` | integer(int64) | Y |  |
| `payoutPrepared` | integer(int64) | Y |  |
| `payoutWithheld` | integer(int64) | Y |  |
| `payoutExcluded` | integer(int64) | Y |  |
| `payoutFailed` | integer(int64) | Y |  |
| `payoutCancelled` | integer(int64) | Y |  |
| `payoutConfirmed` | integer(int64) | Y |  |
| `inPayout` | integer(int64) | Y |  |
| `paidOut` | integer(int64) | Y |  |

### PlatformPartnerSettlementType

정산 유형

- `MANUAL` — 수동 정산
- `ORDER` — 주문 정산
- `ORDER_CANCEL` — 주문 취소 정산

### PlatformPayment

결제 정보

oneOf, discriminator `type`:

- `EXTERNAL` → [PlatformExternalPayment](#platformexternalpayment)
- `PORT_ONE` → [PlatformPortOnePayment](#platformportonepayment)

### PlatformPaymentMethod

결제 수단

oneOf, discriminator `type`:

- `CARD` → [PlatformPaymentMethodCard](#platformpaymentmethodcard)
- `EASY_PAY` → [PlatformPaymentMethodEasyPay](#platformpaymentmethodeasypay)
- `GIFT_CERTIFICATE` → [PlatformPaymentMethodGiftCertificate](#platformpaymentmethodgiftcertificate)
- `MOBILE` → [PlatformPaymentMethodMobile](#platformpaymentmethodmobile)
- `TRANSFER` → [PlatformPaymentMethodTransfer](#platformpaymentmethodtransfer)
- `VIRTUAL_ACCOUNT` → [PlatformPaymentMethodVirtualAccount](#platformpaymentmethodvirtualaccount)

### PlatformPaymentMethodCard

카드

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodCardInput

PlatformPaymentMethodCardInput

type: object

### PlatformPaymentMethodEasyPay

간편 결제

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `provider` | [EasyPayProvider](../00-basics/schemas-common.md#easypayprovider) |  | 간편 결제사 |
| `methodType` | [EasyPayMethodType](#easypaymethodtype) |  | 간편 결제 수단 |

### PlatformPaymentMethodEasyPayInput

간편 결제 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `provider` | [EasyPayProvider](../00-basics/schemas-common.md#easypayprovider) |  | 간편 결제사 |
| `methodType` | [EasyPayMethodType](#easypaymethodtype) |  | 간편 결제 수단 |

### PlatformPaymentMethodGiftCertificate

상품권

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodGiftCertificateInput

PlatformPaymentMethodGiftCertificateInput

type: object

### PlatformPaymentMethodInput

결제 수단 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `card` | [PlatformPaymentMethodCardInput](#platformpaymentmethodcardinput) |  | 카드 |
| `transfer` | [PlatformPaymentMethodTransferInput](#platformpaymentmethodtransferinput) |  | 계좌이체 |
| `virtualAccount` | [PlatformPaymentMethodVirtualAccountInput](#platformpaymentmethodvirtualaccountinput) |  | 가상계좌 |
| `giftCertificate` | [PlatformPaymentMethodGiftCertificateInput](#platformpaymentmethodgiftcertificateinput) |  | 상품권 |
| `mobile` | [PlatformPaymentMethodMobileInput](#platformpaymentmethodmobileinput) |  | 모바일 |
| `easyPay` | [PlatformPaymentMethodEasyPayInput](#platformpaymentmethodeasypayinput) |  | 간편 결제 |

### PlatformPaymentMethodMobile

모바일

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodMobileInput

PlatformPaymentMethodMobileInput

type: object

### PlatformPaymentMethodTransfer

계좌이체

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodTransferInput

PlatformPaymentMethodTransferInput

type: object

### PlatformPaymentMethodVirtualAccount

가상계좌

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformPaymentMethodVirtualAccountInput

PlatformPaymentMethodVirtualAccountInput

type: object

### PlatformPayout

PlatformPayout

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 지급 고유 아이디 |
| `graphqlId` | string | Y |  |
| `bulkPayoutId` | string | Y |  |
| `bulkPayoutGraphqlId` | string | Y |  |
| `method` | [PlatformPayoutMethod](#platformpayoutmethod) | Y |  |
| `status` | [PlatformPayoutStatus](#platformpayoutstatus) | Y |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `memo` | string |  |  |
| `partner` | [PlatformPartner](schemas.md#platformpartner) | Y |  |
| `account` | [PlatformPayoutAccount](#platformpayoutaccount) | Y |  |
| `taxInvoiceId` | string |  | 세금계산서 아이디 |
| `taxInvoiceStatus` | [PlatformPayoutTaxInvoiceStatus](#platformpayouttaxinvoicestatus) | Y | 세금계산서 상태 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `amount` | integer(int64) | Y | 지급금액 |
| `supplyAmount` | integer(int64) | Y | 공급가액 |
| `taxFreeAmount` | integer(int64) | Y | 면세 금액 |
| `vatAmount` | integer(int64) | Y | 부가세 |
| `settlementAmount` | integer(int64) | Y | 정산 금액 |
| `settlementTaxFreeAmount` | integer(int64) | Y | 정산 면세 금액 |
| `incomeTaxAmount` | integer(int64) | Y | 원천징수세액 (소득세) |
| `localIncomeTaxAmount` | integer(int64) | Y | 원천징수세액 (지방소득세) |
| `withdrawalMemo` | string |  |  |
| `depositMemo` | string |  |  |
| `createdAt` | string(date-time) | Y |  |
| `scheduledAt` | string(date-time) |  |  |
| `failReason` | string |  | 실패 사유 |
| `deductWht` | boolean | Y | 지급 금액에서 원천징수세 차감 여부 |
| `settlementAmountType` | [SettlementAmountType](schemas.md#settlementamounttype) | Y | 정산 금액 취급 기준 |
| `settlementStatement` | [PlatformPayoutSettlementStatementSummary](#platformpayoutsettlementstatementsummary) | Y | 정산 내역서 요약 정보 |

### PlatformPayoutAccount

PlatformPayoutAccount

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y |  |
| `number` | string | Y |  |
| `holder` | string | Y |  |

### PlatformPayoutFilterInput

지급 내역 필터 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `statuses` | [PlatformPayoutStatus](#platformpayoutstatus)[] |  | 지급 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 상태를 가진 지급 내역을 조회합니다. |
| `partnerIds` | string[] |  | 파트너 아이디 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 아이디를 가진 지급 내역을 조회합니다. |
| `criteria` | [PlatformPayoutFilterInputCriteria](#platformpayoutfilterinputcriteria) | Y | 조회 기준 |
| `payoutAccountBanks` | [Bank](../00-basics/schemas-common.md#bank)[] |  | 지급 계좌 은행 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 계좌 은행을 가진 지급 내역을 조회합니다. |
| `partnerTags` | string[] |  | 파트너 태그 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 태그를 하나 이상 가진 지급 내역을 조회합니다. |
| `payoutCurrencies` | [Currency](../00-basics/schemas-common.md#currency)[] |  | 지급 통화 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 통화를 가진 지급 내역을 조회합니다. |
| `payoutIds` | string[] |  | 지급 아이디 — 값이 존재하는 경우 해당 리스트에 포함되는 지급 아이디를 가진 지급 내역을 조회합니다. |
| `taxInvoiceStatuses` | [PlatformPayoutTaxInvoiceStatus](#platformpayouttaxinvoicestatus)[] |  | 세금계산서 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 세금계산서 상태를 가진 지급 내역을 조회합니다. |
| `partnerTypes` | [PlatformPartnerTypeName](schemas.md#platformpartnertypename)[] |  | 파트너 유형 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 유형을 가진 지급 내역을 조회합니다. |
| `partnerTaxationTypes` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype)[] |  | 파트너 과세 유형 — 값이 존재하는 경우 해당 리스트에 포함되는 파트너 과세 유형을 가진 지급 내역을 조회합니다. |
| `settlementStatementStatuses` | [PlatformPayoutSettlementStatementStatus](#platformpayoutsettlementstatementstatus)[] |  | 정산 내역서 상태 — 값이 존재하는 경우 해당 리스트에 포함되는 정산 내역서 상태를 가진 지급 내역을 조회합니다. |

### PlatformPayoutFilterInputCriteria

검색 기준 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `timestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 생성 일시 범위 |
| `statusUpdatedTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 상태 업데이트 일시 범위 |
| `scheduledTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 지급 예정 일시 범위 |
| `settlementStatementIssuedTimestampRange` | [DateTimeRange](../00-basics/schemas-common.md#datetimerange) |  | 정산 내역서 발송 일시 범위 |
| `payoutId` | string |  | 지급 아이디 |
| `bulkPayoutId` | string |  | 일괄 지급 아이디 |
| `taxInvoiceId` | string |  | 세금계산서 아이디 |
| `settlementStatementId` | string |  | 정산 내역서 아이디 |

### PlatformPayoutMethod

PlatformPayoutMethod

- `DIRECT`
- `AGENCY`

### PlatformPayoutSettlementStatementStatus

정산 내역서 발송 상태

- `UNSENT` — 미발송
- `SENT` — 발송 성공
- `SEND_FAILED` — 발송 실패
- `SEND_PREPARED` — 발송 대기

### PlatformPayoutSettlementStatementSummary

정산 내역서 요약 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `status` | [PlatformPayoutSettlementStatementStatus](#platformpayoutsettlementstatementstatus) | Y | 상태 |
| `id` | string |  | 아이디 |
| `issuedAt` | string(date-time) |  | 발송 일시 |

### PlatformPayoutStatus

PlatformPayoutStatus

- `CONFIRMED`
- `PREPARED`
- `CANCELLED`
- `STOPPED`
- `PROCESSING`
- `SUCCEEDED`
- `FAILED`
- `SCHEDULED`

### PlatformPayoutStatusStats

PlatformPayoutStatusStats

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `confirmed` | integer(int64) | Y |  |
| `prepared` | integer(int64) | Y |  |
| `cancelled` | integer(int64) | Y |  |
| `stopped` | integer(int64) | Y |  |
| `processing` | integer(int64) | Y |  |
| `succeeded` | integer(int64) | Y |  |
| `failed` | integer(int64) | Y |  |
| `scheduled` | integer(int64) | Y |  |

### PlatformPayoutTaxInvoiceStatus

PlatformPayoutTaxInvoiceStatus

- `NONE`
- `DRAFTED`
- `DRAFT_PENDING`
- `DRAFT_FAILED`
- `REQUESTED`
- `REQUEST_PENDING`
- `REQUEST_FAILED`
- `REQUEST_CANCELLED`
- `REQUEST_REFUSED`
- `ISSUED`
- `ISSUE_PENDING`
- `ISSUE_FAILED`
- `ISSUANCE_CANCELLED`
- `CANCEL_PENDING`
- `BEFORE_SENDING`
- `WAITING_SENDING`
- `SENDING`
- `SENDING_PENDING`
- `SENDING_COMPLETED`
- `SENDING_FAILED`

### PlatformPortOnePayment

포트원 결제 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y | 결제 아이디 |
| `storeId` | string | Y | 상점 아이디 |
| `channelKey` | string | Y | 채널 키 |
| `orderName` | string | Y | 주문 명 |
| `method` | [PlatformPaymentMethod](#platformpaymentmethod) |  | 결제 수단 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `paidAt` | string(date-time) | Y | 결제 일시 |

### PlatformSettlementParameterValue

플랫폼 정산 파라미터 값

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `decimal` | integer(int64) | Y | 크기가 조정되지 않은 숫자 |
| `decimalScale` | integer(int32) |  | 소수 자리수 — 정산 시 필요한 `decimalScale`이 지정되지 않은 경우 기본값으로 0을 사용합니다.<br>입력 가능한 법위는 0 ~ 5 입니다. |

### PlatformTransfer

정산건 — 정산건<br>정산건은 파트너에 정산해줄 정산 금액과 정산 방식 등이 포함되어 있는 정산 정보입니다.<br>정산 방식은은 주문 정산, 주문 취소 정산, 수기 정산이 있습니다.

oneOf, discriminator `type`:

- `MANUAL` → [PlatformManualTransfer](#platformmanualtransfer)
- `ORDER_CANCEL` → [PlatformOrderCancelTransfer](#platformordercanceltransfer)
- `ORDER` → [PlatformOrderTransfer](#platformordertransfer)

### PlatformTransferFilterInput

정산건 필터 입력 정보 — 정산건 필터 입력 정보<br>정산 시작일 범위와 정산 일 범위는 둘 중 하나만 입력 가능합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `settlementStartDateRange` | [DateRange](../00-basics/schemas-common.md#daterange) |  | 정산 시작일 범위 |
| `settlementDateRange` | [DateRange](../00-basics/schemas-common.md#daterange) |  | 정산 일 범위 |
| `partnerTags` | string[] |  | 파트너 태그 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 태그를 하나 이상 가지는 파트너에 대한 정산건만 조회합니다. |
| `contractIds` | string[] |  | 계약 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 계약 아이디를 가지는 정산건만 조회합니다. |
| `discountSharePolicyIds` | string[] |  | 할인 분담 정책 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 할인 분담 정책 아이디를 하나 이상 가지는 정산건만 조회합니다. |
| `additionalFeePolicyIds` | string[] |  | 추가 수수료 정책 아이디 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 추가 수수료 아이디를 하나 이상 가지는 정산건만 조회합니다. |
| `paymentMethodTypes` | [PaymentMethodType](../00-basics/schemas-common.md#paymentmethodtype)[] |  | 결제 수단 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 결제 수단을 가지는 파트너만 조회합니다. |
| `channelKeys` | string[] |  | 채널 키 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 채널 키를 가지는 정산건만 조회합니다. |
| `types` | [PlatformTransferType](#platformtransfertype)[] |  | 정산 방식 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 방식의 정산건만 조회합니다. |
| `statuses` | [PlatformTransferStatus](#platformtransferstatus)[] |  | 정산 상태 리스트 — 하나 이상의 값이 존재하는 경우 해당 리스트에 포함되는 정산 상태인 정산건만 조회합니다. |
| `userDefinedProperty` | [PlatformTransferPropertyExactMatchInput](#platformtransferpropertyexactmatchinput) |  | 사용자 정의 속성 exact match — 사용자 정의 속성의 key와 value가 정확히 일치하는 정산건만 조회합니다. |
| `keyword` | [PlatformTransferFilterInputKeyword](#platformtransferfilterinputkeyword) |  | 검색 키워드 |
| `isForTest` | boolean |  | 테스트 모드 여부 — Query Parameter의 test에 값이 제공된 경우 Query Parameter의 test를 사용하고 해당 값은 무시됩니다.<br>Query Parameter의 test와 Filter의 isForTest에 모두 값이 제공되지 않으면 기본값인 false로 적용됩니다. |

### PlatformTransferFilterInputKeyword

정산건 검색 키워드 입력 정보 — 정산건 검색 키워드 입력 정보<br>검색 키워드 적용을 위한 옵션으로, 명시된 키워드를 포함하는 정산건만 조회합니다. 하나의 하위 필드에만 값을 명시하여 요청합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `all` | string |  | 해당 값이 포함된 정보를 가진 정산건만 조회합니다. |
| `paymentId` | string |  | 해당 값이랑 일치하는 paymentId 를 가진 정산건만 조회합니다. |
| `transferId` | string |  | 해당 값이랑 일치하는 transferId 를 가진 정산건만 조회합니다. |
| `transferMemo` | string |  | 해당 값이 포함된 transferMemo 를 가진 정산건만 조회합니다. |
| `productId` | string |  | 해당 값이랑 일치하는 productId 를 가진 정산건만 조회합니다. |
| `productName` | string |  | 해당 값이랑 일치하는 productName 을 가진 정산건만 조회합니다. |
| `partnerId` | string |  | 해당 값이랑 일치하는 partnerId 를 가진 정산건만 조회합니다. |
| `partnerName` | string |  | 해당 값이 포함된 partnerName 을 가진 정산건만 조회합니다. |
| `partnerMemo` | string |  | 해당 값이 포함된 partnerMemo 를 가진 정산건만 조회합니다. |
| `partnerSettlementId` | string |  | 해당 값이 포함된 partnerSettlementId 를 가진 정산건만 조회합니다. |
| `payoutId` | string |  | 해당 값과 일치하는 지급건에 연관된 정산건만 조회합니다. |

### PlatformTransferPropertyExactMatchInput

사용자 정의 속성 key/value exact match

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `key` | string | Y | 키 |
| `value` | string | Y | 값 |

### PlatformTransferStatus

정산 상태

- `SCHEDULED` — 정산 예약
- `IN_PROCESS` — 정산 중
- `SETTLED` — 정산 완료
- `IN_PAYOUT` — 지급 중
- `PAID_OUT` — 지급 완료
- `PAYOUT_CONFIRMED` — 지급 확정

### PlatformTransferSummary

PlatformTransferSummary

oneOf, discriminator `type`:

- `MANUAL` → [PlatformManualTransferSummary](#platformmanualtransfersummary)
- `ORDER_CANCEL` → [PlatformOrderCancelTransferSummary](#platformordercanceltransfersummary)
- `ORDER` → [PlatformOrderTransferSummary](#platformordertransfersummary)

### PlatformTransferSummaryExternalPayment

PlatformTransferSummaryExternalPayment

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `orderName` | string |  |  |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `methodType` | [PaymentMethodType](../00-basics/schemas-common.md#paymentmethodtype) |  |  |

### PlatformTransferSummaryPartner

PlatformTransferSummaryPartner

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `name` | string | Y |  |
| `type` | [PlatformTransferSummaryPartnerType](#platformtransfersummarypartnertype) | Y |  |
| `taxationType` | [PlatformPartnerTaxationType](schemas.md#platformpartnertaxationtype) |  |  |
| `userDefinedProperties` | [PlatformUserDefinedPropertyKeyValue](#platformuserdefinedpropertykeyvalue)[] | Y | 사용자 정의 속성 |

### PlatformTransferSummaryPartnerType

파트너 유형

- `BUSINESS` — 사업자
- `WHT_PAYER` — 원천징수 대상자
- `NON_WHT_PAYER` — 원천징수 비대상자

### PlatformTransferSummaryPayment

PlatformTransferSummaryPayment

oneOf, discriminator `type`:

- `EXTERNAL` → [PlatformTransferSummaryExternalPayment](#platformtransfersummaryexternalpayment)
- `PORT_ONE` → [PlatformTransferSummaryPortOnePayment](#platformtransfersummaryportonepayment)

### PlatformTransferSummaryPortOnePayment

PlatformTransferSummaryPortOnePayment

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `id` | string | Y |  |
| `orderName` | string | Y |  |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y |  |
| `methodType` | [PaymentMethodType](../00-basics/schemas-common.md#paymentmethodtype) |  |  |

### PlatformTransferType

PlatformTransferType

- `ORDER`
- `ORDER_CANCEL`
- `MANUAL`

### PlatformUserDefinedFormulaResults

PlatformUserDefinedFormulaResults

type: map<string, integer(int64)>

### PlatformUserDefinedPropertyKeyValue

사용자 정의 속성

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `key` | string | Y | 사용자 정의 속성 키 |
| `value` | [PlatformUserDefinedPropertyValue](schemas.md#platformuserdefinedpropertyvalue) | Y | 사용자 정의 속성 값 |

### PlatformWithdrawalAccountTransfer

PlatformWithdrawalAccountTransfer

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y | 계좌 이체 유형 |
| `id` | string | Y | 계좌 이체 아이디 |
| `bankAccountId` | string | Y | 출금 계좌 아이디 |
| `bankAccountGraphqlId` | string | Y |  |
| `bankAccount` | [PlatformBankAccountDetail](#platformbankaccountdetail) | Y | 출금 계좌 정보 |
| `sequenceNumber` | integer(int32) |  | 거래 일련번호 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 통화 |
| `depositBank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 이체 계좌 은행 |
| `depositAccountNumber` | string | Y | 이체 계좌 번호 |
| `depositAccountHolder` | string | Y | 예금주 |
| `amount` | integer(int64) | Y | 금액 |
| `withdrawalMemo` | string |  | 보내는 이 통장 메모 |
| `depositMemo` | string |  | 받는 이 통장 메모 |
| `balance` | integer(int64) |  | 잔액 |
| `failReason` | string |  | 실패 사유 |
| `tradedAt` | string(date-time) |  | 이체 일시 |
| `createdAt` | string(date-time) | Y | 생성 일시 |
| `updatedAt` | string(date-time) | Y | 수정 일시 |
| `isForTest` | boolean | Y | 테스트 모드 여부 |
| `withdrawalType` | [Type](#type) | Y | 출금 유형 |
| `partnerId` | string |  | 파트너 고유 아이디 |
| `partnerGraphqlId` | string |  |  |
| `bulkPayoutId` | string |  | 일괄 지급 고유 아이디 |
| `bulkPayoutGraphqlId` | string |  |  |
| `payoutId` | string |  | 지급 고유 아이디 |
| `payoutGraphqlId` | string |  |  |
| `bulkAccountTransferId` | string |  |  |
| `bulkAccountTransferGraphqlId` | string |  |  |
| `documentId` | string |  | 전자서명 아이디 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 업데이트 일시 |
| `status` | [PlatformAccountTransferStatus](#platformaccounttransferstatus) | Y | 상태 |
| `scheduledAt` | string(date-time) |  | 예정 일시 |

### TransferParameters

TransferParameters

type: map<string, [PlatformSettlementParameterValue](#platformsettlementparametervalue)>

### Type

Type

- `PARTNER_PAYOUT`
- `REMIT`
