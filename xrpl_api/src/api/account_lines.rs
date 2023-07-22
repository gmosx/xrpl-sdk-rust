//! <https://xrpl.org/account_lines.html>

use crate::{
    Request, RequestPaginationFragment, RequestWithPagination, ResponsePaginationFragment,
    ResponseWithPagination, RetrieveDataLedgerSpecFragment, ReturnDataLedgerSpecFragment,
    WithRetrieveDataLedgerSpec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountLinesRequest {
    pub account: String,
    /// The Address of a second account. If provided, show only lines of trust
    /// connecting the two accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    peer: Option<String>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveDataLedgerSpecFragment,
    #[serde(flatten)]
    pub pagination: RequestPaginationFragment,
}

impl Request for AccountLinesRequest {
    type Response = AccountLinesResponse;

    fn method(&self) -> String {
        "account_lines".to_owned()
    }
}

impl WithRetrieveDataLedgerSpec for AccountLinesRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveDataLedgerSpecFragment {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveDataLedgerSpecFragment {
        &mut self.ledger_spec
    }
}

impl RequestWithPagination for AccountLinesRequest {
    fn as_pagination(&self) -> &RequestPaginationFragment {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPaginationFragment {
        &mut self.pagination
    }
}

impl AccountLinesRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }

    pub fn peer(self, peer: &str) -> Self {
        Self {
            peer: Some(peer.to_owned()),
            ..self
        }
    }
}

// #TODO consider extracting as a type.

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLine {
    pub account: String,
    pub balance: String,
    pub currency: String,
    pub limit: String,
    pub limit_peer: String,
    /// Rate at which the account values incoming balances on this trust line,
    /// as a ratio of this value per 1 billion units. (For example, a value of
    /// 500 million represents a 0.5:1 ratio.) As a special case, 0 is treated
    /// as a 1:1 ratio.
    pub quality_in: u64,
    /// Rate at which the account values outgoing balances on this trust line, as a ratio of this value per 1 billion
    /// units. (For example, a value of 500 million represents a 0.5:1 ratio.)
    /// As a special case, 0 is treated as a 1:1 ratio.
    pub quality_out: u64,
    /// If true, this account has enabled the No Ripple flag for this trust line.
    /// If present and false, this account has disabled the No Ripple flag, but,
    /// because the account also has the Default Ripple flag disabled, that is
    /// not considered the default state. If omitted, the account has the
    /// No Ripple flag disabled for this trust line and Default Ripple enabled.
    pub no_ripple: Option<bool>,
    /// If true, the peer account has enabled the No Ripple flag for this trust
    /// line. If present and false, this account has disabled the No Ripple
    /// flag, but, because the account also has the Default Ripple flag
    /// disabled, that is not considered the default state. If omitted,
    /// the account has the No Ripple flag disabled for this trust line and
    /// Default Ripple enabled..
    pub no_ripple_peer: Option<bool>,
    /// If true, this account has authorized this trust line. The default is false.
    pub authorized: Option<bool>,
    /// If true, the peer account has authorized this trust line. The default is
    /// false.
    pub peer_authorized: Option<bool>,
    /// If true, this account has frozen this trust line. The default is false.
    pub freeze: Option<bool>,
    /// If true, the peer account has frozen this trust line. The default is false.
    pub freeze_peer: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AccountLinesResponse {
    pub lines: Vec<AccountLine>,
    #[serde(flatten)]
    pub ledger_spec: ReturnDataLedgerSpecFragment,
    #[serde(flatten)]
    pub pagination: ResponsePaginationFragment,
}

impl ResponseWithPagination for AccountLinesResponse {
    fn as_pagination(&self) -> &ResponsePaginationFragment {
        &self.pagination
    }
}
