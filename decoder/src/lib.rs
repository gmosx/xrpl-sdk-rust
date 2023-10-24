use std::collections::HashMap;
use std::str::FromStr;

use serde_json::Value;
use sha2::Digest;

use xrpl_binary_codec::serializer::field_info::{FieldInfo, create_field_info_map};
use xrpl_binary_codec::serializer::field_id::{FieldId, TypeCode};

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SerializedData {
    type_code: TypeCode,
    data: Vec<u8>,
}

impl From<TypeCode> for SerializedData {
    fn from(type_code: TypeCode) -> Self {
        Self { type_code, data: vec![] }
    }
}

impl SerializedData {
    pub const ACCOUNT_ID_BUF: &[u8] = &[0];

    pub const OBJECT_NAME: &str = "STObject";
    pub const OBJECT_END_MARKER_NAME: &str = "ObjectEndMarker";
    pub const OBJECT_END_MARKER_BYTE: &[u8] = &[0xE1];

    pub const ARRAY_END_MARKER: &[u8] = &[0xf1];
    pub const ARRAY_END_MARKER_NAME: &str = "ArrayEndMarker";
    pub const OBJECT_END_MARKER_ARRAY: &[u8] = &[0xE1];

    pub fn from_parser(&self, parser: &mut BinaryParser, hint: Option<usize>) -> Result<Self, &'static str> {
        match self.type_code {
            TypeCode::Hash256 => {
                let bytes: [u8; 32] = parser.read(32)?.try_into().map_err(|_| "Invalid length")?;
                Ok(Self{ type_code: self.type_code, data: bytes.to_vec() })
            },
            TypeCode::AccountId => {
                let bytes: [u8; 20] = parser.read(20)?.try_into().map_err(|_| "Invalid length")?;
                Ok(Self{ type_code: self.type_code, data: bytes.to_vec() })
            },
            TypeCode::Blob => {
                let hint = hint.ok_or("Blob hint not found")?;
                let bytes = parser.read(hint)?;
                Ok(Self{ type_code: self.type_code, data: bytes })
            },
            TypeCode::Object => {
                let mut sink: Vec<Vec<u8>> = Vec::new();
                loop {
                    if parser.end() {
                        break;
                    }
                    let field = parser.read_field()?;
                    if field.name == Self::OBJECT_END_MARKER_NAME {
                        break;
                    }
                    let data = parser.read_field_value(&field)?;
                    write_field_and_value(&mut sink, &field, data)?;
                    if field.name == Self::OBJECT_NAME {
                        sink.push(Self::OBJECT_END_MARKER_BYTE.to_vec());
                    }
                }
                let concatenated_sink: Vec<u8> = sink.into_iter().flatten().collect();
                Ok(Self{ type_code: self.type_code, data: concatenated_sink })
            },
            TypeCode::Array => {
                let mut bytes = Vec::new();
                
                while !parser.end() {
                    let field = parser.read_field()?;
                    if field.name == Self::ARRAY_END_MARKER_NAME {
                        break;
                    }
                    bytes.extend_from_slice(&field.header);
                    let data = parser.read_field_value(&field)?;
                    bytes.extend_from_slice(data.as_ref());
                    bytes.extend_from_slice(Self::OBJECT_END_MARKER_ARRAY);
                }
                bytes.extend_from_slice(Self::ARRAY_END_MARKER);
                Ok(Self{ type_code: self.type_code, data: bytes })
            },
            _ => Ok(Self { type_code: TypeCode::Blob, data: vec![] }), // TODO: default other types to Blob for now
        }
    }

    pub fn to_json(&self, field_lookup: &FieldLookup) -> Result<Value, &'static str> {
        match self.type_code {
            TypeCode::Hash256 => Ok(Value::String(self.to_string())),
            TypeCode::AccountId => Ok(Value::String(self.to_string())),
            TypeCode::Blob => Ok(Value::String(self.to_string())),
            TypeCode::Object => {
                let mut object_parser = BinaryParser::new(self.data.to_vec(), field_lookup.field_map.clone());
                let mut accumulator: HashMap<String, Value> = HashMap::new();
                
                while !object_parser.end() {
                    let field: FieldInstance = object_parser.read_field()?; {
                        if field.name == "ObjectEndMarker" {
                            break;
                        }
                        let data = object_parser.read_field_value(&field)?;
                        let json_value = Self { type_code: field.info.field_type, data }.to_json(field_lookup)?;
                        accumulator.insert(field.name, json_value);
                    }
                }
                Ok(Value::Object(accumulator.into_iter().collect()))
            },
            TypeCode::Array => {
                let mut result = Vec::new();
                let mut array_parser = BinaryParser::new(self.data.to_vec(), field_lookup.field_map.clone());
        
                while !array_parser.end() {
                    let field = array_parser.read_field()?;
                    if field.name == Self::ARRAY_END_MARKER_NAME {
                        break;
                    }
                    let mut obj = HashMap::new();
                    let data = array_parser.read_field_value(&field)?;
                    // let json_value = to_json_value(field.info.field_type, &data, field_lookup)?;
                    let json_value = Self { type_code: field.info.field_type, data }.to_json(field_lookup)?;
                    obj.insert(
                        field.name.clone(),
                        json_value,
                    );
                    result.push(Value::Object(obj.into_iter().collect()));
                }
                Ok(Value::Array(result))
            },
            _ => Ok(Value::String(self.to_string())), // TODO: default other types to Blob for now
        }
    }
}
impl AsRef<[u8]> for SerializedData {
    fn as_ref(&self) -> &[u8] { &self.data }
}
impl ToString for SerializedData {
    fn to_string(&self) -> String {
        match self.type_code {
            TypeCode::AccountId => {
                let mut hasher = sha2::Sha256::new();
        
                // init buffer with total length (ACCOUNT_ID (1) self.0 bytes len (20) and checksum (4))
                let mut buffer = Vec::with_capacity(Self::ACCOUNT_ID_BUF.len() + self.data.len() + 4);
                buffer.extend_from_slice(Self::ACCOUNT_ID_BUF);
                buffer.extend_from_slice(&self.data);
                
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

                encoded_buf
            },
            _ => hex::encode_upper(&self.data),
        }
    }
}

