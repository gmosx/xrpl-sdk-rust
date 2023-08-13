//! The subscribe method requests periodic notifications from the server when
//! certain events happen.
//!
//! <https://xrpl.org/subscribe.html>

use crate::{
    types::{Meta, Transaction},
    Request, ReturnLedgerSpec,
};
use serde::{Deserialize, Serialize};
use xrpl_types::Book;

#[derive(Default, Debug, Clone, Serialize)]
pub struct SubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts_proposed: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    books: Option<Vec<Book>>,
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

    /// The ledger stream only sends ledgerClosed messages when the consensus
    /// process declares a new validated ledger. The message identifies the
    /// ledger and provides some information about its contents.
    pub fn streams(streams: Vec<String>) -> Self {
        Self {
            streams: Some(streams),
            ..Default::default()
        }
    }

    pub fn accounts(accounts: Vec<String>) -> Self {
        Self {
            accounts: Some(accounts),
            ..Default::default()
        }
    }

    pub fn accounts_proposed(accounts: Vec<String>) -> Self {
        Self {
            accounts_proposed: Some(accounts),
            ..Default::default()
        }
    }

    /// When you subscribe to one or more order books with the books field, you
    /// get back any transactions that affect those order books.
    pub fn books(books: Vec<Book>) -> Self {
        Self {
            books: Some(books),
            ..Default::default()
        }
    }

    pub fn url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Default::default()
        }
    }

    pub fn url_username(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubscribeResponse {}

// Streaming Events

#[derive(Debug, Deserialize)]
pub struct LedgerClosedEvent {
    pub fee_base: u32,
    pub fee_ref: u32,
    pub ledger_hash: String,
    pub ledger_index: u32,
    pub ledger_time: u64,
    pub reserve_base: u32,
    pub reserve_inc: u32,
    pub txn_count: u32,
    pub validated_ledgers: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidationReceivedEvent {
    pub base_fee: u32,
    pub cookie: Option<String>,
    pub flags: u32,
    pub ledger_hash: String,
    pub ledger_index: String,
    pub signature: String,
    // #TODO add missing fields
}

#[derive(Debug, Deserialize)]
pub struct TransactionEvent {
    pub engine_result: String,
    pub transaction: Transaction,
    pub meta: Meta,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
