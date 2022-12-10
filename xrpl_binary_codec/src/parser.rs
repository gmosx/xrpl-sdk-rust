use xrpl_types::{Amount, Memo, Transaction, TransactionType};

use crate::{
    error::BinaryCodecError,
    serializer::{HASH_PREFIX_TRANSACTION, HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE},
};

#[derive(Default)]
pub struct Parser {
    bytes: Vec<u8>,
    transaction: Transaction,
}

#[derive(Debug)]
struct FieldId {
    type_code: u8,
    field_code: u8,
}

#[derive(Debug)]
pub struct ParsedTransaction<'a> {
    pub transaction: &'a Transaction,
    pub prefix: Option<[u8; 4]>,
}

impl<'a> ParsedTransaction<'a> {
    /// Creates a new [`ParsedTransaction`].
    pub fn new(transaction: &'a Transaction, prefix: Option<[u8; 4]>) -> Self {
        Self {
            transaction,
            prefix,
        }
    }
}

impl Parser {
    pub fn new(tx_bytes: Vec<u8>) -> Self {
        Self {
            bytes: tx_bytes,
            transaction: Transaction::default(),
        }
    }

    fn parse_prefix(&self) -> Option<[u8; 4]> {
        let first_four_bytes: [u8; 4] = self.peek_n(4).try_into().unwrap();
        if first_four_bytes == HASH_PREFIX_TRANSACTION
            || first_four_bytes == HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE
        {
            Some(first_four_bytes)
        } else {
            None
        }
    }

    pub fn parse_transaction(&mut self) -> Result<ParsedTransaction, BinaryCodecError> {
        // Check the prefix. The prefix is stored as a 4 byte section at the front of the byte array
        let prefix: Option<[u8; 4]> = self.parse_prefix();
        if prefix.is_some() {
            // If a prefix is found we discard the bytes before proceeding
            let _ = self.read_n_bytes(4);
        }

        // Process the remaining bytes by first reading the field_id to check what is about to be consumed next
        while !self.bytes.is_empty() {
            let field_id = self.read_field_id().unwrap();
            println!(
                "The type code is: {} and the field code is: {}",
                field_id.type_code, field_id.field_code,
            );

            // Transaction type
            let read_result = self.read_field(field_id);
            match read_result {
                Ok(_) => continue,
                Err(err) => return Err(err),
            }
        }

        Ok(ParsedTransaction {
            transaction: &self.transaction,
            prefix,
        })
    }

    fn read_field_id(&mut self) -> Result<FieldId, BinaryCodecError> {
        let mut type_code: u8 = self.read_u_int_8()?;

        let mut field_code: u8 = type_code & 15;
        type_code >>= 4;
        if type_code == 0 {
            // the field id was too large to be stored as a single byte we need to pop the next one to create it
            type_code = self.read_u_int_8()?;
            if type_code == 0 || type_code < 16 {
                return Err(BinaryCodecError::ParseError(
                    "Cannot read FieldOrdinal, type_code out of range".to_string(),
                ));
            }
        }
        if field_code == 0 {
            field_code = self.read_u_int_8()?;
            if field_code == 0 || field_code < 16 {
                return Err(BinaryCodecError::ParseError(
                    "Cannot read FieldOrdinal, type_code out of range".to_string(),
                ));
            }
        }

        Ok(FieldId {
            type_code,
            field_code,
        })
    }

