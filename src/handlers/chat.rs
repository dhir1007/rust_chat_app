use crate::state::SharedState;
use crate::models::message::ChatMessage;
use futures_util::{StreamExt, SinkExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, state: SharedState) {
    // 1. Perform the WebSocket Handshake
    let ws_stream = accept_async(raw_stream) //takes TcpStream and returns a WebSocketStream
        .await
        .expect("Error during the websocket handshake occurred");

    println!("WebSocket connection established: {}", addr);

    // 2. Split the stream into a Sender (Sink) and a Receiver (Stream)
    let (mut user_ws_tx, mut user_ws_rx) = ws_stream.split(); 

    // 3. Logic for Room Joining and Messaging goes here...
    // We will need a loop to listen for messages from the user
    // and a way to broadcast those messages to the SharedState.
}