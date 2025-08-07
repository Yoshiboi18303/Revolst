use thiserror::Error;

/// Errors that could occur while using Revolst
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Authentication failed")]
    Auth,

    #[error("Rate limited")]
    RateLimit,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Gateway connection error: {message}")]
    Gateway { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
