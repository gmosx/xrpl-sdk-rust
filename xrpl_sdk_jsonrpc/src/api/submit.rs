use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Serialize};

/// - https://xrpl.org/submit.html
#[derive(Default, Clone, Serialize)]
pub struct SubmitParams {
    tx_blob: String,
}

#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone, Serialize)]
pub struct SubmitRequest {
    #[serde(skip_serializing)]
    client: Client,
    params: SubmitParams,
}

impl SubmitRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "submit".to_string(),
            params: vec![self.params],
        };
        self.client.send_old::<SubmitParams, T>(request).await
    }

    pub async fn send(self) -> Result<SubmitResponse> {
        self.execute().await
    }
}

// #[derive(Debug, Deserialize)]
// pub struct SubmitResponse {
//     pub accepted: Option<bool>,
//     pub applied: bool,
//     pub engine_result: String,
//     pub status: String,
// }

pub type SubmitResponse = serde_json::Value;

impl Client {
    pub fn submit(&self, tx_blob: &str) -> SubmitRequest {
        SubmitRequest {
            client: self.clone(),
            params: SubmitParams {
                tx_blob: tx_blob.to_string(),
            },
        }
    }
}
