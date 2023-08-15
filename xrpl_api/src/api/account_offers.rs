//! The account_offers method retrieves a list of offers made by a given account
//! that are outstanding as of a particular ledger version.
//!
//! <https://xrpl.org/account_offers.html>

use std::cmp::Ordering;

use crate::{
    OfferFlags, Request, RequestPagination, ResponsePagination, RetrieveLedgerSpec,
    ReturnLedgerSpec, WithLedgerSpec, WithRequestPagination, WithResponsePagination,
};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountOffersRequest {
    account: String,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for AccountOffersRequest {
    type Response = AccountOffersResponse;

    fn method(&self) -> String {
        "account_offers".to_owned()
    }
}

impl WithLedgerSpec for AccountOffersRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl WithRequestPagination for AccountOffersRequest {
    fn as_pagination(&self) -> &RequestPagination {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPagination {
        &mut self.pagination
    }
}

impl AccountOffersRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffer {
    pub flags: BitFlags<OfferFlags>,
    pub quality: String,
    pub seq: u32,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

impl PartialEq for AccountOffer {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for AccountOffer {}

impl PartialOrd for AccountOffer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// #insight
// The offer sequence is the natural ordering key for AccountOffers.
// Moreover, the sequence acts as a unique id (within an account) so we can use
// it for Eq/PartialEq.

impl Ord for AccountOffer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seq.cmp(&other.seq)
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountOffersResponse {
    pub offers: Vec<AccountOffer>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for AccountOffersResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
