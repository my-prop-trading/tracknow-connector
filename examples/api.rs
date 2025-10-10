use tracknow_connector::{client::TracknowApiClient, model::PostbackParams};

fn main() {
    let client = TracknowApiClient::new(std::env::var("TRACKNOW_URL").unwrap());

    let result = pollster::block_on(client.postback(&PostbackParams {
        click_id: "test_click_id",
        campaign_id: "7",
        order_id: "test_order_id",
        amount: 10,
        currency: "USD",
    }));

    if let Err(err) = result {
        println!("Failed: {err}");
    }
}
