//! <https://xrpl.org/account_objects.html>

use crate::{
    types::LedgerObject, Request, RequestPaginationFragment, RequestWithPagination,
    ResponsePaginationFragment, ResponseWithPagination, RetrieveDataLedgerSpecFragment,
    ReturnDataLedgerSpecFragment, WithRetrieveDataLedgerSpec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountObjectsRequest {
    pub account: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<AccountObjectType>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveDataLedgerSpecFragment,
    #[serde(flatten)]
    pub pagination: RequestPaginationFragment,
}

impl Request for AccountObjectsRequest {
    type Response = AccountObjectsResponse;

    fn method(&self) -> String {
        "account_objects".to_owned()
    }
}

impl WithRetrieveDataLedgerSpec for AccountObjectsRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveDataLedgerSpecFragment {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveDataLedgerSpecFragment {
        &mut self.ledger_spec
    }
}

impl RequestWithPagination for AccountObjectsRequest {
    fn as_pagination(&self) -> &RequestPaginationFragment {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPaginationFragment {
        &mut self.pagination
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
    pub account_objects: Vec<LedgerObject>,
    #[serde(flatten)]
    pub ledger_spec: ReturnDataLedgerSpecFragment,
    #[serde(flatten)]
    pub pagination: ResponsePaginationFragment,
}

impl ResponseWithPagination for AccountObjectsResponse {
    fn as_pagination(&self) -> &ResponsePaginationFragment {
        &self.pagination
    }
}
