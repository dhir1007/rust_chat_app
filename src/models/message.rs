use std::string;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub time: String,
}
