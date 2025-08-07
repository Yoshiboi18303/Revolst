use crate::{
    Result,
    error::Error,
    model::{Message, SendMessage, attachment::MessageAttachment, embed::SendMessageEmbed},
};
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
        let url = Url::parse("https://api.revolt.chat");

        if let Err(why) = url {
            return Err(Error::Url(why));
        }

        Ok(Self {
            client: ReqwestClient::new(),
            token: token.into(),
            base_url: url.unwrap(),
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
            request = request.json(&json_data);
        }

        let response = request.send().await?;
        let json: T = response.json().await?;

        Ok(json)
    }

    pub async fn send_message(
        &self,
        channel_id: impl Into<String>,
        message: SendMessage,
    ) -> Result<Message> {
        let path = format!("/channels/{}/messages", channel_id.into());

        self.request_with_data(Method::POST, &path, Some(message))
            .await
    }

    pub async fn send_message_content(
        &self,
        channel_id: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<Message> {
        let data = SendMessage {
            attachments: None,
            content: Some(content.into()),
            embeds: None,
        };

        self.send_message(&channel_id.into(), data).await
    }

    pub async fn send_message_attachments(
        &self,
        channel_id: impl Into<String>,
        attachments: Vec<MessageAttachment>,
    ) -> Result<Message> {
        let data = SendMessage {
            attachments: Some(attachments),
            content: None,
            embeds: None,
        };

        self.send_message(&channel_id.into(), data).await
    }

    pub async fn send_message_embeds(
        &self,
        channel_id: impl Into<String>,
        embeds: Vec<SendMessageEmbed>,
    ) -> Result<Message> {
        let data = SendMessage {
            attachments: None,
            content: None,
            embeds: Some(embeds),
        };

        self.send_message(&channel_id.into(), data).await
    }
}
