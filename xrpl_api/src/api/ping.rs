//! The ping command returns an acknowledgement, so that clients can test the
//! connection status and latency.
//!
//! - https://xrpl.org/ping.html

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct PingRequest {}

impl Request for PingRequest {
    type Response = PingResponse;

    fn method(&self) -> String {
        "ping".to_owned()
    }
}

impl PingRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Deserialize)]
pub struct PingResponse {}