// pub trait SerializedType: Sized + ToString + AsRef<[u8]> {
//     type Parser: Sized;

//     fn from_parser(parser: &mut Self::Parser, hint: Option<usize>) -> Result<Self, &'static str>;
//     fn to_json(&self, _field: &FieldLookup) -> Result<Value, &'static str> {
//         Ok(Value::String(self.to_string()))
//     }
// }

// fn to_json_value(code: TypeCode, data: &[u8], field_lookup: &FieldLookup) -> Result<Value, &'static str> {
//     match code {
//         TypeCode::Hash256 => Ok(Value::String(Hash256::try_from(data)?.to_string())),
//         TypeCode::Blob => Ok(Value::String(Blob::try_from(data)?.to_string())),
//         TypeCode::AccountId => Ok(Value::String(AccountID::try_from(data)?.to_string())),
//         TypeCode::Object => STObject::try_from(data)?.to_json(&field_lookup),
//         TypeCode::Array => STArray::try_from(data)?.to_json(&field_lookup),
//         _ => Ok(Value::String(Blob::try_from(data)?.to_string())), // TODO: default other types to Blob for now
//     }
// }

#[derive(Debug, Clone)]
pub struct FieldInstance {
    info: FieldInfo,
    ordinal: u32,
    name: String,
    header: Vec<u8>,
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

fn write_field_and_value(sink: &mut Vec<Vec<u8>>, field: &FieldInstance, bytes: Vec<u8>) -> Result<(), &'static str> {
    sink.push(field.header.clone());
    if field.info.is_vl_encoded {
        let vl = encode_variable_length(bytes.len())?;
        sink.push(vl);
        sink.push(bytes);
    } else {
        sink.push(bytes);
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
        self.field
            .get(&ordinal.to_string())
            .cloned()
            .ok_or("Field not found")
    }

    fn read_field_value(&mut self, field: &FieldInstance) -> Result<Vec<u8>, &'static str> {
        let size_hint = if field.info.is_vl_encoded {
            Some(self.read_variable_length()?)
        } else {
            None
        };
        let serialized_type = SerializedData::from(field.info.field_type).from_parser(self, size_hint)?;
        Ok(serialized_type.data)
    }
}



