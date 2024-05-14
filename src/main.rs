use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod handler;
mod ws;

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.events";

    println!("Connecting to - {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect ");
    println!("Connected");

    let (mut write, mut read) = ws_stream.split();

    let msg = Message::Text("Hi".into());

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read message");
        println!("Received message: {}", message);
    }
    println!("Sending message: {}", msg);
    write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read message");
        println!("Received message: {}", message);
    }
}
