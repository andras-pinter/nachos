mod configuration;

use futures::{SinkExt, StreamExt, TryStreamExt};
use warp::{Filter, Reply, Rejection};
use warp::ws::Message;
use libnachos::{Protocol, Connection, SocketError, ConnectionError};
use crate::configuration::Config;

async fn handle_websocket(ws: warp::ws::Ws, connection: Connection) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(|sock| async move {
        let (mut send, recv) = sock.split();
        println!("<----------------ON-UPGRADE---------------->");

        match send.send(Message::text(connection.ready())).await {
            Ok(_) => match (connection.socket(), connection.socket()) {
                (Ok(inbound), Ok(mut outbound)) => {
                    let id = connection.id();
                    let forward_client = tokio::spawn(
                        recv
                            .map_err(|e| SocketError::CommutationError(e.to_string()))
                            .map_ok(|msg| {
                                println!("To Server: {:?}", msg);
                                msg
                            })
                            .forward(inbound)
                    );
                    let forward_server = tokio::spawn(async move {
                        let mut buffer = Vec::new();
                        while let Ok(len) = outbound.read(&mut buffer) {
                            match len {
                                0 => break,
                                _ => {
                                    let message = Message::text(String::from_utf8_lossy(&buffer));
                                    println!("To client: {:?}", message);
                                    if send.send(message).await.is_err() || send.flush().await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }

                        Ok::<_, ConnectionError>(())
                    });
                    let guac_t = tokio::task::spawn_blocking(move || connection.star_session());

                    println!("Handling connection {}", id);
                    let res: std::result::Result<(), ConnectionError> = tokio::select! {
                        Ok(Err(err)) = guac_t => Err(err),
                        Ok(Err(err)) = forward_server => Err(err),
                        Ok(Err(err)) = forward_client => Err(err.into()),
                        else => Ok(println!("Connection {} closed", id)),
                    };

                    if let Err(err) = res {
                        eprintln!("Connection error occurred: {}", err)
                    }
                }
                _ => eprintln!("Failed to open communication sockets!"),
            },
            Err(err) => eprintln!("Unable ready signal client: {}", err)
        };
    }))
}

fn configuration() -> Config {
    configuration::Config {
        protocol: Protocol::SSH,
        hostname: "10.0.1.46".to_string(),
        port: 22,
        username: Some("pinter".to_string()),
        password: Some("Le00rosyl4n".to_string()),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    let websocket = warp::path("tunnel")
        .and(warp::ws())
        .and(warp::any().map(|| Connection::new(configuration()).unwrap()))
        .and_then(handle_websocket);

    let routes = websocket.with(warp::cors().allow_any_origin());

    println!("Serving...");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
