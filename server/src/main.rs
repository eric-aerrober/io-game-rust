#![deny(warnings)]

use futures::StreamExt;
use futures::FutureExt;
use warp::Filter;

#[tokio::main]
async fn main() {

    let routes = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                println!("Websocket connected");
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    println!("Server started at http://[127, 0, 0, 1]:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}