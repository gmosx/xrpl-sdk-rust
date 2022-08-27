//! <https://xrpl.org/unsubscribe.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct UnsubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<String>>,
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
pub struct UnsubscribeResponse {}
