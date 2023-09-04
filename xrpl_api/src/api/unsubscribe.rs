//! <https://xrpl.org/unsubscribe.html>

use crate::{Book, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct UnsubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts_proposed: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    books: Option<Vec<Book>>,
}

impl Request for UnsubscribeRequest {
    type Response = UnsubscribeResponse;

    fn method(&self) -> String {
        "unsubscribe".to_owned()
    }
}

impl UnsubscribeRequest {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn books(books: Vec<Book>) -> Self {
        Self {
            books: Some(books),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeResponse {}
