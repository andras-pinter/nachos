mod api;
mod connections;
mod protos;

use api::{AppEndpoint, ConnectionsEndpoint, SessionEndpoint};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

type ConnectionStore = Arc<Mutex<connections::ConnectionStore>>;

const CLIENT_DIR: &str = "client";

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let connection_store = connections::ConnectionStore::new();

    #[cfg(not(feature = "testConnections"))]
    let root = warp::path::end()
        .map(|| warp::reply::with_status("No Content", warp::http::StatusCode::NO_CONTENT));
    #[cfg(feature = "testConnections")]
    let root = warp::path::end().and(warp::fs::file("client/index.html"));
    let api_connection = warp::path::path(common::Api::Base)
        .and(ConnectionsEndpoint::handle(connection_store.clone()));
    let app =
        warp::get()
            .and(warp::fs::dir(CLIENT_DIR))
            .or(AppEndpoint::handle_session_components(
                connection_store.clone(),
            ));
    let session = warp::get().and(SessionEndpoint::handle(connection_store.clone()));

    let routes = root.or(api_connection).or(app).or(session);
    println!("Serving on {}", addr);
    warp::serve(routes).run(addr).await
}
