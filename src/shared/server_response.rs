use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ServerResponse {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}
