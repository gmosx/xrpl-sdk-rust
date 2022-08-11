//! The subscribe method requests periodic notifications from the server when
//! certain events happen.
//!
//! - https://xrpl.org/subscribe.html

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct SubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_password: Option<String>,
}

impl Request for SubscribeRequest {
    type Response = SubscribeResponse;

    fn method(&self) -> String {
        "subscribe".to_owned()
    }
}

impl SubscribeRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn streams(streams: &[&str]) -> Self {
        let streams = streams.iter().map(|s| s.to_string()).collect();
        Self {
            streams: Some(streams),
            ..Default::default()
        }
    }

    pub fn accounts(accounts: &[&str]) -> Self {
        let accounts = accounts.iter().map(|a| a.to_string()).collect();
        Self {
            accounts: Some(accounts),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubscribeResponse {}

// Streaming Events

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerClosedEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub fee_base: u32,
    pub fee_ref: u32,
    pub ledger_hash: String,
    pub ledger_index: u64,
    pub ledger_time: i64,
    pub reserve_base: u32,
    pub reserve_inc: u32,
    pub txn_count: u32,
    pub validated_ledgers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReceivedEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub base_fee: u32,
    pub cookie: Option<String>,
    pub flags: u32,
    pub ledger_hash: String,
    pub ledger_index: String,
    pub signature: String,
    // #TODO add missing fields
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub engine_result: String,
    // #TODO add missing fields
}
