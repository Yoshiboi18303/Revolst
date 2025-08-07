use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageAttachmentType {
    File,
    Text,
    Image(u64, u64),
    Video(u64, u64),
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachmentMetadata {
    // Type is a reserved keyword, so we have to use something OTHER
    // than that. But we can rename it using serde so it matches
    // the schema!
    #[serde(rename = "type")]
    pub attachment_type: MessageAttachmentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    #[serde(rename = "filename")]
    pub file_name: String,
    pub metadata: MessageAttachmentMetadata,
    pub size: u64,
    pub deleted: Option<bool>,
    pub reported: Option<bool>,
}
