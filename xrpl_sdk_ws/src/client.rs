use crate::util::Result;
use futures::{future, stream::SplitSink, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;
use xrpl_api::{AccountInfoResponse, LedgerClosedEvent, Request};

// https://xrpl.org/public-servers.html

pub const XRPL_CLUSTER_MAINNET_WS_URL: &str = "wss://xrplcluster.com";
pub const S1_MAINNET_WS_URL: &str = "wss://s1.ripple.com";
pub const S2_MAINNET_WS_URL: &str = "wss://s2.ripple.com";
pub const TESTNET_WS_URL: &str = "wss://s.altnet.rippletest.net/";
pub const DEVNET_WS_URL: &str = "wss://s.devnet.rippletest.net/";
pub const NFT_DEVNET_WS_URL: &str = "wss://xls20-sandbox.rippletest.net:51233";

pub const DEFAULT_WS_URL: &str = XRPL_CLUSTER_MAINNET_WS_URL;

// #TODO extract Connection

#[derive(Serialize, Deserialize, Debug)]
pub enum Datum {
    AccountInfo(AccountInfoResponse),
    LedgeClosed(LedgerClosedEvent),
    Other(String),
}

/// A WebSocket client for the XRP Ledger.
pub struct Client {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub messages: Pin<Box<dyn Stream<Item = Result<Datum>>>>,
    requests: Rc<RefCell<HashMap<String, String>>>,
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        let (sender, receiver) = stream.split();
        let requests: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(HashMap::new()));
        let cloned_requests = requests.clone();
        let receiver = receiver
            .map(move |msg| {
                if let Message::Text(string) = msg.unwrap() {
                    let mut value: serde_json::Value = serde_json::from_str(&string).unwrap();

                    if let Some(id) = value["id"].as_str() {
                        if let Some(method) = requests.borrow_mut().get(id) {
                            // println!("=>>>>> {method}");
                            match method.as_str() {
                                "account_info" => {
                                    let result = value["result"].take();
                                    Ok(Some(Datum::AccountInfo(serde_json::from_value(result)?)))
                                }
                                _ => Ok(Some(Datum::Other(string))),
                            }
                        } else {
                            Ok(Some(Datum::Other(string)))
                        }
                    } else {
                        // No ID, this is a subscription event

                        if let Some(event_type) = value["type"].as_str() {
                            // println!("**** {event_type}");
                            match event_type {
                                "ledgerClosed" => {
                                    Ok(Some(Datum::LedgeClosed(serde_json::from_value(value)?)))
                                }
                                _ => Ok(Some(Datum::Other(string))),
                            }
                        } else {
                            Ok(Some(Datum::Other(string)))
                        }
                    }
                } else {
                    Ok(None)
                }
            })
            .filter_map(|res| future::ready(res.transpose()));

        Ok(Self {
            sender,
            messages: Box::pin(receiver),
            requests: cloned_requests,
        })
    }

    pub async fn call<Req>(&mut self, req: Req) -> Result<()>
    where
        Req: Request + Serialize,
    {
        let id = self.next_id();

        let msg = serde_json::to_value(&req).unwrap(); // #TODO use `?`.

        // #TODO, this is temp code, add error-handling!

        if let serde_json::Value::Object(mut map) = msg {
            map.insert("id".to_owned(), serde_json::Value::String(id.clone()));
            map.insert(
                "command".to_owned(),
                serde_json::Value::String(req.method()),
            );
            let msg = serde_json::to_string(&map).unwrap();

            self.sender.send(Message::Text(msg.to_string())).await?;

            self.requests.borrow_mut().insert(id, req.method());
        }

        Ok(())
    }

    // #TODO make this customizable.
    pub fn next_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
