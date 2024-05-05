mod connections;

use warp::Filter;
use iogame_common::utils::logger::{Logger, clear_screen};
use connections::connection_handshaker::handle_incomming_websocket_connection;

#[tokio::main]
async fn main() {

    clear_screen("IO Game - Server");
    let logger : Logger = Logger::new("server");

    let routes = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_incomming_websocket_connection)
        });

    logger.debug("Server started on ws://localhost:3030/websocket");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}
