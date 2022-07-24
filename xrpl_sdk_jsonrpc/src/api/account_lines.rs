use crate::{client::RpcRequest, Client, Result};
use serde::de::DeserializeOwned;
use xrpl_api::{AccountLinesRequestPayload, AccountLinesResponsePayload};

/// The account_lines method returns information about an account's trust lines,
/// including balances in all non-XRP currencies and assets. All information
/// retrieved is relative to a particular version of the ledger.
///
/// https://xrpl.org/account_lines.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountLinesRequest {
    client: Client,
    params: AccountLinesRequestPayload,
}

impl AccountLinesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_lines".to_string(),
            params: vec![self.params],
        };
        self.client
            .send_old::<AccountLinesRequestPayload, T>(request)
            .await
    }

    pub async fn send(self) -> Result<AccountLinesResponsePayload> {
        self.execute().await
    }
}

impl Client {
    pub fn account_lines(&self, account: &str) -> AccountLinesRequest {
        AccountLinesRequest {
            client: self.clone(),
            params: AccountLinesRequestPayload {
                account: account.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn account_lines_works() {
        let client = Client::default();

        let resp = client
            .account_lines("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59")
            .send()
            .await;

        dbg!(&resp);
    }
}
