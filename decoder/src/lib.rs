use std::collections::HashMap;
use std::error::Error;
use std::convert::TryInto;
use std::str::FromStr;

use serde_json::Value;
use sha2::Digest;
use std::any::Any;
use std::any::TypeId;

pub trait SerializedType: Sized + Clone + ToString + AsRef<[u8]> + FromStr + Default + std::fmt::Debug {
    type Parser: Sized;
    type Field: Sized;

    fn from_parser(parser: &mut Self::Parser, hint: Option<usize>) -> Result<Self, &'static str>;
    fn to_bytes_sink(&self, sink: &mut Vec<Vec<u8>>) {
        sink.push(self.as_ref().to_vec());
    }
    fn to_json(&self, _field: &Self::Field) -> Result<Value, &'static str> {
        Ok(Value::String(self.to_string()))
    }
}

pub struct FieldInfo {
    nth: u16,
    is_vl_encoded: bool,
    field_type: String,
}

#[derive(Debug, Clone)]
enum DynamicSerializedType {
    Hash256(Hash256),
    Blob(Blob),
    AccountID(AccountID),
    STObject(STObject),
    STArray(STArray),
}
impl DynamicSerializedType {
    fn from_parser_dispatch(field: &FieldInstance, parser: &mut BinaryParser) -> Result<DynamicSerializedType, &'static str> {
        match &field.serialized_type {  // Use the stored serialized_type
            DynamicSerializedType::Hash256(_) => Ok(DynamicSerializedType::Hash256(parser.read_field_value::<Hash256>(&field)?)),
            DynamicSerializedType::Blob(_) => Ok(DynamicSerializedType::Blob(parser.read_field_value::<Blob>(&field)?)),
            DynamicSerializedType::AccountID(_) => Ok(DynamicSerializedType::AccountID(parser.read_field_value::<AccountID>(&field)?)),
            DynamicSerializedType::STObject(_) => Ok(DynamicSerializedType::STObject(parser.read_field_value::<STObject>(&field)?)),
            DynamicSerializedType::STArray(_) => Ok(DynamicSerializedType::STArray(parser.read_field_value::<STArray>(&field)?)),
        }
    }
    fn to_json_dispatch(&self, field_lookup: &FieldLookup) -> Result<Value, &'static str> {
        match self {
            DynamicSerializedType::Hash256(val) => val.to_json(field_lookup),
            DynamicSerializedType::Blob(val) => val.to_json(field_lookup),
            DynamicSerializedType::AccountID(val) => val.to_json(field_lookup),
            DynamicSerializedType::STObject(val) => val.to_json(field_lookup),
            DynamicSerializedType::STArray(val) => val.to_json(field_lookup),
        }
    }
    fn to_bytes_sink(&self, sink: &mut Vec<Vec<u8>>) {
        match self {
            DynamicSerializedType::Hash256(val) => val.to_bytes_sink(sink),
            DynamicSerializedType::Blob(val) => val.to_bytes_sink(sink),
            DynamicSerializedType::AccountID(val) => val.to_bytes_sink(sink),
            DynamicSerializedType::STObject(val) => val.to_bytes_sink(sink),
            DynamicSerializedType::STArray(val) => val.to_bytes_sink(sink),
        }
    }
}
impl AsRef<[u8]> for DynamicSerializedType {
    fn as_ref(&self) -> &[u8] {
        match self {
            DynamicSerializedType::Hash256(val) => val.as_ref(),
            DynamicSerializedType::Blob(val) => val.as_ref(),
            DynamicSerializedType::AccountID(val) => val.as_ref(),
            DynamicSerializedType::STObject(val) => val.as_ref(),
            DynamicSerializedType::STArray(val) => val.as_ref(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInstance {
    is_variable_length_encoded: bool,
    ordinal: u32,
    name: String,
    header: Vec<u8>,
    serialized_type: DynamicSerializedType,
}

fn encode_variable_length(length: usize) -> Result<Vec<u8>, &'static str> {
    let mut len_bytes = vec![0u8; 3];
    if length <= 192 {
        len_bytes[0] = length as u8;
        Ok(len_bytes[0..1].to_vec())
    } else if length <= 12480 {
        let length = length - 193;
        len_bytes[0] = 193 + ((length >> 8) as u8);
        len_bytes[1] = (length & 0xff) as u8;
        Ok(len_bytes[0..2].to_vec())
    } else if length <= 918744 {
        let length = length - 12481;
        len_bytes[0] = 241 + ((length >> 16) as u8);
        len_bytes[1] = ((length >> 8) & 0xff) as u8;
        len_bytes[2] = (length & 0xff) as u8;
        Ok(len_bytes[0..3].to_vec())
    } else {
        Err("Overflow error")
    }
}

fn write_field_and_value(sink: &mut Vec<Vec<u8>>, field: &FieldInstance, value: DynamicSerializedType) -> Result<(), &'static str> {
    sink.push(field.header.clone());
    if field.is_variable_length_encoded {
        let bytes = value.as_ref().to_vec();
        let vl = encode_variable_length(bytes.len())?;
        sink.push(vl);
        sink.push(bytes);
    } else {
        value.to_bytes_sink(sink);
    }
    Ok(())
}

pub struct BinaryParser {
    bytes: Vec<u8>,
    pub field: HashMap<String, FieldInstance>,
}
impl BinaryParser {
    fn new(bytes: Vec<u8>, field: HashMap<String, FieldInstance>) -> Self {
        Self { bytes, field }
    }

    fn read(&mut self, n: usize) -> Result<Vec<u8>, &'static str> {
        if n <= self.bytes.len() {
            let front = self.bytes[0..n].to_vec();
            self.bytes.drain(0..n);
            Ok(front)
        } else {
            Err("Insufficient bytes")
        }
    }

    fn read_u8(&mut self) -> Result<u8, &'static str> {
        let bytes = self.read(1)?;
        Ok(bytes[0])
    }

