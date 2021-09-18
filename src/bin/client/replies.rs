use async_std::io;
use async_std::net;
use async_std::prelude::*;
use chat_rs::shared::server_response::ServerResponse;
use chat_rs::shared::utils::{self, ChatResult};

pub async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            ServerResponse::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            ServerResponse::Error(message) => {
                println!("error from server: {}", message);
            }
        }
    }

    Ok(())
}