    fn read_field(&mut self, field_id: FieldId) -> Result<(), BinaryCodecError> {
        match field_id {
            FieldId {
                type_code: 1,
                field_code,
            } => {
                if field_code == 2 {
                    let maybe_tx_type = self.parse_transaction_type();
                    maybe_tx_type
                        .and_then(|tx_type| Ok(self.transaction.transaction_type = tx_type))
                } else {
                    Err(BinaryCodecError::ParseError(format!(
                        "Unrecognized field code: {} for type code: {}",
                        field_code, 1
                    )))
                }
            }
            FieldId {
                type_code: 2,
                field_code,
            } => match field_code {
                2 => self
                    .read_u_int_32()
                    .and_then(|flags| Ok(self.transaction.flags = Some(flags))),
                4 => self
                    .read_u_int_32()
                    .and_then(|sequence| Ok(self.transaction.sequence = Some(sequence))),
                20 => self
                    .read_u_int_32()
                    .and_then(|quality_in| Ok(self.transaction.quality_in = Some(quality_in))),
                21 => self
                    .read_u_int_32()
                    .and_then(|quality_out| Ok(self.transaction.quality_out = Some(quality_out))),
                25 => self.read_u_int_32().and_then(|offer_sequence| {
                    Ok(self.transaction.offer_sequence = Some(offer_sequence))
                }),
                27 => self.read_u_int_32().and_then(|last_ledger_sequence| {
                    Ok(self.transaction.last_ledger_sequence = Some(last_ledger_sequence))
                }),
                _ => Err(BinaryCodecError::ParseError(format!(
                    "Unrecognized field code: {} for type code: {}",
                    field_code, 2
                ))),
            },
            FieldId {
                type_code: 6,
                field_code,
            } => match field_code {
                1 => self
                    .parse_amount()
                    .and_then(|amount| Ok(self.transaction.amount = Some(amount))),
                3 => self.parse_amount().and_then(|limit_amount| {
                    Ok(self.transaction.limit_amount = Some(limit_amount))
                }),
                4 => self
                    .parse_amount()
                    .and_then(|taker_pays| Ok(self.transaction.taker_pays = Some(taker_pays))),
                5 => self
                    .parse_amount()
                    .and_then(|taker_gets| Ok(self.transaction.taker_gets = Some(taker_gets))),
                8 => {
                    let amount = self.parse_amount()?;
                    match amount {
                        Amount::Drops(drops) => Ok(drops.parse::<u64>()),
                        _ => Err(BinaryCodecError::ParseError(
                            "fee should never come back as anything other than drops".to_string(),
                        )),
                    }?
                    .map_err(|err| BinaryCodecError::from(err))
                    .and_then(|amount_u64| Ok(self.transaction.fee = Some(amount_u64)))
                }
                _ => Err(BinaryCodecError::ParseError(format!(
                    "Unrecognized field code: {} for type code: {}",
                    field_code, 6
                ))),
            },
            FieldId {
                type_code: 7,
                field_code,
            } => match field_code {
                3 => {
                    println!("Going to parse signing_public_key");
                    let length = self.read_variable_length_length()?;
                    self.read_n_bytes(length).and_then(|signing_key_bytes| {
                        Ok(self.transaction.signing_public_key = Some(signing_key_bytes))
                    })
                }
                4 => {
                    println!("Going to parse signature");
                    let length = self.read_variable_length_length()?;
                    self.read_n_bytes(length).and_then(|signing_key_bytes| {
                        Ok(self.transaction.signing_public_key = Some(signing_key_bytes))
                    })
                }
                _ => Err(BinaryCodecError::ParseError(format!(
                    "Unrecognized field code: {} for type code: {}",
                    field_code, 7
                ))),
            },
            FieldId {
                type_code: 8,
                field_code,
            } => match field_code {
                1 => self
                    .parse_account_id()
                    .and_then(|account_id| Ok(self.transaction.account = account_id)),
                3 => self
                    .parse_account_id()
                    .and_then(|destination| Ok(self.transaction.destination = Some(destination))),
                _ => Err(BinaryCodecError::ParseError(format!(
                    "Unrecognized field code: {} for type code: {}",
                    field_code, 7
                ))),
            },
            FieldId {
                type_code: 15,
                field_code,
            } => {
                if field_code == 9 {
                    self.transaction.memos = None;
                    Ok(())
                } else {
                    Err(BinaryCodecError::ParseError(format!(
                        "Unrecognized field code: {} for type code: {}",
                        field_code, 15
                    )))
                }
            }
            _ => Err(BinaryCodecError::ParseError(
                "Encountered unrecognized field code".to_string(),
            )),
        }
    }

    fn parse_memos(&mut self) -> Result<Vec<Memo>, BinaryCodecError> {
        Ok(Vec::<Memo>::new())
    }

    fn parse_transaction_type(&mut self) -> Result<TransactionType, BinaryCodecError> {
        let transaction_type_int = self.read_u_int_16()?;

        match transaction_type_int as u32 {
            0 => Ok(TransactionType::Payment),
            1 => Ok(TransactionType::EscrowCreate),
            2 => Ok(TransactionType::EscrowFinish),
            3 => Ok(TransactionType::AccountSet),
            4 => Ok(TransactionType::EscrowCancel),
            5 => Ok(TransactionType::SetRegularKey),
            6 => Ok(TransactionType::NickNameSet),
            7 => Ok(TransactionType::OfferCreate),
            8 => Ok(TransactionType::OfferCancel),
            9 => Ok(TransactionType::Contract),
            10 => Ok(TransactionType::TicketCreate),
            11 => Ok(TransactionType::TicketCancel),
            12 => Ok(TransactionType::SignerListSet),
            13 => Ok(TransactionType::PaymentChannelCreate),
            14 => Ok(TransactionType::PaymentChannelFund),
            15 => Ok(TransactionType::PaymentChannelClaim),
            16 => Ok(TransactionType::CheckCreate),
            17 => Ok(TransactionType::CheckCash),
            18 => Ok(TransactionType::CheckCancel),
            19 => Ok(TransactionType::DepositPreauth),
            20 => Ok(TransactionType::TrustSet),
            21 => Ok(TransactionType::AccountDelete),
            100 => Ok(TransactionType::EnableAmendment),
            101 => Ok(TransactionType::SetFee),
            102 => Ok(TransactionType::UNLModify),
            _ => Err(BinaryCodecError::ParseError(
                "unable to parse TransactionType".to_string(),
            )),
        }
    }