    fn end(&self) -> bool {
        self.bytes.is_empty()
    }

    fn read_variable_length(&mut self) -> Result<usize, &'static str> {
        let b1 = self.read_u8()? as usize;
        if b1 <= 192 {
            Ok(b1)
        } else if b1 <= 240 {
            let b2 = self.read_u8()? as usize;
            Ok(193 + (b1 - 193) * 256 + b2)
        } else if b1 <= 254 {
            let b2 = self.read_u8()? as usize;
            let b3 = self.read_u8()? as usize;
            Ok(12481 + (b1 - 241) * 65536 + b2 * 256 + b3)
        } else {
            Err("Invalid variable length indicator")
        }
    }

    fn read_field_ordinal(&mut self) -> Result<u32, &'static str> {
        let mut type_code = self.read_u8()? as u32;
        let mut nth = type_code & 15;
        type_code >>= 4;

        if type_code == 0 {
            type_code = self.read_u8()? as u32;
            if type_code == 0 || type_code < 16 {
                return Err("Cannot read FieldOrdinal, type_code out of range");
            }
        }

        if nth == 0 {
            nth = self.read_u8()? as u32;
            if nth == 0 || nth < 16 {
                return Err("Cannot read FieldOrdinal, field_code out of range");
            }
        }

        Ok((type_code << 16) | nth)
    }

    fn read_field(&mut self) -> Result<FieldInstance, &'static str> {
        let ordinal = self.read_field_ordinal()?;
        println!("ordinal: {}", ordinal);
        self.field
            .get(&ordinal.to_string())
            .cloned()
            .ok_or("Field not found")
    }

    fn read_field_value<T: SerializedType<Parser=Self>>(&mut self, field: &FieldInstance) -> Result<T, &'static str> {
        let size_hint = if field.is_variable_length_encoded {
            Some(self.read_variable_length()?)
        } else {
            None
        };
        T::from_parser(self, size_hint)
    }
}

pub struct FieldLookup {
    field_map: HashMap<String, FieldInstance>,
}

impl  FieldLookup {
    pub fn new(types: HashMap<String, u32>, fields: Vec<(String, FieldInfo)>) -> Self {
        let mut field_map = HashMap::new();
        for (name, info) in fields {
            let type_ordinal = types[&info.field_type];
            let header = Self::field_header(type_ordinal, info.nth);
            let serialized_type = match info.field_type.as_str() {
                "Hash256" => DynamicSerializedType::Hash256(Hash256::default()),
                "Blob" => DynamicSerializedType::Blob(Blob::default()),
                "AccountID" => DynamicSerializedType::AccountID(AccountID::default()),
                "STObject" => DynamicSerializedType::STObject(STObject::default()),
                "STArray" => DynamicSerializedType::STArray(STArray::default()),
                _ => panic!("Unknown field type"),
            };
            let field = FieldInstance {
                is_variable_length_encoded: info.is_vl_encoded,
                ordinal: (type_ordinal << 16) | (info.nth as u32),
                name: name.clone(),
                header,
                serialized_type,
            };
            field_map.insert(name.clone(), field.clone());
            field_map.insert(field.ordinal.to_string(), field);
        }
        Self { field_map }
    }

