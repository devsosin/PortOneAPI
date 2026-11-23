# 공통 스키마

> 둘 이상의 카테고리 폴더가 참조하는 스키마. 한 문서(또는 한 폴더)에서만 쓰는 스키마는 그쪽에 있다.

### Address

분리 형식 주소 — 분리 형식 주소<br>oneLine(한 줄 형식 주소) 필드는 항상 존재합니다.

oneOf, discriminator `type`:

- `ONE_LINE` → [OneLineAddress](#onelineaddress)
- `SEPARATED` → [SeparatedAddress](#separatedaddress)

### Bank

은행

- `BANK_OF_KOREA` — 한국은행
- `KDB` — 산업은행
- `IBK` — 기업은행
- `KOOKMIN` — 국민은행
- `SUHYUP` — 수협은행
- `KEXIM` — 수출입은행
- `NONGHYUP` — NH농협은행
- `LOCAL_NONGHYUP` — 지역농축협
- `WOORI` — 우리은행
- `STANDARD_CHARTERED` — SC제일은행
- `CITI` — 한국씨티은행
- `SUHYUP_FEDERATION` — 수협중앙회
- `DAEGU` — 아이엠뱅크
- `BUSAN` — 부산은행
- `KWANGJU` — 광주은행
- `JEJU` — 제주은행
- `JEONBUK` — 전북은행
- `KYONGNAM` — 경남은행
- `KFCC` — 새마을금고
- `SHINHYUP` — 신협
- `SAVINGS_BANK` — 저축은행
- `MORGAN_STANLEY` — 모간스탠리은행
- `HSBC` — HSBC은행
- `DEUTSCHE` — 도이치은행
- `JPMC` — 제이피모간체이스은행
- `MIZUHO` — 미즈호은행
- `MUFG` — 엠유에프지은행
- `BANK_OF_AMERICA` — BOA은행
- `BNP_PARIBAS` — 비엔피파리바은행
- `ICBC` — 중국공상은행
- `BANK_OF_CHINA` — 중국은행
- `NFCF` — 산림조합중앙회
- `UOB` — 대화은행
- `BOCOM` — 교통은행
- `CCB` — 중국건설은행
- `POST` — 우체국
- `KODIT` — 신용보증기금
- `KIBO` — 기술보증기금
- `HANA` — 하나은행
- `SHINHAN` — 신한은행
- `K_BANK` — 케이뱅크
- `KAKAO` — 카카오뱅크
- `TOSS` — 토스뱅크
- `MISC_FOREIGN` — 기타 외국계은행(중국 농업은행 등)
- `SGI` — 서울보증보험
- `KCIS` — 한국신용정보원
- `YUANTA_SECURITIES` — 유안타증권
- `KB_SECURITIES` — KB증권
- `SANGSANGIN_SECURITIES` — 상상인증권
- `HANYANG_SECURITIES` — 한양증권
- `LEADING_SECURITIES` — 리딩투자증권
- `BNK_SECURITIES` — BNK투자증권
- `IBK_SECURITIES` — IBK투자증권
- `DAOL_SECURITIES` — 다올투자증권
- `MIRAE_ASSET_SECURITIES` — 미래에셋증권
- `SAMSUNG_SECURITIES` — 삼성증권
- `KOREA_SECURITIES` — 한국투자증권
- `NH_SECURITIES` — NH투자증권
- `KYOBO_SECURITIES` — 교보증권
- `HI_SECURITIES` — 하이투자증권
- `HYUNDAI_MOTOR_SECURITIES` — 현대차증권
- `KIWOOM_SECURITIES` — 키움증권
- `EBEST_SECURITIES` — LS증권
- `SK_SECURITIES` — SK증권
- `DAISHIN_SECURITIES` — 대신증권
- `HANHWA_SECURITIES` — 한화투자증권
- `HANA_SECURITIES` — 하나증권
- `TOSS_SECURITIES` — 토스증권
- `SHINHAN_SECURITIES` — 신한투자증권
- `DB_SECURITIES` — DB금융투자
- `EUGENE_SECURITIES` — 유진투자증권
- `MERITZ_SECURITIES` — 메리츠증권
- `KAKAO_PAY_SECURITIES` — 카카오페이증권
- `BOOKOOK_SECURITIES` — 부국증권
- `SHINYOUNG_SECURITIES` — 신영증권
- `CAPE_SECURITIES` — 케이프투자증권
- `KOREA_SECURITIES_FINANCE` — 한국증권금융
- `KOREA_FOSS_SECURITIES` — 한국포스증권
- `WOORI_INVESTMENT_BANK` — 우리종합금융

### Card

카드 상세 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `publisher` | string |  | 발행사 코드 |
| `issuer` | string |  | 발급사 코드 |
| `brand` | [CardBrand](#cardbrand) |  | 카드 브랜드 |
| `type` | [CardType](#cardtype) |  | 카드 유형 |
| `ownerType` | [CardOwnerType](#cardownertype) |  | 카드 소유주 유형 |
| `bin` | string |  | 카드 번호 앞 6자리 또는 8자리의 BIN (Bank Identification Number) |
| `name` | string |  | 카드 상품명 |
| `number` | string |  | 마스킹된 카드 번호 |

### CardBrand

카드 브랜드

- `LOCAL`
- `MASTER`
- `UNIONPAY`
- `VISA`
- `JCB`
- `AMEX`
- `DINERS`

### CardCredential

카드 인증 관련 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `number` | string | Y | 카드 번호 (숫자만) |
| `expiryYear` | string | Y | 유효 기간 만료 연도 (2자리) |
| `expiryMonth` | string | Y | 유효 기간 만료 월 (2자리) |
| `birthOrBusinessRegistrationNumber` | string |  | 생년월일 (yyMMdd) 또는 사업자 등록 번호 (10자리, 숫자만) |
| `passwordTwoDigits` | string |  | 비밀번호 앞 2자리 |

### CardOwnerType

카드 소유주 유형

- `PERSONAL` — 개인
- `CORPORATE` — 법인

### CardType

카드 유형

- `CREDIT` — 신용카드
- `DEBIT` — 체크카드
- `GIFT` — 기프트카드

### CashReceiptInput

현금영수증 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [CashReceiptInputType](#cashreceiptinputtype) | Y | 현금영수증 유형 |
| `customerIdentityNumber` | string |  | 사용자 식별 번호 — 미발행 유형 선택 시 입력하지 않습니다. |
| `customerIdentityNumberType` | [IssueCashReceiptCustomerInputIdentityNumberType](#issuecashreceiptcustomerinputidentitynumbertype) |  | 사용자 식별 번호 유형 — 사용자 식별 번호가 어떤 종류의 번호인지 명시합니다. |

### CashReceiptInputType

입력 시 발급 유형

- `PERSONAL` — 소득공제용
- `CORPORATE` — 지출증빙용
- `NO_RECEIPT` — 미발행

### CashReceiptType

발급 유형

- `PERSONAL` — 소득공제용
- `CORPORATE` — 지출증빙용

### ChannelGroupSummary

채널 그룹 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 채널 그룹 아이디 |
| `name` | string | Y | 채널 그룹 이름 |
| `isForTest` | boolean | Y | 테스트 채널 그룹 여부 |

### Country

국가

- `AD` — Andorra
- `AE` — United Arab Emirates (the)
- `AF` — Afghanistan
- `AG` — Antigua and Barbuda
- `AI` — Anguilla
- `AL` — Albania
- `AM` — Armenia
- `AO` — Angola
- `AQ` — Antarctica
- `AR` — Argentina
- `AS` — American Samoa
- `AT` — Austria
- `AU` — Australia
- `AW` — Aruba
- `AX` — Åland Islands
- `AZ` — Azerbaijan
- `BA` — Bosnia and Herzegovina
- `BB` — Barbados
- `BD` — Bangladesh
- `BE` — Belgium
- `BF` — Burkina Faso
- `BG` — Bulgaria
- `BH` — Bahrain
- `BI` — Burundi
- `BJ` — Benin
- `BL` — Saint Barthélemy
- `BM` — Bermuda
- `BN` — Brunei Darussalam
- `BO` — Bolivia (Plurinational State of)
- `BQ` — Bonaire, Sint Eustatius and Saba
- `BR` — Brazil
- `BS` — Bahamas (the)
- `BT` — Bhutan
- `BV` — Bouvet Island
- `BW` — Botswana
- `BY` — Belarus
- `BZ` — Belize
- `CA` — Canada
- `CC` — Cocos (Keeling) Islands (the)
- `CD` — Congo (the Democratic Republic of the)
- `CF` — Central African Republic (the)
- `CG` — Congo (the)
- `CH` — Switzerland
- `CI` — Côte d'Ivoire
- `CK` — Cook Islands (the)
- `CL` — Chile
- `CM` — Cameroon
- `CN` — China
- `CO` — Colombia
- `CR` — Costa Rica
- `CU` — Cuba
- `CV` — Cabo Verde
- `CW` — Curaçao
- `CX` — Christmas Island
- `CY` — Cyprus
- `CZ` — Czechia
- `DE` — Germany
- `DJ` — Djibouti
- `DK` — Denmark
- `DM` — Dominica
- `DO` — Dominican Republic (the)
- `DZ` — Algeria
- `EC` — Ecuador
- `EE` — Estonia
- `EG` — Egypt
- `EH` — Western Sahara
- `ER` — Eritrea
- `ES` — Spain
- `ET` — Ethiopia
- `FI` — Finland
- `FJ` — Fiji
- `FK` — Falkland Islands (the) [Malvinas]
- `FM` — Micronesia (Federated States of)
- `FO` — Faroe Islands (the)
- `FR` — France
- `GA` — Gabon
- `GB` — United Kingdom of Great Britain and Northern Ireland (the)
- `GD` — Grenada
- `GE` — Georgia
- `GF` — French Guiana
- `GG` — Guernsey
- `GH` — Ghana
- `GI` — Gibraltar
- `GL` — Greenland
- `GM` — Gambia (the)
- `GN` — Guinea
- `GP` — Guadeloupe
- `GQ` — Equatorial Guinea
- `GR` — Greece
- `GS` — South Georgia and the South Sandwich Islands
- `GT` — Guatemala
- `GU` — Guam
- `GW` — Guinea-Bissau
- `GY` — Guyana
- `HK` — Hong Kong
- `HM` — Heard Island and McDonald Islands
- `HN` — Honduras
- `HR` — Croatia
- `HT` — Haiti
- `HU` — Hungary
- `ID` — Indonesia
- `IE` — Ireland
- `IL` — Israel
- `IM` — Isle of Man
- `IN` — India
- `IO` — British Indian Ocean Territory (the)
- `IQ` — Iraq
- `IR` — Iran (Islamic Republic of)
- `IS` — Iceland
- `IT` — Italy
- `JE` — Jersey
- `JM` — Jamaica
- `JO` — Jordan
- `JP` — Japan
- `KE` — Kenya
- `KG` — Kyrgyzstan
- `KH` — Cambodia
- `KI` — Kiribati
- `KM` — Comoros (the)
- `KN` — Saint Kitts and Nevis
- `KP` — Korea (the Democratic People's Republic of)
- `KR` — Korea (the Republic of)
- `KW` — Kuwait
- `KY` — Cayman Islands (the)
- `KZ` — Kazakhstan
- `LA` — Lao People's Democratic Republic (the)
- `LB` — Lebanon
- `LC` — Saint Lucia
- `LI` — Liechtenstein
- `LK` — Sri Lanka
- `LR` — Liberia
- `LS` — Lesotho
- `LT` — Lithuania
- `LU` — Luxembourg
- `LV` — Latvia
- `LY` — Libya
- `MA` — Morocco
- `MC` — Monaco
- `MD` — Moldova (the Republic of)
- `ME` — Montenegro
- `MF` — Saint Martin (French part)
- `MG` — Madagascar
- `MH` — Marshall Islands (the)
- `MK` — North Macedonia
- `ML` — Mali
- `MM` — Myanmar
- `MN` — Mongolia
- `MO` — Macao
- `MP` — Northern Mariana Islands (the)
- `MQ` — Martinique
- `MR` — Mauritania
- `MS` — Montserrat
- `MT` — Malta
- `MU` — Mauritius
- `MV` — Maldives
- `MW` — Malawi
- `MX` — Mexico
- `MY` — Malaysia
- `MZ` — Mozambique
- `NA` — Namibia
- `NC` — New Caledonia
- `NE` — Niger (the)
- `NF` — Norfolk Island
- `NG` — Nigeria
- `NI` — Nicaragua
- `NL` — Netherlands (Kingdom of the)
- `NO` — Norway
- `NP` — Nepal
- `NR` — Nauru
- `NU` — Niue
- `NZ` — New Zealand
- `OM` — Oman
- `PA` — Panama
- `PE` — Peru
- `PF` — French Polynesia
- `PG` — Papua New Guinea
- `PH` — Philippines (the)
- `PK` — Pakistan
- `PL` — Poland
- `PM` — Saint Pierre and Miquelon
- `PN` — Pitcairn
- `PR` — Puerto Rico
- `PS` — Palestine, State of
- `PT` — Portugal
- `PW` — Palau
- `PY` — Paraguay
- `QA` — Qatar
- `RE` — Réunion
- `RO` — Romania
- `RS` — Serbia
- `RU` — Russian Federation (the)
- `RW` — Rwanda
- `SA` — Saudi Arabia
- `SB` — Solomon Islands
- `SC` — Seychelles
- `SD` — Sudan (the)
- `SE` — Sweden
- `SG` — Singapore
- `SH` — Saint Helena, Ascension and Tristan da Cunha
- `SI` — Slovenia
- `SJ` — Svalbard and Jan Mayen
- `SK` — Slovakia
- `SL` — Sierra Leone
- `SM` — San Marino
- `SN` — Senegal
- `SO` — Somalia
- `SR` — Suriname
- `SS` — South Sudan
- `ST` — Sao Tome and Principe
- `SV` — El Salvador
- `SX` — Sint Maarten (Dutch part)
- `SY` — Syrian Arab Republic (the)
- `SZ` — Eswatini
- `TC` — Turks and Caicos Islands (the)
- `TD` — Chad
- `TF` — French Southern Territories (the)
- `TG` — Togo
- `TH` — Thailand
- `TJ` — Tajikistan
- `TK` — Tokelau
- `TL` — Timor-Leste
- `TM` — Turkmenistan
- `TN` — Tunisia
- `TO` — Tonga
- `TR` — Türkiye
- `TT` — Trinidad and Tobago
- `TV` — Tuvalu
- `TW` — Taiwan (Province of China)
- `TZ` — Tanzania, the United Republic of
- `UA` — Ukraine
- `UG` — Uganda
- `UM` — United States Minor Outlying Islands (the)
- `US` — United States of America (the)
- `UY` — Uruguay
- `UZ` — Uzbekistan
- `VA` — Holy See (the)
- `VC` — Saint Vincent and the Grenadines
- `VE` — Venezuela (Bolivarian Republic of)
- `VG` — Virgin Islands (British)
- `VI` — Virgin Islands (U.S.)
- `VN` — Viet Nam
- `VU` — Vanuatu
- `WF` — Wallis and Futuna
- `WS` — Samoa
- `YE` — Yemen
- `YT` — Mayotte
- `ZA` — South Africa
- `ZM` — Zambia
- `ZW` — Zimbabwe

### Currency

통화

- `KRW` — 대한민국 원화
- `USD` — 미국 달러
- `JPY` — 일본 엔화
- `AED` — UAE Dirham
- `AFN` — Afghani
- `ALL` — Lek
- `AMD` — Armenian Dram
- `ANG` — Netherlands Antillean Guilder
- `AOA` — Kwanza
- `ARS` — Argentine Peso
- `AUD` — Australian Dollar
- `AWG` — Aruban Florin
- `AZN` — Azerbaijan Manat
- `BAM` — Convertible Mark
- `BBD` — Barbados Dollar
- `BDT` — Taka
- `BGN` — Bulgarian Lev
- `BHD` — Bahraini Dinar
- `BIF` — Burundi Franc
- `BMD` — Bermudian Dollar
- `BND` — Brunei Dollar
- `BOB` — Boliviano
- `BOV` — Mvdol
- `BRL` — Brazilian Real
- `BSD` — Bahamian Dollar
- `BTN` — Ngultrum
- `BWP` — Pula
- `BYN` — Belarusian Ruble
- `BZD` — Belize Dollar
- `CAD` — Canadian Dollar
- `CDF` — Congolese Franc
- `CHE` — WIR Euro
- `CHF` — Swiss Franc
- `CHW` — WIR Franc
- `CLF` — Unidad de Fomento
- `CLP` — Chilean Peso
- `CNY` — Yuan Renminbi
- `COP` — Colombian Peso
- `COU` — Unidad de Valor Real
- `CRC` — Costa Rican Colon
- `CUC` — Peso Convertible
- `CUP` — Cuban Peso
- `CVE` — Cabo Verde Escudo
- `CZK` — Czech Koruna
- `DJF` — Djibouti Franc
- `DKK` — Danish Krone
- `DOP` — Dominican Peso
- `DZD` — Algerian Dinar
- `EGP` — Egyptian Pound
- `ERN` — Nakfa
- `ETB` — Ethiopian Birr
- `EUR` — Euro
- `FJD` — Fiji Dollar
- `FKP` — Falkland Islands Pound
- `GBP` — Pound Sterling
- `GEL` — Lari
- `GHS` — Ghana Cedi
- `GIP` — Gibraltar Pound
- `GMD` — Dalasi
- `GNF` — Guinean Franc
- `GTQ` — Quetzal
- `GYD` — Guyana Dollar
- `HKD` — Hong Kong Dollar
- `HNL` — Lempira
- `HRK` — Kuna (Replaced by EUR)
- `HTG` — Gourde
- `HUF` — Forint
- `IDR` — Rupiah
- `ILS` — New Israeli Sheqel
- `INR` — Indian Rupee
- `IQD` — Iraqi Dinar
- `IRR` — Iranian Rial
- `ISK` — Iceland Krona
- `JMD` — Jamaican Dollar
- `JOD` — Jordanian Dinar
- `KES` — Kenyan Shilling
- `KGS` — Som
- `KHR` — Riel
- `KMF` — Comorian Franc
- `KPW` — North Korean Won
- `KWD` — Kuwaiti Dinar
- `KYD` — Cayman Islands Dollar
- `KZT` — Tenge
- `LAK` — Lao Kip
- `LBP` — Lebanese Pound
- `LKR` — Sri Lanka Rupee
- `LRD` — Liberian Dollar
- `LSL` — Loti
- `LYD` — Libyan Dinar
- `MAD` — Moroccan Dirham
- `MDL` — Moldovan Leu
- `MGA` — Malagasy Ariary
- `MKD` — Denar
- `MMK` — Kyat
- `MNT` — Tugrik
- `MOP` — Pataca
- `MRU` — Ouguiya
- `MUR` — Mauritius Rupee
- `MVR` — Rufiyaa
- `MWK` — Malawi Kwacha
- `MXN` — Mexican Peso
- `MXV` — Mexican Unidad de Inversion (UDI)
- `MYR` — Malaysian Ringgit
- `MZN` — Mozambique Metical
- `NAD` — Namibia Dollar
- `NGN` — Naira
- `NIO` — Cordoba Oro
- `NOK` — Norwegian Krone
- `NPR` — Nepalese Rupee
- `NZD` — New Zealand Dollar
- `OMR` — Rial Omani
- `PAB` — Balboa
- `PEN` — Sol
- `PGK` — Kina
- `PHP` — Philippine Peso
- `PKR` — Pakistan Rupee
- `PLN` — Zloty
- `PYG` — Guarani
- `QAR` — Qatari Rial
- `RON` — Romanian Leu
- `RSD` — Serbian Dinar
- `RUB` — Russian Ruble
- `RWF` — Rwanda Franc
- `SAR` — Saudi Riyal
- `SBD` — Solomon Islands Dollar
- `SCR` — Seychelles Rupee
- `SDG` — Sudanese Pound
- `SEK` — Swedish Krona
- `SGD` — Singapore Dollar
- `SHP` — Saint Helena Pound
- `SLE` — Leone
- `SLL` — Leone
- `SOS` — Somali Shilling
- `SRD` — Surinam Dollar
- `SSP` — South Sudanese Pound
- `STN` — Dobra
- `SVC` — El Salvador Colon
- `SYP` — Syrian Pound
- `SZL` — Lilangeni
- `THB` — Baht
- `TJS` — Somoni
- `TMT` — Turkmenistan New Manat
- `TND` — Tunisian Dinar
- `TOP` — Pa’anga
- `TRY` — Turkish Lira
- `TTD` — Trinidad and Tobago Dollar
- `TWD` — New Taiwan Dollar
- `TZS` — Tanzanian Shilling
- `UAH` — Hryvnia
- `UGX` — Uganda Shilling
- `USN` — US Dollar (Next day)
- `UYI` — Uruguay Peso en Unidades Indexadas (UI)
- `UYU` — Peso Uruguayo
- `UYW` — Unidad Previsional
- `UZS` — Uzbekistan Sum
- `VED` — Bolívar Soberano
- `VES` — Bolívar Soberano
- `VND` — Dong
- `VUV` — Vatu
- `WST` — Tala
- `XAF` — CFA Franc BEAC
- `XAG` — Silver
- `XAU` — Gold
- `XBA` — Bond Markets Unit European Composite Unit (EURCO)
- `XBB` — Bond Markets Unit European Monetary Unit (E.M.U.-6)
- `XBC` — Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
- `XBD` — Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
- `XCD` — East Caribbean Dollar
- `XDR` — SDR (Special Drawing Right)
- `XOF` — CFA Franc BCEAO
- `XPD` — Palladium
- `XPF` — CFP Franc
- `XPT` — Platinum
- `XSU` — Sucre
- `XTS` — Codes specifically reserved for testing purposes
- `XUA` — ADB Unit of Account
- `XXX` — The codes assigned for transactions where no currency is involved
- `YER` — Yemeni Rial
- `ZAR` — Rand
- `ZMW` — Zambian Kwacha
- `ZWL` — Zimbabwe Dollar

### Customer

고객 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 고객 아이디 — 고객사가 지정한 고객의 고유 식별자입니다. |
| `name` | string |  | 이름 |
| `birthYear` | string |  | 출생 연도 |
| `birthMonth` | string |  | 출생 월 |
| `birthDay` | string |  | 출생 일자 |
| `gender` | [Gender](#gender) |  | 성별 |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 전화번호 |
| `address` | [Address](#address) |  | 주소 |
| `zipcode` | string |  | 우편번호 |

### CustomerInput

고객 정보 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string |  | 고객 아이디 — 고객사가 지정한 고객의 고유 식별자입니다. |
| `name` | [CustomerNameInput](#customernameinput) |  | 이름 |
| `birthYear` | string |  | 출생 연도 |
| `birthMonth` | string |  | 출생월 |
| `birthDay` | string |  | 출생일 |
| `country` | [Country](#country) |  | 국가 |
| `gender` | [Gender](#gender) |  | 성별 |
| `email` | string |  | 이메일 |
| `phoneNumber` | string |  | 전화번호 |
| `address` | [SeparatedAddressInput](#separatedaddressinput) |  | 주소 |
| `zipcode` | string |  | 우편번호 |
| `businessRegistrationNumber` | string |  | 사업자 등록 번호 |

### CustomerNameInput

고객 이름 입력 정보 — 고객 이름 입력 정보<br>두 개의 이름 형식 중 한 가지만 선택하여 입력해주세요.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `full` | string |  | 한 줄 이름 형식 |
| `separated` | [CustomerSeparatedName](#customerseparatedname) |  | 분리형 이름 형식 |

### CustomerSeparatedName

고객 분리형 이름

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `first` | string | Y | 이름 |
| `last` | string | Y | 성 |

### DateRange

DateRange

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `from` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |
| `until` | string(date) | Y | 날짜를 나타내는 문자열로, `yyyy-MM-dd` 형식을 따릅니다. |

### DateTimeRange

시간 범위

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `from` | string(date-time) | Y |  |
| `until` | string(date-time) | Y |  |

### EasyPayProvider

간편 결제사

- `SAMSUNGPAY`
- `KAKAOPAY`
- `NAVERPAY`
- `PAYCO`
- `SSGPAY`
- `CHAI`
- `LPAY`
- `KPAY`
- `TOSSPAY`
- `LGPAY`
- `PINPAY`
- `APPLEPAY`
- `SKPAY`
- `TOSS_BRANDPAY`
- `KB_APP`
- `ALIPAY`
- `HYPHEN`
- `TMONEY`
- `PAYPAL`
- `SMILEPAY`
- `MIR`
- `WECHAT`
- `LINEPAY`
- `KLARNA`
- `GRABPAY`
- `SHOPEEPAY`
- `JKOPAY`
- `PAYPAY`
- `AMAZONPAY`
- `RAKUTENPAY`
- `DBARAI`
- `AUPAY`
- `MERPAY`
- `MONEYTREE`
- `KPLUS`
- `TINABA`
- `BILL_EASE`
- `KREDIVO`
- `RABBIT_LINE_PAY`
- `ALIPAY_HK`
- `AKULAKU_PAY_LATER`
- `BOOST`
- `BPI`
- `DANA`
- `G_CASH`
- `HIPAY`
- `MPAY`
- `TOUCH_N_GO`
- `TRUE_MONEY`
- `DOKU_WALLET`
- `JENIUS_PAY`
- `OVO`
- `MAYA`
- `QRIS`
- `THAI_QR`
- `GOOGLE_PAY`
- `ROCKET_PAY`

### Gender

성별

- `MALE` — 남성
- `FEMALE` — 여성
- `OTHER` — 그 외 성별

### IssueCashReceiptCustomerInputIdentityNumberType

현금영수증 발급 시 고객 식별 정보 유형

- `PHONE` — 휴대전화번호
- `CARD` — 카드번호
- `BUSINESS` — 사업자등록번호

### Locale

결제 언어

- `KO_KR` — 한국어 (대한민국)
- `EN_US` — 영어 (미국)
- `ZH_CN` — 중국어 (중국)
- `ZH_TW` — 중국어 (대만)
- `JA_JP` — 일본어 (일본)
- `RU_RU` — 러시아어 (러시아)
- `TH_TH` — 타이어 (타이)
- `VI_VN` — 베트남어 (베트남)

### OneLineAddress

한 줄 형식 주소 — 한 줄 형식 주소<br>한 줄 형식 주소만 존재합니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `oneLine` | string | Y | 주소 (한 줄) |

### PageInfo

반환된 페이지 결과 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `number` | integer(int32) | Y | 요청된 페이지 번호 |
| `size` | integer(int32) | Y | 요청된 페이지 당 객체 수 |
| `totalCount` | integer(int32) | Y | 실제 반환된 객체 수 |

### PageInput

다건 조회 API 에 사용되는 페이지 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `number` | integer(int32) |  | 0부터 시작하는 페이지 번호 |
| `size` | integer(int32) |  | 각 페이지 당 포함할 객체 수 |

### PaymentAmountInput

금액 세부 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `total` | integer(int64) | Y | 총 금액 |
| `taxFree` | integer(int64) |  | 면세액 |
| `vat` | integer(int64) |  | 부가세액 — 고객사에서 직접 계산이 필요한 경우 입력합니다.<br>입력하지 않으면 면세 금액을 제외한 금액의 1/11 로 자동 계산됩니다. |

### PaymentClientType

결제가 발생한 클라이언트 환경

- `SDK_MOBILE`
- `SDK_PC`
- `API`

### PaymentMethodType

PaymentMethodType

- `CARD`
- `TRANSFER`
- `VIRTUAL_ACCOUNT`
- `GIFT_CERTIFICATE`
- `MOBILE`
- `EASY_PAY`
- `CONVENIENCE_STORE`
- `CRYPTO`

### PaymentProduct

상품 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `id` | string | Y | 상품 고유 식별자 — 고객사가 직접 부여한 식별자입니다. |
| `name` | string | Y | 상품명 |
| `tag` | string |  | 상품 태그 — 카테고리 등으로 활용될 수 있습니다. |
| `code` | string |  | 상품 코드 |
| `amount` | integer(int64) | Y | 상품 단위가격 |
| `quantity` | integer(int32) | Y | 주문 수량 |
| `link` | string |  | 판매 링크 |

### PaymentProductType

상품 유형

- `PHYSICAL` — 실물 상품
- `DIGITAL` — 디지털 상품

### PgCompany

PG사

- `INICIS`
- `NICE`
- `KCP`
- `DANAL`
- `TOSSPAYMENTS`
- `MOBILIANS`
- `KICC`
- `SMARTRO`
- `DAOU`
- `BLUEWALNUT`
- `PAYPAL`
- `ALIPAY`
- `EXIMBAY`
- `PAYMENTWALL`
- `SETTLE`
- `GALAXIA`
- `NAVERPAY`
- `KAKAOPAY`
- `SMILEPAY`
- `KAKAO`
- `TOSSPAY`
- `CHAI`
- `PAYCO`
- `PAYPLE`
- `SYRUP`
- `KSNET`
- `WELCOME`
- `JTNET`
- `KPN`
- `HYPHEN`
- `PAYLETTER`
- `TRIPLE_A`
- `INNOPAY`

### PgProvider

PG사 결제 모듈

- `HTML5_INICIS`
- `PAYPAL`
- `PAYPAL_V2`
- `INICIS`
- `DANAL`
- `NICE`
- `DANAL_TPAY`
- `JTNET`
- `UPLUS`
- `NAVERPAY`
- `KAKAO`
- `SETTLE`
- `KCP`
- `MOBILIANS`
- `KAKAOPAY`
- `NAVERCO`
- `SYRUP`
- `KICC`
- `EXIMBAY`
- `SMILEPAY`
- `PAYCO`
- `KCP_BILLING`
- `ALIPAY`
- `PAYPLE`
- `CHAI`
- `BLUEWALNUT`
- `SMARTRO`
- `SMARTRO_V2`
- `PAYMENTWALL`
- `TOSSPAYMENTS`
- `KCP_QUICK`
- `DAOU`
- `GALAXIA`
- `TOSSPAY`
- `KCP_DIRECT`
- `SETTLE_ACC`
- `SETTLE_FIRM`
- `INICIS_UNIFIED`
- `KSNET`
- `PINPAY`
- `NICE_V2`
- `TOSS_BRANDPAY`
- `WELCOME`
- `TOSSPAY_V2`
- `INICIS_V2`
- `KPN`
- `KCP_V2`
- `HYPHEN`
- `EXIMBAY_V2`
- `INICIS_JP`
- `PAYLETTER_GLOBAL`
- `MOBILIANS_V2`
- `TRIPLE_A`
- `KICC_V2`
- `INNOPAY`
- `HECTO_GLOBAL`
- `PAYLETTER`

### PortOneVersion

포트원 버전

- `V1`
- `V2`

### SelectedChannel

(결제, 본인인증 등에) 선택된 채널 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | [SelectedChannelType](#selectedchanneltype) | Y | 채널 타입 |
| `id` | string |  | 채널 아이디 |
| `key` | string |  | 채널 키 |
| `name` | string |  | 채널 명 |
| `pgProvider` | [PgProvider](#pgprovider) | Y | PG사 결제 모듈 |
| `pgMerchantId` | string | Y | PG사 고객사 식별 아이디 |

### SelectedChannelType

채널 타입

- `LIVE` — 실 연동 채널
- `TEST` — 테스트 연동 채널

### SeparatedAddress

분리 형식 주소 — 분리 형식 주소<br>한 줄 형식 주소와 분리 형식 주소 모두 존재합니다.<br>한 줄 형식 주소는 분리 형식 주소를 이어 붙인 형태로 생성됩니다.

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `type` | string | Y |  |
| `oneLine` | string | Y | 주소 (한 줄) |
| `addressLine1` | string | Y | 상세 주소 1 |
| `addressLine2` | string | Y | 상세 주소 2 |
| `city` | string |  | 시/군/구 |
| `province` | string |  | 주/도/시 |
| `country` | [Country](#country) |  | 국가 |

### SeparatedAddressInput

분리 형식 주소 입력 정보

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `addressLine1` | string | Y | 상세 주소 1 |
| `addressLine2` | string | Y | 상세 주소 2 |
| `city` | string |  | 시/군/구 |
| `province` | string |  | 주/도/시 |
| `country` | [Country](#country) |  | 국가 |

### SortOrder

정렬 방식

- `DESC` — 내림차순
- `ASC` — 오름차순
