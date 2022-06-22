use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountLinesParams {
    account: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // ledger_hash: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // ledger_index: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // strict: Option<bool>,
    // TODO: add more parameters!
}

/// The account_lines method returns information about an account's trust lines,
/// including balances in all non-XRP currencies and assets. All information
/// retrieved is relative to a particular version of the ledger.
///
/// https://xrpl.org/account_lines.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountLinesRequest {
    client: Client,
    params: AccountLinesParams,
}

impl AccountLinesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_lines".to_string(),
            params: vec![self.params],
        };
        self.client.send::<AccountLinesParams, T>(request).await
    }

    pub async fn send(self) -> Result<AccountLinesResponse> {
        self.execute().await
    }
}

// TODO: consider extracting as a type.

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLine {
    pub account: String,
    pub balance: String,
    pub currency: String,
    pub limit: String,
    pub limit_peer: String,
    pub no_ripple: bool,
    pub quality_in: u64,
    pub quality_out: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccountLinesResponse {
    pub lines: Vec<AccountLine>,
}

impl Client {
    pub fn account_lines(&self, account: &str) -> AccountLinesRequest {
        AccountLinesRequest {
            client: self.clone(),
            params: AccountLinesParams {
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

        // if let Ok(resp) = resp {
        //     let order_book = resp.order_book;

        //     assert_eq!(order_book.bid_queue().len() as u32, depth);
        //     assert_eq!(order_book.ask_queue().len() as u32, depth);
        // }
    }
}