pub struct FieldLookup {
    field_map: HashMap<String, FieldInstance>,
}
impl From<HashMap<String, FieldInfo>> for FieldLookup {
    fn from(field_info_map: HashMap<String, FieldInfo>) -> Self {
        let mut field_map = HashMap::new();
        for (name, info) in field_info_map {
            let field_type = info.field_type;
            let code = info.field_code.0;
            let header: Vec<u8> = FieldId::from(info.clone()).into();
            let field = FieldInstance {
                info,
                ordinal: ((field_type as u32) << 16) | (code as u32),
                name: name.clone(),
                header,
            };
            field_map.insert(name.clone(), field.clone());
            field_map.insert(field.ordinal.to_string(), field);
        }
        Self { field_map }
    }
}

// #[derive(Clone, Debug, Default)]
// pub struct Hash256(pub [u8; 32]);
// impl TryFrom<&[u8]> for Hash256 {
//     type Error = &'static str;
//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         if bytes.len() != 32 {
//             Err("Invalid length")
//         } else {
//             let mut buf = [0u8; 32];
//             buf.copy_from_slice(bytes);
//             Ok(Self(buf))
//         }
//     }
// }
// impl SerializedType for Hash256 {
//     type Parser = BinaryParser;
//     fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
//         let bytes = parser.read(32)?.try_into().map_err(|_| "Invalid length")?;
//         Ok(Self(bytes))
//     }
// }
// impl AsRef<[u8]> for Hash256 {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
// impl ToString for Hash256 {
//     fn to_string(&self) -> String {
//         hex::encode_upper(&self.0)
//     }
// }
// impl FromStr for Hash256 {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let bytes = hex::decode(s)
//             .map_err(|_| "Invalid hex string")?
//             .try_into()
//             .map_err(|_| "Invalid length")?;
//         Ok(Self(bytes))
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct AccountID(pub [u8; 20]);
// impl AccountID {
//     pub const ACCOUNT_ID_BUF: &[u8] = &[0];
//     pub const WIDTH: usize = 20;
// }
// impl SerializedType for AccountID {
//     type Parser = BinaryParser;
//     fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
//         let bytes = parser.read(20)?.try_into().map_err(|_| "Invalid length")?;
//         Ok(Self(bytes))
//     }
// }
// impl TryFrom<&[u8]> for AccountID {
//     type Error = &'static str;
//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         if bytes.len() != 20 {
//             Err("Invalid length")
//         } else {
//             let mut buf = [0u8; 20];
//             buf.copy_from_slice(bytes);
//             Ok(Self(buf))
//         }
//     }
// }
// impl AsRef<[u8]> for AccountID {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
// impl ToString for AccountID {
//     fn to_string(&self) -> String {
//         let mut hasher = sha2::Sha256::new();
        
//         // init buffer with total length (ACCOUNT_ID (1) self.0 bytes len (20) and checksum (4))
//         let mut buffer = Vec::with_capacity(Self::ACCOUNT_ID_BUF.len() + self.0.len() + 4);
//         buffer.extend_from_slice(Self::ACCOUNT_ID_BUF);
//         buffer.extend_from_slice(&self.0);
        
//         // first SHA-256 hash
//         hasher.update(&buffer);
//         let first_hash = hasher.finalize_reset();
        
//         // second SHA-256 hash
//         hasher.update(&first_hash);
//         let second_hash = hasher.finalize();
        
//         // take the first 4 bytes as a check and append to buffer
//         buffer.extend_from_slice(&second_hash[0..4]);
        
//         // Base58 encode the final buffer
//         let encoded_buf = bs58::encode(buffer)
//             .with_alphabet(bs58::Alphabet::RIPPLE)
//             .into_string();

//         encoded_buf
//     }
// }
// impl FromStr for AccountID {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let bytes = hex::decode(s)
//             .map_err(|_| "Invalid hex string")?
//             .try_into()
//             .map_err(|_| "Invalid length")?;
//         Ok(Self(bytes))
//     }
// }

