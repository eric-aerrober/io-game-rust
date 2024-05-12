use serde::{Serialize, Deserialize};
use crate::networking::event;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(into = "u8", from = "u8")]
#[repr(u8)]
pub enum NetworkingEvent {
    ClientIdentifySelf,
    ClientForceDisconnectedFromServer,
}

pub struct NetworkingEventMessage {
    pub event: NetworkingEvent,
    pub payload: Vec<u8>
}


impl From<u8> for NetworkingEvent {
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkingEvent::ClientIdentifySelf,
            1 => NetworkingEvent::ClientForceDisconnectedFromServer,
            _ => panic!("Invalid value for NetworkingEvent"),
        }
    }
}

impl From<NetworkingEvent> for u8 {
    fn from(value: NetworkingEvent) -> Self {
        value as u8
    }
}

pub fn obj_from_msg(msg: Vec<u8>) -> (NetworkingEvent, Vec<u8>) {
    let event = NetworkingEvent::from(msg[0]);
    return (event, msg[1..].to_vec());
}