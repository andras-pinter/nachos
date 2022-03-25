use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use warp::{Filter, Reply};
use warp::http::StatusCode;
use libnachos::{Api, Handler, Session};
use ssh_handler::{Ssh, SshConfiguration};
use super::{Configurations, Sessions};

pub struct Nachos {
    sessions: Arc<RwLock<HashSet<Sessions>>>,
}

impl Nachos {
    pub fn new() -> Self {
        Nachos {
            sessions: Arc::new(RwLock::new(HashSet::default())),
        }
    }

    pub fn api(&self) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        let api = self.connections()
            .or(Self::forbidden());
        warp::path::path(Api::Base)
            .and(api)
    }

    fn connections(&self) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        let session_db = self.sessions.clone();
        warp::post()
            .and(warp::path(Api::Connection))
            .and(warp::path::end())
            .and(warp::body::json())
            .map(|config: Configurations| {
                match config {
                    Configurations::Ssh(c) => Sessions::Ssh(Session::new(Ssh, c)),
                }
            })
            .map(move |session: Sessions| {
                session_db.write().unwrap().insert(session)
            })
            .map(|_| warp::reply())
    }

    fn forbidden() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        warp::path::end()
            .map(|| warp::reply::with_status("Forbidden", StatusCode::FORBIDDEN))
    }
}
