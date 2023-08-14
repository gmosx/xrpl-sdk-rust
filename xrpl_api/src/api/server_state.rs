//! The server_state command asks the server for various machine-readable
//! information about the rippled server's current state. The response is
//! almost the same as the server_info method, but uses units that are easier
//! to process instead of easier to read. (For example, XRP values are given in
//! integer drops instead of scientific notation or decimal values, and time is
//! given in milliseconds instead of seconds.)
//!
//! <https://xrpl.org/server_state.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct ServerStateRequest {}

impl Request for ServerStateRequest {
    type Response = ServerStateResponse;

    fn method(&self) -> String {
        "server_state".to_owned()
    }
}

impl ServerStateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Deserialize)]
pub struct SSValidatedLedger {
    pub seq: u32,
    pub base_fee: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerState {
    pub validated_ledger: SSValidatedLedger,
}

#[derive(Debug, Deserialize)]
pub struct ServerStateResponse {
    pub state: ServerState,
}
