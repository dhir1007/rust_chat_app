mod models;
mod state;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::state::{ChatState, SharedState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initializing the Global State 🧠
    let state: SharedState = Arc::new(Mutex::new(ChatState {
        rooms: HashMap::new(),
    }));

    // 2. Binding the TCP Listener
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chat server running on port 8080... 🚀");

    loop {
        // 3. Waiting for a new user
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        let state_clone = Arc::clone(&state);

        // 4. Spawning a task for each user
        tokio::spawn(async move {
            // handle_connection(socket, state_clone).await; 
            // ^ Ye hum 'handlers/chat.rs' mein likhenge
        });
    }
}