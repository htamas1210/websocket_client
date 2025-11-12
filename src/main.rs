use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    username: String,
    text: String,
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        println!("New connection!");

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let txt = msg.to_text().unwrap();
                    if let Ok(json) = serde_json::from_str::<Message>(txt) {
                        println!("Received: {:?}", json);

                        // Echo it back to the client
                        let reply = serde_json::to_string(&json).unwrap();
                        write
                            .send(tokio_tungstenite::tungstenite::Message::Text(reply))
                            .await
                            .unwrap();
                    }
                }
            }
        });
    }
}
