// use xrpl_types::{AccountId, Amount, Memo, Transaction, TransactionType};
//
// use crate::{
//     error::BinaryCodecError,
//     serializer::{HASH_PREFIX_TRANSACTION, HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE},
// };
//
// pub struct Parser {
//     bytes: Vec<u8>,
//     transaction: Transaction,
// }
//
// #[derive(Debug)]
// struct FieldId {
//     type_code: u8,
//     field_code: u8,
// }
//
// #[derive(Debug, Clone)]
// pub struct ParsedTransaction<'a> {
//     pub transaction: &'a Transaction,
//     pub prefix: Option<[u8; 4]>,
// }
//
// impl<'a> ParsedTransaction<'a> {
//     /// Creates a new [`ParsedTransaction`].
//     pub fn new(transaction: &'a Transaction, prefix: Option<[u8; 4]>) -> Self {
//         Self {
//             transaction,
//             prefix,
//         }
//     }
// }
//
// impl Parser {
//     pub fn new(tx_bytes: Vec<u8>) -> Self {
//         let transaction = Transaction {
//             transaction_type: TransactionType::Payment,
//             account: AccountId([0u8; 20]),
//             flags: None,
//             last_ledger_sequence: None,
//             fee: None,
//             sequence: None,
//             signing_public_key: None,
//             signature: None,
//             memos: None,
//             amount: None,
//             destination: None,
//             offer_sequence: None,
//             taker_pays: None,
//             taker_gets: None,
//             expiration: None,
//             limit_amount: None,
//             quality_in: None,
//             quality_out: None,
//         };
//         Self {
//             bytes: tx_bytes,
//             transaction,
//         }
//     }
//
//     fn parse_prefix(&self) -> Option<[u8; 4]> {
//         let first_four_bytes: [u8; 4] = self.peek_n(4).try_into().unwrap();
//         if first_four_bytes == HASH_PREFIX_TRANSACTION
//             || first_four_bytes == HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE
//         {
//             Some(first_four_bytes)
//         } else {
//             None
//         }
//     }
//
//     pub fn parse_transaction(&mut self) -> Result<ParsedTransaction, BinaryCodecError> {
//         // Check the prefix. The prefix is stored as a 4 byte section at the front of the byte array
//         let prefix: Option<[u8; 4]> = self.parse_prefix();
//         if prefix.is_some() {
//             // If a prefix is found we discard the bytes before proceeding
//             let _ = self.read_n_bytes(4);
//         }
//
//         // Process the remaining bytes by first reading the field_id to check what is about to be consumed next
//         while !self.bytes.is_empty() {
//             let field_id = self.read_field_id().unwrap();
//
//             // Transaction type
//             let read_result = self.read_field(field_id);
//             match read_result {
//                 Ok(_) => continue,
//                 Err(err) => return Err(err),
//             }
//         }
//
//         Ok(ParsedTransaction {
//             transaction: &self.transaction,
//             prefix,
//         })
//     }
//
//     fn read_field_id(&mut self) -> Result<FieldId, BinaryCodecError> {
//         let mut type_code: u8 = self.read_u_int_8()?;
//
//         let mut field_code: u8 = type_code & 15;
//         type_code >>= 4;
//         if type_code == 0 {
//             // the field id was too large to be stored as a single byte we need to pop the next one to create it
//             type_code = self.read_u_int_8()?;
//             if type_code == 0 || type_code < 16 {
//                 return Err(BinaryCodecError::ParseError(
//                     "Cannot read FieldOrdinal, type_code out of range".to_string(),
//                 ));
//             }
//         }
//         if field_code == 0 {
//             field_code = self.read_u_int_8()?;
//             if field_code == 0 || field_code < 16 {
//                 return Err(BinaryCodecError::ParseError(
//                     "Cannot read FieldOrdinal, type_code out of range".to_string(),
//                 ));
//             }
//         }
//
//         Ok(FieldId {
//             type_code,
//             field_code,
//         })
//     }
//
//     fn read_field(&mut self, field_id: FieldId) -> Result<(), BinaryCodecError> {
//         match field_id {
//             FieldId {
//                 type_code: 1,
//                 field_code,
//             } => {
//                 if field_code == 2 {
//                     let maybe_tx_type = self.parse_transaction_type();
//                     maybe_tx_type.map(|tx_type| self.transaction.transaction_type = tx_type)
//                 } else {
//                     Err(BinaryCodecError::ParseError(format!(
//                         "Unrecognized field code: {} for type code: {}",
//                         field_code, 1
//                     )))
//                 }
//             }
//             FieldId {
//                 type_code: 2,
//                 field_code,
//             } => match field_code {
//                 2 => self
//                     .read_u_int_32()
//                     .map(|flags| self.transaction.flags = Some(flags)),
//                 4 => self
//                     .read_u_int_32()
//                     .map(|sequence| self.transaction.sequence = Some(sequence)),
//                 20 => self
//                     .read_u_int_32()
//                     .map(|quality_in| self.transaction.quality_in = Some(quality_in)),
//                 21 => self
//                     .read_u_int_32()
//                     .map(|quality_out| self.transaction.quality_out = Some(quality_out)),
//                 25 => self
//                     .read_u_int_32()
//                     .map(|offer_sequence| self.transaction.offer_sequence = Some(offer_sequence)),
//                 27 => self.read_u_int_32().map(|last_ledger_sequence| {
//                     self.transaction.last_ledger_sequence = Some(last_ledger_sequence)
//                 }),
//                 _ => Err(BinaryCodecError::ParseError(format!(
//                     "Unrecognized field code: {} for type code: {}",
//                     field_code, 2
//                 ))),
//             },
//             FieldId {
//                 type_code: 6,
//                 field_code,
//             } => match field_code {
//                 1 => self
//                     .parse_amount()
//                     .map(|amount| self.transaction.amount = Some(amount)),
//                 3 => self
//                     .parse_amount()
//                     .map(|limit_amount| self.transaction.limit_amount = Some(limit_amount)),
//                 4 => self
//                     .parse_amount()
//                     .map(|taker_pays| self.transaction.taker_pays = Some(taker_pays)),
//                 5 => self
//                     .parse_amount()
//                     .map(|taker_gets| self.transaction.taker_gets = Some(taker_gets)),
//                 8 => {
//                     let amount = self.parse_amount()?;
//                     match amount {
//                         Amount::Drops(drops) => Ok(drops.parse::<u64>()),
//                         _ => Err(BinaryCodecError::ParseError(
//                             "fee should never come back as anything other than drops".to_string(),
//                         )),
//                     }?
//                     .map_err(BinaryCodecError::from)
//                     .map(|amount_u64| self.transaction.fee = Some(amount_u64))
//                 }
//                 _ => Err(BinaryCodecError::ParseError(format!(
//                     "Unrecognized field code: {} for type code: {}",
//                     field_code, 6
//                 ))),
//             },
//             FieldId {
//                 type_code: 7,
//                 field_code,
//             } => match field_code {
//                 3 => {
//                     let length = self.read_variable_length_length()?;
//                     self.read_n_bytes(length).map(|signing_key_bytes| {
//                         self.transaction.signing_public_key = Some(signing_key_bytes)
//                     })
//                 }
//                 4 => {
//                     let length = self.read_variable_length_length()?;
//                     self.read_n_bytes(length)
//                         .map(|signature| self.transaction.signature = Some(signature))
//                 }
//                 _ => Err(BinaryCodecError::ParseError(format!(
//                     "Unrecognized field code: {} for type code: {}",
//                     field_code, 7
//                 ))),
//             },
//             FieldId {
//                 type_code: 8,
//                 field_code,
//             } => match field_code {
//                 1 => self
//                     .parse_account_id()
//                     .map(|account_id| self.transaction.account = account_id),
//                 3 => self
//                     .parse_account_id()
//                     .map(|destination| self.transaction.destination = Some(destination)),
//                 _ => Err(BinaryCodecError::ParseError(format!(
//                     "Unrecognized field code: {} for type code: {}",
//                     field_code, 7
//                 ))),
//             },
//             FieldId {
//                 type_code: 15,
//                 field_code,
//             } => {
//                 if field_code == 9 {
//                     let memos = self.parse_memos().ok();
//                     self.transaction.memos = memos;
//                     Ok(())
//                 } else {
//                     Err(BinaryCodecError::ParseError(format!(
//                         "Unrecognized field code: {} for type code: {}",
//                         field_code, 15
//                     )))
//                 }
//             }
//             _ => Err(BinaryCodecError::ParseError(
//                 "Encountered unrecognized field code".to_string(),
//             )),
//         }
//     }
//
//     // #TODO investigate if this function is correct!
//     fn parse_memos(&mut self) -> Result<Vec<Memo>, BinaryCodecError> {
//         // While the next byte is not the array end indicator 0xf1 keep reading a memo object
//         let mut next_byte = self.peek();
//         let mut memos = Vec::<Memo>::new();
//         while next_byte != 0xf1 {
//             // parse_memo
//             let memo = self.parse_memo()?;
//             memos.push(memo);
//             next_byte = self.peek();
//         }
//
//         Ok(memos)
//     }
//
//     fn parse_memo(&mut self) -> Result<Memo, BinaryCodecError> {
//         // read field code and verify it is a memo object coming up (14, 10)
//         let field_id = self.read_field_id()?;
//         let FieldId {
//             type_code,
//             field_code,
//         } = field_id;
//
//         if type_code != 14 && field_code != 10 {
//             Err(BinaryCodecError::ParseError(format!(
//                 "Expected field id for object 14 , 10 but found type code: {} and field code: {}",
//                 type_code, field_code
//             )))
//         } else {
//             // Attempt to read the memo_type field (7,12)
//             let field_id = self.read_field_id()?;
//             let FieldId {
//                 type_code,
//                 field_code,
//             } = field_id;
//             if type_code != 7 && field_code != 12 {
//                 return Err(BinaryCodecError::ParseError(format!("Expected field id for memo type 7 , 12 but found type code: {} and field code: {}",type_code, field_code  )));
//             }
//             let memo_tyoe_blob_length = self.read_variable_length_length()?;
//             let memo_type = self.read_n_bytes(memo_tyoe_blob_length)?;
//
//             // Attempt to read the memo_type field (7,13)
//             let field_id = self.read_field_id()?;
//             let FieldId {
//                 type_code,
//                 field_code,
//             } = field_id;
//             if type_code != 7 && field_code != 12 {
//                 return Err(BinaryCodecError::ParseError(format!("Expected field id for memo data 7 , 13 but found type code: {} and field code: {}",type_code, field_code  )));
//             }
//
//             let memo_data_blob_length = self.read_variable_length_length()?;
//             let memo_data = self.read_n_bytes(memo_data_blob_length)?;
//
//             // Check if the next byte is the end byte, if not then we have a format field to parse
//             let memo_format: Option<Vec<u8>>;
//             let next_byte = self.peek();
//             if next_byte == 0xe1 {
//                 //consume the object end byte and return a None format
//                 self.read_u_int_8()?;
//                 memo_format = None;
//             } else {
//                 // parse the format field of the object
//                 let field_id = self.read_field_id()?;
//                 let FieldId {
//                     type_code,
//                     field_code,
//                 } = field_id;
//                 if type_code != 7 && field_code != 12 {
//                     return Err(BinaryCodecError::ParseError(format!("Expected field id for memo format 7 , 14 but found type code: {} and field code: {}",type_code, field_code  )));
//                 } else {
//                     let memo_format_blob_length = self.read_variable_length_length()?;
//                     memo_format = self.read_n_bytes(memo_format_blob_length).ok();
//                 }
//             }
//
//             Ok(Memo {
//                 memo_type,
//                 memo_data,
//                 memo_format,
//             })
//         }
//     }
//
//     fn parse_transaction_type(&mut self) -> Result<TransactionType, BinaryCodecError> {
//         let transaction_type_int = self.read_u_int_16()?;
//
//         match transaction_type_int as u32 {
//             0 => Ok(TransactionType::Payment),
//             1 => Ok(TransactionType::EscrowCreate),
//             2 => Ok(TransactionType::EscrowFinish),
//             3 => Ok(TransactionType::AccountSet),
//             4 => Ok(TransactionType::EscrowCancel),
//             5 => Ok(TransactionType::SetRegularKey),
//             6 => Ok(TransactionType::NickNameSet),
//             7 => Ok(TransactionType::OfferCreate),
//             8 => Ok(TransactionType::OfferCancel),
//             9 => Ok(TransactionType::Contract),
//             10 => Ok(TransactionType::TicketCreate),
//             11 => Ok(TransactionType::TicketCancel),
//             12 => Ok(TransactionType::SignerListSet),
//             13 => Ok(TransactionType::PaymentChannelCreate),
//             14 => Ok(TransactionType::PaymentChannelFund),
//             15 => Ok(TransactionType::PaymentChannelClaim),
//             16 => Ok(TransactionType::CheckCreate),
//             17 => Ok(TransactionType::CheckCash),
//             18 => Ok(TransactionType::CheckCancel),
//             19 => Ok(TransactionType::DepositPreauth),
//             20 => Ok(TransactionType::TrustSet),
//             21 => Ok(TransactionType::AccountDelete),
//             22 => Ok(TransactionType::SetHook),
//             25 => Ok(TransactionType::NFTokenMint),
//             26 => Ok(TransactionType::NFTokenBurn),
//             27 => Ok(TransactionType::NFTokenCreateOffer),
//             28 => Ok(TransactionType::NFTokenCancelOffer),
//             29 => Ok(TransactionType::NFTokenAcceptOffer),
//             100 => Ok(TransactionType::EnableAmendment),
//             101 => Ok(TransactionType::SetFee),
//             102 => Ok(TransactionType::UNLModify),
//             _ => Err(BinaryCodecError::ParseError(
//                 "unable to parse TransactionType".to_string(),
//             )),
//         }
//     }
//
//     fn parse_amount(&mut self) -> Result<Amount, BinaryCodecError> {
//         // Check the first byte to see if it is an xrp or an issued currency
//         // if xrp omit the next byte which is the sign bit and read the next 62 bits
//         // 1.read 64 bit number
//         // Check if the first first bit
//
//         let mask = 0x80;
//
//         let last_bit_is_not_set = self.peek() & mask != mask;
//
//         if last_bit_is_not_set {
//             // If the last bit is not set this is an xrp amount
//             // And the next value to read is a 64 bit int
//             let raw_amount = self.read_u_int_64()?;
//             let mask = 0x4000000000000000;
//             let drops = raw_amount ^ mask;
//             Ok(Amount::drops(drops))
//         } else {
//             Err(BinaryCodecError::ParseError(
//                 "Issued Amount currently not supported".to_string(),
//             ))
//         }
//     }
//
//     fn parse_account_id(&mut self) -> Result<AccountId, BinaryCodecError> {
//         let vl_length = self.read_variable_length_length()?;
//         let account_bytes: Vec<u8> = self.read_n_bytes(vl_length)?;
//         let account_bytes: [u8; 20] = account_bytes.try_into().map_err(|_|
//         BinaryCodecError::ParseError("AccountID not 20 bytes".to_string()))?;
//         Ok(AccountId(account_bytes))
//     }
//
//     fn read_u_int_8(&mut self) -> Result<u8, BinaryCodecError> {
//         if self.bytes.is_empty() {
//             Err(BinaryCodecError::ParseError(
//                 "Unable to read byte from empty buffer".to_string(),
//             ))
//         } else {
//             Ok(self.bytes.remove(0))
//         }
//     }
//
//     fn read_u_int_16(&mut self) -> Result<u16, BinaryCodecError> {
//         // Pop the bytes in reverse order so we get the bytes in correct order
//         if self.bytes.len() >= 2 {
//             let p1 = self.bytes.remove(0);
//             let p2 = self.bytes.remove(0);
//             Ok(((p1 as u16) << 8) | p2 as u16)
//         } else {
//             Err(BinaryCodecError::ParseError(
//                 "Unable to ready 16 bit integer with less then two bytes left of the buffer"
//                     .to_string(),
//             ))
//         }
//     }
//
//     fn read_u_int_32(&mut self) -> Result<u32, BinaryCodecError> {
//         // Pop the bytes in reverse order so we get the bytes in correct order
//         if self.bytes.len() >= 4 {
//             let p1 = self.bytes.remove(0);
//             let p2 = self.bytes.remove(0);
//             let p3 = self.bytes.remove(0);
//             let p4 = self.bytes.remove(0);
//             Ok(((p1 as u32) << 24) | ((p2 as u32) << 16) | ((p3 as u32) << 8) | p4 as u32)
//         } else {
//             Err(BinaryCodecError::ParseError(
//                 "Unable to ready 32 bit integer with less then 4 bytes left of the buffer"
//                     .to_string(),
//             ))
//         }
//     }
//
//     fn read_u_int_64(&mut self) -> Result<u64, BinaryCodecError> {
//         // Pop the bytes in reverse order so we get the bytes in correct order
//         if self.bytes.len() >= 8 {
//             let p1 = self.bytes.remove(0);
//             let p2 = self.bytes.remove(0);
//             let p3 = self.bytes.remove(0);
//             let p4 = self.bytes.remove(0);
//             let p5 = self.bytes.remove(0);
//             let p6 = self.bytes.remove(0);
//             let p7 = self.bytes.remove(0);
//             let p8 = self.bytes.remove(0);
//
//             Ok(((p1 as u64) << 56)
//                 | ((p2 as u64) << 48)
//                 | ((p3 as u64) << 40)
//                 | ((p4 as u64) << 32)
//                 | ((p5 as u64) << 24)
//                 | ((p6 as u64) << 16)
//                 | ((p7 as u64) << 8)
//                 | p8 as u64)
//         } else {
//             Err(BinaryCodecError::ParseError(
//                 "Unable to ready 64 bit integer with less then 8 bytes left of the buffer"
//                     .to_string(),
//             ))
//         }
//     }
//
//     fn peek(&self) -> u8 {
//         self.bytes.first().unwrap().to_owned()
//     }
//
//     fn peek_n(&self, n: i32) -> Vec<u8> {
//         let n_usize: usize = n.try_into().unwrap();
//
//         self.bytes.clone()[..n_usize].to_vec()
//     }
//
//     fn read_n_bytes(&mut self, n: i32) -> Result<Vec<u8>, BinaryCodecError> {
//         let n_usize: usize = n.try_into().unwrap();
//         if self.bytes.len() < n_usize {
//             Err(BinaryCodecError::ParseError(format!(
//                 "Unable to ready {} bytes with less then {} bytes left of the buffer",
//                 n, n
//             )))
//         } else {
//             let read_bytes = self.bytes[..n_usize].to_vec();
//             self.bytes = self.bytes[n_usize..].to_vec();
//
//             Ok(read_bytes)
//         }
//     }
//
//     // https://xrpl.org/serialization.html#length-prefixing
//     fn read_variable_length_length(&mut self) -> Result<i32, BinaryCodecError> {
//         let first_byte = self.read_u_int_8()?;
//         if first_byte <= 192 {
//             Ok(first_byte as i32)
//         } else {
//             let second_byte: u8;
//             if first_byte <= 240 {
//                 second_byte = self.read_u_int_8()?;
//                 Ok(193 + (first_byte as i32 - 193) * 256 + second_byte as i32)
//             } else if first_byte <= 254 {
//                 second_byte = self.read_u_int_8()?;
//                 let third_byte = self.read_u_int_8()?;
//                 Ok(12481
//                     + (first_byte as i32 - 240 - 1) * 65536
//                     + second_byte as i32 * 256
//                     + third_byte as i32)
//             } else {
//                 Err(BinaryCodecError::ParseError(
//                     "Invalid variable length indicator".to_string(),
//                 ))
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use xrpl_types::{Amount, TransactionType};
//
//     use crate::serializer::Serializer;
//
//     use super::*;
//
//     #[test]
//     fn test_payment_parsing_without_prefix() {
//         let account = "rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe";
//
//         let destination = "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe";
//         let mut tx: Transaction = Transaction::payment(AccountId::from_address(account).unwrap(), AccountId::from_address(destination).unwrap(), Amount::xrp("10"));
//         tx.sequence = Some(12_u32);
//         tx.last_ledger_sequence = Some(12_u32);
//         tx.fee = Some(100_u64);
//         tx.memos = Some(vec![Memo {
//             memo_data: vec![0x21, 0x33],
//             memo_type: vec![0x21, 0x33],
//             memo_format: Some(vec![0x21, 0x33]),
//         }]);
//
//         let public_key =
//             hex::decode("165F2F406B5DCC37E666B7A0C9686CD4C92B67D5D362C618A96627E394F2FF45")
//                 .unwrap();
//         tx.signing_public_key = Some(public_key.clone());
//
//         let signature = vec![
//             48, 68, 2, 32, 89, 232, 71, 94, 242, 31, 56, 10, 10, 143, 247, 15, 249, 118, 245, 61,
//             251, 46, 234, 173, 217, 136, 96, 246, 66, 191, 64, 4, 160, 8, 190, 247, 2, 32, 20, 39,
//             148, 153, 33, 141, 209, 70, 11, 117, 49, 53, 174, 174, 213, 166, 57, 53, 172, 229, 151,
//             88, 105, 195, 32, 72, 134, 177, 243, 70, 86, 158,
//         ];
//         tx.signature = Some(signature.clone());
//
//         let mut s = Serializer::new();
//         s.push_transaction(&tx, None);
//
//         let mut p = Parser::new(s.to_vec());
//         let p_tx = p.parse_transaction().unwrap().transaction;
//
//         assert_eq!(p_tx.transaction_type, TransactionType::Payment);
//         assert_eq!(p_tx.flags, None);
//         assert_eq!(p_tx.sequence, Some(12_u32));
//         assert_eq!(p_tx.last_ledger_sequence, Some(12_u32));
//
//         // Currency amount common
//         assert_eq!(p_tx.amount, Some(Amount::xrp("10")));
//         assert_eq!(p_tx.fee, Some(100_u64));
//
//         assert_eq!(p_tx.account.to_address(), account);
//         assert_eq!(p_tx.destination.map(|d| d.to_address()), Some(String::from(destination)));
//
//         assert_eq!(p_tx.signing_public_key, Some(public_key));
//         assert_eq!(p_tx.signature, Some(signature));
//     }
//
//     #[test]
//     fn test_payment_parsing_with_prefix() {
//         let account = "rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe";
//
//         let destination = "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe";
//         let mut tx: Transaction = Transaction::payment(AccountId::from_address(account).unwrap(), AccountId::from_address(destination).unwrap(), Amount::xrp("10"));
//         tx.sequence = Some(12_u32);
//         tx.last_ledger_sequence = Some(12_u32);
//         tx.fee = Some(100_u64);
//         let mut s = Serializer::new();
//         s.push_transaction(&tx, Some(&HASH_PREFIX_TRANSACTION));
//
//         let mut p = Parser::new(s.to_vec());
//         let parsed: ParsedTransaction = p.parse_transaction().unwrap();
//         let p_tx = parsed.transaction;
//         let p_prefix = parsed.prefix;
//
//         assert_eq!(p_prefix, Some(HASH_PREFIX_TRANSACTION));
//
//         assert_eq!(p_tx.transaction_type, TransactionType::Payment);
//         assert_eq!(p_tx.flags, None);
//         assert_eq!(p_tx.sequence, Some(12_u32));
//         assert_eq!(p_tx.last_ledger_sequence, Some(12_u32));
//
//         // Currency amount common
//         assert_eq!(p_tx.amount, Some(Amount::xrp("10")));
//         assert_eq!(p_tx.fee, Some(100_u64));
//
//         assert_eq!(p_tx.account.to_address(), account);
//         assert_eq!(p_tx.destination.map(|d|d.to_address()), Some(String::from(destination)));
//     }
// }

// TODO allan
