# 에러 코드

> 출처: https://developers.portone.io/schema/v2.openapi.json (`x-portone-status-code`). 에러 응답 본문은 `{"type": "<에러 이름>", "message": "..."}` 형태이며, `type` 값이 아래 이름과 같다(PgProviderError 등은 `pgCode`·`pgMessage` 가 추가됨). 엔드포인트별 발생 가능 에러는 각 카테고리 문서의 응답 절에 있다.

| 이름 | HTTP | 설명 | 추가 필드 |
|---|---|---|---|
| <a id="alreadypaiderror"></a>`AlreadyPaidError` | 409 | 결제가 이미 완료된 경우 |  |
| <a id="alreadypaidorwaitingerror"></a>`AlreadyPaidOrWaitingError` | 409 | 결제가 이미 완료되었거나 대기중인 경우 |  |
| <a id="b2bcannotchangetaxtypeerror"></a>`B2BCannotChangeTaxTypeError` | 400 | 세금계산서 과세 유형을 수정할 수 없는 경우 |  |
| <a id="b2btaxinvoicestatusnotsendingcompletederror"></a>`B2BTaxInvoiceStatusNotSendingCompletedError` | 400 | 원본 세금계산서가 전송완료 상태가 아닌 경우 |  |
| <a id="b2bbulktaxinvoicenotfounderror"></a>`B2bBulkTaxInvoiceNotFoundError` | 404 | 일괄 세금계산서가 존재하지 않은 경우 |  |
| <a id="b2bcertificateunregisterederror"></a>`B2bCertificateUnregisteredError` | 404 | 인증서가 등록되어 있지 않은 경우 |  |
| <a id="b2bcounterpartybrninvaliderror"></a>`B2bCounterpartyBrnInvalidError` | 400 | 사업자등록번호가 유효하지 않은 경우 |  |
| <a id="b2bcounterpartybrnmodificationnotallowederror"></a>`B2bCounterpartyBrnModificationNotAllowedError` | 400 | 사업자등록번호 수정이 허용되지 않는 경우 |  |
| <a id="b2bcounterpartyidalreadyexistsbypartnererror"></a>`B2bCounterpartyIdAlreadyExistsByPartnerError` | 409 | 파트너 연동으로 생성된 거래처 ID가 이미 사용중인 경우 |  |
| <a id="b2bcounterpartyidalreadyexistserror"></a>`B2bCounterpartyIdAlreadyExistsError` | 409 | 거래처 ID가 이미 사용중인 경우 |  |
| <a id="b2bcounterpartymissingrequiredfieldserror"></a>`B2bCounterpartyMissingRequiredFieldsError` | 400 | 필수 입력 항목이 누락된 경우 |  |
| <a id="b2bcounterpartynotfounderror"></a>`B2bCounterpartyNotFoundError` | 404 | 거래처가 존재하지 않는 경우 | `counterpartyId` |
| <a id="b2bcounterpartyntsconnectionfailederror"></a>`B2bCounterpartyNtsConnectionFailedError` | 502 | 국세청 연동에 실패한 경우 |  |
| <a id="b2bcounterpartyntsnotconnectederror"></a>`B2bCounterpartyNtsNotConnectedError` | 400 | 국세청에 연동되어 있지 않은 경우 | `brn`, `counterpartyId` |
| <a id="b2bcounterpartyongoingtaxinvoiceexistserror"></a>`B2bCounterpartyOngoingTaxInvoiceExistsError` | 409 | 진행 중인 세금계산서가 존재하여 거래처를 삭제할 수 없는 경우 |  |
| <a id="b2bcounterpartypartnernotconnectableerror"></a>`B2bCounterpartyPartnerNotConnectableError` | 403 | 파트너 연동 거래처는 국세청 연동이 허용되지 않는 경우 |  |
| <a id="b2bcounterpartypartnernotdeletableerror"></a>`B2bCounterpartyPartnerNotDeletableError` | 403 | 파트너 연동 거래처는 삭제할 수 없는 경우 |  |
| <a id="b2bcounterpartypartnernotupdatableerror"></a>`B2bCounterpartyPartnerNotUpdatableError` | 403 | 파트너 연동 거래처는 수정할 수 없는 경우 |  |
| <a id="b2bcounterpartyselforiginbrnmismatcherror"></a>`B2bCounterpartySelfOriginBrnMismatchError` | 400 | 자사 사업자등록번호와 동일한 거래처를 생성할 수 없는 경우 |  |
| <a id="b2bcounterpartytoomanyadditionalcontactserror"></a>`B2bCounterpartyTooManyAdditionalContactsError` | 400 | 추가 담당자가 너무 많은 경우 |  |
| <a id="b2bcounterpartyverificationbrnmismatcherror"></a>`B2bCounterpartyVerificationBrnMismatchError` | 400 | 검증 결과의 사업자등록번호가 일치하지 않는 경우 |  |
| <a id="b2bcounterpartyverificationinvaliderror"></a>`B2bCounterpartyVerificationInvalidError` | 400 | 검증 결과가 유효하지 않은 경우 |  |
| <a id="b2bcounterpartyverificationnotfounderror"></a>`B2bCounterpartyVerificationNotFoundError` | 404 | 검증 결과를 찾을 수 없는 경우 |  |
| <a id="b2bcounterpartyverificationtypemismatcherror"></a>`B2bCounterpartyVerificationTypeMismatchError` | 400 | 검증 유형이 일치하지 않는 경우 |  |
| <a id="b2bdocumentkeycannotbechangederror"></a>`B2bDocumentKeyCannotBeChangedError` | 400 | 문서번호 수정이 요청된 경우 |  |
| <a id="b2bexternalserviceerror"></a>`B2bExternalServiceError` | 502 | 외부 서비스에서 에러가 발생한 경우 |  |
| <a id="b2bfilenotfounderror"></a>`B2bFileNotFoundError` | 404 | 업로드한 파일을 찾을 수 없는 경우 |  |
| <a id="b2bidalreadyexistserror"></a>`B2bIdAlreadyExistsError` | 409 | ID가 이미 사용중인 경우 |  |
| <a id="b2bissuancetypemismatcherror"></a>`B2bIssuanceTypeMismatchError` | 400 | 세금계산서 발행 유형이 올바르지 않은 경우 |  |
| <a id="b2bmodificationnotprovidederror"></a>`B2bModificationNotProvidedError` | 400 | 세금계산서 수정 입력 정보를 찾을 수 없는 경우 |  |
| <a id="b2bnotenablederror"></a>`B2bNotEnabledError` | 403 | B2B 기능이 활성화되지 않은 경우 |  |
| <a id="b2boriginaltaxinvoicenotfounderror"></a>`B2bOriginalTaxInvoiceNotFoundError` | 404 | 원본 세금계산서가 존재하지 않은 경우 |  |
| <a id="b2brecipientnotfounderror"></a>`B2bRecipientNotFoundError` | 404 | 공급받는자가 존재하지 않은 경우 | `brn`, `counterpartyId` |
| <a id="b2bsuppliernotfounderror"></a>`B2bSupplierNotFoundError` | 404 | 공급자가 존재하지 않은 경우 | `brn`, `counterpartyId` |
| <a id="b2btaxinvoiceattachmentnotfounderror"></a>`B2bTaxInvoiceAttachmentNotFoundError` | 404 | 세금계산서의 첨부파일을 찾을 수 없는 경우 |  |
| <a id="b2btaxinvoicenorecipientdocumentkeyerror"></a>`B2bTaxInvoiceNoRecipientDocumentKeyError` | 400 | 세금계산서에 공급받는자 문서 번호가 기입되지 않은 경우 |  |
| <a id="b2btaxinvoicenosupplierdocumentkeyerror"></a>`B2bTaxInvoiceNoSupplierDocumentKeyError` | 400 | 세금계산서에 공급자 문서 번호가 기입되지 않은 경우 |  |
| <a id="b2btaxinvoicenondeletablestatuserror"></a>`B2bTaxInvoiceNonDeletableStatusError` | 400 | 세금계산서가 삭제 가능한 상태가 아닌 경우 |  |
| <a id="b2btaxinvoicenotdraftedstatuserror"></a>`B2bTaxInvoiceNotDraftedStatusError` | 400 | 세금계산서가 임시저장 완료 상태가 아닌 경우 |  |
| <a id="b2btaxinvoicenotfounderror"></a>`B2bTaxInvoiceNotFoundError` | 404 | 세금계산서가 존재하지 않은 경우 |  |
| <a id="b2btaxinvoicenotissuedstatuserror"></a>`B2bTaxInvoiceNotIssuedStatusError` | 400 | 세금계산서가 발행된(ISSUED) 상태가 아닌 경우 |  |
| <a id="b2btaxinvoicenotrequestedstatuserror"></a>`B2bTaxInvoiceNotRequestedStatusError` | 400 | 세금계산서가 역발행 대기 상태가 아닌 경우 |  |
| <a id="b2btaxinvoicerecipientdocumentkeyalreadyusederror"></a>`B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError` | 400 | 세금계산서에 공급 받는자 문서 번호가 이미 사용 중인 경우 |  |
| <a id="b2btaxinvoicesupplierdocumentkeyalreadyusederror"></a>`B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError` | 400 | 세금계산서에 공급자 문서 번호가 이미 사용 중인 경우 |  |
| <a id="billingkeyalreadydeletederror"></a>`BillingKeyAlreadyDeletedError` | 409 | 빌링키가 이미 삭제된 경우 |  |
| <a id="billingkeyalreadyissuederror"></a>`BillingKeyAlreadyIssuedError` | 409 | BillingKeyAlreadyIssuedError |  |
| <a id="billingkeynotfounderror"></a>`BillingKeyNotFoundError` | 404 | 빌링키가 존재하지 않는 경우 |  |
| <a id="billingkeynotissuederror"></a>`BillingKeyNotIssuedError` | 404 | BillingKeyNotIssuedError |  |
| <a id="cancelamountexceedscancellableamounterror"></a>`CancelAmountExceedsCancellableAmountError` | 409 | 결제 취소 금액이 취소 가능 금액을 초과한 경우 |  |
| <a id="canceltaxamountexceedscancellabletaxamounterror"></a>`CancelTaxAmountExceedsCancellableTaxAmountError` | 409 | 취소 과세 금액이 취소 가능한 과세 금액을 초과한 경우 |  |
| <a id="canceltaxfreeamountexceedscancellabletaxfreeamounterror"></a>`CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError` | 409 | 취소 면세 금액이 취소 가능한 면세 금액을 초과한 경우 |  |
| <a id="cancellableamountconsistencybrokenerror"></a>`CancellableAmountConsistencyBrokenError` | 409 | 취소 가능 잔액 검증에 실패한 경우 |  |
| <a id="cashreceiptalreadyissuederror"></a>`CashReceiptAlreadyIssuedError` | 409 | 현금영수증이 이미 발급된 경우 |  |
| <a id="cashreceiptnotfounderror"></a>`CashReceiptNotFoundError` | 404 | 현금영수증이 존재하지 않는 경우 |  |
| <a id="cashreceiptnotissuederror"></a>`CashReceiptNotIssuedError` | 404 | 현금영수증이 발급되지 않은 경우 |  |
| <a id="channelnotfounderror"></a>`ChannelNotFoundError` | 404 | 요청된 채널이 존재하지 않는 경우 |  |
| <a id="channelspecificerror"></a>`ChannelSpecificError` | 502 | 여러 채널을 지정한 요청에서, 채널 각각에서 오류가 발생한 경우 | `failures`, `succeededChannels` |
| <a id="channelspecificfailureinvalidrequest"></a>`ChannelSpecificFailureInvalidRequest` | 400 | 요청된 입력 정보가 유효하지 않은 경우 | `channel` |
| <a id="channelspecificfailurepgprovider"></a>`ChannelSpecificFailurePgProvider` | 502 | PG사에서 오류를 전달한 경우 | `channel`, `pgCode`, `pgMessage` |
| <a id="discountamountexceedstotalamounterror"></a>`DiscountAmountExceedsTotalAmountError` | 400 | 프로모션 할인 금액이 결제 시도 금액 이상인 경우 |  |
| <a id="forbiddenerror"></a>`ForbiddenError` | 403 | 요청이 거절된 경우 |  |
| <a id="identityverificationalreadysenterror"></a>`IdentityVerificationAlreadySentError` | 409 | 본인인증 건이 이미 API로 요청된 상태인 경우 |  |
| <a id="identityverificationalreadyverifiederror"></a>`IdentityVerificationAlreadyVerifiedError` | 409 | 본인인증 건이 이미 인증 완료된 상태인 경우 |  |
| <a id="identityverificationnotfounderror"></a>`IdentityVerificationNotFoundError` | 404 | 요청된 본인인증 건이 존재하지 않는 경우 |  |
| <a id="identityverificationnotsenterror"></a>`IdentityVerificationNotSentError` | 404 | 본인인증 건이 API로 요청된 상태가 아닌 경우 |  |
| <a id="informationmismatcherror"></a>`InformationMismatchError` | 409 | 정보가 일치하지 않는 경우 |  |
| <a id="invalidpaymenttokenerror"></a>`InvalidPaymentTokenError` | 400 | 유효하지 않은 결제 토큰인 경우 |  |
| <a id="invalidrequesterror"></a>`InvalidRequestError` | 400 | 요청된 입력 정보가 유효하지 않은 경우 |  |
| <a id="maxcancelcountreachederror"></a>`MaxCancelCountReachedError` | 409 | 취소 시도 횟수가 초과된 경우 |  |
| <a id="maxtransactioncountreachederror"></a>`MaxTransactionCountReachedError` | 400 | 결제 혹은 본인인증 시도 횟수가 최대에 도달한 경우 |  |
| <a id="maxttlexceedederror"></a>`MaxTtlExceededError` | 400 | 요청된 TTL이 정책 상한을 초과한 경우 |  |
| <a id="maxwebhookretrycountreachederror"></a>`MaxWebhookRetryCountReachedError` | 400 | 동일한 webhook id에 대한 수동 재시도 횟수가 최대에 도달한 경우 |  |
| <a id="negativepromotionadjustedcancelamounterror"></a>`NegativePromotionAdjustedCancelAmountError` | 400 | 프로모션에 의해 조정된 취소 금액이 음수인 경우 |  |
| <a id="paymentalreadycancellederror"></a>`PaymentAlreadyCancelledError` | 409 | 결제가 이미 취소된 경우 |  |
| <a id="paymentcancellationnotfounderror"></a>`PaymentCancellationNotFoundError` | 404 | 결제 취소 내역이 존재하지 않는 경우 |  |
| <a id="paymentcancellationnotpendingerror"></a>`PaymentCancellationNotPendingError` | 409 | 결제 취소 내역이 취소 요청 상태가 아닌 경우 |  |
| <a id="paymentnotfounderror"></a>`PaymentNotFoundError` | 404 | 결제 건이 존재하지 않는 경우 |  |
| <a id="paymentnotpaiderror"></a>`PaymentNotPaidError` | 409 | 결제가 완료되지 않은 경우 |  |
| <a id="paymentnotwaitingfordepositerror"></a>`PaymentNotWaitingForDepositError` | 409 | 결제 건이 입금 대기 상태가 아닌 경우 |  |
| <a id="paymentschedulealreadyexistserror"></a>`PaymentScheduleAlreadyExistsError` | 409 | 결제 예약건이 이미 존재하는 경우 |  |
| <a id="paymentschedulealreadyprocessederror"></a>`PaymentScheduleAlreadyProcessedError` | 409 | 결제 예약건이 이미 처리된 경우 |  |
| <a id="paymentschedulealreadyrevokederror"></a>`PaymentScheduleAlreadyRevokedError` | 409 | 결제 예약건이 이미 취소된 경우 |  |
| <a id="paymentschedulenotfounderror"></a>`PaymentScheduleNotFoundError` | 404 | 결제 예약건이 존재하지 않는 경우 |  |
| <a id="pgprovidererror"></a>`PgProviderError` | 502 | PG사에서 오류를 전달한 경우 | `pgCode`, `pgMessage` |
| <a id="platformaccountverificationalreadyusederror"></a>`PlatformAccountVerificationAlreadyUsedError` | 409 | 파트너 계좌 검증 아이디를 이미 사용한 경우 |  |
| <a id="platformaccountverificationfailederror"></a>`PlatformAccountVerificationFailedError` | 400 | 파트너 계좌 인증이 실패한 경우 |  |
| <a id="platformaccountverificationnotfounderror"></a>`PlatformAccountVerificationNotFoundError` | 404 | 파트너 계좌 검증 아이디를 찾을 수 없는 경우 |  |
| <a id="platformadditionalfeepoliciesnotfounderror"></a>`PlatformAdditionalFeePoliciesNotFoundError` | 404 | PlatformAdditionalFeePoliciesNotFoundError | `ids`, `graphqlIds` |
| <a id="platformadditionalfeepolicyalreadyexistserror"></a>`PlatformAdditionalFeePolicyAlreadyExistsError` | 409 | PlatformAdditionalFeePolicyAlreadyExistsError |  |
| <a id="platformadditionalfeepolicynotfounderror"></a>`PlatformAdditionalFeePolicyNotFoundError` | 404 | PlatformAdditionalFeePolicyNotFoundError |  |
| <a id="platformadditionalfeepolicyschedulealreadyexistserror"></a>`PlatformAdditionalFeePolicyScheduleAlreadyExistsError` | 409 | PlatformAdditionalFeePolicyScheduleAlreadyExistsError |  |
| <a id="platformadditionalfixedamountfeecurrencyandsettlementcurrencymismatchederror"></a>`PlatformAdditionalFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError` | 400 | PlatformAdditionalFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError | `id`, `graphqlId`, `feeCurrency`, `settlementCurrency` |
| <a id="platformarchivedadditionalfeepolicyerror"></a>`PlatformArchivedAdditionalFeePolicyError` | 409 | 보관된 추가 수수료 정책을 업데이트하려고 하는 경우 |  |
| <a id="platformarchivedcontracterror"></a>`PlatformArchivedContractError` | 409 | 보관된 계약을 업데이트하려고 하는 경우 |  |
| <a id="platformarchiveddiscountsharepolicyerror"></a>`PlatformArchivedDiscountSharePolicyError` | 409 | 보관된 할인 분담 정책을 업데이트하려고 하는 경우 |  |
| <a id="platformarchivedpartnererror"></a>`PlatformArchivedPartnerError` | 409 | 보관된 파트너를 업데이트하려고 하는 경우 |  |
| <a id="platformarchivedpartnerntsnotallowederror"></a>`PlatformArchivedPartnerNtsNotAllowedError` | 409 | 보관된 파트너는 국세청 연동/연동해제를 할 수 없는 경우 |  |
| <a id="platformarchivedpartnerscannotbeschedulederror"></a>`PlatformArchivedPartnersCannotBeScheduledError` | 409 | 보관된 파트너들을 예약 업데이트하려고 하는 경우 |  |
| <a id="platformbtxnotenablederror"></a>`PlatformBtxNotEnabledError` | 403 | BTX 기능이 활성화되지 않아 요청을 처리할 수 없는 경우 |  |
| <a id="platformbulkpayoutidalreadyexistserror"></a>`PlatformBulkPayoutIdAlreadyExistsError` | 409 | 일괄 지급 아이디가 이미 존재하는 경우 |  |
| <a id="platformcancelordertransfersexistserror"></a>`PlatformCancelOrderTransfersExistsError` | 400 | PlatformCancelOrderTransfersExistsError |  |
| <a id="platformcancellableamountexceedederror"></a>`PlatformCancellableAmountExceededError` | 400 | 취소 가능한 금액이 초과한 경우 | `cancellableAmount`, `requestAmount`, `amountType` |
| <a id="platformcancellablediscountamountexceedederror"></a>`PlatformCancellableDiscountAmountExceededError` | 400 | PlatformCancellableDiscountAmountExceededError | `discountSharePolicyId`, `discountSharePolicyGraphqlId`, `cancellableAmount`, `requestAmount`, `productId` |
| <a id="platformcancellablediscounttaxfreeamountexceedederror"></a>`PlatformCancellableDiscountTaxFreeAmountExceededError` | 400 | PlatformCancellableDiscountTaxFreeAmountExceededError | `discountSharePolicyId`, `discountSharePolicyGraphqlId`, `cancellableAmount`, `requestAmount`, `productId` |
| <a id="platformcancellableproductquantityexceedederror"></a>`PlatformCancellableProductQuantityExceededError` | 400 | PlatformCancellableProductQuantityExceededError | `productId`, `cancellableQuantity` |
| <a id="platformcancellationandpaymenttypemismatchederror"></a>`PlatformCancellationAndPaymentTypeMismatchedError` | 400 | PlatformCancellationAndPaymentTypeMismatchedError |  |
| <a id="platformcancellationnotfounderror"></a>`PlatformCancellationNotFoundError` | 404 | PlatformCancellationNotFoundError |  |
| <a id="platformcannotarchivescheduledadditionalfeepolicyerror"></a>`PlatformCannotArchiveScheduledAdditionalFeePolicyError` | 409 | 예약된 업데이트가 있는 추가 수수료 정책을 보관하려고 하는 경우 |  |
| <a id="platformcannotarchivescheduledcontracterror"></a>`PlatformCannotArchiveScheduledContractError` | 409 | 예약된 업데이트가 있는 계약을 보관하려고 하는 경우 |  |
| <a id="platformcannotarchivescheduleddiscountsharepolicyerror"></a>`PlatformCannotArchiveScheduledDiscountSharePolicyError` | 409 | 예약된 업데이트가 있는 할인 분담 정책을 보관하려고 하는 경우 |  |
| <a id="platformcannotarchivescheduledpartnererror"></a>`PlatformCannotArchiveScheduledPartnerError` | 409 | 예약된 업데이트가 있는 파트너를 보관하려고 하는 경우 |  |
| <a id="platformcannotspecifytransfererror"></a>`PlatformCannotSpecifyTransferError` | 400 | 정산 건 식별에 실패한 경우 |  |
| <a id="platformcompanynotfounderror"></a>`PlatformCompanyNotFoundError` | 404 | 사업자 정보를 찾을 수 없는 경우 |  |
| <a id="platformcompanyverificationalreadyusederror"></a>`PlatformCompanyVerificationAlreadyUsedError` | 409 | 파트너 사업자 검증 아이디를 이미 사용한 경우 |  |
| <a id="platformcontractalreadyexistserror"></a>`PlatformContractAlreadyExistsError` | 409 | PlatformContractAlreadyExistsError |  |
| <a id="platformcontractnotfounderror"></a>`PlatformContractNotFoundError` | 404 | PlatformContractNotFoundError |  |
| <a id="platformcontractplatformfixedamountfeecurrencyandsettlementcurrencymismatchederror"></a>`PlatformContractPlatformFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError` | 400 | PlatformContractPlatformFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError | `id`, `graphqlId`, `feeCurrency`, `settlementCurrency` |
| <a id="platformcontractschedulealreadyexistserror"></a>`PlatformContractScheduleAlreadyExistsError` | 409 | PlatformContractScheduleAlreadyExistsError |  |
| <a id="platformcontractsnotfounderror"></a>`PlatformContractsNotFoundError` | 404 | PlatformContractsNotFoundError | `ids`, `graphqlIds` |
| <a id="platformcounterpartynotconnectablestatuserror"></a>`PlatformCounterpartyNotConnectableStatusError` | 400 | 파트너 거래처 연동 상태가 연동 가능한 상태가 아닌 경우 |  |
| <a id="platformcounterpartynotconnectederror"></a>`PlatformCounterpartyNotConnectedError` | 400 | 파트너가 거래처로 연동 되어있지 않은 경우 |  |
| <a id="platformcounterpartyongoingtaxinvoiceexistserror"></a>`PlatformCounterpartyOngoingTaxInvoiceExistsError` | 409 | 연동된 거래처에 진행 중인 세금계산서가 있는 경우 |  |
| <a id="platformcurrencynotsupportederror"></a>`PlatformCurrencyNotSupportedError` | 400 | 지원 되지 않는 통화를 선택한 경우 |  |
| <a id="platformdiscountsharepoliciesnotfounderror"></a>`PlatformDiscountSharePoliciesNotFoundError` | 404 | PlatformDiscountSharePoliciesNotFoundError | `ids`, `graphqlIds` |
| <a id="platformdiscountsharepolicyalreadyexistserror"></a>`PlatformDiscountSharePolicyAlreadyExistsError` | 409 | PlatformDiscountSharePolicyAlreadyExistsError |  |
| <a id="platformdiscountsharepolicyidduplicatederror"></a>`PlatformDiscountSharePolicyIdDuplicatedError` | 400 | PlatformDiscountSharePolicyIdDuplicatedError | `id`, `graphqlId` |
| <a id="platformdiscountsharepolicynotfounderror"></a>`PlatformDiscountSharePolicyNotFoundError` | 404 | PlatformDiscountSharePolicyNotFoundError |  |
| <a id="platformdiscountsharepolicyschedulealreadyexistserror"></a>`PlatformDiscountSharePolicyScheduleAlreadyExistsError` | 409 | PlatformDiscountSharePolicyScheduleAlreadyExistsError |  |
| <a id="platformduplicatedpartnersettlementidserror"></a>`PlatformDuplicatedPartnerSettlementIdsError` | 400 | 선택된 정산건 아이디에 중복이 있는 경우 |  |
| <a id="platformexternalapifailederror"></a>`PlatformExternalApiFailedError` | 503 | 외부 api 오류 |  |
| <a id="platformexternalapitemporarilyfailederror"></a>`PlatformExternalApiTemporarilyFailedError` | 503 | 외부 api의 일시적인 오류 |  |
| <a id="platforminsufficientdatatochangepartnertypeerror"></a>`PlatformInsufficientDataToChangePartnerTypeError` | 400 | 파트너 타입 수정에 필요한 데이터가 부족한 경우 |  |
| <a id="platformmembercompanyconnectedpartnerbrnunchangeableerror"></a>`PlatformMemberCompanyConnectedPartnerBrnUnchangeableError` | 409 | 연동 사업자로 연동된 파트너의 사업자등록번호를 변경하려고 시도한 경우 |  |
| <a id="platformmembercompanyconnectedpartnercannotbeschedulederror"></a>`PlatformMemberCompanyConnectedPartnerCannotBeScheduledError` | 409 | 연동 사업자로 연동된 파트너를 예약 수정하려고 시도한 경우 |  |
| <a id="platformmembercompanyconnectedpartnertypeunchangeableerror"></a>`PlatformMemberCompanyConnectedPartnerTypeUnchangeableError` | 409 | 연동 사업자로 연동된 파트너의 파트너 유형을 변경하려고 시도한 경우 |  |
| <a id="platformmembercompanyconnectedpartnerscannotbeschedulederror"></a>`PlatformMemberCompanyConnectedPartnersCannotBeScheduledError` | 409 | 연동 사업자로 연동된 파트너들을 예약 수정하려고 시도한 경우 | `ids`, `graphqlIds` |
| <a id="platformnegativeamountnotallowederror"></a>`PlatformNegativeAmountNotAllowedError` | 400 | 정산 건별 옵션이 켜진 플랫폼에서 음수 금액 수기 정산 생성을 시도한 경우 |  |
| <a id="platformnegativepayoutamountpartnerserror"></a>`PlatformNegativePayoutAmountPartnersError` | 400 | 지급 금액의 총합이 음수인 파트너가 존재하는 경우 |  |
| <a id="platformnoselectedpartnersettlementserror"></a>`PlatformNoSelectedPartnerSettlementsError` | 400 | 선택된 정산건이 없는 경우 |  |
| <a id="platformnondeletablepartnersettlementserror"></a>`PlatformNonDeletablePartnerSettlementsError` | 400 | 삭제할 수 없는 정산건이 포함된 경우 | `ids`, `graphqlIds` |
| <a id="platformnonpayablepartnersettlementserror"></a>`PlatformNonPayablePartnerSettlementsError` | 400 | 지급할 수 없는 정산건이 포함된 경우 | `ids`, `graphqlIds` |
| <a id="platformnotenablederror"></a>`PlatformNotEnabledError` | 403 | 플랫폼 기능이 활성화되지 않아 요청을 처리할 수 없는 경우 |  |
| <a id="platformnotsupportedbankerror"></a>`PlatformNotSupportedBankError` | 400 | 지원하지 않는 은행인 경우 |  |
| <a id="platformongoingtaxinvoiceexistserror"></a>`PlatformOngoingTaxInvoiceExistsError` | 409 | 진행 중인 세금계산서가 존재하는 경우 |  |
| <a id="platformorderdetailmismatchederror"></a>`PlatformOrderDetailMismatchedError` | 400 | PlatformOrderDetailMismatchedError |  |
| <a id="platformordertransferalreadycancellederror"></a>`PlatformOrderTransferAlreadyCancelledError` | 400 | PlatformOrderTransferAlreadyCancelledError |  |
| <a id="platformpartneridalreadyexistserror"></a>`PlatformPartnerIdAlreadyExistsError` | 409 | PlatformPartnerIdAlreadyExistsError |  |
| <a id="platformpartneridsalreadyexisterror"></a>`PlatformPartnerIdsAlreadyExistError` | 409 | PlatformPartnerIdsAlreadyExistError | `ids`, `graphqlIds` |
| <a id="platformpartneridsduplicatederror"></a>`PlatformPartnerIdsDuplicatedError` | 400 | PlatformPartnerIdsDuplicatedError | `ids`, `graphqlIds` |
| <a id="platformpartnernotfounderror"></a>`PlatformPartnerNotFoundError` | 404 | PlatformPartnerNotFoundError |  |
| <a id="platformpartnerpendingntsoperationerror"></a>`PlatformPartnerPendingNtsOperationError` | 409 | 파트너의 국세청 연동/해제가 진행 중인 경우 |  |
| <a id="platformpartnerschedulealreadyexistserror"></a>`PlatformPartnerScheduleAlreadyExistsError` | 409 | PlatformPartnerScheduleAlreadyExistsError |  |
| <a id="platformpartnerscheduleexistserror"></a>`PlatformPartnerScheduleExistsError` | 409 | 파트너 수정 예약 건이 존재하는 경우 |  |
| <a id="platformpartnerschedulesalreadyexisterror"></a>`PlatformPartnerSchedulesAlreadyExistError` | 409 | PlatformPartnerSchedulesAlreadyExistError | `ids`, `graphqlIds` |
| <a id="platformpartnersettlementsnotfounderror"></a>`PlatformPartnerSettlementsNotFoundError` | 404 | 요청한 정산건 목록을 찾을 수 없는 경우 | `ids`, `graphqlIds` |
| <a id="platformpartnertaxationtypeissimpleerror"></a>`PlatformPartnerTaxationTypeIsSimpleError` | 400 | 파트너의 과세 유형이 간이 과세 세금계산서 미발행 유형인 경우 |  |
| <a id="platformpartnertypeisnotbusinesserror"></a>`PlatformPartnerTypeIsNotBusinessError` | 400 | 파트너 유형이 사업자가 아닌 경우 |  |
| <a id="platformpaymentnotfounderror"></a>`PlatformPaymentNotFoundError` | 404 | PlatformPaymentNotFoundError |  |
| <a id="platformproductidduplicatederror"></a>`PlatformProductIdDuplicatedError` | 400 | PlatformProductIdDuplicatedError | `id` |
| <a id="platformproductidnotfounderror"></a>`PlatformProductIdNotFoundError` | 404 | PlatformProductIdNotFoundError | `id` |
| <a id="platformreferencedcancelordertransfersexisterror"></a>`PlatformReferencedCancelOrderTransfersExistError` | 400 | 취소 정산건이 참조 중인 정산건이 포함된 경우 | `ids` |
| <a id="platformsettlementamountexceedederror"></a>`PlatformSettlementAmountExceededError` | 400 | 정산 가능한 금액을 초과한 경우 | `productId`, `requestedAmount`, `allowedAmount` |
| <a id="platformsettlementcancelamountexceededportonecancelerror"></a>`PlatformSettlementCancelAmountExceededPortOneCancelError` | 400 | 정산 취소 요청 금액이 포트원 결제 취소 내역의 취소 금액을 초과한 경우 | `registeredSettlementCancelAmount`, `requestSettlementCancelAmount`, `portOneCancelAmount`, `amountType` |
| <a id="platformsettlementdateearlierthansettlementstartdateerror"></a>`PlatformSettlementDateEarlierThanSettlementStartDateError` | 400 | 정산일이 정산 시작일보다 빠른 경우 | `settlementStartDate`, `settlementDate` |
| <a id="platformsettlementparameternotfounderror"></a>`PlatformSettlementParameterNotFoundError` | 404 | 정산 파라미터가 존재하지 않는 경우 |  |
| <a id="platformsettlementpaymentamountexceededportonepaymenterror"></a>`PlatformSettlementPaymentAmountExceededPortOnePaymentError` | 400 | 정산 요청 결제 금액이 포트원 결제 내역의 결제 금액을 초과한 경우 | `registeredSettlementPaymentAmount`, `requestSettlementPaymentAmount`, `portOnePaymentAmount` |
| <a id="platformsettlementsupplywithvatamountexceededportonepaymenterror"></a>`PlatformSettlementSupplyWithVatAmountExceededPortOnePaymentError` | 400 | 정산 요청 공급대가가 포트원 결제 내역의 공급대가를 초과한 경우 | `registeredSettlementSupplyWithVatAmount`, `requestSettlementSupplyWithVatAmount`, `portOneSupplyWithVatAmount` |
| <a id="platformsettlementtaxfreeamountexceededportonepaymenterror"></a>`PlatformSettlementTaxFreeAmountExceededPortOnePaymentError` | 400 | 정산 요청 면세 금액이 포트원 결제 내역의 면세 금액을 초과한 경우 | `registeredSettlementTaxFreeAmount`, `requestSettlementTaxFreeAmount`, `portOneTaxFreeAmount` |
| <a id="platformtargetpartnernotfounderror"></a>`PlatformTargetPartnerNotFoundError` | 404 | 처리 대상 파트너가 존재하지 않는 경우 |  |
| <a id="platformtransferalreadyexistserror"></a>`PlatformTransferAlreadyExistsError` | 409 | PlatformTransferAlreadyExistsError | `transferId`, `transferGraphqlId` |
| <a id="platformtransferdiscountsharepolicynotfounderror"></a>`PlatformTransferDiscountSharePolicyNotFoundError` | 404 | PlatformTransferDiscountSharePolicyNotFoundError | `discountSharePolicyId`, `discountSharePolicyGraphqlId`, `productId` |
| <a id="platformtransferidalreadyusederror"></a>`PlatformTransferIdAlreadyUsedError` | 409 | PlatformTransferIdAlreadyUsedError |  |
| <a id="platformtransfernondeletablestatuserror"></a>`PlatformTransferNonDeletableStatusError` | 400 | PlatformTransferNonDeletableStatusError |  |
| <a id="platformtransfernotfounderror"></a>`PlatformTransferNotFoundError` | 404 | PlatformTransferNotFoundError |  |
| <a id="platformuserdefinedpropertynotfounderror"></a>`PlatformUserDefinedPropertyNotFoundError` | 404 | 사용자 정의 속성이 존재 하지 않는 경우 |  |
| <a id="profilesettingsnotfounderror"></a>`ProfileSettingsNotFoundError` | 404 | 프로필 설정이 존재하지 않는 경우 |  |
| <a id="promotiondiscountretainoptionshouldnotbechangederror"></a>`PromotionDiscountRetainOptionShouldNotBeChangedError` | 400 | 프로모션 혜택 유지 옵션을 이전 부분 취소와 다른 것으로 입력한 경우 |  |
| <a id="promotionnotfounderror"></a>`PromotionNotFoundError` | 404 | 프로모션이 존재하지 않는 경우 |  |
| <a id="promotionpaymethoddoesnotmatcherror"></a>`PromotionPayMethodDoesNotMatchError` | 400 | 결제수단이 프로모션에 지정된 것과 일치하지 않는 경우 |  |
| <a id="sessionexpirederror"></a>`SessionExpiredError` | 410 | 결제 세션이 만료된 경우 |  |
| <a id="sessionnotfounderror"></a>`SessionNotFoundError` | 404 | 결제 세션이 존재하지 않는 경우 |  |
| <a id="subscriptionalreadyexistserror"></a>`SubscriptionAlreadyExistsError` | 409 | 같은 아이디의 구독이 이미 존재하는 경우 |  |
| <a id="subscriptionbillingkeyalreadyregisterederror"></a>`SubscriptionBillingKeyAlreadyRegisteredError` | 409 | 다른 구독 고객이 이미 사용 중인 빌링키인 경우 | `customerId` |
| <a id="subscriptionbillingkeyhasscheduleserror"></a>`SubscriptionBillingKeyHasSchedulesError` | 409 | 해지하려는 빌링키에 예약 결제가 걸려 있는 경우 |  |
| <a id="subscriptionbillingkeymodemismatcherror"></a>`SubscriptionBillingKeyModeMismatchError` | 409 | 구독 고객 모드와 다른 모드의 빌링키인 경우 | `customerMode`, `billingKeyMode` |
| <a id="subscriptionbillingkeynotfounderror"></a>`SubscriptionBillingKeyNotFoundError` | 404 | 구독에 사용할 수 있는 빌링키가 아닌 경우 |  |
| <a id="subscriptionbillingkeyrevokefailederror"></a>`SubscriptionBillingKeyRevokeFailedError` | 502 | 빌링키 해지가 실패한 경우 | `pgCode`, `pgMessage` |
| <a id="subscriptioncurrencymismatcherror"></a>`SubscriptionCurrencyMismatchError` | 409 | 담긴 상품들의 통화가 다른 경우 |  |
| <a id="subscriptioncustomeremailrequirederror"></a>`SubscriptionCustomerEmailRequiredError` | 409 | 인보이스 메일을 보내려는데 구독 고객의 이메일이 없는 경우 |  |
| <a id="subscriptioncustomerhasnobillingkeyerror"></a>`SubscriptionCustomerHasNoBillingKeyError` | 409 | 구독 고객에게 사용 가능한 빌링키가 없는 경우 |  |
| <a id="subscriptioncustomernotfounderror"></a>`SubscriptionCustomerNotFoundError` | 404 | 요청된 구독 고객이 존재하지 않는 경우 |  |
| <a id="subscriptioncustomernotintestmodeerror"></a>`SubscriptionCustomerNotInTestModeError` | 409 | 구독 고객이 테스트 고객이 아닌 경우 |  |
| <a id="subscriptiondiscountalreadyappliederror"></a>`SubscriptionDiscountAlreadyAppliedError` | 409 | 같은 구독 할인이 이미 붙어 있는 경우 |  |
| <a id="subscriptiondiscountalreadyexistserror"></a>`SubscriptionDiscountAlreadyExistsError` | 409 | 같은 아이디의 구독 할인이 이미 존재하는 경우 |  |
| <a id="subscriptiondiscountarchivederror"></a>`SubscriptionDiscountArchivedError` | 409 | 구독 할인이 보관된 경우 |  |
| <a id="subscriptiondiscountcurrencymismatcherror"></a>`SubscriptionDiscountCurrencyMismatchError` | 409 | 구독 할인의 통화가 구독의 통화와 다른 경우 |  |
| <a id="subscriptiondiscountnotfounderror"></a>`SubscriptionDiscountNotFoundError` | 404 | 요청된 구독 할인이 존재하지 않는 경우 |  |
| <a id="subscriptiondiscountproductsnotinsubscriptionerror"></a>`SubscriptionDiscountProductsNotInSubscriptionError` | 409 | 구독 할인의 적용 대상 상품이 구독에 담겨 있지 않은 경우 | `productIds` |
| <a id="subscriptiondiscountredeembypassederror"></a>`SubscriptionDiscountRedeemByPassedError` | 409 | 구독 할인의 적용 마감일이 지난 경우 |  |
| <a id="subscriptionintervalmismatcherror"></a>`SubscriptionIntervalMismatchError` | 409 | 담긴 정기 상품들의 청구 주기가 서로 다르거나, 시작한 구독의 청구 주기와 다른 경우 |  |
| <a id="subscriptioninvoicealreadyregeneratederror"></a>`SubscriptionInvoiceAlreadyRegeneratedError` | 409 | 이미 재생성된 구독 인보이스인 경우 |  |
| <a id="subscriptioninvoiceemailtemplateactiveerror"></a>`SubscriptionInvoiceEmailTemplateActiveError` | 409 | 활성 인보이스 메일 템플릿이라 삭제할 수 없는 경우 |  |
| <a id="subscriptioninvoiceemailtemplatenotfounderror"></a>`SubscriptionInvoiceEmailTemplateNotFoundError` | 404 | 요청된 인보이스 메일 템플릿이 존재하지 않는 경우 |  |
| <a id="subscriptioninvoicenotfounderror"></a>`SubscriptionInvoiceNotFoundError` | 404 | 요청된 구독 인보이스가 존재하지 않는 경우 |  |
| <a id="subscriptioninvoicestatusnotallowederror"></a>`SubscriptionInvoiceStatusNotAllowedError` | 409 | 현재 인보이스 상태에서 허용되지 않는 요청인 경우 | `status` |
| <a id="subscriptionnotfounderror"></a>`SubscriptionNotFoundError` | 404 | 요청된 구독이 존재하지 않는 경우 |  |
| <a id="subscriptionproductalreadyexistserror"></a>`SubscriptionProductAlreadyExistsError` | 409 | 같은 아이디의 구독 상품이 이미 존재하는 경우 |  |
| <a id="subscriptionproductarchivederror"></a>`SubscriptionProductArchivedError` | 409 | 구독 상품이 보관되어 수정할 수 없는 경우 |  |
| <a id="subscriptionproductbillingpausederror"></a>`SubscriptionProductBillingPausedError` | 409 | 구독 상품의 청구가 중지된 경우 | `productIds` |
| <a id="subscriptionproductnotfounderror"></a>`SubscriptionProductNotFoundError` | 404 | 요청된 구독 상품이 존재하지 않는 경우 |  |
| <a id="subscriptionstatusnotallowederror"></a>`SubscriptionStatusNotAllowedError` | 409 | 현재 구독 상태에서 허용되지 않는 요청인 경우 | `status` |
| <a id="sumofpartsexceedscancelamounterror"></a>`SumOfPartsExceedsCancelAmountError` | 409 | 면세 금액 등 하위 항목들의 합이 전체 취소 금액을 초과한 경우 |  |
| <a id="sumofpartsexceedstotalamounterror"></a>`SumOfPartsExceedsTotalAmountError` | 409 | 면세 금액 등 하위 항목들의 합이 전체 결제 금액을 초과한 경우 |  |
| <a id="unauthorizederror"></a>`UnauthorizedError` | 401 | 인증 정보가 올바르지 않은 경우 |  |
| <a id="webhooknotfounderror"></a>`WebhookNotFoundError` | 404 | 웹훅 내역이 존재하지 않는 경우 |  |

