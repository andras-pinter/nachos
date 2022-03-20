use crate::ConnectionStore;
use warp::{Filter, Reply};

pub struct AppEndpoint;

impl AppEndpoint {
    pub fn handle_session_components(
        sessions: ConnectionStore,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        warp::path(common::Api::Session)
            .and(warp::path::param())
            .and(warp::fs::dir("client"))
            .map(move |session_id: String, file: warp::fs::File| {
                let is_session = sessions
                    .lock()
                    .ok()
                    .map(|sdb| sdb.contains(&session_id))
                    .unwrap_or_default();

                (is_session, file)
            })
            .and_then(
                move |(is_session, file): (bool, warp::fs::File)| async move {
                    if is_session {
                        Ok(file)
                    } else {
                        Err(warp::reject::not_found())
                    }
                },
            )
    }
}
