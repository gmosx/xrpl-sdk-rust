use super::util::internal_number_from_string;
use xrpl_types::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, Memo, Transaction, UInt16, UInt32, UInt8};
use xrpl_types::field_id::FieldId;
use crate::error::BinaryCodecError;

// https://xrpl.org/serialization.html
// https://github.com/ripple/ripple-binary-codec/blob/master/src/enums/definitions.json
// https://xrpl.org/basic-data-types.html#hash-prefixes

pub const HASH_PREFIX_TRANSACTION: [u8; 4] = [0x53, 0x4E, 0x44, 0x00];
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

// TODO: Define type_code constants / enum

#[derive(Default)]
pub struct Serializer {
    pub buf: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize_account_id(&mut self, field_id: FieldId, account_id: &AccountId) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_blob(&mut self, field_id: FieldId, blob: &Blob) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash128(&mut self, field_id: FieldId, hash128: Hash128) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash160(&mut self, field_id: FieldId, hash160: Hash160) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash256(&mut self, field_id: FieldId, hash256: Hash256) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint8(&mut self, field_id: FieldId, uint8: UInt8) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint16(&mut self, field_id: FieldId, uint16: UInt16) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint32(&mut self, field_id: FieldId, uint32: UInt32) -> Result<(), BinaryCodecError> {
        todo!()
    }


    pub fn push(&mut self, value: u8) {
        self.buf.push(value);
    }

