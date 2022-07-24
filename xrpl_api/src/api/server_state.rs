use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
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
pub struct ValidatedLedger {
    pub seq: u32,
    pub base_fee: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerState {
    pub validated_ledger: ValidatedLedger,
}

#[derive(Debug, Deserialize)]
pub struct ServerStateResponse {
    pub state: ServerState,
}
