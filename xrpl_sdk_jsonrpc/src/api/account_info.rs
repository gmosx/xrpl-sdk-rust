use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountInfoParams {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signer_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

/// - https://xrpl.org/account_info.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountInfoRequest {
    client: Client,
    params: AccountInfoParams,
}

impl AccountInfoRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_info".to_string(),
            params: vec![self.params],
        };
        self.client.send::<AccountInfoParams, T>(request).await
    }

    pub async fn send(self) -> Result<AccountInfoResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponsePayload {
    // TODO!
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "Balance")]
    pub balance: String,

    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponse {
    pub account_data: AccountInfoResponsePayload,
}

impl Client {
    pub fn account_info(&self, account: &str) -> AccountInfoRequest {
        AccountInfoRequest {
            client: self.clone(),
            params: AccountInfoParams {
                account: account.to_string(),
                queue: None,
                ledger_hash: None,
                ledger_index: None,
                signer_lists: None,
                strict: None,
            },
        }
    }
}
