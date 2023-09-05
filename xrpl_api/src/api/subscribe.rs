//! The subscribe method requests periodic notifications from the server when
//! certain events happen.
//!
//! <https://xrpl.org/subscribe.html>

use crate::{Currency, Request};
use serde::{Deserialize, Serialize};

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

/// A book on the ledger.
#[derive(Debug, Clone, Serialize)]
pub struct Book {
    /// Specification of which currency the account taking the Offer would pay.
    pub taker_gets: Currency,
    /// Specification of which currency the account taking the Offer would receive.
    pub taker_pays: Currency,
    /// Unique account address to use as a perspective for viewing offers, in the XRP Ledger's base58 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    /// If true, return the current state of the order book once when you subscribe before sending updates.
    /// The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// If true, return both sides of the order book. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub both: Option<bool>,
}

impl Book {
    pub fn new(taker_gets: Currency, taker_pays: Currency) -> Self {
        Self {
            taker_gets,
            taker_pays,
            taker: None,
            snapshot: None,
            both: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }

    pub fn taker(self, taker: impl Into<String>) -> Self {
        Self {
            taker: Some(taker.into()),
            ..self
        }
    }

    pub fn both(self, both: bool) -> Self {
        Self {
            both: Some(both),
            ..self
        }
    }
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
