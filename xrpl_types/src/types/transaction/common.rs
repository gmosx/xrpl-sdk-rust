use crate::{AccountId, Blob, DropsAmount, Hash256, UInt32};
use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}

#[derive(Debug)]
pub struct Memo {
    pub memo_type: Vec<u8>,
    pub memo_data: Vec<u8>,
    pub memo_format: Option<Vec<u8>>,
}

/// A ledger transaction <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone)]
pub struct TransactionCommon {
    // Common fields https://xrpl.org/transaction-common-fields.html#transaction-common-fields
    pub account: AccountId,
    pub transaction_type: TransactionType,
    pub fee: Option<DropsAmount>,
    pub sequence: Option<UInt32>,
    pub account_txn_id: Option<Hash256>,
    pub flags: BitFlags<GlobalTransactionFlags>,
    pub last_ledger_sequence: Option<UInt32>,
    // pub memos: Option<Vec<Memo>>, // todo allan
    pub network_id: Option<UInt32>,
    pub source_tag: Option<UInt32>,
    pub signing_pub_key: Option<Blob>,
    pub ticket_sequence: Option<UInt32>,
    pub txn_signature: Option<Blob>,
}

/// Flags that apply to all transaction types <https://xrpl.org/transaction-common-fields.html#global-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GlobalTransactionFlags {
    FullyCanonicalSig = 0x80000000,
}
