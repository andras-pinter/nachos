#[derive(Clone)]
pub enum Api {
    Base,
    Connection,
}

impl AsRef<str> for Api {
    fn as_ref(&self) -> &str {
        match self {
            Self::Base => "api",
            Self::Connection => "connection"
        }
    }
}
