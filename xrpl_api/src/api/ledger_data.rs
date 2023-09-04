//! The ledger_data method retrieves contents of the specified ledger. You can
//! iterate through several calls to retrieve the entire contents of a single
//! ledger version.
//!
//! <https://xrpl.org/ledger_data.html>

use crate::{
    LedgerIndex, LedgerObject, ObjectType, Request, RequestPagination, ResponsePagination,
    RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec, WithRequestPagination,
    WithResponsePagination,
};
use serde::{Deserialize, Serialize};

// TIP: Better use the more specialized methods, like `get_offer_object`.

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerDataRequest {
    /// If set to true, return ledger objects as hashed hex strings instead of JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for LedgerDataRequest {
    type Response = LedgerDataResponse;

    fn method(&self) -> String {
        "ledger_data".to_owned()
    }
}

impl WithLedgerSpec for LedgerDataRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl WithRequestPagination for LedgerDataRequest {
    fn as_pagination(&self) -> &RequestPagination {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPagination {
        &mut self.pagination
    }
}

impl LedgerDataRequest {
    pub fn with_ledger_hash(ledger_hash: impl Into<String>) -> Self {
        LedgerDataRequest::default().ledger_hash(ledger_hash)
    }

    pub fn with_ledger_index(ledger_index: LedgerIndex) -> Self {
        LedgerDataRequest::default().ledger_index(ledger_index)
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerData {
    /// Hex representation of the requested data.
    /// (Only included if "binary":true)
    pub data: Option<String>,
    #[serde(flatten)]
    pub object: Option<LedgerObject>,
    /// Unique identifier for this ledger entry, as hex.
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct LedgerDataResponse {
    /// Array of JSON objects containing data from the ledger's state tree.
    pub state: Vec<LedgerData>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for LedgerDataResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
