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
        let result2 = result.clone();

        assert!(!result.id.is_empty());
        assert!(result.content.is_some());
        assert_eq!(result.content.unwrap(), content);
        assert_eq!(result.id, result2.id);
    }
}
