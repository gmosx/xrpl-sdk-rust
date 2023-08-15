use super::{AccountId, Amount};
use serde::{Deserialize, Serialize};

// https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TransactionType {
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

pub type DropsAmount = u64;

#[derive(Debug)]
pub struct Memo {
    pub memo_type: Vec<u8>,
    pub memo_data: Vec<u8>,
    pub memo_format: Option<Vec<u8>>,
}

// Universal Flags.

pub const TF_FULLY_CANONICAL_SIG: u32 = 0x8000_0000;

// TrustSet Transaction Flags.

pub const TF_SETF_AUTH: u32 = 0x0001_0000;
pub const TF_SET_NO_RIPPLE: u32 = 0x0002_0000;
pub const TF_CLEAR_NO_RIPPLE: u32 = 0x0004_0000;

// #todo consider separate structs per TransactionType?
// #todo add the serde metadata.

/// A ledger transaction
///
/// ## Links
///
/// - <https://xrpl.org/transaction-formats.html>
/// - <https://github.com/ripple/rippled/blob/master/src/ripple/protocol/impl/SField.cpp>
/// - <https://github.com/ripple/ripple-binary-codec/blob/master/src/enums/definitions.json>
#[derive(Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub account: AccountId,
    pub flags: Option<u32>,
    pub last_ledger_sequence: Option<u32>,
    pub fee: Option<DropsAmount>,
    pub sequence: Option<u32>,
    pub signing_public_key: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub memos: Option<Vec<Memo>>,

    // Payment
    pub amount: Option<Amount>,
    pub destination: Option<AccountId>,

    // OfferCancel/OfferCreate
    pub offer_sequence: Option<u32>,

    // OfferCreate
    pub taker_pays: Option<Amount>,
    pub taker_gets: Option<Amount>,
    pub expiration: Option<u32>,

    // TrustSet
    pub limit_amount: Option<Amount>,
    pub quality_in: Option<u32>,
    pub quality_out: Option<u32>,
}

// #todo PaymentTransaction or Transaction<Payment>
// #todo TransactionBuilder?

impl Transaction {
    // #todo Synthetic offer_replace constructor?

    /// <https://xrpl.org/offercreate.html>
    pub fn offer_create(account: &str, taker_pays: Amount, taker_gets: Amount) -> Self {
        // #todo Add support for expiration, offer_sequence
        Self {
            transaction_type: TransactionType::OfferCreate,
            account: account.to_string(),
            flags: None,
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: None,
            destination: None,
            offer_sequence: None,
            taker_pays: Some(taker_pays),
            taker_gets: Some(taker_gets),
            expiration: None,
            limit_amount: None,
            quality_in: None,
            quality_out: None,
        }
    }

    /// <https://xrpl.org/offercancel.html>
    pub fn offer_cancel(account: &str, offer_sequence: u32) -> Self {
        Self {
            transaction_type: TransactionType::OfferCancel,
            account: account.to_string(),
            flags: None,
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: None,
            destination: None,
            offer_sequence: Some(offer_sequence),
            taker_pays: None,
            taker_gets: None,
            expiration: None,
            limit_amount: None,
            quality_in: None,
            quality_out: None,
        }
    }

    /// <https://xrpl.org/payment.html>
    pub fn payment(account: &str, destination: &str, amount: Amount) -> Self {
        Self {
            transaction_type: TransactionType::Payment,
            account: account.to_string(),
            flags: None,
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: Some(amount),
            destination: Some(destination.to_string()),
            offer_sequence: None,
            taker_pays: None,
            taker_gets: None,
            expiration: None,
            limit_amount: None,
            quality_in: None,
            quality_out: None,
        }
    }

    // #todo make sure we add the NO RIPPLE flag!!!!
    /// <https://xrpl.org/trustset.html>
    pub fn trust_set(
        account: &str,
        limit_amount: Amount,
        quality_in: Option<u32>,
        quality_out: Option<u32>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::TrustSet,
            account: account.to_string(),
            flags: None,
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: None,
            destination: None,
            offer_sequence: None,
            taker_pays: None,
            taker_gets: None,
            expiration: None,
            limit_amount: Some(limit_amount),
            quality_in,
            quality_out,
        }
    }

    /// <https://xrpl.org/trustset.html>
    pub fn trust_set_no_ripple(
        account: &str,
        limit_amount: Amount,
        quality_in: Option<u32>,
        quality_out: Option<u32>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::TrustSet,
            account: account.to_string(),
            // #todo remove TF_FULLY_CANONICAL_SIG, it's deprecated!
            flags: Some(TF_SET_NO_RIPPLE | TF_FULLY_CANONICAL_SIG),
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: None,
            destination: None,
            offer_sequence: None,
            taker_pays: None,
            taker_gets: None,
            expiration: None,
            limit_amount: Some(limit_amount),
            quality_in,
            quality_out,
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            transaction_type: TransactionType::Payment,
            account: String::from(""),
            flags: None,
            last_ledger_sequence: None,
            fee: None,
            sequence: None,
            signing_public_key: None,
            signature: None,
            memos: None,
            amount: None,
            destination: None,
            offer_sequence: None,
            taker_pays: None,
            taker_gets: None,
            expiration: None,
            limit_amount: None,
            quality_in: None,
            quality_out: None,
        }
    }
}

impl Transaction {
    pub fn with_memo(self, memo_type: &str, memo_data: &str) -> Self {
        let mut memos = self.memos.unwrap_or_default();

        memos.push(Memo {
            memo_type: memo_type.to_string().into_bytes(),
            memo_data: memo_data.to_string().into_bytes(),
            memo_format: None,
        });

        Transaction {
            memos: Some(memos),
            ..self
        }
    }

    pub fn with_flags(self, flags: u32) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn add_flags(self, flags: u32) -> Self {
        Self {
            flags: if self.flags.is_some() {
                Some(self.flags.unwrap() | flags)
            } else {
                Some(flags)
            },
            ..self
        }
    }

    // #todo with_fee
}
