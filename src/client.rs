use crate::model::{PostbackParams, PostbackResponse};

pub struct TracknowApiClient {
    base_url: String,
}

impl TracknowApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub async fn postback<'a>(
        &self,
        params: &PostbackParams<'a>,
    ) -> Result<PostbackResponse, String> {
        let query = serde_urlencoded::to_string(&params).unwrap();
        let url = format!("{}/postback?{}", self.base_url, query);

        let flurl = flurl::FlUrl::new(&url);
        let result = flurl.get().await;

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {} {:?}",
                url,
                result.unwrap_err()
            )
            .into());
        };

        let result = resp.receive_body().await;

        let Ok(body_bytes) = result else {
            return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
        };

        let body_str = String::from_utf8(body_bytes).unwrap();
        let result: Result<PostbackResponse, _> = serde_json::from_str(&body_str);

        let Ok(resp) = result else {
            let msg = format!(
                "Failed to deserialize: {:?}. Url: {:?} Body: {}",
                result, url, body_str
            );
            return Err(msg.into());
        };

        Ok(resp)
    }
}
