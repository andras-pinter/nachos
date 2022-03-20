#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "protocol", rename_all = "lowercase")]
pub enum Connection {
    SSH(crate::SshConfig),
}
