use crate::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

pub const XRPL_CLUSTER_WS_URL: &str = "wss://xrplcluster.com";

pub const DEFAULT_WS_URL: &str = XRPL_CLUSTER_WS_URL;

/// A WebSocket client for the XRP Ledger.
pub struct Client {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    // #TODO return Result
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, msg: &str) -> Result<()> {
        self.stream.send(Message::Text(msg.to_string())).await?;
        Ok(())
    }

    // pub async fn publish(topic: String) {
    //     todo!()
    // }

    pub async fn subscribe(topics: &[String]) {
        todo!()
    }

    pub async fn subscribe_accounts(accounts: &[String]) {
        todo!()
    }

    // subscribe_accounts
    // subscribe_books
    // subscribe_streams
    // unsubscribe
    // unsubscribe_accounts
    // unsubscribe_books
    // unsubscribe_streams
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
                .send(r#"{"id": 1, "command": "subscribe", "streams": ["ledger"]}"#)
                .await
                .expect("cannot subscribe");

            let stream = client.stream;

            tokio::pin!(stream);

            while let Some(msg) = stream.next().await {
                dbg!(&msg);
            }
        });
    }
}
