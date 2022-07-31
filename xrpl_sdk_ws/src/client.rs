use crate::util::Result;
use futures_util::SinkExt;
use serde::Serialize;
use tokio::net::TcpStream;
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

pub const DEFAULT_WS_URL: &str = XRPL_CLUSTER_MAINNET_WS_URL;

// #TODO extract Connection

/// A WebSocket client for the XRP Ledger.
pub struct Client {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        Ok(Self { stream })
    }

    // // #Deprecated will be removed!
    // pub async fn send_old(&mut self, msg: &str) -> Result<()> {
    //     self.stream.send(Message::Text(msg.to_string())).await?;
    //     Ok(())
    // }

    pub async fn send<Req>(&mut self, req: Req) -> Result<()>
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

            self.stream.send(Message::Text(msg.to_string())).await?;
        }

        Ok(())
    }

    // #TODO make this customizable.
    pub fn next_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
