use crate::connections::connected_client::ConnectedClient;
use iogame_common::networking::event::NetworkingEvent;

pub async fn handle_incomming_websocket_connection (websocket: warp::ws::WebSocket) {
    let mut connected_client: ConnectedClient = ConnectedClient::new(websocket);
    connected_client.wait_for_event(NetworkingEvent::ClientIdentifySelf).await;

}