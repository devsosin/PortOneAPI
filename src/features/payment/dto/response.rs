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
        match self.status.as_str() {
            "PAID" => true,
            _ => false,
        }
    }
    pub fn get_total_amount(&self) -> i64 {
        self.amount.total
    }
    pub fn get_order_name(&self) -> &str {
        &self.order_name
    }
    pub fn get_customer_name(&self) -> &str {
        &self.customer.name.as_ref().unwrap()
    }
    pub fn get_customer_email(&self) -> &str {
        &self.customer.email.as_ref().unwrap()
    }
    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmount {
    total: i64,
    tax_free: i64,
    vat: i64,
    supply: i64,
    discount: i64,
    paid: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}
