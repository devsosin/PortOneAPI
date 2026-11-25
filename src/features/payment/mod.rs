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
        let url = self.build_url(&["payments", payment_id], &[])?;
        self.request::<(), _>(reqwest::Method::GET, url, None, None)
            .await
    }
}
