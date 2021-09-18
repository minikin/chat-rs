use async_std::net;
use async_std::prelude::*;
use async_std::task;

use chat_rs::shared::utils::ChatResult;
use commands::send_commands;
use replies::handle_replies;

mod commands;
mod replies;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}
