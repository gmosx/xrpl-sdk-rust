use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountCurrenciesParams {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

/// https://xrpl.org/account_currencies.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountCurrenciesRequest {
    client: Client,
    params: AccountCurrenciesParams,
}

impl AccountCurrenciesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_currencies".to_string(),
            params: vec![self.params],
        };
        self.client
            .send::<AccountCurrenciesParams, T>(request)
            .await
    }

    pub async fn send(self) -> Result<AccountCurrenciesResponse> {
        self.execute().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCurrenciesResponsePayload {
    // TODO!
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "Balance")]
    pub balance: String,

    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCurrenciesResponse {
    /// The ledger index of the ledger version used to retrieve this data.
    pub ledger_index: u32,
    /// Array of Currency Codes for currencies that this account can receive.
    pub receive_currencies: Vec<String>,
    /// Array of Currency Codes for currencies that this account can send.
    pub send_currencies: Vec<String>,
    /// If true, this data comes from a validated ledger.
    pub validated: bool,
}

impl Client {
    pub fn account_currencies(&self, account: &str) -> AccountCurrenciesRequest {
        AccountCurrenciesRequest {
            client: self.clone(),
            params: AccountCurrenciesParams {
                account: account.to_string(),
                ledger_hash: None,
                ledger_index: None,
                strict: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn account_currencies_should_return_receive_and_send_currencies() {
        let client = Client::default();

        let resp = client
            .account_currencies("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59")
            .send()
            .await;

        dbg!(&resp);
    }
}
