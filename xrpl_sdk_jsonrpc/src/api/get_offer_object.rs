use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xrpl_types::Offer;

#[derive(Default, Clone, Serialize)]
pub struct OfferParams {
    account: String,
    seq: u32,
}

#[derive(Default, Clone, Serialize)]
pub struct GetOfferObjectParams {
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    offer: OfferParams,
}

// https://xrpl.org/ledger_entry.html
// https://xrpl.org/ledger_entry.html#get-offer-object
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct GetOfferObjectRequest {
    client: Client,
    params: GetOfferObjectParams,
}

impl GetOfferObjectRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "ledger_entry".to_string(),
            params: vec![self.params],
        };
        self.client
            .send_old::<GetOfferObjectParams, T>(request)
            .await
    }

    pub async fn send(self) -> Result<GetOfferObjectResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetOfferObjectResponse {
    /// The unique ID of this ledger object.
    pub index: String,
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: u32,
    /// (Omitted if "binary": true specified.) Object containing the data of this ledger object, according to the ledger format.
    pub node: Option<Offer>,
    /// (Omitted unless "binary":true specified) The binary representation of the ledger object, as hexadecimal.
    pub node_binary: Option<String>,
}

impl Client {
    pub fn get_offer_object(&self, account: &str, sequence: u32) -> GetOfferObjectRequest {
        GetOfferObjectRequest {
            client: self.clone(),
            params: GetOfferObjectParams {
                ledger_hash: None,
                ledger_index: None,
                binary: Some(false),
                offer: OfferParams {
                    account: account.to_owned(),
                    seq: sequence,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn get_offer_object_works() {
        let client = Client::default();

        let resp = client
            .get_offer_object("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn", 359)
            .send()
            .await;

        dbg!(&resp);
    }
}
