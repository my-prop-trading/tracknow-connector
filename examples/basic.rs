use tracknow_connector::{client::TracknowApiClient, model::PostbackParams};

#[tokio::main]
async fn main() {
    let base_url = std::env::var("TRACKNOW_URL").unwrap();
    let client = TracknowApiClient::new(base_url);

    let res = client
        .postback(&PostbackParams {
            click_id: "d8b325-ff6e-4f8a-8529-f17c939f9fb",
            campaign_id: "7",
            order_id: "62fd4b33-ce16-4c04-8a31-a947c68496fd1",
            amount: "0",
            currency: "USD",
            coupon: Some("T"),
            new_customer: false,
        })
        .await;

    println!("postback: {:?}", res);
    println!("Run basic example: FINISHED");
}
