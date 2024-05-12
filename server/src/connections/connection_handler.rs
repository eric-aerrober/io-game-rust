use std::{sync::mpsc::channel, time::Duration};

use iogame_common::networking::event::{obj_from_msg, NetworkingEvent, NetworkingEventMessage};
use iogame_common::networking::events::client_identify_self;

use crate::connections::connected_client::ConnectedClient;

pub async fn handle_incomming_websocket_connection (websocket: warp::ws::WebSocket) {

    // Connected client represents a single user connection through a websocket
    let mut connected_client = ConnectedClient::new(websocket);

    // Set up a way to listen for the ClientIdentifySelf event to be sent by the client
    let (tx, rx) = channel::<Vec<u8>>();
    let identity_listen = connected_client.add_listener(NetworkingEvent::ClientIdentifySelf, tx);

    // Wait for the client to send the ClientIdentifySelf event or time out after 2 seconds
    let wait_for_identity = connected_client.handle_until_event_or_timeout(NetworkingEvent::ClientIdentifySelf, Duration::from_secs(0)).await;
    connected_client.remove_listener(NetworkingEvent::ClientIdentifySelf, identity_listen);

    match wait_for_identity {
        Some(_) => {
            let message = rx.recv().unwrap();
            match obj_from_msg(message) {
                (NetworkingEvent::ClientIdentifySelf, identity_bin) => {
                    let client_identify_self : client_identify_self::ClientIdentifySelf = bincode::deserialize(&identity_bin).unwrap();
                    connected_client.log.info(&format!("Client identified as {}", client_identify_self.display_name));
                },
                _ => {}
            }
        },
        None => {
            connected_client.disconnect("Client did not identify themselves in time").await;
        }
    }

}

