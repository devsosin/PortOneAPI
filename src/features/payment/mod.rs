pub mod dto;

use crate::{
    AuthedPortOneAPI,
    features::payment::dto::response::PaymentDetailData,
    types::{ClientResult, PortOneResponse},
};

pub trait PortOnePaymentService: Send + Sync {
    async fn get_payment_detail(
        &self,
        payment_id: &str,
    ) -> ClientResult<PortOneResponse<PaymentDetailData>>; //
}

impl PortOnePaymentService for AuthedPortOneAPI {
    async fn get_payment_detail(
        &self,
        payment_id: &str,
    ) -> ClientResult<PortOneResponse<PaymentDetailData>> {
        let uri = format!("payments/{}", payment_id);
        let result = self.send(&uri).await?;

        Ok(result)
    }
}
