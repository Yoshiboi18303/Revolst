use crate::{
    Result,
    http::Http,
    model::message::{Message, embed::SendMessageEmbed},
};

pub struct ChannelId(pub String);

impl ChannelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub async fn say(&self, http: &Http, content: impl Into<String>) -> Result<Message> {
        http.send_message_content(&self.0, content.into()).await
    }

    pub async fn say_embeds(&self, http: &Http, embeds: Vec<SendMessageEmbed>) -> Result<Message> {
        http.send_message_embeds(&self.0, embeds).await
    }
}
