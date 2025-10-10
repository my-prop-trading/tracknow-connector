use serde::Serialize;

#[derive(Serialize)]
pub struct PostbackParams<'a> {
    pub click_id: &'a str,
    pub campaign_id: &'a str,
    pub order_id: &'a str,
    pub amount: &'a str,
    pub currency: &'a str,
    pub coupon: Option<&'a str>,
    pub new_customer: bool,
}
