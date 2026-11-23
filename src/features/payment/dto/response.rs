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
