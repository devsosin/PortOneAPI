# 공통 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /banks` — 은행 정보 조회

## GET /banks

**은행 정보 조회** (operationId: `getBankInfos`)

은행의 이름 등 정보를 조회합니다. 사용자에게 한국어 명칭을 제공하는 등의 용도로 사용하실 수 있습니다.

### 응답

- **200** 성공 응답으로 조회된 은행 정보 리스트가 반환됩니다.: [GetBankInfosResponse](#getbankinfosresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우

---

# 스키마

### BankInfo

은행 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](schemas-common.md#bank) | Y | 은행 |
| `name` | [BankInfoName](#bankinfoname) | Y | 언어별 명칭 |

### BankInfoName

은행 명칭

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `ko` | string | Y | 한국어 명칭 |

### GetBankInfosResponse

은행 정보 조회 성공 응답 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [BankInfo](#bankinfo)[] | Y | 조회된 은행 정보 리스트 |
