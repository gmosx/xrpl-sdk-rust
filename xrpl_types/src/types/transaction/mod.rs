mod common;
mod variants;

use crate::serialize::Serialize;
pub use common::*;
pub use variants::*;

/// XRPL transaction
pub trait Transaction: Serialize {
    fn common(&self) -> &TransactionCommon;
    fn common_mut(&mut self) -> &mut TransactionCommon;
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum TransactionType {
   // Discriminant values can be found at https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json        
    Payment = 0,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    SetRegularKey = 5,
    NickNameSet = 6,
    OfferCreate = 7,
    OfferCancel = 8,
    Contract = 9,
    TicketCreate = 10,
    TicketCancel = 11,
    SignerListSet = 12,
    PaymentChannelCreate = 13,
    PaymentChannelFund = 14,
    PaymentChannelClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    SetHook = 22,
    NFTokenMint = 25,
    NFTokenBurn = 26,
    NFTokenCreateOffer = 27,
    NFTokenCancelOffer = 28,
    NFTokenAcceptOffer = 29,
    Clawback = 30,
    AMMCreate = 35,
    AMMDeposit = 36,
    AMMWithdraw = 37,
    AMMVote = 38,
    AMMBid = 39,
    AMMDelete = 40,
    XChainCreateClaimID = 41,
    XChainCommit = 42,
    XChainClaim = 43,
    XChainAccountCreateCommit = 44,
    XChainAddClaimAttestation = 45,
    XChainAddAccountCreateAttestation = 46,
    XChainModifyBridge = 47,
    XChainCreateBridge = 48,
    DIDSet = 49,
    DIDDelete = 50,
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}
