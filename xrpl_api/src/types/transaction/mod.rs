mod account_delete;
mod account_set;
mod common;
mod offer_cancel;
mod offer_create;
mod payment;
mod trust_set;

pub use account_delete::*;
pub use account_set::*;
pub use common::*;
pub use offer_cancel::*;
pub use offer_create::*;
pub use payment::*;
pub use trust_set::*;

use serde::Deserialize;

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
            Transaction::CheckCancel(t) => &t,
            Transaction::CheckCash(t) => &t,
            Transaction::CheckCreate(t) => &t,
            Transaction::DepositPreauth(t) => &t,
            Transaction::EscrowCancel(t) => &t,
            Transaction::EscrowCreate(t) => &t,
            Transaction::EscrowFinish(t) => &t,
            Transaction::NFTokenAcceptOffer(t) => &t,
            Transaction::NFTokenBurn(t) => &t,
            Transaction::NFTokenCancelOffer(t) => &t,
            Transaction::NFTokenCreateOffer(t) => &t,
            Transaction::NFTokenMint(t) => &t,
            Transaction::PaymentChannelClaim(t) => &t,
            Transaction::PaymentChannelCreate(t) => &t,
            Transaction::PaymentChannelFund(t) => &t,
            Transaction::SetRegularKey(t) => &t,
            Transaction::SignerListSet(t) => &t,
            Transaction::TicketCreate(t) => &t,
        }
    }
}
