use crate::serialize::{FieldCode, Serialize, SerializeArray, Serializer};
use crate::{AccountId, Amount, Blob, DropsAmount, Hash256, UInt32};

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
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}

#[derive(Debug, Clone)]
pub struct Memo {
    pub memo_type: Blob,
    pub memo_data: Blob,
    pub memo_format: Option<Blob>,
}

/// A ledger transaction <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone)]
pub struct TransactionCommon {
    pub account: AccountId,
    pub fee: Option<DropsAmount>,
    pub sequence: Option<UInt32>,
    pub account_txn_id: Option<Hash256>,
    pub last_ledger_sequence: Option<UInt32>,
    pub memos: Vec<Memo>,
    pub network_id: Option<UInt32>,
    pub source_tag: Option<UInt32>,
    pub signing_pub_key: Option<Blob>,
    pub ticket_sequence: Option<UInt32>,
    pub txn_signature: Option<Blob>,
}

impl TransactionCommon {
    pub fn new(account: AccountId) -> Self {
        Self {
            account,
            fee: None,
            sequence: None,
            account_txn_id: None,
            last_ledger_sequence: None,
            memos: Vec::default(),
            network_id: None,
            source_tag: None,
            signing_pub_key: None,
            ticket_sequence: None,
            txn_signature: None,
        }
    }
}

impl Serialize for TransactionCommon {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        if let Some(network_id) = self.network_id {
            s.serialize_uint32(FieldCode(1), network_id)?;
        }
        if let Some(source_tag) = self.source_tag {
            s.serialize_uint32(FieldCode(3), source_tag)?;
        }
        if let Some(sequence) = self.sequence {
            s.serialize_uint32(FieldCode(4), sequence)?;
        }
        if let Some(last_ledger_sequence) = self.last_ledger_sequence {
            s.serialize_uint32(FieldCode(27), last_ledger_sequence)?;
        }
        if !self.memos.is_empty() {
            let mut array = s.serialize_array(FieldCode(9))?;
            for memo in &self.memos {
                array.serialize_object(FieldCode(10), memo)?;
            }
            array.end()?;
        }
        if let Some(ticket_sequence) = self.ticket_sequence {
            s.serialize_uint32(FieldCode(41), ticket_sequence)?;
        }
        if let Some(account_txn_id) = self.account_txn_id {
            s.serialize_hash256(FieldCode(9), account_txn_id)?;
        }
        if let Some(fee) = self.fee {
            s.serialize_amount(FieldCode(8), Amount::Drops(fee))?;
        }
        if let Some(signing_pub_key) = self.signing_pub_key.as_ref() {
            s.serialize_blob(FieldCode(3), signing_pub_key)?;
        }
        if let Some(txn_signature) = self.txn_signature.as_ref() {
            s.serialize_blob(FieldCode(4), txn_signature)?;
        }
        s.serialize_account_id(FieldCode(1), self.account)?;
        Ok(())
    }
}

impl Serialize for Memo {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_blob(FieldCode(12), &self.memo_type)?;
        s.serialize_blob(FieldCode(13), &self.memo_data)?;
        if let Some(memo_format) = self.memo_format.as_ref() {
            s.serialize_blob(FieldCode(14), memo_format)?;
        }
        Ok(())
    }
}
