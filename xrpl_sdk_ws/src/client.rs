use crate::{util::format_joined_keys, Result};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

pub const XRPL_CLUSTER_WS_URL: &str = "wss://xrplcluster.com";

pub const DEFAULT_WS_URL: &str = XRPL_CLUSTER_WS_URL;

// #TODO extract Connection
// #TODO split into multiple `api` files

/// A WebSocket client for the XRP Ledger.
pub struct Client {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, msg: &str) -> Result<()> {
        self.stream.send(Message::Text(msg.to_string())).await?;
        Ok(())
    }

    pub async fn subscribe_account(&mut self, account: &str) -> Result<()> {
        let msg =
            format!("{{\"id\": 1, \"command\": \"subscribe\", \"accounts\": [\"{account}\"]}}");
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn subscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let accounts = format_joined_keys(accounts);
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"accounts\": [{accounts}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topic`
    pub async fn subscribe_stream(&mut self, stream: &str) -> Result<()> {
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"streams\": [\"{stream}\"]}}");
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topics`
    pub async fn subscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let streams = format_joined_keys(streams);
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"streams\": [{streams}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn unsubscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let accounts = format_joined_keys(accounts);
        let msg =
            format!("{{\"id\": 1, \"command\": \"unsubscribe\", \"accounts\": [{accounts}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn unsubscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let streams = format_joined_keys(streams);
        let msg = format!("{{\"id\": 1, \"command\": \"unsubscribe\", \"streams\": [{streams}]}}");
        self.send(&msg).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_should_connect() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let mut client = Client::connect(DEFAULT_WS_URL)
                .await
                .expect("cannot connect");

            client
                .subscribe_streams(&["ledger"])
                .await
                .expect("cannot subscribe");

            let (_, rx) = client.stream.split();

            tokio::pin!(rx);

            while let Some(msg) = rx.next().await {
                dbg!(&msg);
            }
        });
    }
}
