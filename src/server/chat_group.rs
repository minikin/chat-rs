use ::std::sync::Arc;
use async_std::task;
use tokio::sync::broadcast::{channel, Reseiver, Sender};

pub struct ChatGroup {
    name: Arc<String>,
    sender: Sender<Arc<String>>,
}

impl ChatGroup {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _receiver) = channel(1000);
        Self { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();

        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post_message(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }
}

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: Reseiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {}
}
