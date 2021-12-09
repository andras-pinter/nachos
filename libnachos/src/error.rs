#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Unable to crate client!")]
    UnableToCreate,
    #[error("Error occurred while handling connection: {0}")]
    HandleError(libguac::GuacError),
    #[error("Failed to add user to client: {0}")]
    AddUserError(libguac::GuacError),
    #[error("Invalid UUID format")]
    InvalidUuid(#[from] uuid::Error)
}

#[derive(thiserror::Error, Debug)]
pub enum SocketError {
    #[error("Unable to open socket pair")]
    UnableToOpen,
    #[error("Unable to create socket: {0}")]
    CreationError(#[from] nix::Error),
    #[error("Unable to communicate with socket: {0}")]
    CommutationError(String),
    #[error("Unable to flush or send data to socket: {0}")]
    SendError(#[from] std::io::Error),
    #[error("Unable to clone socket")]
    UnableToClone,
}

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("Unable to crate user!")]
    UnableToCreate,
    #[error("Failed to handle instruction: {0}")]
    FailedToHandleInstruction(libguac::GuacError),
}

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("Unable to crate parser!")]
    UnableToCreate,
    #[error("Unable to parse and handle instruction!")]
    HandlingError(#[from] UserError),
    #[error("Error occurred meanwhile parsing: {0}!")]
    ParsingError(#[from] libguac::GuacError),
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    ClientInitializationError(#[from] ClientError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    SocketInitializationError(#[from] SocketError),
    #[error(transparent)]
    UserInitializationError(#[from] UserError),
    #[error(transparent)]
    ParserError(#[from] ParserError),
}
