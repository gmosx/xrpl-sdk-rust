use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct LedgerClosedParams {}

// https://xrpl.org/ledger_closed.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct LedgerClosedRequest {
    client: Client,
}

impl LedgerClosedRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "ledger_closed".to_string(),
            params: vec![],
        };
        self.client.send_old::<LedgerClosedParams, T>(request).await
    }

    pub async fn send(self) -> Result<LedgerClosedResponse> {
        self.execute().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerClosedResponse {
    /// The unique Hash of this ledger version, in hexadecimal.
    pub ledger_hash: String,
    /// The ledger index of this ledger version.
    pub ledger_index: u32,
}

impl Client {
    pub fn ledger_closed(&self) -> LedgerClosedRequest {
        LedgerClosedRequest {
            client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn should_implement_ledger_closed() {
        let client = Client::default();

        let resp = client.ledger_closed().send().await;

        dbg!(&resp);
    }
}
