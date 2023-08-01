//! <https://xrpl.org/account_objects.html>

use crate::{
    LedgerObject, Request, RequestPagination, ResponsePagination, RetrieveLedgerSpec,
    ReturnLedgerSpec, WithLedgerSpec, WithRequestPagination, WithResponsePagination,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountObjectsRequest {
    pub account: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<AccountObjectType>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for AccountObjectsRequest {
    type Response = AccountObjectsResponse;

    fn method(&self) -> String {
        "account_objects".to_owned()
    }
}

impl WithLedgerSpec for AccountObjectsRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl WithRequestPagination for AccountObjectsRequest {
    fn as_pagination(&self) -> &RequestPagination {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPagination {
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
    pub ledger_spec: ReturnLedgerSpec,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for AccountObjectsResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