    fn parse_amount(&mut self) -> Result<Amount, BinaryCodecError> {
        // Check the first byte to see if it is an xrp or an issued currency
        // if xrp omit the next byte which is the sign bit and read the next 62 bits
        // 1.read 64 bit number
        // Check if the first first bit

        let mask = 0x80;

        let last_bit_is_not_set = !(self.peek() & mask == mask);

        if last_bit_is_not_set {
            // If the last bit is not set this is an xrp amount
            // And the next value to read is a 64 bit int
            let raw_amount = self.read_u_int_64()?;
            let mask = 0x4000000000000000;
            let drops = raw_amount ^ mask;
            Ok(Amount::drops(drops))
        } else {
            Err(BinaryCodecError::ParseError(
                "Issued Amount currently not supported".to_string(),
            ))
        }

        // let n_bytes_to_read = if is_xrp { 48 } else { 8 };

        // if token
    }

    fn parse_account_id(&mut self) -> Result<String, BinaryCodecError> {
        let vl_length = self.read_variable_length_length()?;
        let account_bytes: Vec<u8> = self.read_n_bytes(vl_length)?;

        let encoded = bs58::encode(account_bytes)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_string();
        // When serializing we skipped the ('r') version prefix and the 4-byte checksum postfix.
        let account_id = format!("{}{}", String::from("r"), encoded);

        let re_decoded = bs58::decode(account_id)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_vec()?;
        let re_encoded = bs58::encode(re_decoded)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .with_check()
            .into_string();
        Ok(re_encoded)
    }

    fn read_u_int_8(&mut self) -> Result<u8, BinaryCodecError> {
        if self.bytes.is_empty() {
            Err(BinaryCodecError::ParseError(
                "Unable to read byte from empty buffer".to_string(),
            ))
        } else {
            Ok(self.bytes.remove(0))
        }
    }

    fn read_u_int_16(&mut self) -> Result<u16, BinaryCodecError> {
        // Pop the bytes in reverse order so we get the bytes in correct order
        if self.bytes.len() >= 2 {
            let p1 = self.bytes.remove(0);
            let p2 = self.bytes.remove(0);
            Ok(((p1 as u16) << 8) | p2 as u16)
        } else {
            Err(BinaryCodecError::ParseError(
                "Unable to ready 16 bit integer with less then two bytes left of the buffer"
                    .to_string(),
            ))
        }
    }

    fn read_u_int_32(&mut self) -> Result<u32, BinaryCodecError> {
        // Pop the bytes in reverse order so we get the bytes in correct order
        if self.bytes.len() >= 4 {
            let p1 = self.bytes.remove(0);
            let p2 = self.bytes.remove(0);
            let p3 = self.bytes.remove(0);
            let p4 = self.bytes.remove(0);
            Ok(((p1 as u32) << 24) | ((p2 as u32) << 16) | ((p3 as u32) << 8) | p4 as u32)
        } else {
            Err(BinaryCodecError::ParseError(
                "Unable to ready 32 bit integer with less then 4 bytes left of the buffer"
                    .to_string(),
            ))
        }
    }

    fn read_u_int_64(&mut self) -> Result<u64, BinaryCodecError> {
        // Pop the bytes in reverse order so we get the bytes in correct order
        if self.bytes.len() >= 8 {
            let p1 = self.bytes.remove(0);
            let p2 = self.bytes.remove(0);
            let p3 = self.bytes.remove(0);
            let p4 = self.bytes.remove(0);
            let p5 = self.bytes.remove(0);
            let p6 = self.bytes.remove(0);
            let p7 = self.bytes.remove(0);
            let p8 = self.bytes.remove(0);

            Ok(((p1 as u64) << 56)
                | ((p2 as u64) << 48)
                | ((p3 as u64) << 40)
                | ((p4 as u64) << 32)
                | ((p5 as u64) << 24)
                | ((p6 as u64) << 16)
                | ((p7 as u64) << 8)
                | p8 as u64)
        } else {
            Err(BinaryCodecError::ParseError(
                "Unable to ready 64 bit integer with less then 8 bytes left of the buffer"
                    .to_string(),
            ))
        }
    }

