pub mod dto;

use crate::{
    AuthedPortOneAPI,
    features::payment::dto::response::PaymentDetailData,
    types::{ClientResult, PortOneResponse},
};

pub trait PortOnePaymentService: Send + Sync {
    fn get_payment_detail(
        &self,
        payment_id: &str,
    ) -> impl Future<Output = ClientResult<PortOneResponse<PaymentDetailData>>> + Send;
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
