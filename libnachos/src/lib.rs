mod session;
mod api;

pub use api::Api;
pub use session::Session;

pub trait Handler {
    type Configuration: serde::Serialize + for <'de> serde::Deserialize<'de> + Clone + AsRef<str>;
}
