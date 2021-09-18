use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::Mutex;
use chat_rs::shared::{
    server_response::ServerResponse,
    utils::{self, ChatResult},
};

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(to_client: TcpStream) -> Outbound {
        Outbound(Mutex::new(to_client))
    }

    pub async fn send(&self, packet: ServerResponse) -> ChatResult<()> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
