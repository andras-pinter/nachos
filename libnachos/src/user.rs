use crate::client::Client;
use crate::socket::Socket;
use crate::error::UserError;
use crate::parser::Parser;

pub struct User {
    guac_user: libguac::guac_user,
}

impl User {
    pub fn new(socket: &Socket, client: &Client) -> Result<Self, UserError> {
        let user = libguac::guac_user::alloc()
            .ok_or(UserError::UnableToCreate)?;
        user.set_socket(socket.as_ref());
        user.set_client(client.as_ref());
        user.set_protocol_version(libguac::guac_version::V1_3_0);
        user.set_owner(true);

        Ok(User {
            guac_user: user
        })
    }

    pub fn protocol_version(&self) -> libguac::guac_version {
        self.guac_user.get_protocol_version()
    }

    pub fn handle_instruction(&self, parser: &Parser) -> Result<(), UserError> {
        self.guac_user.handle_instruction(parser.as_ref()).map_err(UserError::FailedToHandleInstruction)
    }

    pub fn is_active(&self) -> bool {
        self.guac_user.active() == 1
    }

    pub fn stop(&self) {
        self.guac_user.stop()
    }

    pub fn set_resolution(&self, width: i32, height: i32) {
        self.guac_user.info().set_width(width);
        self.guac_user.info().set_height(height);
        self.guac_user.info().set_optimal_resolution(96);
    }
}

impl AsRef<libguac::guac_user> for User {
    fn as_ref(&self) -> &libguac::guac_user {
        &self.guac_user
    }
}
