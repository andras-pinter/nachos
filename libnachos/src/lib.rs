mod client;
mod connection;
mod error;
mod protocol;
mod socket;
mod user;
mod parser;
mod config;

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);

pub use connection::Connection;
pub use error::{ConnectionError, SocketError};
pub use protocol::Protocol;
pub use config::Configuration;