    fn field_header(type_: u32, nth: u16) -> Vec<u8> {
        let mut header = Vec::new();
        if type_ < 16 {
            if nth < 16 {
                header.push(((type_ << 4) | nth as u32) as u8);
            } else {
                header.push((type_ << 4) as u8);
                header.push(nth as u8);
            }
        } else if nth < 16 {
            header.push(nth as u8);
            header.push(type_ as u8);
        } else {
            header.push(0);
            header.push(type_ as u8);
            header.push(nth as u8);
        }
        header
    }
}

#[derive(Clone, Debug, Default)]
pub struct Hash256(Vec<u8>);
impl Hash256 {
    pub const WIDTH: usize = 32;
}
impl SerializedType for Hash256 {
    type Parser = BinaryParser;
    type Field = FieldLookup;
    fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
        parser.read(Self::WIDTH).map(|bytes| Self(bytes))
    }
}
impl AsRef<[u8]> for Hash256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl ToString for Hash256 {
    fn to_string(&self) -> String {
        hex::encode_upper(&self.0)
    }
}
impl FromStr for Hash256 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
        Ok(Self(bytes))
    }
}

#[derive(Clone, Debug, Default)]
pub struct AccountID(Vec<u8>);
impl AccountID {
    pub const ACCOUNT_ID_BUF: &[u8] = &[0];
    pub const WIDTH: usize = 20;
}
impl SerializedType for AccountID {
    type Parser = BinaryParser;
    type Field = FieldLookup;
    fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
        parser.read(Self::WIDTH).map(|bytes| Self(bytes))
    }
    fn to_json(&self, _: &Self::Field) -> Result<Value, &'static str> {
        let mut hasher = sha2::Sha256::new();
        
        // init buffer with total length (ACCOUNT_ID (1) self.0 bytes len (20) and checksum (4))
        let mut buffer = Vec::with_capacity(Self::ACCOUNT_ID_BUF.len() + self.0.len() + 4);
        buffer.extend_from_slice(Self::ACCOUNT_ID_BUF);
        buffer.extend_from_slice(&self.0);
        
        // first SHA-256 hash
        hasher.update(&buffer);
        let first_hash = hasher.finalize_reset();
        
        // second SHA-256 hash
        hasher.update(&first_hash);
        let second_hash = hasher.finalize();
        
        // take the first 4 bytes as a check and append to buffer
        buffer.extend_from_slice(&second_hash[0..4]);
        
        // Base58 encode the final buffer
        let encoded_buf = bs58::encode(buffer)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_string();

        Ok(Value::String(encoded_buf))
    }
}
impl AsRef<[u8]> for AccountID {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl ToString for AccountID {
    fn to_string(&self) -> String {
        hex::encode_upper(&self.0)
    }
}
impl FromStr for AccountID {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Blob(Vec<u8>);
impl SerializedType for Blob {
    type Parser = BinaryParser;
    type Field = FieldLookup;
    fn from_parser(parser: &mut Self::Parser, hint: Option<usize>) -> Result<Self, &'static str> {
        let hint = hint.ok_or("Blob hint not found")?;
        parser.read(hint).map(|bytes| Self(bytes))
    }
}
impl AsRef<[u8]> for Blob {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl ToString for Blob {
    fn to_string(&self) -> String {
        hex::encode_upper(&self.0)
    }
}
impl FromStr for Blob {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, Default)]
pub struct STObject(Vec<u8>);
impl STObject {
    pub const OBJECT_NAME: &str = "STObject";
    pub const OBJECT_END_MARKER_NAME: &str = "ObjectEndMarker";
    pub const OBJECT_END_MARKER_BYTE: &[u8] = &[0xE1];
}
impl SerializedType for STObject {
    type Parser = BinaryParser;
    type Field = FieldLookup;
    fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
        let mut sink: Vec<Vec<u8>> = Vec::new();
        loop {
            if parser.end() {
                break;
            }
            let field = parser.read_field()?;
            if field.name == Self::OBJECT_END_MARKER_NAME {
                break;
            }
            let associated_value = DynamicSerializedType::from_parser_dispatch(&field, parser)?;
            write_field_and_value(&mut sink, &field, associated_value)?;
            if field.name == Self::OBJECT_NAME {
                sink.push(Self::OBJECT_END_MARKER_BYTE.to_vec());
            }
        }

        let concatenated_sink: Vec<u8> = sink.into_iter().flatten().collect();
        Ok(Self(concatenated_sink))
    }
    fn to_json(&self, field_lookup: &Self::Field) -> Result<Value, &'static str> {
        let mut object_parser = BinaryParser::new(self.0.clone(), field_lookup.field_map.clone());
        let mut accumulator: HashMap<String, Value> = HashMap::new();
        
