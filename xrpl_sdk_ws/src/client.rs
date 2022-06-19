use crate::Result;
use futures_util::SinkExt;
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
}
