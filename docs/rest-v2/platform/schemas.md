# 공유 스키마 (platform)

> 이 폴더의 둘 이상의 문서가 참조하는 스키마.

### DayOfWeek

요일

- `SUN`
- `MON`
- `TUE`
- `WED`
- `THU`
- `FRI`
- `SAT`

### MonthDay

월 및 일자 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `month` | integer(int32) | Y |  |
| `day` | integer(int32) | Y |  |

### PlatformAccount

플랫폼 정산 계좌 — 플랫폼 정산 계좌<br>`currency` 가 KRW 일 경우 예금주 조회 API 를 통해 올바른 계좌인지 검증합니다. 그 외의 화폐일 경우 따로 검증하지는 않습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `bank` | [Bank](../00-basics/schemas-common.md#bank) | Y | 은행 |
| `currency` | [Currency](../00-basics/schemas-common.md#currency) | Y | 정산에 사용할 통화 |
| `number` | string | Y | 계좌번호 |
| `holder` | string | Y | 예금주명 |
| `status` | [PlatformAccountStatus](#platformaccountstatus) | Y | 계좌 상태 |

### PlatformAccountStatus

플랫폼 계좌 상태

- `VERIFIED` — 계좌 인증 완료됨
- `VERIFY_FAILED` — 계좌주 불일치
- `VERIFY_ERROR` — 계좌 인증 오류
- `NOT_VERIFIED` — 계좌 인증 안됨
- `UNKNOWN` — 알 수 없는 상태

### PlatformAdditionalFeePolicy

추가 수수료 정책 — 추가 수수료 정책<br>추가 수수료 정책는 고객사의 주문건에 대한 중개수수료에 별도로 추가로 부여되는 수수료입니다. 대표적인 사용 예시로 풀필먼트 수수료, 로켓배송 수수료, 마케팅 채널 수수료등이 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 추가 수수료 정책 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 추가 수수료 정책 이름 |
| `fee` | [PlatformFee](#platformfee) | Y | 책정 수수료 |
| `memo` | string |  | 해당 추가 수수료 정책에 대한 메모 |
| `vatPayer` | [PlatformPayer](#platformpayer) | Y | 부가세를 부담할 주체 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformContact

플랫폼 파트너 담당자 연락 정보 — 플랫폼 파트너 담당자 연락 정보<br>파트너 담당자에게 연락하기 위한 정보들 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `name` | string | Y | 담당자 이름 |
| `phoneNumber` | string |  | 담당자 휴대폰 번호 |
| `email` | string | Y | 담당자 이메일 |

### PlatformContract

계약 — 계약<br>계약은 플랫폼 고객사가 파트너에게 정산해줄 대금과 정산일을 계산하는 데 적용되는 정보입니다.<br>고객사의 플랫폼에서 재화 및 서비스를 판매하기 위한 중개수수료와 판매금에 대한 정산일로 구성되어 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 계약 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 계약 이름 |
| `memo` | string |  | 계약 내부 표기를 위한 메모 |
| `platformFee` | [PlatformFee](#platformfee) | Y | 중개수수료 |
| `settlementCycle` | [PlatformSettlementCycle](#platformsettlementcycle) | Y | 정산 주기 |
| `platformFeeVatPayer` | [PlatformPayer](#platformpayer) | Y | 중개수수료에 대한 부가세 부담 주체 |
| `subtractPaymentVatAmount` | boolean | Y | 정산 시 결제금액 부가세 감액 여부 — false인 경우 정산금에서 결제 금액 부가세를 감액하지 않고, true인 경우 정산금에서 결제 금액 부가세를 감액합니다. |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformDiscountSharePolicy

할인 분담 정책 — 할인 분담 정책<br>할인 분담은 고객사의 주문건에 쿠폰 및 포인트와 같은 할인금액이 적용될 때, 파트너 정산 시 할인금액에 대한 분담 정책을 가지는 객체입니다.<br>할인 유형에 대한 아이디와 메모, 그리고 파트너 분담율을 가집니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y |  |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 할인 분담 정책 이름 |
| `partnerShareRate` | integer(int32) | Y | 할인 분담율 — 파트너가 분담할 할인금액의 비율을 의미하는 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수이며, 파트너가 부담할 금액은 `할인금액 * partnerShareRate * 10^5` 로 책정합니다. |
| `memo` | string |  | 해당 할인 분담에 대한 메모 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `isForTest` | boolean | Y |  |

### PlatformFee

플랫폼 중개수수료 정보

oneOf, discriminator `type`:

- `FIXED_AMOUNT` → [PlatformFixedAmountFee](#platformfixedamountfee)
- `FIXED_RATE` → [PlatformFixedRateFee](#platformfixedratefee)

### PlatformFixedAmountFee

정액 수수료 — 정액 수수료<br>총 금액에 무관하게 정해진 수수료 금액을 책정합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `amount` | integer(int64) | Y | 고정된 수수료 금액 |

### PlatformFixedRateFee

정률 수수료 — 정률 수수료<br>총 금액에 정해진 비율을 곱한 만큼의 수수료를 책정합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `rate` | integer(int32) | Y | 수수료율 — 총 금액 대비 수수료 비율을 의미하며, 밀리 퍼센트 단위 (10^-5) 의 음이 아닌 정수입니다. `총 금액 * rate * 10^5` (`rate * 10^3 %`) 만큼 수수료를 책정합니다. |

### PlatformPartner

파트너 — 파트너<br>파트너는 고객사가 정산해주어야 할 대상입니다.<br>기본 사업자 정보와 정산정보, 그리고 적용될 계약의 정보를 등록 및 관리할 수 있습니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 파트너 고유 아이디 |
| `graphqlId` | string | Y |  |
| `name` | string | Y | 파트너 법인명 혹은 이름 |
| `contact` | [PlatformContact](#platformcontact) | Y | 파트너 담당자 연락 정보 |
| `account` | [PlatformAccount](#platformaccount) | Y | 정산 계좌 |
| `status` | [PlatformPartnerStatus](#platformpartnerstatus) | Y | 파트너의 상태 |
| `defaultContractId` | string | Y | 파트너에 설정된 기본 계약 아이디 |
| `memo` | string |  | 파트너에 대한 메모 |
| `tags` | string[] | Y | 파트너의 태그 리스트 |
| `type` | [PlatformPartnerType](#platformpartnertype) | Y | 파트너 유형별 정보 |
| `isArchived` | boolean | Y | 보관 여부 |
| `appliedAt` | string(date-time) | Y | 변경 적용 시점 |
| `userDefinedProperties` | [PlatformProperties](#platformproperties) | Y | 사용자 정의 속성 |
| `isForTest` | boolean | Y |  |

### PlatformPartnerBusinessStatus

플랫폼 파트너 사업자 상태

- `NOT_VERIFIED` — 조회 되지 않음
- `VERIFY_ERROR` — 조회 오류
- `NOT_FOUND` — 대응되는 사업자 없음
- `IN_BUSINESS` — 사업 중
- `CLOSED` — 폐업
- `SUSPENDED` — 휴업

### PlatformPartnerMemberCompanyConnectionStatus

플랫폼 파트너 연동 사업자 연결 상태

- `NOT_CONNECTED` — 연결되지 않음
- `CONNECT_PENDING` — 연결 대기
- `CONNECTED` — 연결됨
- `CONNECT_FAILED` — 연결 실패
- `DISCONNECT_PENDING` — 연결 해제 대기

### PlatformPartnerStatus

플랫폼 파트너 상태

- `PENDING` — 승인 대기 중
- `APPROVED` — 승인 완료
- `REJECTED` — 승인 거절

### PlatformPartnerTaxationType

플랫폼 파트너 과세 유형

- `NORMAL` — 일반 과세
- `SIMPLE_TAX_INVOICE_ISSUER` — 간이과세(세금계산서 발행)
- `SIMPLE` — 간이과세(세금계산서 미발행)
- `TAX_FREE` — 면세

### PlatformPartnerType

파트너 유형별 추가 정보

oneOf, discriminator `type`:

- `BUSINESS` → [PlatformPartnerTypeBusiness](#platformpartnertypebusiness)
- `NON_WHT_PAYER` → [PlatformPartnerTypeNonWhtPayer](#platformpartnertypenonwhtpayer)
- `WHT_PAYER` → [PlatformPartnerTypeWhtPayer](#platformpartnertypewhtpayer)

### PlatformPartnerTypeBusiness

사업자 파트너 정보 — 사업자 파트너 정보<br>사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `companyName` | string | Y | 상호명 |
| `taxationType` | [PlatformPartnerTaxationType](#platformpartnertaxationtype) | Y | 과세 유형 |
| `businessStatus` | [PlatformPartnerBusinessStatus](#platformpartnerbusinessstatus) | Y | 사업자 상태 |
| `businessRegistrationNumber` | string | Y | 사업자등록번호 |
| `representativeName` | string | Y | 대표자 이름 |
| `companyAddress` | string |  | 사업장 주소 |
| `businessType` | string |  | 업태 |
| `businessClass` | string |  | 업종 |
| `memberCompanyConnectionStatus` | [PlatformPartnerMemberCompanyConnectionStatus](#platformpartnermembercompanyconnectionstatus) | Y | 연동사업자 연동 상태 |
| `memberCompanyConnectionFailedReason` | string |  | 연동사업자 연동 실패 사유 |

### PlatformPartnerTypeName

플랫폼 파트너 유형 이름

- `BUSINESS` — 사업자
- `WHT_PAYER` — 원천징수 대상자
- `NON_WHT_PAYER` — 원천징수 비대상자

### PlatformPartnerTypeNonWhtPayer

원천징수 비대상자 파트너 정보 — 원천징수 비대상자 파트너 정보<br>비사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPartnerTypeWhtPayer

원천징수 대상자 파트너 정보 — 원천징수 대상자 파트너 정보<br>비사업자 유형의 파트너 추가 정보 입니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `birthdate` | string(date) |  | 생년월일 — 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### PlatformPayer

금액 부담 주체 — 금액 부담 주체<br>플랫폼에서 발생한 결제 수수료, 부가세 등 금액을 부담하는 주체를 나타냅니다.

- `PARTNER` — 파트너가 부담하는 경우
- `MERCHANT` — 고객사가 부담하는 경우

### PlatformProperties

PlatformProperties

type: map<string, [PlatformUserDefinedPropertyValue](#platformuserdefinedpropertyvalue)>

### PlatformSettlementCycle

정산 주기 — 정산 주기<br>지체일, 정산일, 기준일로 구성되며, 해당 요소들의 조합으로 실제 정산일을 계산합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `lagDays` | integer(int32) | Y | 지체일 (d+n 의 n) — 정산시작일(통상 주문완료일)로부터 더해진 다음 날짜로부터 가장 가까운 날에 정산이 됩니다. 최소 1 에서 최대 10 까지 지정할 수 있습니다. |
| `datePolicy` | [PlatformSettlementCycleDatePolicy](#platformsettlementcycledatepolicy) | Y | 기준일로, 정산일 계산 시 공휴일을 고려하기 위한 정보입니다. |
| `method` | [PlatformSettlementCycleMethod](#platformsettlementcyclemethod) | Y | 정산 주기 계산 방식 |

### PlatformSettlementCycleDatePolicy

플랫폼 정산 기준일

- `HOLIDAY_BEFORE` — 공휴일 전 영업일
- `HOLIDAY_AFTER` — 공휴일 후 영업일
- `CALENDAR_DAY` — 달력일

### PlatformSettlementCycleMethod

플랫폼 정산 주기 계산 방식

oneOf, discriminator `type`:

- `DAILY` → [PlatformSettlementCycleMethodDaily](#platformsettlementcyclemethoddaily)
- `MANUAL_DATES` → [PlatformSettlementCycleMethodManualDates](#platformsettlementcyclemethodmanualdates)
- `MONTHLY` → [PlatformSettlementCycleMethodMonthly](#platformsettlementcyclemethodmonthly)
- `WEEKLY` → [PlatformSettlementCycleMethodWeekly](#platformsettlementcyclemethodweekly)

### PlatformSettlementCycleMethodDaily

매일 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |

### PlatformSettlementCycleMethodManualDates

정해진 날짜(월, 일)에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `dates` | [MonthDay](#monthday)[] | Y | 월 및 일자 정보 |

### PlatformSettlementCycleMethodMonthly

매월 정해진 날(일)에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `daysOfMonth` | integer(int32)[] | Y |  |

### PlatformSettlementCycleMethodWeekly

매주 정해진 요일에 정산

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `daysOfWeek` | [DayOfWeek](#dayofweek)[] | Y | 요일 |

### PlatformUserDefinedPropertyValue

PlatformUserDefinedPropertyValue

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `string` | string | Y |  |

### SettlementAmountType

SettlementAmountType

- `NET` — 순액(공급가액)
- `GROSS` — 총액(공급가액, 부가세)