        while !object_parser.end() {
            let field = object_parser.read_field()?; {
                if field.name == "ObjectEndMarker" {
                    break;
                }
                let associated_value = DynamicSerializedType::from_parser_dispatch(&field, &mut object_parser)?;
                let json_value = associated_value.to_json_dispatch(field_lookup)?;
                accumulator.insert(field.name, json_value);
            }
        }
        Ok(Value::Object(accumulator.into_iter().collect()))
    }
}
impl AsRef<[u8]> for STObject {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl ToString for STObject {
    fn to_string(&self) -> String {
        hex::encode_upper(&self.0)
    }
}
impl FromStr for STObject {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, Default)]
pub struct STArray(Vec<u8>);
impl STArray {
    pub const ARRAY_END_MARKER: &[u8] = &[0xf1];
    pub const ARRAY_END_MARKER_NAME: &str = "ArrayEndMarker";
    pub const OBJECT_END_MARKER_ARRAY: &[u8] = &[0xE1];
}
impl SerializedType for STArray {
    type Parser = BinaryParser;
    type Field = FieldLookup;
    fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
        let mut bytes = Vec::new();
        
        while !parser.end() {
            let field = parser.read_field()?;
            if field.name == Self::ARRAY_END_MARKER_NAME {
                break;
            }
            bytes.extend_from_slice(&field.header);
            let associated_value = DynamicSerializedType::from_parser_dispatch(&field, parser)?;
            bytes.extend_from_slice(associated_value.as_ref());
            bytes.extend_from_slice(Self::OBJECT_END_MARKER_ARRAY);
        }
        bytes.extend_from_slice(Self::ARRAY_END_MARKER);
        
        Ok(Self(bytes))
    }
    fn to_json(&self, field_lookup: &Self::Field) -> Result<Value, &'static str> {
        let mut result = Vec::new();
        let mut array_parser = BinaryParser::new(self.0.clone(), field_lookup.field_map.clone());

        while !array_parser.end() {
            let field = array_parser.read_field()?;
            if field.name == Self::ARRAY_END_MARKER_NAME {
                break;
            }
            let mut obj = HashMap::new();
            let associated_value = DynamicSerializedType::from_parser_dispatch(&field, &mut array_parser)?;
            let json_value = associated_value.to_json_dispatch(field_lookup)?;
            obj.insert(
                field.name.clone(),
                json_value,
            );
            result.push(Value::Object(obj.into_iter().collect()));
        }
        Ok(Value::Array(result))
    }
}
impl AsRef<[u8]> for STArray {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl ToString for STArray {
    fn to_string(&self) -> String {
        hex::encode_upper(&self.0)
    }
}
impl FromStr for STArray {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    fn get_field_lookup() -> FieldLookup {
        let types = vec![
            ("Hash256".to_string(), 5),
            ("Blob".to_string(), 7),
            ("AccountID".to_string(), 8),
            ("STObject".to_string(), 14),
            ("STArray".to_string(), 15),
        ].into_iter().collect();
        let fields = vec![
            // ("TransactionType".to_string(), FieldInfo { nth: 2, is_vl_encoded: false, field_type: "Hash256".to_string() }),
            // ("Flags".to_string(), FieldInfo { nth: 2, is_vl_encoded: false, field_type: "Hash256".to_string() }),
            // ("Sequence".to_string(), FieldInfo { nth: 2, is_vl_encoded: false, field_type: "Hash256".to_string() }),
            // ("LastLedgerSequence".to_string(), FieldInfo { nth: 2, is_vl_encoded: false, field_type: "Hash256".to_string() }),
            
            ("AccountTxnID".to_string(), FieldInfo { nth: 9, is_vl_encoded: false, field_type: "Hash256".to_string() }),
            ("SigningPubKey".to_string(), FieldInfo { nth: 3, is_vl_encoded: true, field_type: "Blob".to_string() }),
            ("TxnSignature".to_string(), FieldInfo { nth: 4, is_vl_encoded: true, field_type: "Blob".to_string() }),
            ("MemoType".to_string(), FieldInfo { nth: 12, is_vl_encoded: true, field_type: "Blob".to_string() }),
            ("MemoData".to_string(), FieldInfo { nth: 13, is_vl_encoded: true, field_type: "Blob".to_string() }),
            ("MemoFormat".to_string(), FieldInfo { nth: 14, is_vl_encoded: true, field_type: "Blob".to_string() }),
            ("Account".to_string(), FieldInfo { nth: 1, is_vl_encoded: true, field_type: "AccountID".to_string() }),
            ("Memo".to_string(), FieldInfo { nth: 10, is_vl_encoded: false, field_type: "STObject".to_string() }),
            ("Memos".to_string(), FieldInfo { nth: 9, is_vl_encoded: false, field_type: "STArray".to_string() }),
            ("ObjectEndMarker".to_string(), FieldInfo { nth: 1, is_vl_encoded: false, field_type: "STObject".to_string() }),
            ("ArrayEndMarker".to_string(), FieldInfo { nth: 1, is_vl_encoded: false, field_type: "STArray".to_string() }),
        ];
        FieldLookup::new(types, fields)
    }

