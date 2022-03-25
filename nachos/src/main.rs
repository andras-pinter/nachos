mod nachos;

use std::net::SocketAddr;
use warp::Filter;
use libnachos::Session;
use ssh_handler::{Ssh, SshConfiguration};
use nachos::Nachos;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "protocol", rename_all = "lowercase")]
enum Configurations {
    Ssh(SshConfiguration),
}

#[derive(Eq, PartialEq, Hash)]
enum Sessions {
    Ssh(Session<Ssh>),
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let nachos = Nachos::new();

    let routes = nachos.api()
        .or(warp::path::end().map(|| warp::reply()));

    println!("Listening on http://{}", addr);
    warp::serve(routes).run(addr).await
}
