use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use uuid::Uuid;
use futures::StreamExt;
use futures::stream::{SplitStream, SplitSink};
use iogame_common::utils::logger::Logger;
use iogame_common::networking::event;

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

    pub async fn wait_for_event(&mut self, event: event::NetworkingEvent) -> event::WsEvent {
        let logger = &self.logger;
        let stream = &mut self.stream;
        let sink = &mut self.sink;

        // Read events until we find the one we are looking for
        while let Some(msg) = stream.next().await {
            let msg = msg.unwrap();

            if msg.is_text() {
                let text = msg.to_str().unwrap();
                let result_event = serde_json::from_str::<event::WsEvent>(text).unwrap();
                logger.debug(&format!("Received message: {:?}", event));

                if event == result_event.event {
                    return result_event;
                }
            }

        };

        panic!("Event not found");
    }
}