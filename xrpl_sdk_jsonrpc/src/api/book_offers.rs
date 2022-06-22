use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xrpl_types::{Currency, Offer};

#[derive(Clone, Serialize)]
pub struct CurrencyParams {
    pub currency: String, // TODO: hm, consider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl Default for CurrencyParams {
    fn default() -> Self {
        Self {
            currency: "XRP".to_string(),
            issuer: None,
        }
    }
}

impl CurrencyParams {
    pub fn from_currency(c: &Currency) -> Self {
        match c {
            Currency::Xrp => CurrencyParams {
                currency: "XRP".to_owned(),
                issuer: None,
            },
            Currency::Issued { name, issuer } => CurrencyParams {
                currency: name.clone(),
                issuer: Some(issuer.clone()),
            },
        }
    }
}

#[derive(Default, Clone, Serialize)]
pub struct BookOffersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taker: Option<String>,
    taker_gets: CurrencyParams,
    taker_pays: CurrencyParams,
}

/// - https://xrpl.org/book_offers.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct BookOffersRequest {
    client: Client,
    params: BookOffersParams,
}

impl BookOffersRequest {
    pub fn limit(self, limit: u32) -> Self {
        Self {
            params: BookOffersParams {
                limit: Some(limit),
                ..self.params
            },
            ..self
        }
    }

    pub fn taker(self, taker: &str) -> Self {
        Self {
            params: BookOffersParams {
                taker: Some(taker.to_string()),
                ..self.params
            },
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "book_offers".to_string(),
            params: vec![self.params],
        };
        self.client.send::<BookOffersParams, T>(request).await
    }

    pub async fn send(self) -> Result<BookOffersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct BookOffersResponse {
    pub offers: Vec<Offer>,
}

impl Client {
    /// Returns the offers on a book. Please note that the term book, on XRPL,
    /// refers to one-side of an order book (a queue).
    pub fn book_offers(&self, taker_gets: &Currency, taker_pays: &Currency) -> BookOffersRequest {
        BookOffersRequest {
            client: self.clone(),
            params: BookOffersParams {
                ledger_hash: None,
                ledger_index: None,
                limit: None,
                taker: None,
                taker_gets: CurrencyParams::from_currency(taker_gets),
                taker_pays: CurrencyParams::from_currency(taker_pays),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use xrpl_types::Currency;

    #[tokio::test]
    async fn book_offers_works() {
        let client = Client::default();

        let resp = client
            .book_offers(
                &Currency::xrp(),
                &Currency::issued("USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
            )
            .send()
            .await;

        dbg!(&resp);
    }
}