## 엔드포인트별 에러 묶음 (union)

- `ActivateSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceEmailTemplateNotFoundError`, `UnauthorizedError`
- `AddSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountAlreadyAppliedError`, `SubscriptionDiscountArchivedError`, `SubscriptionDiscountCurrencyMismatchError`, `SubscriptionDiscountNotFoundError`, `SubscriptionDiscountProductsNotInSubscriptionError`, `SubscriptionDiscountRedeemByPassedError`, `SubscriptionNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `ApplyEscrowLogisticsError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `UnauthorizedError`
- `ArchivePlatformAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformCannotArchiveScheduledAdditionalFeePolicyError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ArchivePlatformContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCannotArchiveScheduledContractError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ArchivePlatformDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCannotArchiveScheduledDiscountSharePolicyError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ArchivePlatformPartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCannotArchiveScheduledPartnerError`, `PlatformCounterpartyOngoingTaxInvoiceExistsError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformPartnerPendingNtsOperationError`, `UnauthorizedError`
- `ArchiveSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountNotFoundError`, `UnauthorizedError`
- `ArchiveSubscriptionProductError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `AttachB2bTaxInvoiceFileError`: `B2bExternalServiceError`, `B2bFileNotFoundError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotDraftedStatusError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CancelB2bTaxInvoiceIssuanceError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceNotIssuedStatusError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CancelB2bTaxInvoiceRequestError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNoRecipientDocumentKeyError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceNotRequestedStatusError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CancelCashReceiptError`: `CashReceiptNotFoundError`, `CashReceiptNotIssuedError`, `ForbiddenError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `CancelPaymentError`: `CancelAmountExceedsCancellableAmountError`, `CancelTaxAmountExceedsCancellableTaxAmountError`, `CancelTaxFreeAmountExceedsCancellableTaxFreeAmountError`, `CancellableAmountConsistencyBrokenError`, `ForbiddenError`, `InvalidRequestError`, `MaxCancelCountReachedError`, `NegativePromotionAdjustedCancelAmountError`, `PaymentAlreadyCancelledError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `PromotionDiscountRetainOptionShouldNotBeChangedError`, `SumOfPartsExceedsCancelAmountError`, `UnauthorizedError`
- `CancelPlatformAdditionalFeePolicyScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CancelPlatformContractScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CancelPlatformDiscountSharePolicyScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CancelPlatformPartnerScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `UnauthorizedError`
- `CancelSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `UnauthorizedError`
- `CancelSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceNotFoundError`, `SubscriptionInvoiceStatusNotAllowedError`, `UnauthorizedError`
- `CapturePaymentError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `UnauthorizedError`
- `ChannelSpecificFailure`: `ChannelSpecificFailureInvalidRequest`, `ChannelSpecificFailurePgProvider`
- `ChargeTestSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionCustomerNotInTestModeError`, `SubscriptionNotFoundError`, `SubscriptionProductBillingPausedError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `ClosePaymentSessionError`: `ForbiddenError`, `InvalidRequestError`, `SessionNotFoundError`, `UnauthorizedError`
- `CloseVirtualAccountError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotWaitingForDepositError`, `PgProviderError`, `UnauthorizedError`
- `CompletePlatformPayoutByPartnerSettlementIdsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformBulkPayoutIdAlreadyExistsError`, `PlatformDuplicatedPartnerSettlementIdsError`, `PlatformNegativePayoutAmountPartnersError`, `PlatformNoSelectedPartnerSettlementsError`, `PlatformNonPayablePartnerSettlementsError`, `PlatformNotEnabledError`, `PlatformPartnerSettlementsNotFoundError`, `UnauthorizedError`
- `ConfirmBillingKeyError`: `BillingKeyAlreadyIssuedError`, `BillingKeyNotFoundError`, `ForbiddenError`, `InformationMismatchError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `ConfirmBillingKeyIssueAndPayError`: `BillingKeyAlreadyIssuedError`, `BillingKeyNotFoundError`, `ForbiddenError`, `InformationMismatchError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `ConfirmEscrowError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `UnauthorizedError`
- `ConfirmIdentityVerificationError`: `ForbiddenError`, `IdentityVerificationAlreadyVerifiedError`, `IdentityVerificationNotFoundError`, `IdentityVerificationNotSentError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `ConfirmPaymentError`: `AlreadyPaidError`, `ForbiddenError`, `InformationMismatchError`, `InvalidPaymentTokenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PgProviderError`, `UnauthorizedError`
- `ConfirmPaymentwallDeliveryError`: `InvalidRequestError`, `PaymentNotFoundError`, `UnauthorizedError`
- `ConnectBulkPartnerCounterpartyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformBtxNotEnabledError`, `PlatformExternalApiFailedError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformTargetPartnerNotFoundError`, `UnauthorizedError`
- `ConnectPartnerCounterpartyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedPartnerError`, `PlatformArchivedPartnerNtsNotAllowedError`, `PlatformBtxNotEnabledError`, `PlatformCounterpartyNotConnectableStatusError`, `PlatformExternalApiFailedError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformPartnerScheduleExistsError`, `PlatformPartnerTaxationTypeIsSimpleError`, `PlatformPartnerTypeIsNotBusinessError`, `UnauthorizedError`
- `CreateB2bCounterpartyError`: `B2bCounterpartyBrnInvalidError`, `B2bCounterpartyIdAlreadyExistsByPartnerError`, `B2bCounterpartyIdAlreadyExistsError`, `B2bCounterpartyMissingRequiredFieldsError`, `B2bCounterpartyNtsConnectionFailedError`, `B2bCounterpartyPartnerNotConnectableError`, `B2bCounterpartySelfOriginBrnMismatchError`, `B2bCounterpartyTooManyAdditionalContactsError`, `B2bCounterpartyVerificationBrnMismatchError`, `B2bCounterpartyVerificationInvalidError`, `B2bCounterpartyVerificationNotFoundError`, `B2bCounterpartyVerificationTypeMismatchError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CreateB2bFileUploadUrlError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CreatePaymentScheduleError`: `AlreadyPaidOrWaitingError`, `BillingKeyAlreadyDeletedError`, `BillingKeyNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `PaymentScheduleAlreadyExistsError`, `SumOfPartsExceedsTotalAmountError`, `UnauthorizedError`
- `CreatePaymentSessionError`: `ForbiddenError`, `InvalidRequestError`, `MaxTtlExceededError`, `UnauthorizedError`
- `CreatePlatformAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyAlreadyExistsError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CreatePlatformContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractAlreadyExistsError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CreatePlatformDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyAlreadyExistsError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `CreatePlatformManualTransferError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCurrencyNotSupportedError`, `PlatformNegativeAmountNotAllowedError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformTransferIdAlreadyUsedError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `CreatePlatformOrderCancelTransferError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCancellableAmountExceededError`, `PlatformCancellableDiscountAmountExceededError`, `PlatformCancellableDiscountTaxFreeAmountExceededError`, `PlatformCancellableProductQuantityExceededError`, `PlatformCancellationAndPaymentTypeMismatchedError`, `PlatformCancellationNotFoundError`, `PlatformCannotSpecifyTransferError`, `PlatformDiscountSharePolicyIdDuplicatedError`, `PlatformNotEnabledError`, `PlatformOrderDetailMismatchedError`, `PlatformOrderTransferAlreadyCancelledError`, `PlatformPaymentNotFoundError`, `PlatformProductIdDuplicatedError`, `PlatformProductIdNotFoundError`, `PlatformSettlementAmountExceededError`, `PlatformSettlementCancelAmountExceededPortOneCancelError`, `PlatformSettlementDateEarlierThanSettlementStartDateError`, `PlatformTransferAlreadyExistsError`, `PlatformTransferDiscountSharePolicyNotFoundError`, `PlatformTransferIdAlreadyUsedError`, `PlatformTransferNotFoundError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `CreatePlatformOrderTransferError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePoliciesNotFoundError`, `PlatformAdditionalFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`, `PlatformContractNotFoundError`, `PlatformContractPlatformFixedAmountFeeCurrencyAndSettlementCurrencyMismatchedError`, `PlatformCurrencyNotSupportedError`, `PlatformDiscountSharePoliciesNotFoundError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformPaymentNotFoundError`, `PlatformProductIdDuplicatedError`, `PlatformSettlementAmountExceededError`, `PlatformSettlementDateEarlierThanSettlementStartDateError`, `PlatformSettlementParameterNotFoundError`, `PlatformSettlementPaymentAmountExceededPortOnePaymentError`, `PlatformSettlementSupplyWithVatAmountExceededPortOnePaymentError`, `PlatformSettlementTaxFreeAmountExceededPortOnePaymentError`, `PlatformTransferAlreadyExistsError`, `PlatformTransferIdAlreadyUsedError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `CreatePlatformPartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAccountVerificationAlreadyUsedError`, `PlatformAccountVerificationFailedError`, `PlatformAccountVerificationNotFoundError`, `PlatformCompanyVerificationAlreadyUsedError`, `PlatformContractNotFoundError`, `PlatformCurrencyNotSupportedError`, `PlatformNotEnabledError`, `PlatformPartnerIdAlreadyExistsError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `CreatePlatformPartnersError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractsNotFoundError`, `PlatformCurrencyNotSupportedError`, `PlatformNotEnabledError`, `PlatformPartnerIdsAlreadyExistError`, `PlatformPartnerIdsDuplicatedError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `CreateSubscriptionCustomerError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionAlreadyExistsError`, `SubscriptionBillingKeyAlreadyRegisteredError`, `SubscriptionBillingKeyModeMismatchError`, `SubscriptionBillingKeyNotFoundError`, `SubscriptionCurrencyMismatchError`, `SubscriptionCustomerEmailRequiredError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionCustomerNotFoundError`, `SubscriptionDiscountAlreadyAppliedError`, `SubscriptionDiscountArchivedError`, `SubscriptionDiscountCurrencyMismatchError`, `SubscriptionDiscountNotFoundError`, `SubscriptionDiscountProductsNotInSubscriptionError`, `SubscriptionDiscountRedeemByPassedError`, `SubscriptionIntervalMismatchError`, `SubscriptionProductArchivedError`, `SubscriptionProductBillingPausedError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `CreateSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountAlreadyExistsError`, `UnauthorizedError`
- `CreateSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionAlreadyExistsError`, `SubscriptionCurrencyMismatchError`, `SubscriptionCustomerEmailRequiredError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionCustomerNotFoundError`, `SubscriptionDiscountAlreadyAppliedError`, `SubscriptionDiscountArchivedError`, `SubscriptionDiscountCurrencyMismatchError`, `SubscriptionDiscountNotFoundError`, `SubscriptionDiscountProductsNotInSubscriptionError`, `SubscriptionDiscountRedeemByPassedError`, `SubscriptionIntervalMismatchError`, `SubscriptionProductArchivedError`, `SubscriptionProductBillingPausedError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `CreateSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `CreateSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionNotFoundError`, `SubscriptionProductBillingPausedError`, `SubscriptionProductNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `CreateSubscriptionProductError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductAlreadyExistsError`, `UnauthorizedError`
- `DeactivateSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceEmailTemplateNotFoundError`, `UnauthorizedError`
- `DeleteB2bCounterpartyError`: `B2bCounterpartyNotFoundError`, `B2bCounterpartyOngoingTaxInvoiceExistsError`, `B2bCounterpartyPartnerNotDeletableError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `DeleteB2bTaxInvoiceAttachmentError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceAttachmentNotFoundError`, `B2bTaxInvoiceNotDraftedStatusError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `DeleteB2bTaxInvoiceError`: `B2bBulkTaxInvoiceNotFoundError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNonDeletableStatusError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `DeleteBillingKeyError`: `BillingKeyAlreadyDeletedError`, `BillingKeyNotFoundError`, `BillingKeyNotIssuedError`, `ChannelSpecificError`, `ForbiddenError`, `InvalidRequestError`, `PaymentScheduleAlreadyExistsError`, `PgProviderError`, `UnauthorizedError`
- `DeletePlatformPartnerSettlementsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNonDeletablePartnerSettlementsError`, `PlatformNotEnabledError`, `PlatformPartnerSettlementsNotFoundError`, `PlatformReferencedCancelOrderTransfersExistError`, `UnauthorizedError`
- `DeletePlatformTransferError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCancelOrderTransfersExistsError`, `PlatformNotEnabledError`, `PlatformTransferNonDeletableStatusError`, `PlatformTransferNotFoundError`, `UnauthorizedError`
- `DeleteSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceEmailTemplateActiveError`, `SubscriptionInvoiceEmailTemplateNotFoundError`, `UnauthorizedError`
- `DisconnectBulkPartnerCounterpartyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformBtxNotEnabledError`, `PlatformExternalApiFailedError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformTargetPartnerNotFoundError`, `UnauthorizedError`
- `DisconnectPartnerCounterpartyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedPartnerNtsNotAllowedError`, `PlatformBtxNotEnabledError`, `PlatformCounterpartyNotConnectedError`, `PlatformExternalApiFailedError`, `PlatformNotEnabledError`, `PlatformOngoingTaxInvoiceExistsError`, `PlatformPartnerNotFoundError`, `PlatformPartnerTaxationTypeIsSimpleError`, `PlatformPartnerTypeIsNotBusinessError`, `UnauthorizedError`
- `DownloadB2bTaxInvoicesSheetError`: `InvalidRequestError`, `UnauthorizedError`
- `DownloadPlatformTransferSheetError`: `InvalidRequestError`, `UnauthorizedError`
- `DraftB2bTaxInvoiceError`: `B2BCannotChangeTaxTypeError`, `B2BTaxInvoiceStatusNotSendingCompletedError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bIdAlreadyExistsError`, `B2bIssuanceTypeMismatchError`, `B2bModificationNotProvidedError`, `B2bNotEnabledError`, `B2bOriginalTaxInvoiceNotFoundError`, `B2bRecipientNotFoundError`, `B2bSupplierNotFoundError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`, `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `EvaluateCheckoutProfileError`: `InvalidRequestError`, `ProfileSettingsNotFoundError`
- `GetAllPaymentEventsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetAllPaymentsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bBulkTaxInvoiceError`: `B2bBulkTaxInvoiceNotFoundError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bBusinessInfosError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bCounterpartiesError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bCounterpartyCertificateError`: `B2bCertificateUnregisteredError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bCounterpartyCertificateRegistrationUrlError`: `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bCounterpartyError`: `B2bCounterpartyNotFoundError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoiceAttachmentsError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoiceError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoicePdfDownloadUrlError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoicePopupUrlError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoicePrintUrlError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetB2bTaxInvoicesError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetBankInfosError`: `InvalidRequestError`
- `GetBillingKeyInfoError`: `BillingKeyNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetBillingKeyInfosError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetCashReceiptError`: `CashReceiptNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetCashReceiptsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetIdentityVerificationError`: `ForbiddenError`, `IdentityVerificationNotFoundError`, `InvalidRequestError`, `UnauthorizedError`
- `GetIdentityVerificationsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetKakaopayPaymentOrderError`: `InvalidRequestError`, `UnauthorizedError`
- `GetPaymentError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `UnauthorizedError`
- `GetPaymentReconciliationSettlementVatReportError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetPaymentReconciliationTransactionVatReportError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetPaymentScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PaymentScheduleNotFoundError`, `UnauthorizedError`
- `GetPaymentSchedulesError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetPaymentSessionError`: `InvalidRequestError`, `SessionExpiredError`, `SessionNotFoundError`
- `GetPaymentTransactionsError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `UnauthorizedError`
- `GetPaymentsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetPgCardPromotionsError`: `ChannelNotFoundError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `GetPlatformAccountHolderError`: `ForbiddenError`, `InvalidRequestError`, `PlatformExternalApiFailedError`, `PlatformExternalApiTemporarilyFailedError`, `PlatformNotEnabledError`, `PlatformNotSupportedBankError`, `UnauthorizedError`
- `GetPlatformAccountTransfersError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformAdditionalFeePoliciesError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformAdditionalFeePolicyScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformBulkAccountTransfersError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformBulkPayoutsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformCompanyStateError`: `ForbiddenError`, `InvalidRequestError`, `PlatformCompanyNotFoundError`, `PlatformExternalApiFailedError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformContractScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformContractsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformDiscountSharePoliciesError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformDiscountSharePolicyFilterOptionsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformDiscountSharePolicyScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformPartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `UnauthorizedError`
- `GetPlatformPartnerFilterOptionsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformPartnerScheduleError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `UnauthorizedError`
- `GetPlatformPartnerSettlementsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformPartnersError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformPayoutsError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformSettingError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPlatformTransferError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `PlatformTransferNotFoundError`, `UnauthorizedError`
- `GetPlatformTransferSummariesError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `GetPromotionError`: `ForbiddenError`, `InvalidRequestError`, `PromotionNotFoundError`, `UnauthorizedError`
- `GetSubscriptionCustomerError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerNotFoundError`, `UnauthorizedError`
- `GetSubscriptionCustomerInvoicesError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerNotFoundError`, `UnauthorizedError`
- `GetSubscriptionCustomerSubscriptionsError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerNotFoundError`, `UnauthorizedError`
- `GetSubscriptionCustomersError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountNotFoundError`, `UnauthorizedError`
- `GetSubscriptionDiscountUsagesError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `UnauthorizedError`
- `GetSubscriptionDiscountsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `UnauthorizedError`
- `GetSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceEmailTemplateNotFoundError`, `UnauthorizedError`
- `GetSubscriptionInvoiceEmailTemplatesError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceNotFoundError`, `UnauthorizedError`
- `GetSubscriptionInvoiceStatsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionInvoicesAllError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionInvoicesError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `UnauthorizedError`
- `GetSubscriptionProductError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `GetSubscriptionProductsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `GetSubscriptionsError`: `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `IssueB2bTaxInvoiceError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNoSupplierDocumentKeyError`, `B2bTaxInvoiceNotDraftedStatusError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceNotRequestedStatusError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `IssueB2bTaxInvoiceImmediatelyError`: `B2BCannotChangeTaxTypeError`, `B2BTaxInvoiceStatusNotSendingCompletedError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bIdAlreadyExistsError`, `B2bIssuanceTypeMismatchError`, `B2bModificationNotProvidedError`, `B2bNotEnabledError`, `B2bOriginalTaxInvoiceNotFoundError`, `B2bRecipientNotFoundError`, `B2bSupplierNotFoundError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`, `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `IssueBillingKeyError`: `ChannelNotFoundError`, `ChannelSpecificError`, `ForbiddenError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `IssueCashReceiptError`: `CashReceiptAlreadyIssuedError`, `ChannelNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `LoginViaApiSecretError`: `InvalidRequestError`, `UnauthorizedError`
- `ModifyEscrowLogisticsError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `UnauthorizedError`
- `PauseSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `PauseSubscriptionProductBillingError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `PayInstantlyError`: `AlreadyPaidError`, `ChannelNotFoundError`, `DiscountAmountExceedsTotalAmountError`, `ForbiddenError`, `InvalidRequestError`, `MaxTransactionCountReachedError`, `PaymentScheduleAlreadyExistsError`, `PgProviderError`, `PromotionPayMethodDoesNotMatchError`, `SumOfPartsExceedsTotalAmountError`, `UnauthorizedError`
- `PayWithBillingKeyError`: `AlreadyPaidError`, `BillingKeyAlreadyDeletedError`, `BillingKeyNotFoundError`, `ChannelNotFoundError`, `DiscountAmountExceedsTotalAmountError`, `ForbiddenError`, `InvalidRequestError`, `MaxTransactionCountReachedError`, `PaymentScheduleAlreadyExistsError`, `PgProviderError`, `PromotionPayMethodDoesNotMatchError`, `SumOfPartsExceedsTotalAmountError`, `UnauthorizedError`
- `PreRegisterPaymentError`: `AlreadyPaidError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `PreRegisterSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionAlreadyExistsError`, `SubscriptionCurrencyMismatchError`, `SubscriptionCustomerEmailRequiredError`, `SubscriptionDiscountAlreadyAppliedError`, `SubscriptionDiscountArchivedError`, `SubscriptionDiscountCurrencyMismatchError`, `SubscriptionDiscountNotFoundError`, `SubscriptionDiscountProductsNotInSubscriptionError`, `SubscriptionDiscountRedeemByPassedError`, `SubscriptionIntervalMismatchError`, `SubscriptionProductArchivedError`, `SubscriptionProductBillingPausedError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `PreviewSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `RecoverPlatformAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `RecoverPlatformContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `RecoverPlatformDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `RecoverPlatformPartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformPartnerPendingNtsOperationError`, `UnauthorizedError`
- `RefreshTokenError`: `InvalidRequestError`, `UnauthorizedError`
- `RefuseB2bTaxInvoiceRequestError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNoSupplierDocumentKeyError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceNotRequestedStatusError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `RegenerateSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionInvoiceAlreadyRegeneratedError`, `SubscriptionInvoiceNotFoundError`, `SubscriptionInvoiceStatusNotAllowedError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `RegisterStoreReceiptError`: `ForbiddenError`, `InvalidRequestError`, `PaymentNotFoundError`, `PaymentNotPaidError`, `PgProviderError`, `UnauthorizedError`
- `RemoveSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `RequestB2bTaxInvoiceReverseIssuanceError`: `B2BCannotChangeTaxTypeError`, `B2BTaxInvoiceStatusNotSendingCompletedError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bIdAlreadyExistsError`, `B2bIssuanceTypeMismatchError`, `B2bModificationNotProvidedError`, `B2bNotEnabledError`, `B2bOriginalTaxInvoiceNotFoundError`, `B2bRecipientNotFoundError`, `B2bSupplierNotFoundError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`, `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `RescheduleAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `RescheduleContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `RescheduleDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ReschedulePartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformContractNotFoundError`, `PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `UnauthorizedError`
- `ResendIdentityVerificationError`: `ForbiddenError`, `IdentityVerificationAlreadyVerifiedError`, `IdentityVerificationNotFoundError`, `IdentityVerificationNotSentError`, `InvalidRequestError`, `PgProviderError`, `UnauthorizedError`
- `ResendWebhookError`: `ForbiddenError`, `InvalidRequestError`, `MaxWebhookRetryCountReachedError`, `PaymentNotFoundError`, `UnauthorizedError`, `WebhookNotFoundError`
- `ResumeSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `ResumeSubscriptionProductBillingError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `RetrySubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerHasNoBillingKeyError`, `SubscriptionInvoiceNotFoundError`, `SubscriptionInvoiceStatusNotAllowedError`, `UnauthorizedError`
- `RevokePaymentSchedulesError`: `BillingKeyAlreadyDeletedError`, `BillingKeyNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `PaymentScheduleAlreadyProcessedError`, `PaymentScheduleAlreadyRevokedError`, `PaymentScheduleNotFoundError`, `UnauthorizedError`
- `RevokeSubscriptionCustomerPaymentMethodError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionBillingKeyHasSchedulesError`, `SubscriptionBillingKeyRevokeFailedError`, `SubscriptionCustomerNotFoundError`, `UnauthorizedError`
- `ScheduleAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformAdditionalFeePolicyScheduleAlreadyExistsError`, `PlatformArchivedAdditionalFeePolicyError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ScheduleContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedContractError`, `PlatformContractNotFoundError`, `PlatformContractScheduleAlreadyExistsError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `ScheduleDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedDiscountSharePolicyError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformDiscountSharePolicyScheduleAlreadyExistsError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `SchedulePartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAccountVerificationAlreadyUsedError`, `PlatformAccountVerificationFailedError`, `PlatformAccountVerificationNotFoundError`, `PlatformArchivedPartnerError`, `PlatformCompanyVerificationAlreadyUsedError`, `PlatformContractNotFoundError`, `PlatformInsufficientDataToChangePartnerTypeError`, `PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`, `PlatformMemberCompanyConnectedPartnerCannotBeScheduledError`, `PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformPartnerScheduleAlreadyExistsError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `SchedulePlatformPartnersError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedPartnersCannotBeScheduledError`, `PlatformContractNotFoundError`, `PlatformMemberCompanyConnectedPartnersCannotBeScheduledError`, `PlatformNotEnabledError`, `PlatformPartnerSchedulesAlreadyExistError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `SendIdentityVerificationError`: `ChannelNotFoundError`, `ForbiddenError`, `IdentityVerificationAlreadySentError`, `IdentityVerificationAlreadyVerifiedError`, `IdentityVerificationNotFoundError`, `InvalidRequestError`, `MaxTransactionCountReachedError`, `PgProviderError`, `UnauthorizedError`
- `SendToNtsB2bTaxInvoiceError`: `B2bExternalServiceError`, `B2bNotEnabledError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceNotIssuedStatusError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `SettleSubscriptionInvoiceError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceNotFoundError`, `SubscriptionInvoiceStatusNotAllowedError`, `UnauthorizedError`
- `StopPaymentCancellationError`: `ForbiddenError`, `InvalidRequestError`, `PaymentCancellationNotFoundError`, `PaymentCancellationNotPendingError`, `PaymentNotFoundError`, `PgProviderError`, `UnauthorizedError`
- `UnarchiveSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountNotFoundError`, `UnauthorizedError`
- `UpdateB2bCounterpartyError`: `B2bCounterpartyBrnModificationNotAllowedError`, `B2bCounterpartyMissingRequiredFieldsError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyPartnerNotUpdatableError`, `B2bCounterpartyTooManyAdditionalContactsError`, `B2bCounterpartyVerificationBrnMismatchError`, `B2bCounterpartyVerificationInvalidError`, `B2bCounterpartyVerificationNotFoundError`, `B2bCounterpartyVerificationTypeMismatchError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `UpdateB2bTaxInvoiceDraftError`: `B2BCannotChangeTaxTypeError`, `B2BTaxInvoiceStatusNotSendingCompletedError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bDocumentKeyCannotBeChangedError`, `B2bExternalServiceError`, `B2bIdAlreadyExistsError`, `B2bIssuanceTypeMismatchError`, `B2bModificationNotProvidedError`, `B2bNotEnabledError`, `B2bOriginalTaxInvoiceNotFoundError`, `B2bRecipientNotFoundError`, `B2bSupplierNotFoundError`, `B2bTaxInvoiceNotDraftedStatusError`, `B2bTaxInvoiceNotFoundError`, `B2bTaxInvoiceRecipientDocumentKeyAlreadyUsedError`, `B2bTaxInvoiceSupplierDocumentKeyAlreadyUsedError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `UpdatePlatformAdditionalFeePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAdditionalFeePolicyNotFoundError`, `PlatformArchivedAdditionalFeePolicyError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `UpdatePlatformContractError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedContractError`, `PlatformContractNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `UpdatePlatformDiscountSharePolicyError`: `ForbiddenError`, `InvalidRequestError`, `PlatformArchivedDiscountSharePolicyError`, `PlatformDiscountSharePolicyNotFoundError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `UpdatePlatformPartnerError`: `ForbiddenError`, `InvalidRequestError`, `PlatformAccountVerificationAlreadyUsedError`, `PlatformAccountVerificationFailedError`, `PlatformAccountVerificationNotFoundError`, `PlatformArchivedPartnerError`, `PlatformCompanyVerificationAlreadyUsedError`, `PlatformContractNotFoundError`, `PlatformInsufficientDataToChangePartnerTypeError`, `PlatformMemberCompanyConnectedPartnerBrnUnchangeableError`, `PlatformMemberCompanyConnectedPartnerTypeUnchangeableError`, `PlatformNotEnabledError`, `PlatformPartnerNotFoundError`, `PlatformUserDefinedPropertyNotFoundError`, `UnauthorizedError`
- `UpdatePlatformSettingError`: `ForbiddenError`, `InvalidRequestError`, `PlatformNotEnabledError`, `UnauthorizedError`
- `UpdateSubscriptionCustomerError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCustomerEmailRequiredError`, `SubscriptionCustomerNotFoundError`, `UnauthorizedError`
- `UpdateSubscriptionDiscountError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionDiscountNotFoundError`, `UnauthorizedError`
- `UpdateSubscriptionError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionCurrencyMismatchError`, `SubscriptionCustomerEmailRequiredError`, `SubscriptionIntervalMismatchError`, `SubscriptionNotFoundError`, `SubscriptionProductArchivedError`, `SubscriptionProductBillingPausedError`, `SubscriptionProductNotFoundError`, `SubscriptionStatusNotAllowedError`, `UnauthorizedError`
- `UpdateSubscriptionInvoiceEmailTemplateError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionInvoiceEmailTemplateNotFoundError`, `UnauthorizedError`
- `UpdateSubscriptionProductError`: `ForbiddenError`, `InvalidRequestError`, `SubscriptionProductArchivedError`, `SubscriptionProductNotFoundError`, `UnauthorizedError`
- `ValidateB2bCounterpartyCertificateError`: `B2bCertificateUnregisteredError`, `B2bCounterpartyNotFoundError`, `B2bCounterpartyNtsNotConnectedError`, `B2bExternalServiceError`, `B2bNotEnabledError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
- `requestB2bTaxInvoiceError`: `B2BCannotChangeTaxTypeError`, `B2BTaxInvoiceStatusNotSendingCompletedError`, `B2bExternalServiceError`, `B2bIssuanceTypeMismatchError`, `B2bModificationNotProvidedError`, `B2bNotEnabledError`, `B2bOriginalTaxInvoiceNotFoundError`, `B2bTaxInvoiceNoRecipientDocumentKeyError`, `B2bTaxInvoiceNotDraftedStatusError`, `B2bTaxInvoiceNotFoundError`, `ForbiddenError`, `InvalidRequestError`, `UnauthorizedError`
