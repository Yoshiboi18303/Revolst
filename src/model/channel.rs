use crate::{Result, http::Http, model::message::Message};
use serde::Serialize;

pub struct ChannelId(pub String);

impl ChannelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub async fn say(&self, http: &Http, content: impl Into<String>) -> Result<Message> {
        #[derive(Serialize)]
        struct SendMessage {
            content: String,
        }

        let data = SendMessage {
            content: content.into(),
        };

        http.request_with_data(
            reqwest::Method::POST,
            &format!("/channels/{}/messages", self.0),
            Some(data),
        )
        .await
    }
}
