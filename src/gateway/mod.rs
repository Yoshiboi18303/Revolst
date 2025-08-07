use crate::Result;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::client::IntoClientRequest};

use tokio_tungstenite::connect_async;

pub struct WsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WsClient {
    pub async fn connect(token: impl Into<String>) -> Result<Self> {
        let gateway_url = format!(
            "wss://app.revolt.chat/events?version=1&format=json&token={}",
            token.into()
        );

        let request = gateway_url.into_client_request().unwrap();
        let (stream, _) = connect_async(request).await.unwrap();

        Ok(Self(stream))
    }
}
