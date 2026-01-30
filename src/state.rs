use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use crate::models::message::ChatMessage;

pub struct User {
    pub username: String,
    pub tx: broadcast::Sender<ChatMessage>, // Message bhejane ke liye pipe 📢
}

pub struct ChatState {
    pub rooms: HashMap<String, Vec<User>>, // Room Name -> Users ki list
}

pub type SharedState = Arc<Mutex<ChatState>>;