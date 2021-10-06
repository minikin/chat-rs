use async_std::io;
use async_std::net;
use async_std::prelude::*;
use chat_rs::shared::client_response::ClientResponse;
use chat_rs::shared::utils::{self, ChatResult};
use std::sync::Arc;

pub async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
              join CHAT GROUP\n\
              post MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows) \
              to close the connection."
    );

    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

pub fn parse_command(line: &str) -> Option<ClientResponse> {
    let (command, rest) = get_next_token(line)?;
    if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        Some(ClientResponse::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        })
    } else if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        Some(ClientResponse::Join {
            group_name: Arc::new(group.to_string()),
        })
    } else {
        eprintln!("Unrecognized command: {:?}", line);
        None
    }
}

/// Given a string `input`, return `Some((token, rest))`, where `token` is the
/// first run of non-whitespace characters in `input`, and `rest` is the rest of
/// the string. If the string contains no non-whitespace characters, return
/// `None`.
fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}
