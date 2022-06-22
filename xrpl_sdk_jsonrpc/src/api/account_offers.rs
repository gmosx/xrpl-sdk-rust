use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xrpl_types::Amount;

#[derive(Default, Clone, Serialize)]
pub struct AccountOffersParams {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    // TODO: add more parameters!
}

/// The account_offers method retrieves a list of offers made by a given account
/// that are outstanding as of a particular ledger version.
///
/// https://xrpl.org/account_offers.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountOffersRequest {
    client: Client,
    params: AccountOffersParams,
}

impl AccountOffersRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_offers".to_string(),
            params: vec![self.params],
        };

        self.client.send::<AccountOffersParams, T>(request).await
    }

    pub async fn send(self) -> Result<AccountOffersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffer {
    pub flags: u32,
    pub quality: String,
    pub seq: u32,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffersResponse {
    pub offers: Vec<AccountOffer>,
}

impl Client {
    pub fn account_offers(&self, account: &str) -> AccountOffersRequest {
        AccountOffersRequest {
            client: self.clone(),
            params: AccountOffersParams {
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
    async fn account_offers_works() {
        let client = Client::default();

        let resp = client
            .account_offers("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59")
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
