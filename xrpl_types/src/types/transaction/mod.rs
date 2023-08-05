use super::AccountId;
use crate::{Blob, DropsAmount, Hash256, UInt32};
use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

mod offer_create;
mod trust_set;

pub use offer_create::*;
pub use trust_set::*;

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
pub struct Transaction {
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
    // // Payment
    // pub amount: Option<Amount>,
    // pub destination: Option<AccountId>,
    //
    // // OfferCancel/OfferCreate
    // pub offer_sequence: Option<u32>,
    //
    // // OfferCreate
    // pub taker_pays: Option<Amount>,
    // pub taker_gets: Option<Amount>,
    // pub expiration: Option<u32>,
}

/// Flags that apply to all transaction types <https://xrpl.org/transaction-common-fields.html#global-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GlobalTransactionFlags {
    FullyCanonicalSig = 0x80000000,
}

// TODO: PaymentTransaction or Transaction<Payment>
// TODO: TransactionBuilder?

impl Transaction {
    // TODO: Synthetic offer_replace constructor?
    //
    // /// <https://xrpl.org/offercreate.html>
    // pub fn offer_create(account: AccountId, taker_pays: Amount, taker_gets: Amount) -> Self {
    //     // TODO: Add support for expiration, offer_sequence
    //     Self {
    //         transaction_type: TransactionType::OfferCreate,
    //         account,
    //         flags: None,
    //         last_ledger_sequence: None,
    //         fee: None,
    //         sequence: None,
    //         signing_public_key: None,
    //         signature: None,
    //         memos: None,
    //         amount: None,
    //         destination: None,
    //         offer_sequence: None,
    //         taker_pays: Some(taker_pays),
    //         taker_gets: Some(taker_gets),
    //         expiration: None,
    //         limit_amount: None,
    //         quality_in: None,
    //         quality_out: None,
    //     }
    // }
    //
    // /// <https://xrpl.org/offercancel.html>
    // pub fn offer_cancel(account: AccountId, offer_sequence: u32) -> Self {
    //     Self {
    //         transaction_type: TransactionType::OfferCancel,
    //         account: account,
    //         flags: None,
    //         last_ledger_sequence: None,
    //         fee: None,
    //         sequence: None,
    //         signing_public_key: None,
    //         signature: None,
    //         memos: None,
    //         amount: None,
    //         destination: None,
    //         offer_sequence: Some(offer_sequence),
    //         taker_pays: None,
    //         taker_gets: None,
    //         expiration: None,
    //         limit_amount: None,
    //         quality_in: None,
    //         quality_out: None,
    //     }
    // }
    //
    // /// <https://xrpl.org/payment.html>
    // pub fn payment(account: AccountId, destination: AccountId, amount: Amount) -> Self {
    //     Self {
    //         transaction_type: TransactionType::Payment,
    //         account: account,
    //         flags: None,
    //         last_ledger_sequence: None,
    //         fee: None,
    //         sequence: None,
    //         signing_public_key: None,
    //         signature: None,
    //         memos: None,
    //         amount: Some(amount),
    //         destination: Some(destination),
    //         offer_sequence: None,
    //         taker_pays: None,
    //         taker_gets: None,
    //         expiration: None,
    //         limit_amount: None,
    //         quality_in: None,
    //         quality_out: None,
    //     }
    // }
    //
    // // TODO: make sure we add the NO RIPPLE flag!!!!
    // /// <https://xrpl.org/trustset.html>
    // pub fn trust_set(
    //     account: AccountId,
    //     limit_amount: Amount,
    //     quality_in: Option<u32>,
    //     quality_out: Option<u32>,
    // ) -> Self {
    //     Self {
    //         transaction_type: TransactionType::TrustSet,
    //         account: account,
    //         flags: None,
    //         last_ledger_sequence: None,
    //         fee: None,
    //         sequence: None,
    //         signing_public_key: None,
    //         signature: None,
    //         memos: None,
    //         amount: None,
    //         destination: None,
    //         offer_sequence: None,
    //         taker_pays: None,
    //         taker_gets: None,
    //         expiration: None,
    //         limit_amount: Some(limit_amount),
    //         quality_in,
    //         quality_out,
    //     }
    // }
    //
    // /// <https://xrpl.org/trustset.html>
    // pub fn trust_set_no_ripple(
    //     account: AccountId,
    //     limit_amount: Amount,
    //     quality_in: Option<u32>,
    //     quality_out: Option<u32>,
    // ) -> Self {
    //     Self {
    //         transaction_type: TransactionType::TrustSet,
    //         account: account,
    //         // TODO: remove TF_FULLY_CANONICAL_SIG, it's deprecated!
    //         flags: Some(TF_SET_NO_RIPPLE | TF_FULLY_CANONICAL_SIG),
    //         last_ledger_sequence: None,
    //         fee: None,
    //         sequence: None,
    //         signing_public_key: None,
    //         signature: None,
    //         memos: None,
    //         amount: None,
    //         destination: None,
    //         offer_sequence: None,
    //         taker_pays: None,
    //         taker_gets: None,
    //         expiration: None,
    //         limit_amount: Some(limit_amount),
    //         quality_in,
    //         quality_out,
    //     }
    // }
}

impl Transaction {
    // pub fn with_memo(self, memo_type: &str, memo_data: &str) -> Self {
    //     let mut memos = self.memos.unwrap_or_default();
    //
    //     memos.push(Memo {
    //         memo_type: memo_type.to_string().into_bytes(),
    //         memo_data: memo_data.to_string().into_bytes(),
    //         memo_format: None,
    //     });
    //
    //     Transaction {
    //         memos: Some(memos),
    //         ..self
    //     }
    // }
    //
    // pub fn with_flags(self, flags: u32) -> Self {
    //     Self {
    //         flags: Some(flags),
    //         ..self
    //     }
    // }
    //
    // pub fn add_flags(self, flags: u32) -> Self {
    //     Self {
    //         flags: if self.flags.is_some() {
    //             Some(self.flags.unwrap() | flags)
    //         } else {
    //             Some(flags)
    //         },
    //         ..self
    //     }
    // }

    // TODO: with_fee
}
