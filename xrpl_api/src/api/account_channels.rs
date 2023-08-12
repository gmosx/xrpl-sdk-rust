//! The account_channels method returns information about an account's Payment
//! Channels. This includes only channels where the specified account is the
//! channel's source, not the destination. (A channel's "source" and "owner"
//! are the same.) All information retrieved is relative to a particular version
//! of the ledger.
//!
//! <https://xrpl.org/account_channels.html>

use crate::{
    Request, RequestPagination, ResponsePagination, RetrieveLedgerSpec, ReturnLedgerSpec,
    WithLedgerSpec, WithRequestPagination, WithResponsePagination,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountChannelsRequest {
    /// The unique identifier of an account, typically the account's Address.
    /// The request returns channels where this account is the channel's owner/source.
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_account: Option<String>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for AccountChannelsRequest {
    type Response = AccountChannelsResponse;

    fn method(&self) -> String {
        "account_channels".to_owned()
    }
}

impl WithLedgerSpec for AccountChannelsRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl WithRequestPagination for AccountChannelsRequest {
    fn as_pagination(&self) -> &RequestPagination {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPagination {
        &mut self.pagination
    }
}

impl AccountChannelsRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }
}

// TODO: consider extracting as a type.

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountChannel {
    /// The owner of the channel, as an Address.
    pub account: String,
    /// The destination account of the channel, as an Address. Only this account
    /// can receive the XRP in the channel while it is open.
    pub destination_account: String,
    /// The total amount of XRP, in drops allocated to this channel.
    pub amount: String,
    /// The total amount of XRP, in drops, paid out from this channel, as of
    /// the ledger version used. (You can calculate the amount of XRP left in
    /// the channel by subtracting balance from amount.)
    pub balance: String,
    /// A unique ID for this channel, as a 64-character hexadecimal string. This
    /// is also the ID of the channel object in the ledger's state data.
    pub channel_id: String,
    /// The number of seconds the payment channel must stay open after the owner
    /// of the channel requests to close it.
    pub settle_delay: u32,
    pub public_key: Option<String>,
    pub expiration: Option<u32>,
    pub cancel_after: Option<u32>,
    pub source_tag: Option<u32>,
    pub destination_tag: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AccountChannelsResponse {
    pub account: String,
    pub channels: Vec<AccountChannel>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for AccountChannelsResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
