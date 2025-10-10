use crate::model::PostbackParams;

pub struct TracknowApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl TracknowApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn postback<'a>(&self, params: &PostbackParams<'a>) -> Result<(), String> {
        let url = format!("{}/postback", self.base_url);

        let res = self
            .client
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("HTTP {}: {}", status, body))
        }
    }
}
