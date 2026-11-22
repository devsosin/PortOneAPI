# 세금계산서 발행 관련 API

> 출처: https://developers.portone.io/api/rest-v2?v=v2 (OpenAPI: https://developers.portone.io/schema/v2.openapi.json). Base URL `https://api.portone.io`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).

## 엔드포인트 목록

- `GET /b2b/bulk-tax-invoices/{bulkTaxInvoiceId}` — 일괄 세금계산서 조회
- `POST /b2b/file-upload-url` — 파일 업로드 URL 생성
- `GET /b2b/tax-invoices-sheet` — 세금계산서 엑셀 파일(csv) 다운로드
- `PUT /b2b/tax-invoices/draft` — 세금계산서 임시저장 수정
- `POST /b2b/tax-invoices/draft` — 세금계산서 임시 저장
- `POST /b2b/tax-invoices/issue-immediately` — 세금계산서 즉시 정발행
- `POST /b2b/tax-invoices/request-reverse-issuance` — 세금계산서 역발행 즉시 요청
- `POST /b2b/tax-invoices/{taxInvoiceKey}/attach-file` — 세금계산서 파일 첨부
- `DELETE /b2b/tax-invoices/{taxInvoiceKey}/attachments/{attachmentId}` — 세금계산서 첨부파일 삭제
- `GET /b2b/tax-invoices/{taxInvoiceKey}/attachments` — 세금계산서 첨부파일 목록 조회
- `POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-issuance` — 세금계산서 취소 (공급자에 의한 취소)
- `POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-request` — 세금계산서 역발행 요청 취소 (공급받는자에 의한 취소)
- `POST /b2b/tax-invoices/{taxInvoiceKey}/issue` — 세금계산서 발행 승인
- `GET /b2b/tax-invoices/{taxInvoiceKey}/pdf-download-url` — 세금 계산서 PDF 다운로드 URL 조회
- `GET /b2b/tax-invoices/{taxInvoiceKey}/popup-url` — 세금 계산서 팝업 URL 조회
- `GET /b2b/tax-invoices/{taxInvoiceKey}/print-url` — 세금 계산서 프린트 URL 조회
- `POST /b2b/tax-invoices/{taxInvoiceKey}/refuse-request` — 세금계산서 역발행 요청 거부
- `POST /b2b/tax-invoices/{taxInvoiceKey}/request` — 세금계산서 역발행 요청
- `POST /b2b/tax-invoices/{taxInvoiceKey}/send-to-nts` — 세금계산서 국세청 즉시 전송
- `GET /b2b/tax-invoices/{taxInvoiceKey}` — 세금 계산서 조회
- `DELETE /b2b/tax-invoices/{taxInvoiceKey}` — 세금계산서 삭제
- `GET /b2b/tax-invoices` — 세금 계산서 다건조회

## GET /b2b/bulk-tax-invoices/{bulkTaxInvoiceId}

**일괄 세금계산서 조회** (operationId: `getB2bBulkTaxInvoice`)

