use crate::model::message::{attachment::MessageAttachment, embed::MessageEmbed};
use serde::{Deserialize, Serialize};

pub mod attachment;
pub mod embed;

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
