use crate::model::message::{
    attachment::MessageAttachment,
    embed::{MessageEmbed, SendMessageEmbed},
};
use serde::{Deserialize, Serialize};

pub mod attachment;
pub mod embed;

/// A struct for any messages that come from the API.
///
/// NOTE: DO NOT USE THIS TO SEND MESSAGES, USE THE SendMessage STRUCT INSTEAD!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    pub nonce: Option<String>,
    pub channel: String,
    pub content: Option<String>,
    pub author: String,
    pub attachments: Option<Vec<MessageAttachment>>,
    pub embeds: Option<Vec<MessageEmbed>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendMessage {
    pub content: Option<String>,
    pub attachments: Option<Vec<MessageAttachment>>,
    pub embeds: Option<Vec<SendMessageEmbed>>,
}