등록된 일괄 세금계산서를 일괄 세금계산서 아이디로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `bulkTaxInvoiceId` | path | string | Y | 일괄 세금계산서 아이디 |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [B2bBulkTaxInvoice](#b2bbulktaxinvoice)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bBulkTaxInvoiceNotFoundError`: 일괄 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/file-upload-url

**파일 업로드 URL 생성** (operationId: `createB2bFileUploadUrl`)

S3 파일 업로드를 위한 URL을 생성합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [CreateB2bFileUploadUrlBody](#createb2bfileuploadurlbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fileName` | string | Y | 파일 이름 |

### 응답

- **200** : [CreateB2bFileUploadUrlPayload](#createb2bfileuploadurlpayload)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices-sheet

**세금계산서 엑셀 파일(csv) 다운로드** (operationId: `downloadB2bTaxInvoicesSheet`)

세금계산서를 엑셀 파일(csv)로 다운로드합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [DownloadB2bTaxInvoicesSheetBody](#downloadb2btaxinvoicessheetbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [GetB2bTaxInvoicesBodyFilter](#getb2btaxinvoicesbodyfilter) |  |  |
| `fields` | [TaxInvoicesSheetField](#taxinvoicessheetfield)[] |  | 다운로드 할 시트 컬럼 |
| `test` | boolean |  |  |
| `sort` | [B2bTaxInvoiceSortInput](#b2btaxinvoicesortinput) |  | 정렬 조건 — 미입력 시 상태 업데이트 일시 내림차순 정렬됩니다. |

### 응답

- **200** 
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우

## PUT /b2b/tax-invoices/draft

**세금계산서 임시저장 수정** (operationId: `updateB2bTaxInvoiceDraft`)

임시 저장된 세금계산서를 수정합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [UpdateB2bTaxInvoiceDraftBody](#updateb2btaxinvoicedraftbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string |  | 사업자등록번호 — taxInvoiceKeyType이 TAX_INVOICE_ID가 아닌 경우 필수 값입니다. |
| `taxInvoiceKey` | string | Y | 세금계산서 문서 번호 |
| `taxInvoiceKeyType` | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | 문서 번호 유형 — 기본 값은 RECIPIENT이며 SUPPLIER, RECIPIENT을 지원합니다. |
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 임시저장 수정 정보 |
| `memo` | string |  | 메모 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [UpdateB2bTaxInvoiceDraftResponse](#updateb2btaxinvoicedraftresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`: 세금계산서에 공급자 문서 번호가 이미 사용 중인 경우
  * `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`: 세금계산서에 공급 받는자 문서 번호가 이미 사용 중인 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
  * `B2bModificationNotProvidedError`: 세금계산서 수정 입력 정보를 찾을 수 없는 경우
  * `B2BCannotChangeTaxTypeError`: 세금계산서 과세 유형을 수정할 수 없는 경우
  * `B2BTaxInvoiceStatusNotSendingCompletedError`: 원본 세금계산서가 전송완료 상태가 아닌 경우
  * `B2bTaxInvoiceNotDraftedStatusError`: 세금계산서가 임시저장 완료 상태가 아닌 경우
  * `B2bDocumentKeyCannotBeChangedError`: 문서번호 수정이 요청된 경우
  * `B2bIssuanceTypeMismatchError`: 세금계산서 발행 유형이 올바르지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bSupplierNotFoundError`: 공급자가 존재하지 않은 경우<br>* `B2bRecipientNotFoundError`: 공급받는자가 존재하지 않은 경우<br>* `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bOriginalTaxInvoiceNotFoundError`: 원본 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **409** * `B2bIdAlreadyExistsError`: ID가 이미 사용중인 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/draft

**세금계산서 임시 저장** (operationId: `draftB2bTaxInvoice`)

세금계산서 임시 저장을 요청합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [DraftB2bTaxInvoiceBody](#draftb2btaxinvoicebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |
| `memo` | string |  | 메모 |
| `createRecipientCounterparty` | boolean |  | 공급받는자 거래처 생성 여부 — true인 경우 공급받는자 정보로 거래처를 자동 생성합니다. |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [DraftB2bTaxInvoiceResponse](#draftb2btaxinvoiceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`: 세금계산서에 공급자 문서 번호가 이미 사용 중인 경우
  * `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`: 세금계산서에 공급 받는자 문서 번호가 이미 사용 중인 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
  * `B2bModificationNotProvidedError`: 세금계산서 수정 입력 정보를 찾을 수 없는 경우
  * `B2BCannotChangeTaxTypeError`: 세금계산서 과세 유형을 수정할 수 없는 경우
  * `B2BTaxInvoiceStatusNotSendingCompletedError`: 원본 세금계산서가 전송완료 상태가 아닌 경우
  * `B2bIssuanceTypeMismatchError`: 세금계산서 발행 유형이 올바르지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bSupplierNotFoundError`: 공급자가 존재하지 않은 경우<br>* `B2bRecipientNotFoundError`: 공급받는자가 존재하지 않은 경우<br>* `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bOriginalTaxInvoiceNotFoundError`: 원본 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **409** * `B2bIdAlreadyExistsError`: ID가 이미 사용중인 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/issue-immediately

**세금계산서 즉시 정발행** (operationId: `issueB2bTaxInvoiceImmediately`)

세금계산서를 즉시 정발행합니다. 임시저장 API와 정발행 API 기능을 한 번의 프로세스로 처리합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [IssueB2bTaxInvoiceImmediatelyBody](#issueb2btaxinvoiceimmediatelybody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `memo` | string |  | 메모 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |
| `createRecipientCounterparty` | boolean |  | 공급받는자 거래처 생성 여부 — true인 경우 공급받는자 정보로 거래처를 자동 생성합니다. |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [IssueB2bTaxInvoiceImmediatelyResponse](#issueb2btaxinvoiceimmediatelyresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`: 세금계산서에 공급자 문서 번호가 이미 사용 중인 경우
  * `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`: 세금계산서에 공급 받는자 문서 번호가 이미 사용 중인 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
  * `B2bModificationNotProvidedError`: 세금계산서 수정 입력 정보를 찾을 수 없는 경우
  * `B2BCannotChangeTaxTypeError`: 세금계산서 과세 유형을 수정할 수 없는 경우
  * `B2BTaxInvoiceStatusNotSendingCompletedError`: 원본 세금계산서가 전송완료 상태가 아닌 경우
  * `B2bIssuanceTypeMismatchError`: 세금계산서 발행 유형이 올바르지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bSupplierNotFoundError`: 공급자가 존재하지 않은 경우<br>* `B2bRecipientNotFoundError`: 공급받는자가 존재하지 않은 경우<br>* `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bOriginalTaxInvoiceNotFoundError`: 원본 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **409** * `B2bIdAlreadyExistsError`: ID가 이미 사용중인 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/request-reverse-issuance

**세금계산서 역발행 즉시 요청** (operationId: `requestB2bTaxInvoiceReverseIssuance`)

공급자에게 세금계산서 역발행을 즉시 요청합니다. 임시저장 API와 역발행 요청 API 기능을 한 번의 프로세스로 처리합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [RequestB2bTaxInvoiceReverseIssuanceBody](#requestb2btaxinvoicereverseissuancebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `memo` | string |  | 메모 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [RequestB2bTaxInvoiceReverseIssuanceResponse](#requestb2btaxinvoicereverseissuanceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`: 세금계산서에 공급자 문서 번호가 이미 사용 중인 경우
  * `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`: 세금계산서에 공급 받는자 문서 번호가 이미 사용 중인 경우
  * `B2bCounterpartyNtsNotConnectedError`: 국세청에 연동되어 있지 않은 경우
  * `B2bModificationNotProvidedError`: 세금계산서 수정 입력 정보를 찾을 수 없는 경우
  * `B2BCannotChangeTaxTypeError`: 세금계산서 과세 유형을 수정할 수 없는 경우
  * `B2BTaxInvoiceStatusNotSendingCompletedError`: 원본 세금계산서가 전송완료 상태가 아닌 경우
  * `B2bIssuanceTypeMismatchError`: 세금계산서 발행 유형이 올바르지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bSupplierNotFoundError`: 공급자가 존재하지 않은 경우<br>* `B2bRecipientNotFoundError`: 공급받는자가 존재하지 않은 경우<br>* `B2bCounterpartyNotFoundError`: 거래처가 존재하지 않는 경우<br>* `B2bOriginalTaxInvoiceNotFoundError`: 원본 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **409** * `B2bIdAlreadyExistsError`: ID가 이미 사용중인 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/attach-file

**세금계산서 파일 첨부** (operationId: `attachB2bTaxInvoiceFile`)

세금계산서에 파일을 첨부합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [AttachB2bTaxInvoiceFileBody](#attachb2btaxinvoicefilebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fileId` | string | Y | 파일 아이디 |

### 응답

- **200** 
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotDraftedStatusError`: 세금계산서가 임시저장 완료 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우<br>* `B2bFileNotFoundError`: 업로드한 파일을 찾을 수 없는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## DELETE /b2b/tax-invoices/{taxInvoiceKey}/attachments/{attachmentId}

**세금계산서 첨부파일 삭제** (operationId: `deleteB2bTaxInvoiceAttachment`)

세금계산서 첨부파일을 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `attachmentId` | path | string | Y | 첨부파일 아이디 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** 
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotDraftedStatusError`: 세금계산서가 임시저장 완료 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceAttachmentNotFoundError`: 세금계산서의 첨부파일을 찾을 수 없는 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices/{taxInvoiceKey}/attachments

**세금계산서 첨부파일 목록 조회** (operationId: `getB2bTaxInvoiceAttachments`)

세금계산서에 첨부된 파일 목록을 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [GetB2bTaxInvoiceAttachmentsResponse](#getb2btaxinvoiceattachmentsresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-issuance

**세금계산서 취소 (공급자에 의한 취소)** (operationId: `cancelB2bTaxInvoiceIssuance`)

발행 완료된 세금계산서를 공급자가 국세청 전송 전에 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [CancelB2bTaxInvoiceIssuanceBody](#cancelb2btaxinvoiceissuancebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [CancelB2bTaxInvoiceIssuanceResponse](#cancelb2btaxinvoiceissuanceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotIssuedStatusError`: 세금계산서가 발행된(ISSUED) 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/cancel-request

**세금계산서 역발행 요청 취소 (공급받는자에 의한 취소)** (operationId: `cancelB2bTaxInvoiceRequest`)

공급자가 세금계산서 발행을 승인하기 전에 공급받는자가 해당 역발행 요청을 취소합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [CancelB2bTaxInvoiceRequestBody](#cancelb2btaxinvoicerequestbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [CancelB2bTaxInvoiceRequestResponse](#cancelb2btaxinvoicerequestresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotRequestedStatusError`: 세금계산서가 역발행 대기 상태가 아닌 경우
  * `B2bTaxInvoiceNoRecipientDocumentKeyError`: 세금계산서에 공급받는자 문서 번호가 기입되지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/issue

**세금계산서 발행 승인** (operationId: `issueB2bTaxInvoice`)

역발행의 경우 역발행요청(REQUESTED) 상태, 정발행의 경우 임시저장(DRAFTED) 상태의 세금계산서에 대해 발행을 승인합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [IssueB2bTaxInvoiceBody](#issueb2btaxinvoicebody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |
| `emailSubject` | string |  | 이메일 제목 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [IssueB2bTaxInvoiceResponse](#issueb2btaxinvoiceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotRequestedStatusError`: 세금계산서가 역발행 대기 상태가 아닌 경우
  * `B2bTaxInvoiceNotDraftedStatusError`: 세금계산서가 임시저장 완료 상태가 아닌 경우
  * `B2bTaxInvoiceNoSupplierDocumentKeyError`: 세금계산서에 공급자 문서 번호가 기입되지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices/{taxInvoiceKey}/pdf-download-url

**세금 계산서 PDF 다운로드 URL 조회** (operationId: `getB2bTaxInvoicePdfDownloadUrl`)

등록된 세금 계산서 PDF 다운로드 URL을 공급자 혹은 공급받는자 문서번호로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [GetB2bTaxInvoicePdfDownloadUrlResponse](#getb2btaxinvoicepdfdownloadurlresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices/{taxInvoiceKey}/popup-url

**세금 계산서 팝업 URL 조회** (operationId: `getB2bTaxInvoicePopupUrl`)

등록된 세금 계산서 팝업 URL을 공급자 혹은 공급받는자 문서번호로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `includeMenu` | query | boolean |  | 팝업 URL에 메뉴 레이아웃을 포함 여부를 결정합니다. 기본 값은 true입니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [GetB2bTaxInvoicePopupUrlResponse](#getb2btaxinvoicepopupurlresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices/{taxInvoiceKey}/print-url

**세금 계산서 프린트 URL 조회** (operationId: `getB2bTaxInvoicePrintUrl`)

등록된 세금 계산서 프린트 URL을 공급자 혹은 공급받는자 문서번호로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [GetB2bTaxInvoicePrintUrlResponse](#getb2btaxinvoiceprinturlresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/refuse-request

**세금계산서 역발행 요청 거부** (operationId: `refuseB2bTaxInvoiceRequest`)

공급자가 공급받는자로부터 요청받은 세금계산서 역발행 건을 거부합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 요청 본문 (application/json, 필수)

타입: [RefuseB2bTaxInvoiceRequestBody](#refuseb2btaxinvoicerequestbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [RefuseB2bTaxInvoiceRequestResponse](#refuseb2btaxinvoicerequestresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotRequestedStatusError`: 세금계산서가 역발행 대기 상태가 아닌 경우
  * `B2bTaxInvoiceNoSupplierDocumentKeyError`: 세금계산서에 공급자 문서 번호가 기입되지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/request

**세금계산서 역발행 요청** (operationId: `requestB2bTaxInvoice`)

임시저장(REGISTERED) 상태의 역발행 세금계산서를 공급자에게 발행 요청합니다. 요청이 완료되면 (역)발행대기 상태로 전환됩니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [RequestB2bTaxInvoiceResponse](#requestb2btaxinvoiceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bModificationNotProvidedError`: 세금계산서 수정 입력 정보를 찾을 수 없는 경우
  * `B2BCannotChangeTaxTypeError`: 세금계산서 과세 유형을 수정할 수 없는 경우
  * `B2BTaxInvoiceStatusNotSendingCompletedError`: 원본 세금계산서가 전송완료 상태가 아닌 경우
  * `B2bTaxInvoiceNotDraftedStatusError`: 세금계산서가 임시저장 완료 상태가 아닌 경우
  * `B2bIssuanceTypeMismatchError`: 세금계산서 발행 유형이 올바르지 않은 경우
  * `B2bTaxInvoiceNoRecipientDocumentKeyError`: 세금계산서에 공급받는자 문서 번호가 기입되지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bOriginalTaxInvoiceNotFoundError`: 원본 세금계산서가 존재하지 않은 경우<br>* `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## POST /b2b/tax-invoices/{taxInvoiceKey}/send-to-nts

**세금계산서 국세청 즉시 전송** (operationId: `sendToNtsB2bTaxInvoice`)

발행이 완료된 세금계산서를 국세청에 즉시 전송합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [SendToNtsB2bTaxInvoiceResponse](#sendtontsb2btaxinvoiceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNotIssuedStatusError`: 세금계산서가 발행된(ISSUED) 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices/{taxInvoiceKey}

**세금 계산서 조회** (operationId: `getB2bTaxInvoice`)

등록된 세금 계산서를 세금계산서 아이디로 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** 성공 응답으로 세금계산서를 반환합니다.: [B2bTaxInvoice](#b2btaxinvoice)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## DELETE /b2b/tax-invoices/{taxInvoiceKey}

**세금계산서 삭제** (operationId: `deleteB2bTaxInvoice`)

세금계산서를 삭제합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `taxInvoiceKey` | path | string | Y | 세금계산서 문서 번호 |
| `brn` | query | string |  | 사업자등록번호 |
| `taxInvoiceKeyType` | query | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | query 파라미터로 전달된 문서번호 유형. 기본 값은 TAX_INVOICE_ID이며 SUPPLIER, RECIPIENT, TAX_INVOICE_ID을 지원합니다. |
| `test` | query | boolean |  | true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |

### 응답

- **200** : [DeleteB2bTaxInvoiceResponse](#deleteb2btaxinvoiceresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
  * `B2bTaxInvoiceNonDeletableStatusError`: 세금계산서가 삭제 가능한 상태가 아닌 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우<br>* `B2bBulkTaxInvoiceNotFoundError`: 일괄 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

## GET /b2b/tax-invoices

**세금 계산서 다건조회** (operationId: `getB2bTaxInvoices`)

조회 기간 내 등록된 세금 계산서를 다건 조회합니다.

인증: `Authorization: Bearer <access token>` 또는 `Authorization: PortOne <API secret>`

### 파라미터

| 이름 | 위치 | 타입 | 필수 | 설명 |
|---|---|---|---|---|
| `requestBody` | query |  |  |  |

### 요청 본문 (application/json)

타입: [GetB2bTaxInvoicesBody](#getb2btaxinvoicesbody)

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `test` | boolean |  | 테스트 모드 여부 — true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `pageNumber` | integer(int32) |  | 페이지 번호 — 0부터 시작하는 페이지 번호. 기본 값은 0. |
| `pageSize` | integer(int32) |  | 페이지 크기 — 각 페이지 당 포함할 객체 수. 기본 값은 500이며 최대 1000까지 요청가능합니다. |
| `filter` | [GetB2bTaxInvoicesBodyFilter](#getb2btaxinvoicesbodyfilter) |  | 필터 |
| `sort` | [B2bTaxInvoiceSortInput](#b2btaxinvoicesortinput) |  | 정렬 조건 — 미입력 시 상태 업데이트 일시 내림차순 정렬됩니다. |

### 응답

- **200** : [GetB2bTaxInvoicesResponse](#getb2btaxinvoicesresponse)
- **400**
  * [`InvalidRequestError`](errors.md#invalidrequesterror): 요청된 입력 정보가 유효하지 않은 경우
- **401**
  * [`UnauthorizedError`](errors.md#unauthorizederror): 인증 정보가 올바르지 않은 경우
- **403**
  * `B2bNotEnabledError`: B2B 기능이 활성화되지 않은 경우
  * [`ForbiddenError`](errors.md#forbiddenerror): 요청이 거절된 경우
- **404** * `B2bTaxInvoiceNotFoundError`: 세금계산서가 존재하지 않은 경우
- **502** * `B2bExternalServiceError`: 외부 서비스에서 에러가 발생한 경우

---

# 스키마

### AttachB2bTaxInvoiceFileBody

세금계산서 파일 첨부 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fileId` | string | Y | 파일 아이디 |

### B2bBulkTaxInvoice

B2bBulkTaxInvoice

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 일괄 세금계산서 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string |  |  |
| `status` | [B2bBulkTaxInvoiceStatus](#b2bbulktaxinvoicestatus) | Y |  |
| `totalInvoiceCount` | integer(int32) | Y |  |
| `totalAmount` | integer(int64) | Y |  |
| `stats` | [Map_Stat](#map_stat) | Y |  |
| `createdAt` | string(date-time) | Y |  |
| `scheduledAt` | string(date-time) |  |  |
| `requestedAt` | string(date-time) |  |  |
| `statusUpdatedAt` | string(date-time) | Y |  |
| `sourceType` | [B2bBulkTaxInvoiceSourceType](#b2bbulktaxinvoicesourcetype) | Y |  |
| `issuanceType` | [B2bTaxInvoiceIssuanceType](#b2btaxinvoiceissuancetype) | Y |  |

### B2bBulkTaxInvoiceSourceType

그룹 생성 방식

- `SHEET` — 엑셀 업로드
- `PLATFORM` — 지급 데이터 기반 생성

### B2bBulkTaxInvoiceStat

세금계산서 상태별 집계 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `count` | integer(int32) | Y |  |
| `amountSum` | integer(int64) | Y |  |

### B2bBulkTaxInvoiceStatus

일괄 세금계산서 상태

- `DRAFT_PENDING`
- `DRAFTED`
- `REQUEST_PENDING`
- `ISSUE_PENDING`
- `IN_PROGRESS`
- `COMPLETED`
- `FAILED`
- `CANCELLED`

### B2bCompanyStateBusinessStatus

영업 상태

- `IN_BUSINESS` — 영업중
- `CLOSED` — 폐업
- `SUSPENDED` — 휴업

### B2bSearchDateType

조회 기준

- `REGISTER` — 등록일
- `WRITE` — 작성일
- `ISSUE` — 발행일

### B2bTaxInvoice

세금계산서

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 세금계산서 아이디 |
| `status` | [B2bTaxInvoiceStatus](#b2btaxinvoicestatus) | Y | 상태 |
| `taxationType` | [B2bTaxInvoiceTaxationType](#b2btaxinvoicetaxationtype) | Y | 과세 유형 |
| `documentModificationType` | [B2bTaxInvoiceDocumentModificationType](#b2btaxinvoicedocumentmodificationtype) | Y | 문서 유형 |
| `isDelayed` | boolean |  | 지연 발행 여부 |
| `issuanceType` | [B2bTaxInvoiceIssuanceType](#b2btaxinvoiceissuancetype) | Y | 발행 유형 |
| `bulkTaxInvoiceId` | string |  | 일괄 발행 아이디 |
| `serialNumber` | string |  | 일련번호 |
| `bookVolume` | integer(int32) |  | 책번호 - 권 — 입력 범위(4자리) : 0 ~ 9999 |
| `bookIssue` | integer(int32) |  | 책번호 - 호 — 입력 범위(4자리) : 0 ~ 9999 |
| `writeDate` | string(date) | Y | 작성일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `issuanceDueDate` | string(date) | Y | 발행 마감일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `purposeType` | [B2bTaxInvoicePurposeType](#b2btaxinvoicepurposetype) | Y | 영수/청구 |
| `totalSupplyAmount` | integer(int64) | Y | 공급가액 합계 |
| `totalTaxAmount` | integer(int64) | Y | 세액 합계 |
| `totalAmount` | integer(int64) | Y | 합계 금액 |
| `cashAmount` | integer(int64) |  | 현금 |
| `checkAmount` | integer(int64) |  | 수표 |
| `creditAmount` | integer(int64) |  | 외상 |
| `noteAmount` | integer(int64) |  | 어음 |
| `remarks` | string[] | Y | 비고 — 최대 3개 |
| `supplierDocumentKey` | string |  | 공급자 문서번호 |
| `supplier` | [B2bTaxInvoiceCompany](#b2btaxinvoicecompany) | Y | 공급자 |
| `recipientDocumentKey` | string |  | 공급받는자 문서번호 |
| `recipient` | [B2bTaxInvoiceCompany](#b2btaxinvoicecompany) | Y | 공급받는자 |
| `sendSms` | boolean |  | 문자 전송 여부 |
| `modification` | [B2bTaxInvoiceModification](#b2btaxinvoicemodification) |  | 수정 사유 기재 |
| `items` | [B2bTaxInvoiceItem](#b2btaxinvoiceitem)[] | Y | 품목 — 최대 99개 |
| `additionalContacts` | [B2bTaxInvoiceAdditionalContact](#b2btaxinvoiceadditionalcontact)[] | Y | 추가 담당자 — 최대 5명 |
| `memo` | string |  | 메모 |
| `draftedAt` | string(date-time) |  | 임시 저장 일시 |
| `requestedAt` | string(date-time) |  | 발행 요청 일시 |
| `issuedAt` | string(date-time) |  | 발행 일시 |
| `statusUpdatedAt` | string(date-time) |  | 상태 변경 일시 |
| `ntsSentAt` | string(date-time) |  | 국세청 전송 일시 |
| `ntsApprovalNumber` | string |  | 국세청 승인번호 — 세금계산서 발행(전자서명) 시점에 자동으로 부여 |
| `ntsResult` | string |  | 국세청 전송 결과 |
| `ntsResultCode` | string |  | 국세청 결과 코드 — 국세청 발급 결과 코드로 영문 3자리 + 숫자 3자리로 구성됨 |
| `ntsResultReceivedAt` | string(date-time) |  | 국세청 결과 수신 일시 |
| `deletedAt` | string(date-time) |  | 삭제 일시 |

### B2bTaxInvoiceAdditionalContact

추가 담당자

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 성명 — 최대 100자 |
| `email` | string | Y | 이메일 |
| `phoneNumber` | string |  | 전화번호 |

### B2bTaxInvoiceAttachment

세금계산서 첨부파일

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 첨부 파일 아이디 |
| `name` | string | Y | 첨부 파일명 |
| `attachedAt` | string(date-time) | Y | 첨부 일시 |

### B2bTaxInvoiceCompany

B2bTaxInvoiceCompany

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string |  | 사업자등록번호 — `-`를 제외한 10자리 |
| `counterpartyId` | string |  | 거래처 ID |
| `taxRegistrationId` | string |  | 종사업자 식별 번호 — 4자리 고정 |
| `name` | string |  | 상호명 — 최대 200자 |
| `representativeName` | string |  | 대표자 성명 — 최대 100자 |
| `address` | string |  | 주소 — 최대 300자 |
| `businessType` | string |  | 업태 — 최대 100자 |
| `businessClass` | string |  | 종목 — 최대 100자 |
| `contact` | [B2bTaxInvoiceContact](#b2btaxinvoicecontact) |  | 담당자 |

### B2bTaxInvoiceContact

세금계산서 담당자

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string |  | 성명 |
| `department` | string |  | 부서 |
| `phoneNumber` | string |  | 전화번호 |
| `mobilePhoneNumber` | string |  | 휴대전화번호 |
| `email` | string | Y | 이메일 |

### B2bTaxInvoiceDocumentModificationType

세금계산서 문서 수정 발행 유형

- `NORMAL` — 정상발행
- `MODIFICATION` — 수정발행

### B2bTaxInvoiceInput

세금계산서 생성 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxationType` | [B2bTaxInvoiceTaxationType](#b2btaxinvoicetaxationtype) | Y | 과세 유형 |
| `issuanceType` | [B2bTaxInvoiceIssuanceType](#b2btaxinvoiceissuancetype) |  | 발행 유형 — 기본값은 역발행(REVERSE)입니다. |
| `serialNumber` | string |  | 일련번호 |
| `bookVolume` | integer(int32) |  | 권 |
| `bookIssue` | integer(int32) |  | 호 |
| `writeDate` | string(date) | Y | 작성일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `purposeType` | [B2bTaxInvoicePurposeType](#b2btaxinvoicepurposetype) | Y | 영수/청구 |
| `totalSupplyAmount` | integer(int64) | Y | 공급가액 합계 |
| `totalTaxAmount` | integer(int64) | Y | 세액 합계 |
| `totalAmount` | integer(int64) | Y | 합계 금액 |
| `cashAmount` | integer(int64) |  | 현금 |
| `checkAmount` | integer(int64) |  | 수표 |
| `creditAmount` | integer(int64) |  | 외상 |
| `noteAmount` | integer(int64) |  | 어음 |
| `remarks` | string[] |  | 비고 — 최대 3개 |
| `supplierDocumentKey` | string |  | 공급자 문서번호 — 영문 대소문자, 숫자, 특수문자('-','_')만 이용 가능 |
| `supplier` | [B2bTaxInvoiceCompany](#b2btaxinvoicecompany) |  | 공급자 |
| `recipientDocumentKey` | string |  | 공급받는자 문서번호 — 영문 대소문자, 숫자, 특수문자('-','_')만 이용 가능 |
| `recipient` | [B2bTaxInvoiceCompany](#b2btaxinvoicecompany) | Y | 공급받는자 |
| `sendSms` | boolean |  | 문자 전송 여부 — 공급자 담당자 휴대폰번호 {supplier.contact.mobile_phone_number} 값으로 문자 전송 전송시 포인트 차감되며, 실패시 환불 처리 기본값은 false |
| `items` | [B2bTaxInvoiceItem](#b2btaxinvoiceitem)[] |  | 품목 — 최대 99개 |
| `additionalContacts` | [B2bTaxInvoiceAdditionalContact](#b2btaxinvoiceadditionalcontact)[] |  | 추가 담당자 — 최대 5명 |
| `useCounterpartyAdditionalContacts` | boolean |  | 거래처 추가 담당자 사용 여부 — true인 경우 거래처의 추가 담당자를 세금계산서 추가 담당자로 사용합니다. additional_contacts가 직접 지정된 경우 무시됩니다. |

### B2bTaxInvoiceIssuanceType

발행 유형

- `NORMAL` — 정발행
- `REVERSE` — 역발행

### B2bTaxInvoiceItem

품목

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `purchaseDate` | string(date) |  | 결제일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `name` | string |  | 품명 — 최대 100자 |
| `spec` | string |  | 규격 — 최대 100자 |
| `quantity` | [Decimal](#decimal) |  | 수량 — 입력 범위 : -99999999.99 ~ 999999999.99<br>`quantity.scale`의 입력 범위 : 0 ~ 2<br>`quantity.value` * 10^-`quantity.scale` 단위로 치환됩니다. |
| `unitCostAmount` | [Decimal](#decimal) |  | 단가 — 입력 범위 : -99999999999999.99 ~ 999999999999999.99<br>`unitCostAmount.scale`의 입력 범위 : 0 ~ 2<br>`unitCostAmount.value` * 10^-`unitCostAmount.scale` 단위로 치환됩니다. |
| `supplyCostAmount` | integer(int64) |  | 공급가액 |
| `taxAmount` | integer(int64) |  | 세액 |
| `remark` | string |  | 비고 |

### B2bTaxInvoiceKeyType

세금계산서 식별자 유형

- `SUPPLIER` — 공급자용 문서 번호
- `RECIPIENT` — 공급받는자용 문서 번호
- `TAX_INVOICE_ID` — 세금계산서 아이디

### B2bTaxInvoiceModification

세금 계산서 수정

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [B2bTaxInvoiceModificationType](#b2btaxinvoicemodificationtype) | Y | 수정 사유 |
| `originalNtsApprovalNumber` | string | Y | 수정 대상 원본 세금계산서 국세청 승인 번호 |
| `originalTaxInvoiceId` | string | Y | 원본 세금계산서 아이디 |
| `rootTaxInvoiceId` | string | Y | 최초 원본 세금계산서 아이디 |

### B2bTaxInvoiceModificationCreateBody

수정 세금계산서 생성 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [B2bTaxInvoiceModificationType](#b2btaxinvoicemodificationtype) | Y | 수정 사유 |
| `brn` | string |  | 사업자등록번호 — taxInvoiceKeyType이 TAX_INVOICE_ID가 아닌 경우 필수 값입니다. |
| `taxInvoiceKey` | string | Y | 세금계산서 문서 번호 |
| `taxInvoiceKeyType` | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | 문서 번호 유형 — 기본 값은 RECIPIENT이며 SUPPLIER, RECIPIENT을 지원합니다. |

### B2bTaxInvoiceModificationType

수정 사유

- `CORRECTION_OF_ENTRY_ERRORS` — 기재사항 착오 정정
- `CHANGE_IN_SUPPLY_COST` — 공금가액 변동
- `RETURN` — 환입
- `CANCELLATION_OF_CONTRACT` — 계약 해제
- `DUPLICATE_ISSUANCE_DUE_TO_ERROR` — 착오에 의한 이중 발급

### B2bTaxInvoicePurposeType

영수/청구

- `RECEIPT` — 영수
- `INVOICE` — 청구
- `NONE` — 없음

### B2bTaxInvoiceSortBy

세금계산서 정렬 기준

- `WRITE_DATE` — 작성일자
- `ISSUANCE_DUE_DATE` — 발행마감일
- `TOTAL_AMOUNT` — 합계금액
- `TOTAL_SUPPLY_AMOUNT` — 공급가액
- `TOTAL_TAX_AMOUNT` — 세액
- `REQUESTED_AT` — 발행요청일시
- `ISSUED_AT` — 발행완료일시
- `NTS_SENT_AT` — 국세청전송일시
- `STATUS_UPDATED_AT` — 상태 업데이트 일시

### B2bTaxInvoiceSortInput

세금계산서 다건 조회 시 정렬 조건

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `by` | [B2bTaxInvoiceSortBy](#b2btaxinvoicesortby) |  | 정렬 기준 |
| `order` | [SortOrder](schemas-common.md#sortorder) |  | 정렬 방식 |

### B2bTaxInvoiceStatus

B2bTaxInvoiceStatus

- `DRAFTED` — 임시저장
- `DRAFT_PENDING` — 임시저장 대기
- `DRAFT_FAILED` — 임시저장 실패
- `REQUESTED` — 역발행 요청 완료 (전자 서명 요청됨)
- `REQUEST_PENDING` — 역발행 요청 대기
- `REQUEST_FAILED` — 역발행 요청 실패
- `REQUEST_CANCELLED` — 공급받는자에 의한 발행취소
- `ISSUED` — 발행완료
- `ISSUE_PENDING` — 발행 대기
- `ISSUE_FAILED` — 발행 실패
- `BEFORE_SENDING` — 전송전
- `WAITING_SENDING` — 전송대기
- `SENDING` — 전송중
- `SENDING_COMPLETED` — 전송완료
- `SENDING_FAILED` — 전송실패
- `REQUEST_REFUSED` — 공급자의 발행거부
- `ISSUANCE_CANCELLED` — 공급자에 의한 발행 취소
- `CANCEL_PENDING` — 발행취소 대기
- `SENDING_PENDING` — 국세청 전송 대기

### B2bTaxInvoiceSummary

세금계산서 요약

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 세금계산서 아이디 |
| `taxationType` | [B2bTaxInvoiceTaxationType](#b2btaxinvoicetaxationtype) | Y | 과세 유형 |
| `documentModificationType` | [B2bTaxInvoiceDocumentModificationType](#b2btaxinvoicedocumentmodificationtype) | Y | 문서 유형 |
| `isDelayed` | boolean |  | 지연 발행 여부 |
| `issuanceType` | [B2bTaxInvoiceIssuanceType](#b2btaxinvoiceissuancetype) | Y | 발행 유형 |
| `bulkTaxInvoiceId` | string |  | 일괄 발행 아이디 |
| `payoutId` | string |  | 지급 아이디 |
| `totalSupplyAmount` | integer(int64) | Y | 공급가액 합계 |
| `totalTaxAmount` | integer(int64) | Y | 세액 합계 |
| `totalAmount` | integer(int64) | Y | 합계 금액 |
| `purposeType` | [B2bTaxInvoicePurposeType](#b2btaxinvoicepurposetype) | Y | 영수/청구 |
| `supplierBrn` | string | Y | 공급자 사업자등록번호 |
| `supplierName` | string | Y | 공급자 상호 |
| `supplierRepresentativeName` | string | Y | 공급자 대표자 성명 |
| `supplierDocumentKey` | string |  | 공급자 문서번호 |
| `supplierCounterpartyId` | string |  | 공급자 거래처 아이디 |
| `recipientBrn` | string | Y | 공급받는자 사업자등록번호 |
| `recipientName` | string | Y | 공급받는자 상호 |
| `recipientRepresentativeName` | string | Y | 공급받는자 대표자 성명 |
| `recipientDocumentKey` | string |  | 공급받는자 문서번호 |
| `recipientCounterpartyId` | string |  | 공급받는자 거래처 아이디 |
| `recipientBusinessStatus` | [B2bCompanyStateBusinessStatus](#b2bcompanystatebusinessstatus) |  | 공급받는자 영업 상태 |
| `recipientClosedSuspendedDate` | string(date) |  | 공급받는자 휴폐업일자 — 상태가 CLOSED, SUSPENDED 상태인 경우에만 결과값 반환 |
| `writeDate` | string(date) | Y | 작성일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `issuanceDueDate` | string(date) | Y | 발행 마감일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `status` | [B2bTaxInvoiceStatus](#b2btaxinvoicestatus) | Y | 상태 |
| `draftedAt` | string(date-time) |  | 임시 저장 일시 |
| `requestedAt` | string(date-time) |  | 발행 요청 일시 |
| `issuedAt` | string(date-time) |  | 발행 일시 |
| `openedAt` | string(date-time) |  | 개봉 일시 |
| `statusUpdatedAt` | string(date-time) | Y | 상태 변경 일시 |
| `ntsSentAt` | string(date-time) |  | 국세청 전송 일시 |
| `ntsApprovalNumber` | string |  | 국세청 승인번호 — 세금계산서 발행(전자서명) 시점에 자동으로 부여 |
| `ntsResult` | string |  | 국세청 전송 결과 |
| `ntsResultReceivedAt` | string(date-time) |  | 국세청 결과 수신 일시 |
| `ntsResultCode` | string |  | 국세청 결과 코드 — 국세청 발급 결과 코드로 영문 3자리 + 숫자 3자리로 구성됨 |
| `memo` | string |  | 메모 |
| `items` | [B2bTaxInvoiceItem](#b2btaxinvoiceitem)[] | Y | 품목 — 최대 99개 |

### B2bTaxInvoiceTaxationType

과세 유형

- `TAXABLE` — 과세
- `ZERO_RATED` — 영세
- `FREE` — 면세

### CancelB2bTaxInvoiceIssuanceBody

세금계산서 취소 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### CancelB2bTaxInvoiceIssuanceResponse

세금계산서 취소 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### CancelB2bTaxInvoiceRequestBody

세금계산서 역발행 요청 취소 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### CancelB2bTaxInvoiceRequestResponse

세금계산서 역발행 요청 취소 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### CreateB2bFileUploadUrlBody

파일 업로드 URL 생성 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fileName` | string | Y | 파일 이름 |

### CreateB2bFileUploadUrlPayload

파일 업로드 URL 생성 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `fileId` | string | Y | 파일 아이디 |
| `url` | string | Y | 파일 업로드 URL |

### DateRangeOption

DateRangeOption

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `from` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `until` | string(date) |  | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### Decimal

BigDecimal 타입

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `value` | integer(int64) | Y | 비정규화된 값 — 소수점 숫자의 비정규화된(unscaled) 값을 정수로 저장합니다. 예를 들어, 123.45의 경우 12345가 저장됩니다. |
| `scale` | integer(int32) |  | 소수점 이하 자릿수 — 소수점 숫자의 소수점 이하 자릿수를 저장합니다. 기본값은 0입니다. 예를 들어, 123.45의 경우 2가 저장됩니다. |

### DeleteB2bTaxInvoiceResponse

DeleteB2bTaxInvoiceResponse

type: object

### DownloadB2bTaxInvoicesSheetBody

DownloadB2bTaxInvoicesSheetBody

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `filter` | [GetB2bTaxInvoicesBodyFilter](#getb2btaxinvoicesbodyfilter) |  |  |
| `fields` | [TaxInvoicesSheetField](#taxinvoicessheetfield)[] |  | 다운로드 할 시트 컬럼 |
| `test` | boolean |  |  |
| `sort` | [B2bTaxInvoiceSortInput](#b2btaxinvoicesortinput) |  | 정렬 조건 — 미입력 시 상태 업데이트 일시 내림차순 정렬됩니다. |

### DraftB2bTaxInvoiceBody

세금계산서 임시 저장 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |
| `memo` | string |  | 메모 |
| `createRecipientCounterparty` | boolean |  | 공급받는자 거래처 생성 여부 — true인 경우 공급받는자 정보로 거래처를 자동 생성합니다. |

### DraftB2bTaxInvoiceResponse

세금계산서 임시 저장 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### GetB2bTaxInvoiceAttachmentsResponse

세금계산서 첨부파일 목록 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `attachments` | [B2bTaxInvoiceAttachment](#b2btaxinvoiceattachment)[] | Y | 첨부파일 목록 |

### GetB2bTaxInvoicePdfDownloadUrlResponse

세금계산서 PDF 다운로드 URL 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `url` | string | Y | 세금계산서 PDF 다운로드 URL |

### GetB2bTaxInvoicePopupUrlResponse

세금계산서 팝업 URL 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `url` | string | Y | 세금계산서 팝업 URL |

### GetB2bTaxInvoicePrintUrlResponse

세금계산서 프린트 URL 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `url` | string | Y | 세금계산서 프린트 URL |

### GetB2bTaxInvoicesBody

세금 계산서 다건 조회를 위한 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `test` | boolean |  | 테스트 모드 여부 — true 이면 테스트 모드로 실행되며, false 이거나 주어지지 않은 경우 테스트 모드를 사용하지 않습니다. |
| `pageNumber` | integer(int32) |  | 페이지 번호 — 0부터 시작하는 페이지 번호. 기본 값은 0. |
| `pageSize` | integer(int32) |  | 페이지 크기 — 각 페이지 당 포함할 객체 수. 기본 값은 500이며 최대 1000까지 요청가능합니다. |
| `filter` | [GetB2bTaxInvoicesBodyFilter](#getb2btaxinvoicesbodyfilter) |  | 필터 |
| `sort` | [B2bTaxInvoiceSortInput](#b2btaxinvoicesortinput) |  | 정렬 조건 — 미입력 시 상태 업데이트 일시 내림차순 정렬됩니다. |

### GetB2bTaxInvoicesBodyDateFilter

조회 기간 필터

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dateType` | [B2bSearchDateType](#b2bsearchdatetype) |  | 조회 기간 기준 — 미입력시 기본값은 등록일(`REGISTER`)로 설정됩니다. |
| `dateRange` | [DateRangeOption](#daterangeoption)[] |  | 조회 기간 — 미입력시 `dateRange.from`의 기본값은 한 달 이전, `dateRange.until`의 기본값은 현재 일자로 설정됩니다. |

### GetB2bTaxInvoicesBodyFilter

세금계산서 다건 조회 필터

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `primaryFilter` | [GetB2bTaxInvoicesBodyPrimaryFilter](#getb2btaxinvoicesbodyprimaryfilter) |  | 상위 필터 — 가장 주요 항목을 설정하는 상위 필터이며 사용할 때는 주어진 필드 중 한 개의 필드만 입력합니다. |
| `supplierBrn` | string |  | 공급자 사업자등록번호 |
| `partnerBrn` | string |  | 거래처 사업자등록번호 — 역발행의 경우 공급자 사업자등록번호, 정발행의 경우 공급받는자 사업자등록번호에 대해 조회합니다. |
| `statuses` | [B2bTaxInvoiceStatus](#b2btaxinvoicestatus)[] |  | 세금계산서 상태 — 미입력시 모든 상태를 조회합니다. |
| `taxationTypes` | [B2bTaxInvoiceTaxationType](#b2btaxinvoicetaxationtype)[] |  | 과세 유형 |
| `documentModificationTypes` | [B2bTaxInvoiceDocumentModificationType](#b2btaxinvoicedocumentmodificationtype)[] |  | 문서 유형 |
| `isDelayed` | boolean |  | 지연 발행 여부 |
| `issuanceTypes` | [B2bTaxInvoiceIssuanceType](#b2btaxinvoiceissuancetype)[] |  | 발행 유형 |
| `purposeTypes` | [B2bTaxInvoicePurposeType](#b2btaxinvoicepurposetype)[] |  | 영수/청구 |
| `counterpartyId` | string |  | 거래처 ID — 거래처 ID로 필터링합니다. 공급자 또는 공급받는자 거래처 ID에 대해 조회합니다. |
| `includeBrnMatch` | boolean |  | 거래처 사업자등록번호 매칭 포함 여부 — counterpartyId 조건과 함께 거래처의 사업자등록번호 매칭 조건을 추가할지 여부입니다. |

### GetB2bTaxInvoicesBodyPrimaryFilter

상위 필터 — 상위 필터<br>주어진 필드 중 한 개의 필드만 입력합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `dateFilter` | [GetB2bTaxInvoicesBodyDateFilter](#getb2btaxinvoicesbodydatefilter) |  | 조회 기간 |
| `taxInvoiceId` | string |  | 세금계산서 아이디 |
| `bulkTaxInvoiceId` | string |  | 일괄발행 아이디 |
| `ntsApprovalNumber` | string |  | 국세청 승인번호 |
| `supplierDocumentKey` | string |  | 공급자 문서번호 |
| `recipientDocumentKey` | string |  | 공급받는자 승인번호 |
| `taxInvoiceIds` | string[] |  | 세금계산서 아이디 리스트 |
| `payoutId` | string |  | 지급 아이디 |

### GetB2bTaxInvoicesResponse

세금계산서 다건 조회 성공 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `items` | [B2bTaxInvoiceSummary](#b2btaxinvoicesummary)[] | Y | 조회된 세금계산서 목록 |
| `page` | [PageInfo](schemas-common.md#pageinfo) | Y | 조회된 페이지 정보 |

### IssueB2bTaxInvoiceBody

세금계산서 발행 승인 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |
| `emailSubject` | string |  | 이메일 제목 |

### IssueB2bTaxInvoiceImmediatelyBody

세금계산서 즉시 정발행 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `memo` | string |  | 메모 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |
| `createRecipientCounterparty` | boolean |  | 공급받는자 거래처 생성 여부 — true인 경우 공급받는자 정보로 거래처를 자동 생성합니다. |

### IssueB2bTaxInvoiceImmediatelyResponse

세금계산서 즉시 정발행 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### IssueB2bTaxInvoiceResponse

세금계산서 발행 승인 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### Map_Stat

Map_Stat

type: map<string, [B2bBulkTaxInvoiceStat](#b2bbulktaxinvoicestat)>

### RefuseB2bTaxInvoiceRequestBody

세금계산서 역발행 요청 거부 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `memo` | string |  | 메모 |

### RefuseB2bTaxInvoiceRequestResponse

세금계산서 역발행 요청 거부 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### RequestB2bTaxInvoiceResponse

세금계산서 역발행 요청 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### RequestB2bTaxInvoiceReverseIssuanceBody

세금계산서 역발행 즉시 요청 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 생성 요청 정보 |
| `memo` | string |  | 메모 |
| `modification` | [B2bTaxInvoiceModificationCreateBody](#b2btaxinvoicemodificationcreatebody) |  | 수정 세금계산서 입력 정보 |

### RequestB2bTaxInvoiceReverseIssuanceResponse

세금계산서 역발행 즉시 요청 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### SendToNtsB2bTaxInvoiceResponse

세금계산서 국세청 즉시 전송 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |

### TaxInvoicesSheetField

다운로드 할 시트 컬럼

- `STATUS` — 상태
- `CANCEL_REASON` — 취소사유
- `ISSUANCE_TYPE` — 발행유형
- `DOCUMENT_MODIFICATION_TYPE` — 문서형태
- `IS_DELAYED` — 지연발행
- `WRITE_DATE` — 작성일자
- `ISSUANCE_DUE_DATE` — 발행마감일
- `TAXATION_TYPE` — 과세형태
- `PURPOSE_TYPE` — 영수/청구
- `SUPPLIER_COMPANY_NAME` — 공급자 상호
- `SUPPLIER_BRN` — 공급자 사업자등록번호
- `SUPPLIER_REPRESENTATIVE_NAME` — 공급자 대표자명
- `TOTAL_AMOUNT` — 합계금액
- `TOTAL_SUPPLY_AMOUNT` — 공급가액
- `TOTAL_TAX_AMOUNT` — 세액
- `MEMO` — 관리용 메모
- `REQUESTED_AT` — 발행요청일시
- `ISSUED_AT` — 발행완료일시
- `NTS_SENT_AT` — 국세청 전송일시
- `STATUS_UPDATED_AT` — 상태 업데이트 일시
- `BULK_TAX_INVOICE_ID` — 일괄 세금계산서 아이디
- `PLAIN_ID` — 세금계산서 아이디
- `SUPPLIER_DOCUMENT_KEY` — 공급자 문서번호
- `RECIPIENT_DOCUMENT_KEY` — 공급받는자 문서번호
- `RECIPIENT_COMPANY_NAME` — 공급받는자 상호
- `RECIPIENT_BRN` — 공급받는자 사업자등록번호
- `RECIPIENT_REPRESENTATIVE_NAME` — 공급받는자 대표자명
- `SUPPLIER_COUNTERPARTY_ID` — 공급자 거래처 아이디
- `RECIPIENT_COUNTERPARTY_ID` — 공급받는자 거래처 아이디
- `PAYOUT_ID` — 지급 아이디
- `ITEMS` — 품목

### UpdateB2bTaxInvoiceDraftBody

세금계산서 임시저장 수정 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `brn` | string |  | 사업자등록번호 — taxInvoiceKeyType이 TAX_INVOICE_ID가 아닌 경우 필수 값입니다. |
| `taxInvoiceKey` | string | Y | 세금계산서 문서 번호 |
| `taxInvoiceKeyType` | [B2bTaxInvoiceKeyType](#b2btaxinvoicekeytype) |  | 문서 번호 유형 — 기본 값은 RECIPIENT이며 SUPPLIER, RECIPIENT을 지원합니다. |
| `taxInvoice` | [B2bTaxInvoiceInput](#b2btaxinvoiceinput) | Y | 세금계산서 임시저장 수정 정보 |
| `memo` | string |  | 메모 |

### UpdateB2bTaxInvoiceDraftResponse

세금계산서 임시저장 수정 응답

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `taxInvoice` | [B2bTaxInvoice](#b2btaxinvoice) | Y |  |
