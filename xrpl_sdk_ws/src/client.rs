use crate::util::Result;
use futures::{
    future,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use futures_util::SinkExt;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;
use xrpl_api::Request;

// https://xrpl.org/public-servers.html

pub const XRPL_CLUSTER_MAINNET_WS_URL: &str = "wss://xrplcluster.com";
pub const S1_MAINNET_WS_URL: &str = "wss://s1.ripple.com";
pub const S2_MAINNET_WS_URL: &str = "wss://s2.ripple.com";
pub const TESTNET_WS_URL: &str = "wss://s.altnet.rippletest.net/";
pub const DEVNET_WS_URL: &str = "wss://s.devnet.rippletest.net/";
pub const NFT_DEVNET_WS_URL: &str = "wss://xls20-sandbox.rippletest.net:51233";

pub const DEFAULT_WS_URL: &str = XRPL_CLUSTER_MAINNET_WS_URL;

// #TODO extract Connection

/// A WebSocket client for the XRP Ledger.
pub struct Client {
    // pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        let (sender, receiver) = stream.split();
        Ok(Self { sender, receiver })
    }

    pub async fn call<Req>(&mut self, req: Req) -> Result<()>
    where
        Req: Request + Serialize,
    {
        let id = self.next_id();

        let msg = serde_json::to_value(&req).unwrap(); // #TODO use `?`.

        // #TODO, this is temp code, add error-handling!

        if let serde_json::Value::Object(mut map) = msg {
            map.insert("id".to_owned(), serde_json::Value::String(id));
            map.insert(
                "command".to_owned(),
                serde_json::Value::String(req.method()),
            );
            let msg = serde_json::to_string(&map).unwrap();

            self.sender.send(Message::Text(msg.to_string())).await?;
        }

        Ok(())
    }

    pub fn split(
        self,
    ) -> (
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        impl Stream<Item = Result<String>>,
    ) {
        let receiver = self
            .receiver
            .map(|msg| {
                if let Message::Text(string) = msg.unwrap() {
                    Ok(Some(string))
                } else {
                    Ok(None)
                }
            })
            .filter_map(|res| future::ready(res.transpose()));

        (self.sender, receiver)
    }

    // #TODO make this customizable.
    pub fn next_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