// #[derive(Debug, Clone, Default)]
// pub struct Blob(Vec<u8>);
// impl SerializedType for Blob {
//     type Parser = BinaryParser;
//     fn from_parser(parser: &mut Self::Parser, hint: Option<usize>) -> Result<Self, &'static str> {
//         let hint = hint.ok_or("Blob hint not found")?;
//         parser.read(hint).map(|bytes| Self(bytes))
//     }
// }
// impl TryFrom<&[u8]> for Blob {
//     type Error = &'static str;
//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         Ok(Self(bytes.to_vec()))
//     }
// }
// impl AsRef<[u8]> for Blob {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
// impl ToString for Blob {
//     fn to_string(&self) -> String {
//         hex::encode_upper(&self.0)
//     }
// }
// impl FromStr for Blob {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
//         Ok(Self(bytes))
//     }
// }

// #[derive(Debug, Clone, Default)]
// pub struct STObject(Vec<u8>);
// impl STObject {
//     pub const OBJECT_NAME: &str = "STObject";
//     pub const OBJECT_END_MARKER_NAME: &str = "ObjectEndMarker";
//     pub const OBJECT_END_MARKER_BYTE: &[u8] = &[0xE1];
// }
// impl SerializedType for STObject {
//     type Parser = BinaryParser;
//     fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
//         let mut sink: Vec<Vec<u8>> = Vec::new();
//         loop {
//             if parser.end() {
//                 break;
//             }
//             let field = parser.read_field()?;
//             if field.name == Self::OBJECT_END_MARKER_NAME {
//                 break;
//             }
//             let data = parser.read_field_value(&field)?;
//             write_field_and_value(&mut sink, &field, data)?;
//             if field.name == Self::OBJECT_NAME {
//                 sink.push(Self::OBJECT_END_MARKER_BYTE.to_vec());
//             }
//         }

//         let concatenated_sink: Vec<u8> = sink.into_iter().flatten().collect();
//         Ok(Self(concatenated_sink))
//     }
//     fn to_json(&self, field_lookup: &FieldLookup) -> Result<Value, &'static str> {
//         let mut object_parser = BinaryParser::new(self.0.clone(), field_lookup.field_map.clone());
//         let mut accumulator: HashMap<String, Value> = HashMap::new();
        
//         while !object_parser.end() {
//             let field: FieldInstance = object_parser.read_field()?; {
//                 if field.name == "ObjectEndMarker" {
//                     break;
//                 }
//                 let data = object_parser.read_field_value(&field)?;
//                 let json_value = to_json_value(field.info.field_type, &data, field_lookup)?;
//                 accumulator.insert(field.name, json_value);
//             }
//         }
//         Ok(Value::Object(accumulator.into_iter().collect()))
//     }
// }
// impl TryFrom<&[u8]> for STObject {
//     type Error = &'static str;
//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         Ok(Self(bytes.to_vec()))
//     }
// }
// impl AsRef<[u8]> for STObject {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
// impl ToString for STObject {
//     fn to_string(&self) -> String {
//         hex::encode_upper(&self.0)
//     }
// }
// impl FromStr for STObject {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
//         Ok(Self(bytes))
//     }
// }

// #[derive(Debug, Clone, Default)]
// pub struct STArray(Vec<u8>);
// impl STArray {
//     pub const ARRAY_END_MARKER: &[u8] = &[0xf1];
//     pub const ARRAY_END_MARKER_NAME: &str = "ArrayEndMarker";
//     pub const OBJECT_END_MARKER_ARRAY: &[u8] = &[0xE1];
// }
// impl SerializedType for STArray {
//     type Parser = BinaryParser;
//     fn from_parser(parser: &mut Self::Parser, _hint: Option<usize>) -> Result<Self, &'static str> {
//         let mut bytes = Vec::new();
        
//         while !parser.end() {
//             let field = parser.read_field()?;
//             if field.name == Self::ARRAY_END_MARKER_NAME {
//                 break;
//             }
//             bytes.extend_from_slice(&field.header);
//             let data = parser.read_field_value(&field)?;
//             bytes.extend_from_slice(data.as_ref());
//             bytes.extend_from_slice(Self::OBJECT_END_MARKER_ARRAY);
//         }
//         bytes.extend_from_slice(Self::ARRAY_END_MARKER);
        
