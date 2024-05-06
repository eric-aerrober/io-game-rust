use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NetworkingEvent {
    ClientIdentifySelf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsEvent {
    pub event: NetworkingEvent,
    pub payload: String,
}