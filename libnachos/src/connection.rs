use std::str::FromStr;
use crate::client::{Client, ClientState};
use crate::error::{ClientError, ConnectionError, ParserError, SocketError};
use crate::DEFAULT_TIMEOUT;
use crate::config::Configuration;
use crate::parser::Parser;
use crate::socket::{Socket, SocketStream};
use crate::user::User;

pub struct Connection {
    id: uuid::Uuid,

    client: Client,
    parser: Parser,
    user: User,
    socket: Socket,
}

impl Connection {
    pub fn new<C: Configuration>(configuration: C) -> Result<Self, ConnectionError> {
        let socket = Socket::new()?;
        let client = Client::new(configuration.protocol())?;
        let id = uuid::Uuid::from_str(
            client
                .connection_id()
                .trim_start_matches('$')
        ).map_err(ClientError::InvalidUuid)?;
        let user = User::new(&socket, &client)?;
        let (w, h) = configuration.resolution();
        user.set_resolution(w, h);
        let parser = Parser::new()?;
        let mut configuration = configuration.render(
            &user.protocol_version().as_ref(),
            client.args()
        );
        parser.parse(&mut configuration);
        client.add_user(&user, &parser)?;

        Ok(Connection {
            id,
            client,
            socket,
            user,
            parser
        })
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn socket(&self) -> Result<SocketStream, SocketError> {
        self.socket.socket()
    }

    pub fn ready(&self) -> String {
        libguac::guac_proto::Ready(&self.id.to_string()).to_string()
    }

    pub fn star_session(self) -> Result<(), ConnectionError> {
        let timeout = DEFAULT_TIMEOUT.as_micros();

        while self.client.state() == ClientState::Running && self.user.is_active() {
            match self.parser.read(&self.socket, timeout) {
                Ok(_) => self.user.handle_instruction(&self.parser)
                    .map_err(Into::into),
                Err(ParserError::ParsingError(e))
                    if e.is_status(libguac::GuacStatus::Closed) => Ok(self.user.stop()),
                Err(e) => {
                    self.user.stop();
                    Err(e)
                },
            }?;
        }

        println!("<---END--->");
        Ok(())
    }
}