//         Ok(Self(bytes))
//     }
//     fn to_json(&self, field_lookup: &FieldLookup) -> Result<Value, &'static str> {
//         let mut result = Vec::new();
//         let mut array_parser = BinaryParser::new(self.0.clone(), field_lookup.field_map.clone());

//         while !array_parser.end() {
//             let field = array_parser.read_field()?;
//             if field.name == Self::ARRAY_END_MARKER_NAME {
//                 break;
//             }
//             let mut obj = HashMap::new();
//             let data = array_parser.read_field_value(&field)?;
//             let json_value = to_json_value(field.info.field_type, &data, field_lookup)?;
//             obj.insert(
//                 field.name.clone(),
//                 json_value,
//             );
//             result.push(Value::Object(obj.into_iter().collect()));
//         }
//         Ok(Value::Array(result))
//     }
// }
// impl TryFrom<&[u8]> for STArray {
//     type Error = &'static str;
//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         Ok(Self(bytes.to_vec()))
//     }
// }
// impl AsRef<[u8]> for STArray {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
// impl ToString for STArray {
//     fn to_string(&self) -> String {
//         hex::encode_upper(&self.0)
//     }
// }
// impl FromStr for STArray {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let bytes = hex::decode(s).map_err(|_| "Invalid hex string")?;
//         Ok(Self(bytes))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_decode_account_txn_id() {
        // AccountTxnID encoded
        let encoded_account_id = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C25";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id).unwrap(), field_lookup.field_map);
        let field_type = SerializedData::from(TypeCode::Hash256).from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!(encoded_account_id, str_val);
    }

    #[test]
    fn test_decode_account_txn_id_obj() {
        let encoded_account_id_obj = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id).unwrap(), field_lookup.field_map);
        let field_type = SerializedData::from(TypeCode::AccountId).from_parser(parser, None).unwrap();
        let str_val = field_type.to_string();
        assert_eq!("rUmWJKM2b87GsKeTzSw14NeuubPQc8meTL", str_val);
    }

    #[test]
    fn test_decode_account_id_obj() {
        let encoded_account_id_obj = "811424A53BB5CAAD40A961836FEF648E8424846EC75A";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_account_id_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_signing_pub_key).unwrap(), field_lookup.field_map);
        let field_type = SerializedData::from(TypeCode::Blob).from_parser(parser, Some(encoded_signing_pub_key.len() / 2)).unwrap(); // NOTE: hardcoded hint size
        let str_val = field_type.to_string();
        assert_eq!(encoded_signing_pub_key, str_val);
    }

    #[test]
    fn test_decode_signing_pub_key_obj() {
        let encoded_signing_pub_key_obj = "732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_signing_pub_key_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_sig).unwrap(), field_lookup.field_map);
        let field_type = SerializedData::from(TypeCode::Blob).from_parser(parser, Some(encoded_tx_sig.len() / 2)).unwrap(); // NOTE: hardcoded hint size
        let str_val = field_type.to_string();
        assert_eq!(encoded_tx_sig, str_val);
    }

    #[test]
    fn test_decode_txn_signature_obj() {
        let encoded_tx_sig_obj = "74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_sig_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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

    //     let field_lookup = FieldLookup::from(create_field_info_map());
    //     let parser = &mut BinaryParser::new(hex::decode(encoded_tx_memos_arr).unwrap(), field_lookup.field_map.clone());
    //     let field_type = STArray::from_parser(parser, None).unwrap();
    //     let mut str_val = field_type.to_string();
    //     assert_eq!(encoded_tx_memos_arr, str_val);
    // }

    #[test]
    fn test_decode_memos_txn_obj() {
        let encoded_tx_obj = "F9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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

        let field_lookup = FieldLookup::from(create_field_info_map());
        let parser = &mut BinaryParser::new(hex::decode(encoded_tx_obj).unwrap(), field_lookup.field_map.clone());
        let field_type = SerializedData::from(TypeCode::Object).from_parser(parser, None).unwrap();
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
     