    #[test]
    fn test_decode_account_txn_id() {
        // AccountTxnID encoded
        let encoded_account_id = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C25";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id).unwrap(), field_lookup.field_map);
        let field_type = Hash256::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_account_id, str_val);
    }

    #[test]
    fn test_decode_account_txn_id_obj() {
        let encoded_account_id_obj = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_account_id_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "AccountTxnID": "16969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580"
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }

    #[test]
    fn test_decode_account_id() {
        // AccountID encoded
        let encoded_account_id = "811424A53BB5CAAD40A961836FEF648E8424846E";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id).unwrap(), field_lookup.field_map);
        let field_type = AccountID::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_account_id, str_val);
    }

    #[test]
    fn test_decode_account_id_obj() {
        let encoded_account_id_obj = "811424A53BB5CAAD40A961836FEF648E8424846EC75A";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_account_id_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "Account": "rhLmGWkHr59h9ffYgPEAqZnqiQZMGb71yo"
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }

    #[test]
    fn test_decode_signing_pub_key() {
        // SigningPubKey encoded
        let encoded_signing_pub_key = "732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_signing_pub_key).unwrap(), field_lookup.field_map);
        let field_type = Blob::from_parser(parser, Some(33)).unwrap(); // NOTE: hardcoded hint size
        let str_val = field_type.to_string();
        assert_eq!(encoded_signing_pub_key, str_val);
    }

    #[test]
    fn test_decode_signing_pub_key_obj() {
        let encoded_signing_pub_key_obj = "732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_signing_pub_key_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_signing_pub_key_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "SigningPubKey": "02A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A"
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }

    #[test]
    fn test_decode_txn_signature() {
        // TxnSignature encoded
        let encoded_tx_sig = "74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CD";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_sig).unwrap(), field_lookup.field_map);
        let field_type = Blob::from_parser(parser, Some(71)).unwrap(); // NOTE: hardcoded hint size
        let str_val = field_type.to_string();
        assert_eq!(encoded_tx_sig, str_val);
    }

    #[test]
    fn test_decode_txn_signature_obj() {
        let encoded_tx_sig_obj = "74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_sig_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_tx_sig_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "TxnSignature": "3045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC"
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }

    // #[test]
    // fn test_decode_memos_array() {
    //     let encoded_tx_memos_arr = "7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74F1E1F1F1E1F1";

    //     let field_lookup = get_field_lookup();
    //     let parser = &mut BinaryParser::new(hex::decode(encoded_tx_memos_arr).unwrap(), field_lookup.field_map.clone());
    //     let field_type = STArray::from_parser(parser, None).unwrap();
    //     let mut str_val = field_type.to_string();
    //     assert_eq!(encoded_tx_memos_arr, str_val);
    // }

    #[test]
    fn test_decode_memos_txn_obj() {
        let encoded_tx_obj = "F9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_tx_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "Memos": [
                {
                    "Memo": {
                        "MemoType": "687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E65726963",
                        "MemoData": "72656E74"
                    }
                }
            ]
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }

    #[test]
    fn test_decode_txn_obj() {
        let encoded_tx_obj = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC811424A53BB5CAAD40A961836FEF648E8424846EC75AF9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";

        let field_lookup = get_field_lookup();
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = STObject::from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_tx_obj, str_val);

        let json_val = field_type.to_json(&field_lookup).unwrap();
        let expected_val = r#"{
            "AccountTxnID": "16969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580",
            "SigningPubKey": "02A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A",
            "TxnSignature": "3045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC",
            "Account": "rhLmGWkHr59h9ffYgPEAqZnqiQZMGb71yo",
            "Memos": [
                {
                    "Memo": {
                        "MemoType": "687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E65726963",
                        "MemoData": "72656E74"
                    }
                }
            ]
        }"#;
        assert_eq!(json_val, from_str::<Value>(expected_val).unwrap());
    }
}
     