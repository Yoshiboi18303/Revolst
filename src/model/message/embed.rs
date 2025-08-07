use crate::model::message::attachment::MessageAttachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageEmbed {
    #[serde(rename = "type")]
    pub embed_type: String,
    pub icon_url: Option<String>,
    #[serde(rename = "url")]
    pub title_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "media")]
    pub attachments: Option<Vec<MessageAttachment>>,

    // Yes, I'm American. But what does that
    // have to do with code?
    #[serde(rename = "colour")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendMessageEmbed {
    pub icon_url: Option<String>,
    #[serde(rename = "url")]
    pub title_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "media")]
    pub attachments: Option<Vec<MessageAttachment>>,
    #[serde(rename = "colour")]
    pub color: Option<String>,
}
