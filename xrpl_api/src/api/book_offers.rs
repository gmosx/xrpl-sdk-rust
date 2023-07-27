//! The book_offers method retrieves a list of offers, also known as the order
//! book, between two currencies.
//!
//! <https://xrpl.org/book_offers.html>

use serde::{Deserialize, Serialize};
use xrpl_types::{Currency, Offer};

use crate::Request;

#[derive(Default, Debug, Clone, Serialize)]
pub struct BookOffersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taker: Option<String>,
    taker_gets: Currency,
    taker_pays: Currency,
}

impl Request for BookOffersRequest {
    type Response = BookOffersResponse;

    fn method(&self) -> String {
        "book_offers".to_owned()
    }
}

impl BookOffersRequest {
    pub fn new(taker_gets: Currency, taker_pays: Currency) -> Self {
        Self {
            taker_gets,
            taker_pays,
            ..Default::default()
        }
    }

    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    pub fn taker(self, taker: &str) -> Self {
        Self {
            taker: Some(taker.to_string()),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookOffersResponse {
    pub offers: Vec<Offer>,
}

// impl Client {
//     /// Returns the offers on a book. Please note that the term book, on XRPL,
//     /// refers to one-side of an order book (a queue).
//     pub fn book_offers(&self, taker_gets: &Currency, taker_pays: &Currency) -> BookOffersRequest {
//         BookOffersRequest {
//             client: self.clone(),
//             params: BookOffersParams {
//                 ledger_hash: None,
//                 ledger_index: None,
//                 limit: None,
//                 taker: None,
//                 taker_gets: CurrencyParams::from_currency(taker_gets),
//                 taker_pays: CurrencyParams::from_currency(taker_pays),
//             },
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::client::Client;
//     use xrpl_types::Currency;

//     #[tokio::test]
//     async fn book_offers_works() {
//         let client = Client::default();

//         let resp = client
//             .book_offers(
//                 &Currency::xrp(),
//                 &Currency::issued("USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
//             )
//             .send()
//             .await;

//         dbg!(&resp);
//     }
// }
