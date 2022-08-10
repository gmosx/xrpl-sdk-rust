use crate::Request;
use serde::{Deserialize, Serialize};

/// https://xrpl.org/subscribe.html
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