    pub fn push_u16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push((value & 0xff) as u8);
    }

    pub fn push_u32(&mut self, value: u32) {
        self.push((value >> 24) as u8);
        self.push(((value >> 16) & 0xff) as u8);
        self.push(((value >> 8) & 0xff) as u8);
        self.push((value & 0xff) as u8);
    }

    pub fn push_u64(&mut self, value: u64) {
        self.push((value >> 56) as u8);
        self.push(((value >> 48) & 0xff) as u8);
        self.push(((value >> 40) & 0xff) as u8);
        self.push(((value >> 32) & 0xff) as u8);
        self.push(((value >> 24) & 0xff) as u8);
        self.push(((value >> 16) & 0xff) as u8);
        self.push(((value >> 8) & 0xff) as u8);
        self.push((value & 0xff) as u8);
    }

    /// - <https://xrpl.org/serialization.html#field-ids>
    /// - <https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148>
    pub fn push_field_id(&mut self, type_code: u8, field_code: u8) {
        if type_code < 16 {
            if field_code < 16 {
                self.push(type_code << 4 | field_code);
            } else {
                self.push(type_code << 4);
                self.push(field_code);
            }
        } else if field_code < 16 {
            self.push(field_code);
            self.push(type_code);
        } else {
            self.push(0);
            self.push(type_code);
            self.push(field_code);
        }
    }

    /// <https://xrpl.org/serialization.html#length-prefixing>
    pub fn push_vl_prefix(&mut self, length: u32) {
        if length < 192 {
            self.push(length as u8);
        } else if length <= 12480 {
            let length = length - 192;
            self.push(193 + (length >> 8) as u8);
            self.push((length & 0xff) as u8);
        } else if length <= 918744 {
            let length = length - 12481;
            self.push(241 + (length >> 16) as u8);
            self.push(((length >> 8) & 0xff) as u8);
            self.push((length & 0xff) as u8);
        } else {
            todo!()
        }
    }

    // TODO: use more descriptive name.
    pub fn push_slice(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte);
        }
    }

    pub fn push_drops_amount(&mut self, value: u64) {
        self.push_u64(value | 0x4000000000000000);
    }

    /// - <https://xrpl.org/serialization.html#issued-currency-amount-format>
    /// - <https://github.com/ripple/ripple-binary-codec/blob/master/src/types/amount.ts>
    /// - <https://github.com/ripple/rippled/blob/develop/src/ripple/protocol/impl/STAmount.cpp>
    pub fn push_issued_amount(&mut self, value: &str, currency: &str, issuer: &str) {
        self.push_u64(internal_number_from_string(value));
        self.push_currency(currency);
        self.push_account_id(issuer);
    }

    pub fn push_amount(&mut self, amount: &Amount) {
        match amount {
            Amount::Drops(value) => self.push_drops_amount(value.parse::<u64>().unwrap()),
            Amount::Issued {
                value,
                currency,
                issuer,
            } => self.push_issued_amount(value, currency, issuer),
        }
    }

    /// <https://xrpl.org/serialization.html#currency-codes>
    pub fn push_currency(&mut self, currency: &str) {
        // Non-standard currency codes are 160 bits = 20 bytes in hex (40 chars).

        if currency.len() == 40 {
            // Non-standard currency code.
            let currency_bytes = hex::decode(currency).unwrap();
            // if currency_bytes[0] == 0x00 {
            self.push_slice(&currency_bytes);
            return;
            // }
        }

        // Standard currency code.

        // 8 bits
        self.push(0x00);

        // 88 bits
        for _ in 0..11 {
            self.push(0x00);
        }

        // 24 bits
        self.push_slice(currency.as_bytes());

        // 40 bits
        for _ in 0..5 {
            self.push(0x00);
        }
    }

    pub fn push_account_id(&mut self, id: &str) {
        // https://xrpl.org/serialization.html#accountid-fields
        // https://xrpl.org/accounts.html#address-encoding
        let decoded = bs58::decode(id)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_vec()
            .unwrap();

        // Skip the 0x00 ('r') version prefix, skip the 4-byte checksum postfix.
        let decoded = &decoded[1..21];

        self.push_slice(decoded);
    }

    // TODO: implement generic `push_array`
    // https://xrpl.org/serialization.html#array-fields

    pub fn push_blob(&mut self, field_code: u8, blob: &[u8]) {
        self.push_field_id(7, field_code);
        self.push_vl_prefix(blob.len() as u32);
        self.push_slice(blob);
    }

    pub fn push_memo(&mut self, memo: &Memo) {
        // https://xrpl.org/serialization.html#object-fields

        self.push_field_id(14, 10);

        self.push_blob(12, &memo.memo_type);
        self.push_blob(13, &memo.memo_data);

        if let Some(memo_format) = &memo.memo_format {
            self.push_blob(14, memo_format);
        }

        self.push(0xe1); // Object end
    }

    /// ## Serialization order
    ///
    /// <https://github.com/ripple/rippled/blob/master/src/ripple/protocol/impl/SField.cpp>
    ///
    /// 16 bit integers (common)
    ///
    /// transaction_type: u16,      UInt16, 1, 2 "TransactionType"
    ///
    /// 32 bit integers (common)
    ///
    /// flags: u32,                 UInt32, 2, 2 "Flags"
    /// sequence: u32,              UInt32, 2, 4 "Sequence"
    ///
    /// 32 bit integers (uncommon)
    ///
    /// quality_in: u32,            UInt32, 2, 20 "QualityIn"
    /// quality_out: u32,           UInt32, 2, 21 "QualityOut"
    /// offer_sequence: u32,        UInt32, 2, 25 "OfferSequence"
    /// last_ledger_sequence: u32,  UInt32, 2, 27
    ///
    /// currency amount (common)
    ///
    /// amount: String,             Amount, 6, 1 "Amount"
    /// limit_amount: String,       Amount, 6, 3 "LimitAmount"
    /// taker_pays: String,         Amount, 6, 4 "TakerPays"
    /// taker_gets: String,         Amount, 6, 5 "TakerGets"
    /// fee: String,                Amount, 6, 8 "Fee"
    ///
    /// variable length (common)
    ///
    /// signing_public_key,         SigningPubKey Blob, VL, 7, 3
    /// signature,                  TxnSignature Blob, VL, 7, 4
    ///
    /// account
    ///
    /// account: String,            AccountID, VL, 8, 1 "Account"
    /// destination: String,        AccountID, VL, 8, 3 "Destination"
    ///
    /// array of objects
    ///
    /// memos: Vec<Memo>,           STArray, 15, 9 "Memos"
    pub fn push_transaction(&mut self, tx: &Transaction, prefix: Option<&[u8]>) {
        if let Some(prefix) = prefix {
            self.push_slice(prefix);
        }

        // 16 bit integers (common)

        self.push_field_id(1, 2);
        self.push_u16(tx.transaction_type as u16);

        // 32 bit integers (common)

        if let Some(flags) = tx.flags {
            self.push_field_id(2, 2);
            self.push_u32(flags);
        }

        self.push_field_id(2, 4);
        self.push_u32(tx.sequence.unwrap());

        // 32 bit integers (uncommon)

        if let Some(quality_in) = tx.quality_in {
            self.push_field_id(2, 20);
            self.push_u32(quality_in);
        }

        if let Some(quality_out) = tx.quality_out {
            self.push_field_id(2, 21);
            self.push_u32(quality_out);
        }

        if let Some(offer_sequence) = tx.offer_sequence {
            self.push_field_id(2, 25);
            self.push_u32(offer_sequence);
        }

        self.push_field_id(2, 27);
        self.push_u32(tx.last_ledger_sequence.unwrap());

        // currency amount (common)

        if let Some(amount) = &tx.amount {
            self.push_field_id(6, 1);
            self.push_amount(amount);
        }

        if let Some(limit_amount) = &tx.limit_amount {
            self.push_field_id(6, 3);
            self.push_amount(limit_amount);
        }

        if let Some(taker_pays) = &tx.taker_pays {
            self.push_field_id(6, 4);
            self.push_amount(taker_pays);
        }

        if let Some(taker_gets) = &tx.taker_gets {
            self.push_field_id(6, 5);
            self.push_amount(taker_gets);
        }

        self.push_field_id(6, 8);
        self.push_drops_amount(tx.fee.unwrap());

        // variable length (common)

        if let Some(signing_public_key) = &tx.signing_public_key {
            self.push_field_id(7, 3);
            self.push_vl_prefix(signing_public_key.len() as u32);
            self.push_slice(signing_public_key);
        }

        if let Some(signature) = &tx.signature {
            self.push_field_id(7, 4);
            self.push_vl_prefix(signature.len() as u32);
            self.push_slice(signature);
        }

        // account

        self.push_field_id(8, 1);
        self.push_vl_prefix(160 / 8);
        self.push_account_id(&tx.account);

        if let Some(destination) = &tx.destination {
            self.push_field_id(8, 3);
            self.push_vl_prefix(160 / 8);
            self.push_account_id(destination);
        }

        // array of objects

        if let Some(memos) = &tx.memos {
            // https://xrpl.org/serialization.html#array-fields
            self.push_field_id(15, 9);
            for memo in memos {
                self.push_memo(memo);
            }
            // self.push_field_id(15, 1);
            self.push(0xf1); // Array end
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.buf.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_currency() {
        let mut s = Serializer::new();
        s.push_currency("XRP");
        let h = hex::encode(s.to_vec());
        assert_eq!(h, "0000000000000000000000005852500000000000");
    }

    #[test]
    fn test_push_issued_amount() {
        let mut s = Serializer::new();
        s.push_issued_amount("1200.34", "USD", "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");
        let h = hex::encode(s.to_vec());
        // println!("{}", h);
        assert_eq!(h, "d54443b3ef4f480000000000000000000000000055534400000000002adb0b3959d60a6e6991f729e1918b7163925230");
    }
}
