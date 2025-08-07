use crate::model::message::attachment::MessageAttachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageEmbed {
    #[serde(rename = "type")]
    embed_type: String,
    icon_url: Option<String>,
    #[serde(rename = "url")]
    title_url: Option<String>,
    title: Option<String>,
    description: Option<String>,
    #[serde(rename = "media")]
    attachments: Option<Vec<MessageAttachment>>,

    // Yes, I'm American. But what does that
    // have to do with code?
    #[serde(rename = "colour")]
    color: Option<String>,
}
