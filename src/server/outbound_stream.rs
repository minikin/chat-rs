use std::{io::Write, net::TcpStream};

use async_std::sync::Mutex;

use crate::shared::ChatResult;

pub struct OutboundStream(Mutex<TcpStream>);

impl OutboundStream {
    pub fn new(to_client: TcpStream) -> Self {
        OutboundStream(Mutex::new(to_client))
    }
    pub async fn send(&self, packet: FromServer) -> ChatResult<()> {
        let mut lock = self.0.lock().await;

        lock.flush().await?;
        Ok(())
    }
}
