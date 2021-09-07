use async_std::io::prelude::WriteExt;
use async_std::io::Write;
use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<S, P>(outward: &mut S, data: &P) -> ChatResult<()>
where
    S: Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&data)?;
    json.push('\n');
    outward.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive_as_json<S, P>(incoming: S) -> impl Stream<Item = ChatResult<P>>
where
    S: async_std::io::BufRead + Unpin,
    P: DeserializeOwned,
{
    incoming.lines().map(|line_result| -> ChatResult<P> {
        let line = line_result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
