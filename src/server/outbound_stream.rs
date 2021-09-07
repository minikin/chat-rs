use std::net::TcpStream;

use async_std::sync::Mutex;

use crate::shared::{send_as_json, ChatResult};

use super::ServerMessage;

pub struct OutboundStream(Mutex<TcpStream>);

impl OutboundStream {
    pub fn new(to_client: TcpStream) -> Self {
        OutboundStream(Mutex::new(to_client))
    }
    pub async fn send(&self, packet: ServerMessage) -> ChatResult<()> {
        let mut stream = self.0.lock().await;
        send_as_json(&mut *stream, &packet).await?;

        Ok(())
    }
}
