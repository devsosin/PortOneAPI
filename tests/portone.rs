#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;
    use portone::{PortOneAPI, features::payment::PortOnePaymentService};

    #[tokio::test]
    #[ignore = "calls the live PortOne API; needs PORTONE_KEY in .env"]
    async fn test() {
        dotenv().ok();

        let api = PortOneAPI::new();
        let key = env::var("PORTONE_KEY").expect("Failed to load env");

        let api = api.authed_api(&key);

        let payment_id = "TRV-20260121-2QN0";

        let response = api.get_payment_detail(payment_id).await.unwrap();

        println!("{:?}", response);
    }
}
