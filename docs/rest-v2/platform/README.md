# 파트너 정산(플랫폼)

파트너 정산 API(약 67개). 정책·계약, 파트너·계좌·사업자·설정, 정산·지급·이체 내역 세 문서로 나눴다. 세금계산서 웹훅은 webhook 가이드.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [policy.md](policy.md) | 파트너 정산: 정책·계약 | 31 |
| [partner.md](partner.md) | 파트너 정산: 파트너·계좌·사업자·설정 | 22 |
| [settlement.md](settlement.md) | 파트너 정산: 정산·지급·이체 내역 | 14 |
| [schemas.md](schemas.md) | 이 폴더의 문서들이 공유하는 스키마 | |

## 주요 엔드포인트

- [`POST /platform/partners`](partner.md#post-platformpartners) — 파트너 생성
- [`GET /platform/partners`](partner.md#get-platformpartners) — 파트너 다건 조회
- [`POST /platform/contracts`](policy.md#post-platformcontracts) — 계약 생성
- [`GET /platform/contracts`](policy.md#get-platformcontracts) — 계약 다건 조회
- [`POST /platform/transfers/order`](settlement.md#post-platformtransfersorder) — 주문 정산건 생성
- [`GET /platform/transfer-summaries`](settlement.md#get-platformtransfer-summaries) — 정산건 다건 조회
- [`GET /platform/partner-settlements`](settlement.md#get-platformpartner-settlements) — 정산 내역 다건 조회
- [`GET /platform/payouts`](settlement.md#get-platformpayouts) — 지급 내역 다건 조회

## 관련

- [세금계산서 웹훅 가이드](../webhook/platform-webhook-tax-invoice.md)
