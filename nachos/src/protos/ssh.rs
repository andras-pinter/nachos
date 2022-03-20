use futures::{SinkExt, StreamExt};
use warp::ws::Message;
use warp::Reply;

const MESSAGE: &str = "Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ";

pub struct SshHandler;

impl SshHandler {
    pub fn handle(ws: warp::ws::Ws, config: common::SshConfig) -> impl Reply {
        ws.on_upgrade(|mut sock| async move {
            println!("Opening WebSocket connection...");
            sock.send(Message::binary(MESSAGE.as_bytes())).await.unwrap();
        })
    }
}
