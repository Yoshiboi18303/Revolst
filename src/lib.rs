pub mod client;
pub mod error;
pub mod gateway;
pub mod http;
pub mod model;
pub mod prelude;

pub type Result<T> = error::Result<T>;
pub type Error = error::Error;

#[cfg(test)]
mod tests {
    use crate::gateway::WsClient;

    use super::model::embed::SendMessageEmbed;
    use super::*;
    use std::env;

    #[test]
    fn it_creates_channel_id_successfully() {
        let channel_id = model::ChannelId::new("01HAX1JZP5ANGCT55EK78SGV1J");
        assert_eq!(channel_id.0, "01HAX1JZP5ANGCT55EK78SGV1J")
    }

    #[tokio::test]
    async fn it_sends_messages_successfully() {
        let channel_id = model::ChannelId::new("01HAX1JZP5ANGCT55EK78SGV1J");
        let token = env::var("TEST_BOT_TOKEN").expect("Expected a token!");
        let http = http::Http::new(token).expect("Failed to create Http!");
        let content = "Hello, this is a test message.";

        let result = channel_id.say(&http, content).await;

        if let Err(why) = result {
            panic!("Failed to send message! {why:?}");
        }

        let result = result.unwrap();

        assert!(!result.id.is_empty());
        assert!(result.content.is_some());
        assert_eq!(result.content.unwrap(), content);
    }

    #[tokio::test]
    async fn it_sends_embeds_successfully() {
        let channel_id = model::ChannelId::new("01HAX1JZP5ANGCT55EK78SGV1J");
        let token = env::var("TEST_BOT_TOKEN").expect("Expected a token!");
        let http = http::Http::new(token).expect("Failed to create Http!");
        let embeds = vec![SendMessageEmbed {
            title: Some("Test Embed".to_string()),
            description: Some("This is an embed test!".to_string()),
            color: Some("#00FFFF".to_string()), // Cyan
            attachments: None,
            icon_url: None,
            title_url: None,
        }];
        let embeds2 = &embeds.clone();

        let result = channel_id.say_embeds(&http, embeds).await;

        if let Err(why) = result {
            panic!("Failed to send embed! {why:?}");
        }

        let result = result.unwrap();

        assert!(result.embeds.is_some());
        assert_eq!(result.embeds.unwrap()[0].title, embeds2[0].title);
    }

    #[tokio::test]
    async fn it_connects_to_gateway_successfully() {
        let token = env::var("TEST_BOT_TOKEN").expect("Expected a token!");
        let client = WsClient::connect(&token).await;

        if let Err(why) = client {
            panic!("Failed to connect to the gateway! {why:?}");
        }

        assert!(client.is_ok());
    }
}
