use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use uuid::Uuid;
use futures::StreamExt;
use futures::stream::{SplitStream, SplitSink};
use iogame_common::utils::logger::Logger;

const USER_COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct ConnectedClient {
    pub uid: String,
    pub id: u16,
    pub stream: SplitStream<warp::ws::WebSocket>,
    pub sink: SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    pub logger: Logger,
}

impl ConnectedClient {
    pub fn new(ws : warp::ws::WebSocket) -> ConnectedClient {
        let (sink, mut stream) = ws.split();

        ConnectedClient {
            uid: Uuid::new_v4().to_string(),
            id: USER_COUNTER.fetch_add(1, Ordering::SeqCst),
            sink: sink,
            stream: stream,
            logger: Logger::new("client"),
        }
    }
}