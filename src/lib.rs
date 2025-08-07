pub mod client;
pub mod error;
pub mod gateway;
pub mod http;
pub mod model;
pub mod prelude;

pub type Result<T> = error::Result<T>;
pub type Error = error::Error;
