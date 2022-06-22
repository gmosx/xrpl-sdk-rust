use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// TIP: Better use the more specialized methods, like `get_offer_object`.

#[derive(Default, Clone, Serialize)]
pub struct OfferParams {
    account: String,
    seq: u32,
}

#[derive(Default, Clone, Serialize)]
pub struct LedgerEntryParams {
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offer: Option<OfferParams>,
}

// https://xrpl.org/ledger_entry.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct LedgerEntryRequest {
    client: Client,
    params: LedgerEntryParams,
}

impl LedgerEntryRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "ledger_entry".to_string(),
            params: vec![self.params],
        };
        self.client.send::<LedgerEntryParams, T>(request).await
    }

    pub async fn send(self) -> Result<LedgerEntryResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerEntryResponse {
    /// The unique ID of this ledger object.
    pub index: String,
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: u32,
    /// (Omitted if "binary": true specified.) Object containing the data of this ledger object, according to the ledger format.
    pub node: Option<serde_json::Value>,
    /// (Omitted unless "binary":true specified) The binary representation of the ledger object, as hexadecimal.
    pub node_binary: Option<String>,
}

impl Client {
    pub fn offer_ledger_entry(&self, account: &str, sequence: u32) -> LedgerEntryRequest {
        LedgerEntryRequest {
            client: self.clone(),
            params: LedgerEntryParams {
                ledger_hash: None,
                ledger_index: None,
                binary: Some(false),
                offer: Some(OfferParams {
                    account: account.to_owned(),
                    seq: sequence,
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn offer_ledger_entry_works() {
        let client = Client::default();

        let resp = client
            .offer_ledger_entry("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn", 359)
            .send()
            .await;

        dbg!(&resp);
    }
}
