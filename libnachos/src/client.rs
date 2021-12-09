use crate::error::ClientError;
use crate::parser::Parser;
use crate::Protocol;
use crate::user::User;

#[derive(Debug, PartialEq)]
pub enum ClientState {
    Running,
    Stopping,
}

impl From<libguac::guac_client_state> for ClientState {
    fn from(state: libguac::guac_client_state) -> Self {
        match state {
            libguac::guac_client_state::RUNNING => Self::Running,
            libguac::guac_client_state::STOPPING => Self::Stopping,
        }
    }
}

pub struct Client {
    guac_client: libguac::guac_client,
}

impl Client {
    pub fn new(protocol: Protocol) -> Result<Self, ClientError> {
        let client = libguac::guac_client::alloc().ok_or(ClientError::UnableToCreate)?;
        Self::load(&client, protocol)?;

        Ok(Client {
            guac_client: client,
        })
    }

    pub fn connection_id<'a>(&self) -> std::borrow::Cow<'a, str> {
        self.guac_client.connection_id()
    }

    pub fn args<'a>(&self) -> Vec<std::borrow::Cow<'a, str>> {
        self.guac_client.args()
    }

    pub fn add_user(&self, user: &User, parser: &Parser) -> Result<(), ClientError> {
        self.guac_client.add_user(user.as_ref(), parser.as_ref())
            .map_err(ClientError::AddUserError)
    }

    pub fn state(&self) -> ClientState {
        self.guac_client.state().into()
    }

    fn load(client: &libguac::guac_client, protocol: Protocol) -> Result<(), ClientError> {
        client.load_plugin(protocol.into())
            .map_err(|err| ClientError::HandleError(err))
    }
}

impl AsRef<libguac::guac_client> for Client {
    fn as_ref(&self) -> &libguac::guac_client {
        &self.guac_client
    }
}
