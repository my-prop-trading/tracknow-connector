use tracknow_connector::{client::TracknowApiClient, model::PostbackParams};

#[tokio::main]
async fn main() {
    let base_url = std::env::var("TRACKNOW_URL").unwrap();
    let client = TracknowApiClient::new(base_url);

    client
        .postback(&PostbackParams {
            click_id: "test_click_id",
            campaign_id: "7",
            order_id: "test_order_id",
            amount: "10",
            currency: "USD",
            coupon: Some("test_coupon"),
            new_customer: false,
        })
        .await
        .unwrap();

    println!("Run basic example: FINISHED");
}
