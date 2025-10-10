use serde::Serialize;

#[derive(Serialize)]
pub struct PostbackParams<'a> {
    pub click_id: &'a str,
    pub campaign_id: &'a str,
    pub order_id: &'a str,
    pub amount: u32,
    pub currency: &'a str,
}
