#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub hostname: String,
    pub port: u16,
}
