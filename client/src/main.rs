use std::thread;
use std::time::Duration;

use iogame_common::networking::events::client_force_disconnected::ClientForceDisconnectedFromServer;
use tungstenite::{connect};
use tungstenite::protocol::Message::Binary;
use url::Url;
use iogame_common::utils::logger::{Logger, clear_screen};
use iogame_common::networking::event::{NetworkingEvent, obj_from_msg};
use iogame_common::networking::identity::{IdentitfierSource, Identity};

fn main() {

    clear_screen("IO Game - Client");
    let logger : Logger = Logger::new("client");

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3030/websocket").unwrap()).expect("Can't connect");

    std::thread::spawn(move || {
        loop {
            match socket.read() {
                Ok(msg) => {
                    match msg {
                        Binary(data) => {
                            let (event, obj) = obj_from_msg(data);
                            logger.debug(&format!("Received Event: {:?}", event));
                            match event {
                                NetworkingEvent::ClientForceDisconnectedFromServer => {
                                    let obj = bincode::deserialize::<ClientForceDisconnectedFromServer>(&obj).unwrap();
                                    logger.error(&format!("Server forced disconnection: {}", obj.reason));
                                },
                                _ => {}
                            }
                        },
                        _ => {
                            logger.error("Received non-binary message");
                        }
                    }
                },
                Err(e) => {
                    logger.error(&format!("Error: {:?}", e));
                    break;
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(5));

}