use async_std::task;
use chat_rs::shared::server_response::ServerResponse;
use std::sync::Arc;
use tokio::sync::broadcast;

use tokio::sync::broadcast::error::RecvError;

use crate::outbound::Outbound;

pub struct ChatGroup {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl ChatGroup {
    pub fn new(name: Arc<String>) -> ChatGroup {
        let (sender, _receiver) = broadcast::channel(1000);
        ChatGroup { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();

        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ignored = self.sender.send(message);
    }
}

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => ServerResponse::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },

            Err(RecvError::Lagged(n)) => ServerResponse::Error(format!(
                "Dropped {} messages from {}.",
                n, group_name
            )),

            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
