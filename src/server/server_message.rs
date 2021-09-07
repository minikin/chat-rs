use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ServerMessage {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}
