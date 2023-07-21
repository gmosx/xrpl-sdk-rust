//! <https://xrpl.org/account_objects.html>

use crate::{
    types::LedgerEntry, LedgerSpecRequestFragment, LedgerSpecResponseFragment, Request,
    RequestWithLedgerSpec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountObjectsRequest {
    pub account: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<AccountObjectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
    #[serde(flatten)]
    pub ledger_spec: LedgerSpecRequestFragment,
}

impl Request for AccountObjectsRequest {
    type Response = AccountObjectsResponse;

    fn method(&self) -> String {
        "account_objects".to_owned()
    }
}

impl RequestWithLedgerSpec for AccountObjectsRequest {
    fn as_ledger_index(&self) -> &crate::LedgerSpecRequestFragment {
        &self.ledger_spec
    }

    fn as_ledger_index_mut(&mut self) -> &mut crate::LedgerSpecRequestFragment {
        &mut self.ledger_spec
    }
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountObjectType {
    Check,
    DepositPreauth,
    Escrow,
    NftOffer,
    NftPage,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

impl AccountObjectsRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }

    pub fn object_type(self, object_type: AccountObjectType) -> Self {
        Self {
            object_type: Some(object_type),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountObjectsResponse {
    pub account: String,
    pub account_objects: Vec<LedgerEntry>, // TODO: should we rather use serde_json::Value here and let user deserialize?
    #[serde(flatten)]
    pub ledger_spec: LedgerSpecResponseFragment,
}
