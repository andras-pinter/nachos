use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use ssh2::Session;

fn main() {
    let addr = SocketAddr::new([10, 0, 1, 8].into(), 22);
    let stream = TcpStream::connect(addr)
        .expect("Failed to connect");
    println!("New session");
    let mut session = Session::new()
        .expect("Failed to initialize session");
    println!("Set stream");
    session.set_tcp_stream(stream);
    println!("Handshake");
    session.handshake()
        .expect("Failed to perform handshake");
    println!("Authenticating");
    session.userauth_password("pintera", "le00rosyl4n")
        .expect("Failed to authenticate");
    println!("Get channel");
    let channel = session.channel_session()
        .expect("Failed to get channel");
    println!("Get stream");
    let mut stream = channel.stream(1);

    let t = std::thread::spawn(move ||{
        let mut buffer = [0u8; 1024];
        stream.write_all(&mut "ls".as_bytes()).expect("Failed to write");
        loop {
            println!("Reading");
            stream.read(&mut buffer).expect("Failed to read stream");
        }
    });

    println!("Authenticated: {}", session.authenticated());
    t.join().expect("Failed to join");
}
