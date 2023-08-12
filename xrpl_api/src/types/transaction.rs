use serde::{Deserialize, Serialize};

use crate::types::transactions::*;
use crate::Meta;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionCommon {
    pub account: String,
    pub fee: String,
    pub sequence: u32,
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<String>,
    pub flags: Option<u32>,
    pub last_ledger_sequence: Option<u32>,
    // pub memos: Option<Vec<Memo>>,
    pub memos: Option<Vec<serde_json::Value>>,
    pub network_id: Option<u32>,
    pub source_tag: Option<u32>,
    pub signing_pub_key: Option<String>,
    pub ticket_sequence: Option<u32>,
    pub txn_signature: Option<String>,

    /// Close time of the ledger in which the transaction is included
    #[serde(rename = "date")]
    pub date: Option<u64>,

    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,

    /// The ledger index of the ledger that includes this transaction.
    #[serde(rename = "ledger_index")]
    pub ledger_index: Option<u32>,
    /// If true, this data comes from a validated ledger version; if omitted or
    /// set to false, this data is not final.
    #[serde(rename = "validated")]
    pub validated: Option<bool>,

    /// Meta is present in transactions returned by https://xrpl.org/ledger.html and
    /// also <https://xrpl.org/tx.html>. In other API
    /// methods it is found outside (next to) the transaction field.
    #[serde(rename = "meta", alias = "metaData")]
    pub meta: Option<Meta>,

    /// `owner_funds` is present in transactions returned by book subscription, see
    /// <https://xrpl.org/subscribe.html#order-book-streams>.
    #[serde(rename = "owner_funds")]
    pub owner_funds: Option<String>,
}

// #[derive(Debug, Deserialize)]
// pub struct Memo {
//     #[serde(rename = "MemoData")]
//     pub memo_data: Option<String>,
// }

/// Ledger transaction. See <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "TransactionType")]
pub enum Transaction {
    AccountDelete(AccountDeleteTransaction),
    AccountSet(AccountSetTransaction),
    // TODO add model for remaining transactions
    CheckCancel(TransactionCommon),
    CheckCash(TransactionCommon),
    CheckCreate(TransactionCommon),
    DepositPreauth(TransactionCommon),
    EscrowCancel(TransactionCommon),
    EscrowCreate(TransactionCommon),
    EscrowFinish(TransactionCommon),
    NFTokenAcceptOffer(TransactionCommon),
    NFTokenBurn(TransactionCommon),
    NFTokenCancelOffer(TransactionCommon),
    NFTokenCreateOffer(TransactionCommon),
    NFTokenMint(TransactionCommon),
    OfferCancel(OfferCancelTransaction),
    OfferCreate(OfferCreateTransaction),
    Payment(PaymentTransaction),
    PaymentChannelClaim(TransactionCommon),
    PaymentChannelCreate(TransactionCommon),
    PaymentChannelFund(TransactionCommon),
    SetRegularKey(TransactionCommon),
    SignerListSet(TransactionCommon),
    TicketCreate(TransactionCommon),
    TrustSet(TrustSetTransaction),
}

impl Transaction {
    pub fn common(&self) -> &TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &t.common,
            Transaction::AccountSet(t) => &t.common,
            Transaction::OfferCancel(t) => &t.common,
            Transaction::OfferCreate(t) => &t.common,
            Transaction::Payment(t) => &t.common,
            Transaction::TrustSet(t) => &t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
        }
    }
}

impl Transaction {
    pub fn common_mut(&mut self) -> &mut TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &mut t.common,
            Transaction::AccountSet(t) => &mut t.common,
            Transaction::OfferCancel(t) => &mut t.common,
            Transaction::OfferCreate(t) => &mut t.common,
            Transaction::Payment(t) => &mut t.common,
            Transaction::TrustSet(t) => &mut t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
        }
    }
}
