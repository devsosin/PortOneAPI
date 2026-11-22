use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;

use crate::{AuthedPortOneAPI, features::raw_request::build_url, types::ClientResult};

/// 세금계산서 식별자 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxInvoiceKeyType {
    Supplier,
    Recipient,
    TaxInvoiceId,
}

impl TaxInvoiceKeyType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Supplier => "SUPPLIER",
            Self::Recipient => "RECIPIENT",
            Self::TaxInvoiceId => "TAX_INVOICE_ID",
        }
    }
}

/// 세금계산서 단건 조작 공통 쿼리
#[derive(Debug, Clone, Default)]
pub struct TaxInvoiceLookup<'a> {
    /// 사업자등록번호
    pub brn: Option<&'a str>,
    /// 미지정 시 `TAX_INVOICE_ID`
    pub key_type: Option<TaxInvoiceKeyType>,
    /// 테스트 모드
    pub test: bool,
}

/// PDF·팝업·프린트 URL 응답
#[derive(Debug, Clone, Deserialize)]
pub struct TaxInvoiceUrlResponse {
    pub url: String,
}

/// 세금계산서 조회 API (읽기 전용). 발행·취소는 포함하지 않는다.
pub trait PortOneTaxInvoiceService: Send + Sync {
    /// `GET /b2b/tax-invoices/{taxInvoiceKey}` — 스키마가 커서 원본 JSON 반환
    async fn get_tax_invoice(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<Value>;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}/pdf-download-url`
    async fn get_tax_invoice_pdf_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse>;

    /// `GET /b2b/tax-invoices/{taxInvoiceKey}/print-url`
    async fn get_tax_invoice_print_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse>;
}

fn lookup_query(lookup: &TaxInvoiceLookup<'_>) -> Vec<(&'static str, String)> {
    let mut query = Vec::new();
    if let Some(brn) = lookup.brn {
        query.push(("brn", brn.to_string()));
    }
    if let Some(key_type) = lookup.key_type {
        query.push(("taxInvoiceKeyType", key_type.as_str().to_string()));
    }
    if lookup.test {
        query.push(("test", "true".to_string()));
    }
    query
}

impl PortOneTaxInvoiceService for AuthedPortOneAPI {
    async fn get_tax_invoice(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<Value> {
        let url = build_url(&["b2b", "tax-invoices", key], &lookup_query(lookup))?;
        self.request::<(), _>(Method::GET, url, None).await
    }

    async fn get_tax_invoice_pdf_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        let url = build_url(
            &["b2b", "tax-invoices", key, "pdf-download-url"],
            &lookup_query(lookup),
        )?;
        self.request::<(), _>(Method::GET, url, None).await
    }

    async fn get_tax_invoice_print_url(
        &self,
        key: &str,
        lookup: &TaxInvoiceLookup<'_>,
    ) -> ClientResult<TaxInvoiceUrlResponse> {
        let url = build_url(
            &["b2b", "tax-invoices", key, "print-url"],
            &lookup_query(lookup),
        )?;
        self.request::<(), _>(Method::GET, url, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lookup_has_no_query() {
        assert!(lookup_query(&TaxInvoiceLookup::default()).is_empty());
    }

    #[test]
    fn lookup_query_and_url() {
        let lookup = TaxInvoiceLookup {
            brn: Some("1234567890"),
            key_type: Some(TaxInvoiceKeyType::Supplier),
            test: true,
        };
        let url = build_url(
            &["b2b", "tax-invoices", "k1", "print-url"],
            &lookup_query(&lookup),
        )
        .unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.portone.io/b2b/tax-invoices/k1/print-url?brn=1234567890&taxInvoiceKeyType=SUPPLIER&test=true"
        );
    }

    #[test]
    fn deserializes_url_response() {
        let res: TaxInvoiceUrlResponse =
            serde_json::from_str(r#"{"url":"https://example.com/x"}"#).unwrap();
        assert_eq!(res.url, "https://example.com/x");
    }
}
