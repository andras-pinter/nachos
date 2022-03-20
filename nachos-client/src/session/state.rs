use common::Connection;
use gloo_net::Error;

pub enum SessionState {
    Init,
    Ready,
    Error(String),
}

impl From<Result<Connection, Error>> for SessionState {
    fn from(res: Result<Connection, Error>) -> Self {
        match res {
            Ok(_) => SessionState::Ready,
            Err(err) => SessionState::Error(err.to_string()),
        }
    }
}
