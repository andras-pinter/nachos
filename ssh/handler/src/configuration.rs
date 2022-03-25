#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct SshConfiguration {
    pub hostname: String,
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

impl AsRef<str> for SshConfiguration {
    fn as_ref(&self) -> &str {
        "ssh"
    }
}
