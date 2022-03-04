use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct ServerStateParams {}

/// - https://xrpl.org/server_state.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct ServerStateRequest {
    client: Client,
}

impl ServerStateRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "server_state".to_string(),
            params: vec![],
        };
        self.client.send::<ServerStateParams, T>(request).await
    }

    pub async fn send(self) -> Result<ServerStateResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidatedLedger {
    pub seq: u32,
    pub base_fee: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerStateResponsePayload {
    pub validated_ledger: ValidatedLedger,
}

#[derive(Debug, Deserialize)]
pub struct ServerStateResponse {
    pub state: ServerStateResponsePayload,
}

impl Client {
    pub fn server_state(&self) -> ServerStateRequest {
        ServerStateRequest {
            client: self.clone(),
        }
    }
}
