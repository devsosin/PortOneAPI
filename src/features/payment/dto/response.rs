use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDetailData {
    id: String,
    status: String,
    transaction_id: String,
    merchant_id: String,
    store_id: String,

    order_name: String,

    requested_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    amount: PaymentAmount,
    currency: String,

    customer: Customer,

    #[serde(default)]
    receipt_url: Option<String>,
    #[serde(default)]
    receipt_url_without_authentication: Option<String>,
}

impl PaymentDetailData {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn check_status(&self) -> bool {
        self.status == "PAID"
    }
    pub fn get_total_amount(&self) -> i64 {
        self.amount.total
    }
    pub fn get_order_name(&self) -> &str {
        &self.order_name
    }
    pub fn get_customer_name(&self) -> Option<&str> {
        self.customer.name.as_deref()
    }
    pub fn get_customer_email(&self) -> Option<&str> {
        self.customer.email.as_deref()
    }
    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn get_transaction_id(&self) -> &str {
        &self.transaction_id
    }
    pub fn get_merchant_id(&self) -> &str {
        &self.merchant_id
    }
    pub fn get_store_id(&self) -> &str {
        &self.store_id
    }
    pub fn get_requested_at(&self) -> DateTime<Utc> {
        self.requested_at
    }
    pub fn get_currency(&self) -> &str {
        &self.currency
    }
    pub fn get_customer_id(&self) -> Option<&str> {
        self.customer.id.as_deref()
    }
    pub fn get_receipt_url(&self) -> Option<&str> {
        self.receipt_url.as_deref()
    }
    pub fn get_receipt_url_without_authentication(&self) -> Option<&str> {
        self.receipt_url_without_authentication.as_deref()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmount {
    pub total: i64,
    pub tax_free: i64,
    pub vat: i64,
    pub supply: i64,
    pub discount: i64,
    pub paid: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    fn base() -> Value {
        json!({
            "id": "pay-1",
            "status": "PAID",
            "transactionId": "tx-1",
            "merchantId": "m-1",
            "storeId": "s-1",
            "orderName": "order",
            "requestedAt": "2024-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z",
            "amount": {"total": 1000, "taxFree": 0, "vat": 91, "supply": 909, "discount": 0, "paid": 1000},
            "currency": "KRW",
            "customer": {}
        })
    }

    #[test]
    fn receipt_urls_are_read_when_present() {
        let mut v = base();
        v["receiptUrl"] = json!("https://example.com/r/1");
        v["receiptUrlWithoutAuthentication"] = json!("https://example.com/r/2");
        let d: PaymentDetailData = serde_json::from_value(v).unwrap();
        assert_eq!(d.get_receipt_url(), Some("https://example.com/r/1"));
        assert_eq!(
            d.get_receipt_url_without_authentication(),
            Some("https://example.com/r/2")
        );
    }

    #[test]
    fn receipt_urls_default_to_none_when_absent() {
        let d: PaymentDetailData = serde_json::from_value(base()).unwrap();
        assert_eq!(d.get_receipt_url(), None);
        assert_eq!(d.get_receipt_url_without_authentication(), None);
    }
}
