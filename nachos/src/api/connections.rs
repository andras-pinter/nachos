use crate::ConnectionStore;
use common::Connection;
use warp::{Filter, Reply};

pub struct ConnectionsEndpoint;

impl ConnectionsEndpoint {
    pub fn handle(
        sessions: ConnectionStore,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        warp::path(common::Api::Session)
            .and(warp::get())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .map(move |session_id: String| {
                sessions
                    .lock()
                    .ok()
                    .and_then(|sdb| sdb.get(&session_id).cloned())
            })
            .and_then(move |connection: Option<Connection>| async move {
                match connection {
                    Some(c) => Ok(c),
                    _ => Err(warp::reject::not_found()),
                }
            })
            .map(|c| warp::reply::json(&c))
    }
}
