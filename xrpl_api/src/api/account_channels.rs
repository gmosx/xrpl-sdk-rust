//! The account_channels method returns information about an account's Payment
//! Channels. This includes only channels where the specified account is the
//! channel's source, not the destination. (A channel's "source" and "owner"
//! are the same.) All information retrieved is relative to a particular version
//! of the ledger.
//!
//! <https://xrpl.org/account_channels.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountChannelsRequest {
    /// The unique identifier of an account, typically the account's Address.
    /// The request returns channels where this account is the channel's owner/source.
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// Value from a previous paginated response. Resume retrieving data where
    /// that response left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

impl Request for AccountChannelsRequest {
    type Response = AccountChannelsResponse;

    fn method(&self) -> String {
        "account_channels".to_owned()
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
    pub limit: Option<u32>,
    pub marker: Option<String>,
}
