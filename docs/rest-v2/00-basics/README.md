# 기본

API 호출 전에 알아야 할 것: 호스트·인증 헤더·멱등 키·타임아웃(개요), 인증 토큰 API, 전체 에러 코드, 여러 문서가 공유하는 스키마, 하위호환성, 서버 SDK.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [overview.md](overview.md) | 호스트, 인증 헤더, 멱등 키, 타임아웃, 하위호환성 | |
| [errors.md](errors.md) | 전체 에러 코드(이름, HTTP 상태, 설명) | |
| [schemas-common.md](schemas-common.md) | 여러 카테고리가 공유하는 스키마 | |
| [auth.md](auth.md) | 인증 관련 API | 2 |
| [common.md](common.md) | 공통 API | 1 |

## 가이드

| 파일 | 제목 | 원본 |
|---|---|---|
| [server-sdk.md](server-sdk.md) | 서버 SDK 레퍼런스 | https://developers.portone.io/sdk/ko/v2-server-sdk/readme.md |
| [backward-compatibility.md](backward-compatibility.md) | API 하위호환성 지원 | https://developers.portone.io/api/backward-compatibility.md |

## 주요 엔드포인트

- [`POST /login/api-secret`](auth.md#post-loginapi-secret) — API secret 를 사용한 토큰 발급
- [`POST /token/refresh`](auth.md#post-tokenrefresh) — 토큰 갱신
