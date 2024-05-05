use tungstenite::{connect, Message};
use url::Url;
use iogame_common::utils::logger::{Logger, clear_screen};

fn main() {

    clear_screen("IO Game - Client");

    let logger : Logger = Logger::new("client", 0);

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3030/echo").unwrap()).expect("Can't connect");

    logger.debug("Connected to the server");
    logger.info("Connected to the server");
    logger.warning("Connected to the server");
    logger.error("Connected to the server");

    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.send(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
}