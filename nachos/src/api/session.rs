use crate::protos::SshHandler;
use crate::ConnectionStore;
use common::Connection;
use warp::{Filter, Reply};

pub struct SessionEndpoint;

impl SessionEndpoint {
    pub fn handle(
        sessions: ConnectionStore,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        warp::path(common::Api::Base)
            .and(warp::path(common::Api::Session))
            .and(warp::ws())
            .and(warp::path::param::<String>())
            .map(move |ws: warp::ws::Ws, session_id: String| {
                let session = sessions
                    .lock()
                    .ok()
                    .and_then(|mut sdb| sdb.remove(&session_id));

                (ws, session)
            })
            .and_then(
                move |(ws, connection): (warp::ws::Ws, Option<Connection>)| async move {
                    match connection {
                        Some(c) => Ok((ws, c)),
                        _ => Err(warp::reject::not_found()),
                    }
                },
            )
            .and(warp::path(common::Api::Tunnel))
            .map(
                |(ws, connection): (warp::ws::Ws, Connection)| match connection {
                    Connection::SSH(config) => SshHandler::handle(ws, config),
                },
            )
    }
}
