//! https://xrpl.org/account_currencies.html

use crate::{client::RpcRequest, Client, Result};
use serde::de::DeserializeOwned;
use xrpl_api::{AccountCurrenciesRequestPayload, AccountCurrenciesResponsePayload};

#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountCurrenciesRequest {
    client: Client,
    params: AccountCurrenciesRequestPayload,
}

impl AccountCurrenciesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_currencies".to_string(),
            params: vec![self.params],
        };
        self.client
            .send::<AccountCurrenciesRequestPayload, T>(request)
            .await
    }

    pub async fn send(self) -> Result<AccountCurrenciesResponsePayload> {
        self.execute().await
    }
}

impl Client {
    pub fn account_currencies(&self, account: &str) -> AccountCurrenciesRequest {
        AccountCurrenciesRequest {
            client: self.clone(),
            params: AccountCurrenciesRequestPayload {
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
