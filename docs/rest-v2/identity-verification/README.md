# 본인인증

본인인증 요청·확인·조회·SMS 재전송.

## 파일

| 파일 | 내용 | 엔드포인트 |
|---|---|---|
| [identityVerification.md](identityVerification.md) | 본인인증 관련 API | 5 |

## 주요 엔드포인트

- [`POST /identity-verifications/{identityVerificationId}/confirm`](identityVerification.md#post-identity-verificationsidentityverificationidconfirm) — 본인인증 확인
- [`POST /identity-verifications/{identityVerificationId}/resend`](identityVerification.md#post-identity-verificationsidentityverificationidresend) — SMS 본인인증 요청 재전송
- [`POST /identity-verifications/{identityVerificationId}/send`](identityVerification.md#post-identity-verificationsidentityverificationidsend) — 본인인증 요청 전송
- [`GET /identity-verifications/{identityVerificationId}`](identityVerification.md#get-identity-verificationsidentityverificationid) — 본인인증 단건 조회
- [`GET /identity-verifications`](identityVerification.md#get-identity-verifications) — 본인인증 내역 다건 조회
