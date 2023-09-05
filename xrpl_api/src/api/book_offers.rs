//! The book_offers method retrieves a list of offers, also known as the order
//! book, between two currencies.
//!
//! <https://xrpl.org/book_offers.html>

use serde::{Deserialize, Serialize};

use crate::{Currency, Offer, Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};

#[derive(Default, Debug, Clone, Serialize)]
pub struct BookOffersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taker: Option<String>,
    taker_gets: Currency,
    taker_pays: Currency,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
}

impl Request for BookOffersRequest {
    type Response = BookOffersResponse;

    fn method(&self) -> String {
        "book_offers".to_owned()
    }
}

impl WithLedgerSpec for BookOffersRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
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

    pub fn taker(self, taker: &str) -> Self {
        Self {
            taker: Some(taker.to_string()),
            ..self
        }
    }

    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookOffersResponse {
    pub offers: Vec<Offer>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
