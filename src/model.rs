use serde::Serialize;

#[derive(Serialize)]
pub struct PostbackParams<'a> {
    pub click_id: &'a str,
    pub campaign_id: &'a str,
    pub order_id: &'a str,
    pub amount: &'a str,
    pub currency: &'a str,
}
