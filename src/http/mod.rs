use crate::Result;
use reqwest::{Client as ReqwestClient, Method};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

pub struct Http {
    client: ReqwestClient,
    token: String,
    base_url: Url,
}

impl Http {
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Ok(Self {
            client: ReqwestClient::new(),
            token: token.into(),
            base_url: Url::parse("https://api.revolt.chat")?,
        })
    }

    pub async fn request<T>(&self, method: Method, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_with_data::<T, ()>(method, path, None).await
    }

    pub async fn request_with_data<T, D>(
        &self,
        method: Method,
        path: &str,
        data: Option<D>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        D: Serialize,
    {
        let url = self.base_url.join(path)?;

        let mut request = self
            .client
            .request(method, url)
            .header("X-Bot-Token", &self.token);

        if let Some(json_data) = data {
            request = request
                .header("Content-Type", "application/json")
                .json(&json_data);
        }

        let response = request.send().await?;
        let json: T = response.json().await?;
        Ok(json)
    }
}