    fn peek(&self) -> u8 {
        self.bytes.get(0).unwrap().to_owned()
    }

    fn peek_n(&self, n: i32) -> Vec<u8> {
        let n_usize: usize = n.try_into().unwrap();
        println!("Bytes before: {:?}", self.bytes);
        let v = self.bytes.clone()[..n_usize].to_vec();
        println!("Bytes after: {:?}", self.bytes);
        v
    }

    fn read_n_bytes(&mut self, n: i32) -> Result<Vec<u8>, BinaryCodecError> {
        let n_usize: usize = n.try_into().unwrap();
        if self.bytes.len() < n_usize {
            Err(BinaryCodecError::ParseError(format!(
                "Unable to ready {} bytes with less then {} bytes left of the buffer",
                n, n
            )))
        } else {
            let read_bytes = self.bytes[..n_usize].to_vec();
            self.bytes = self.bytes[n_usize..].to_vec();

            Ok(read_bytes)
        }
    }

    // https://xrpl.org/serialization.html#length-prefixing
    fn read_variable_length_length(&mut self) -> Result<i32, BinaryCodecError> {
        let first_byte = self.read_u_int_8()?;
        if first_byte <= 192 {
            return Ok(first_byte as i32);
        } else {
            let second_byte: u8;
            if first_byte <= 240 {
                second_byte = self.read_u_int_8()?;
                return Ok(193 + (first_byte as i32 - 193) * 256 + second_byte as i32);
            } else if first_byte <= 254 {
                second_byte = self.read_u_int_8()?;
                let third_byte = self.read_u_int_8()?;
                return Ok(12481
                    + (first_byte as i32 - 240 - 1) * 65536
                    + second_byte as i32 * 256
                    + third_byte as i32);
            } else {
                Err(BinaryCodecError::ParseError(
                    "Invalid variable length indicator".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use xrpl_types::{Amount, TransactionType};

    use crate::serializer::Serializer;

    use super::*;

    #[test]
    fn test_payment_parsing_without_prefix() {
        let account = "rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe";

        let destination = "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe";
        let mut tx: Transaction = Transaction::payment(account, destination, Amount::xrp("10"));
        tx.sequence = Some(12 as u32);
        tx.last_ledger_sequence = Some(12 as u32);
        tx.fee = Some(100 as u64);
        let mut s = Serializer::new();
        s.push_transaction(&tx, None);

        let mut p = Parser::new(s.to_vec());
        let p_tx = p.parse_transaction().unwrap().transaction;

        assert_eq!(p_tx.transaction_type, TransactionType::Payment);
        assert_eq!(p_tx.flags, None);
        assert_eq!(p_tx.sequence, Some(12 as u32));
        assert_eq!(p_tx.last_ledger_sequence, Some(12 as u32));

        // Currency amount common
        assert_eq!(p_tx.amount, Some(Amount::xrp("10")));
        assert_eq!(p_tx.fee, Some(100 as u64));

        assert_eq!(p_tx.account, account);
        assert_eq!(p_tx.destination, Some(String::from(destination)));
    }

    #[test]
    fn test_payment_parsing_with_prefix() {
        let account = "rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe";

        let destination = "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe";
        let mut tx: Transaction = Transaction::payment(account, destination, Amount::xrp("10"));
        tx.sequence = Some(12 as u32);
        tx.last_ledger_sequence = Some(12 as u32);
        tx.fee = Some(100 as u64);
        let mut s = Serializer::new();
        s.push_transaction(&tx, Some(&HASH_PREFIX_TRANSACTION));

        let mut p = Parser::new(s.to_vec());
        let parsed: ParsedTransaction = p.parse_transaction().unwrap();
        let p_tx = parsed.transaction;
        let p_prefix = parsed.prefix;

        assert_eq!(p_prefix, Some(HASH_PREFIX_TRANSACTION));

        assert_eq!(p_tx.transaction_type, TransactionType::Payment);
        assert_eq!(p_tx.flags, None);
        assert_eq!(p_tx.sequence, Some(12 as u32));
        assert_eq!(p_tx.last_ledger_sequence, Some(12 as u32));

        // Currency amount common
        assert_eq!(p_tx.amount, Some(Amount::xrp("10")));
        assert_eq!(p_tx.fee, Some(100 as u64));

        assert_eq!(p_tx.account, account);
        assert_eq!(p_tx.destination, Some(String::from(destination)));
    }
}
