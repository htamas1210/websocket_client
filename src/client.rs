use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    username: String,
    uid: Uuid,
    text: String,
}

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:9001").unwrap();
    let (mut socket, _) = connect_async(url).await.expect("Can't connect");

    loop {
        let msg = Message {
            username: "Alice32".into(),
            uid: Uuid::new_v4(),
            text: "Hello, server!".into(),
        };

        let serialized = serde_json::to_string(&msg).unwrap();
        socket
            .send(tokio_tungstenite::tungstenite::Message::Text(serialized))
            .await
            .unwrap();

        if let Some(Ok(reply)) = socket.next().await {
            println!("Got reply: {}", reply);
        }
    }
}
