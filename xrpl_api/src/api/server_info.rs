//! The server_info command asks the server for a human-readable version of
//! various information about the rippled server being queried.
//!
//! <https://xrpl.org/server_info.html>

use crate::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize)]
pub struct ServerInfoRequest {}

impl Request for ServerInfoRequest {
    type Response = ServerInfoResponse;

    fn method(&self) -> String {
        "server_info".to_owned()
    }
}

impl ServerInfoRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Information about the time the server spends in server state. This can be
/// useful for tracking the long-term health of your server's connectivity to
/// the network.
#[derive(Debug, Deserialize)]
pub struct StateAccountingInfo {
    /// The number of microseconds the server has spent in this state.
    /// (This is updated whenever the server transitions into another state.)
    pub duration_us: String,
    /// The number of times the server has changed into this state.
    pub transitions: String,
}

#[derive(Debug, Deserialize)]
pub struct SIValidatedLedger {
    pub seq: u32,
    pub base_fee_xrp: f64,
}

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    pub amendment_blocked: Option<bool>,
    pub build_version: Option<String>,
    pub peers: Option<u32>,
    pub hostid: Option<String>,
    /// <https://xrpl.org/rippled-server-states.html>
    pub server_state: Option<String>,
    pub state_accounting: Option<HashMap<String, StateAccountingInfo>>,
    pub time: Option<String>,
    pub uptime: Option<u32>,
    pub validated_ledger: SIValidatedLedger,
    // #TODO add more fields
}

#[derive(Debug, Deserialize)]
pub struct ServerInfoResponse {
    pub info: ServerInfo,
}
