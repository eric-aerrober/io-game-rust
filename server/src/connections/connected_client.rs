use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use futures::SinkExt;
use iogame_common::networking::event;
use iogame_common::networking::event::NetworkingEvent;
use iogame_common::networking::event::NetworkingEventMessage;
use iogame_common::networking::events::client_force_disconnected::ClientForceDisconnectedFromServer;
use tokio::time::{Duration, timeout};
use uuid::Uuid;
use futures::StreamExt;
use futures::stream::{SplitStream, SplitSink};
use iogame_common::utils::logger::Logger;
use warp::ws::{WebSocket, Message};
use std::collections::HashMap;
use serde::Serialize;

// Each user gets an incrementing ID
const USER_COUNTER: AtomicU32 = AtomicU32::new(0);
const LISTENER_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct ConnectedClient {

    // Unique identifiers for the client
    uid: String,
    id: u32,

    // Websocket stream and sink
    stream: SplitStream<WebSocket>,
    sink: SplitSink<WebSocket, Message>,

    // Event listeners for the client
    listeners: HashMap<event::NetworkingEvent, HashMap<u32, Sender<Vec<u8>>>>,

    // Logger for the client
    pub log: Logger,
}

impl ConnectedClient {

    pub fn new(ws : warp::ws::WebSocket) -> ConnectedClient {

        let (sink, stream) = ws.split();
        let user_id = USER_COUNTER.fetch_add(1, Ordering::SeqCst);
        let uid = Uuid::new_v4().to_string();
        let client_label = format!("conn-{}", user_id);
        
        ConnectedClient {
            uid: uid,
            id: user_id,
            sink: sink,
            stream: stream,
            log: Logger::new(client_label.as_str()),
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener(&mut self, event: event::NetworkingEvent, sender: Sender<Vec<u8>>) -> u32 {
        
        let listener_id: u32 = LISTENER_COUNTER.fetch_add(1, Ordering::SeqCst);
        
        match self.listeners.get_mut(&event) {
            Some(listeners) => {
                listeners.insert(listener_id, sender);
            }
            None => {
                let mut new_hashmap = HashMap::new();
                new_hashmap.insert(listener_id, sender);
                self.listeners.insert(event, new_hashmap);
            }
        };

        return listener_id;
    }

    pub fn remove_listener(&mut self, event: NetworkingEvent, listener_id: u32) {
        
        match self.listeners.get_mut(&event) {
            Some(listeners) => {
                listeners.remove(&listener_id);
            }
            None => {
                self.log.warning(&format!("No listeners for event: {:?}", event));
            }
        };

    }

    fn handle_recieved_event (&mut self, event: &NetworkingEvent, payload: Vec<u8>) {

        let event_listeners = self.listeners.get(&event);

        match event_listeners {
            Some(listeners) => {
                for (_, listener) in listeners {
                    let _ = listener.send(payload.clone());
                }
            },
            None => {
                self.log.warning(&format!("No handler for event: {:?}", event));
            }
        }
    }

    pub async fn handle_next_event (&mut self) -> Option<event::NetworkingEvent> {

        let next_next = self.stream.next().await;

        match next_next {
            Some(msg) => {
                match msg {
                    Ok(data) => {
                        let binary_data = data.as_bytes().to_vec();
                        let (event, payload) = event::obj_from_msg(binary_data);
                        self.handle_recieved_event(&event, payload);
                        return Option::Some(event);
                    },
                    Err(e) => {
                        self.log.info(&format!("Client Disconnected: {:?}", e));
                        return Option::None;
                    }
                }
            },
            None => {
                self.log.info("Client Disconnected, need cleanup?");
                return Option::None;
            }
        }

    }

    pub async fn handle_until_event_or_timeout (&mut self, event: event::NetworkingEvent, timeout_duration: Duration) -> Option<event::NetworkingEvent> {
        loop {
            let next_event_result = timeout(timeout_duration, self.handle_next_event()).await;
            match next_event_result {
                Ok(next_event) => {
                    match next_event {
                        Some(e) => {
                            if e == event {
                                return Option::Some(e);
                            }
                        },
                        None => {
                            return Option::None;
                        }
                    }
                },
                Err(_) => {
                    return Option::None;
                }
            }
        }
    }

    pub async fn handle_all_events (&mut self) {
        loop {
            self.handle_next_event().await;
        };
    }

    pub async fn send<T: Serialize>(&mut self, event: NetworkingEvent, item: T) {
        let raw_event_id = bincode::serialize(&event).unwrap();
        let raw_payload = bincode::serialize(&item).unwrap();
        let vec = [&raw_event_id[..], &raw_payload[..]].concat();
        let message = Message::binary(vec);
        self.sink.send(message).await.unwrap();
    }

    pub async fn disconnect (&mut self, reason: &str) {
        self.log.info(&format!("Disconnecting client: {}", reason));
        self.send(NetworkingEvent::ClientForceDisconnectedFromServer, ClientForceDisconnectedFromServer {
            reason: reason.to_string()
        }).await;
    }


}