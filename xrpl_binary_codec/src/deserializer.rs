use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use crate::error::BinaryCodecError;
use crate::serializer::{
    field_id::{FieldId, TypeCode},
    field_info::FieldInfo,
};
use xrpl_types::{
    AccountId, Amount, Blob, Hash128, Hash160, Hash256,
    UInt16, UInt32, UInt8, Uint64
};

#[cfg(feature = "json")]
use serde_json::Value;

mod constants {
    pub const OBJECT_NAME: &str = "STObject";
    pub const OBJECT_END_MARKER_NAME: &str = "ObjectEndMarker";
    pub const OBJECT_END_MARKER_BYTE: &[u8] = &[0xE1];

    pub const ARRAY_END_MARKER: &[u8] = &[0xf1];
    pub const ARRAY_END_MARKER_NAME: &str = "ArrayEndMarker";
    pub const OBJECT_END_MARKER_ARRAY: &[u8] = &[0xE1];
}

#[derive(Debug, Clone)]
pub struct FieldInstance {
    info: FieldInfo,
    name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Deserializer {
    cursor: Cursor<Vec<u8>>,
    field_ordinal_lookup: HashMap<u32, FieldInstance>,
}

impl Deserializer {
    pub fn new(bytes: Vec<u8>, field_info_map: &HashMap<String, FieldInfo>) -> Self {
        let mut field_ordinal_lookup = HashMap::new();
        for (name, info) in field_info_map {
            let ordinal = info.ordinal();
            let field = FieldInstance {
                info: info.clone(),
                name: name.clone(),
            };
            field_ordinal_lookup.insert(ordinal, field);
        }
        Self {
            cursor: Cursor::new(bytes),
            field_ordinal_lookup,
        }
    }

    #[allow(dead_code)]
    fn read(&mut self, n: usize) -> Result<Vec<u8>, BinaryCodecError> {
        let mut buf = vec![0; n];
        self.cursor.read_exact(&mut buf).map_err(|_| BinaryCodecError::InsufficientBytes("read".into()))?;
        Ok(buf)
    }

    fn read_u8(&mut self) -> Result<u8, BinaryCodecError> {
        let mut buf = [0u8; 1];
        self.cursor.read_exact(&mut buf).map_err(|_| BinaryCodecError::InsufficientBytes("read_u8".into()))?;
        Ok(buf[0])
    }

    fn read_variable_length(&mut self) -> Result<usize, BinaryCodecError> {
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
            Err(BinaryCodecError::InvalidLength("Invalid variable length indicator".into()))
        }
    }

    fn read_field_ordinal(&mut self) -> Result<u32, BinaryCodecError> {
        let mut type_code = self.read_u8()? as u32;
        let mut nth = type_code & 15;
        type_code >>= 4;
        if type_code == 0 {
            type_code = self.read_u8()? as u32;
            if type_code == 0 || type_code < 16 {
                return Err(BinaryCodecError::OutOfRange("FieldOrdinal, type_code out of range".into()));
            }
        }
        if nth == 0 {
            nth = self.read_u8()? as u32;
            if nth == 0 || nth < 16 {
                return Err(BinaryCodecError::OutOfRange("FieldOrdinal, type_code out of range".into()));
            }
        }
        Ok((type_code << 16) | nth)
    }

    fn read_field(&mut self) -> Result<FieldInstance, BinaryCodecError> {
        let ordinal = self.read_field_ordinal()?;
        self.field_ordinal_lookup
            .get(&ordinal)
            .cloned()
            .ok_or(BinaryCodecError::FieldNotFound("Field not found".into()))
    }

    fn read_field_value(&mut self, info: &FieldInfo) -> Result<Vec<u8>, BinaryCodecError> {
        let size_hint: Option<usize> = if info.is_vl_encoded {
            Some(self.read_variable_length()?)
        } else {
            None
        };
        let position = self.cursor.position() as usize;
        let _bytes = match info.field_type {
            TypeCode::Hash256 => self.deserialize_hash256()?.0.to_vec(),
            TypeCode::AccountId => self.deserialize_account_id()?.0.to_vec(),
            TypeCode::Blob => {
                let hint = size_hint.ok_or(BinaryCodecError::FieldNotFound("missing hint".into()))?;
                self.deserialize_blob(hint)?.0.to_vec()
            },
            TypeCode::Object => self.deserialize_object()?,
            TypeCode::Array => self.deserialize_array()?,
            _ => vec![], // TODO: default other types to Blob for now
        };
        let position_upd = self.cursor.position() as usize;
        // TODO: look and use bytes read instead? i.e return back tuple first and validate
        Ok(self.cursor.get_ref()[position..position_upd].to_vec())
        // Ok(bytes)
    }

    pub fn end(&mut self) -> bool {
        self.cursor.position() == self.cursor.get_ref().len() as u64
    }

    #[allow(dead_code)]
    #[cfg(feature = "json")]
    fn to_json(&mut self, type_code: &TypeCode, data: &[u8]) -> Result<Value, BinaryCodecError> {
        match type_code {
            TypeCode::Hash256 => Ok(Value::String(hex::encode_upper(&data))),
            TypeCode::AccountId => {
                let account_bytes: [u8; 20] = data.try_into().map_err(|_| BinaryCodecError::Overflow)?;
                Ok(Value::String(AccountId(account_bytes).to_address()))
            },
            TypeCode::Blob => Ok(Value::String(hex::encode_upper(&data))),
            TypeCode::Object => {
                let mut accumulator: HashMap<String, Value> = HashMap::new();
                self.cursor = Cursor::new(data.to_vec());
                while !self.end() {
                    let field: FieldInstance = self.read_field()?;
                    if field.name == constants::OBJECT_END_MARKER_NAME {
                        break;
                    }
                    let data_read = self.read_field_value(&field.info)?;
                    let json_value = self.to_json(&field.info.field_type, &data_read)?;
                    accumulator.insert(field.name, json_value);
                }
                Ok(Value::Object(accumulator.into_iter().collect()))
            },
            TypeCode::Array => {
                self.cursor = Cursor::new(data.to_vec());
                let mut result = Vec::new();
                while !self.end() {
                    let field = self.read_field()?;
                    if field.name == constants::ARRAY_END_MARKER_NAME {
                        break;
                    }
                    let data_read = self.read_field_value(&field.info)?;
                    let json_value = self.to_json(&field.info.field_type, &data_read)?;

                    let obj: serde_json::Map<String, Value> = vec![(field.name.clone(), json_value)].into_iter().collect();
                    result.push(Value::Object(obj));
                }
                Ok(Value::Array(result))
            },
            _ => Ok(Value::String(hex::encode_upper(&data))), // TODO: default other types to Blob for now
        }
    }
}

#[allow(dead_code)]
impl Deserializer {
    fn deserialize_account_id(&mut self) -> Result<AccountId, BinaryCodecError> {
        let mut bytes = [0u8; 20];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(AccountId(bytes))
    }

    fn deserialize_amount(&mut self) -> Result<Amount, BinaryCodecError> {
        unimplemented!()
    }

    fn deserialize_blob(&mut self, len: usize) -> Result<Blob, BinaryCodecError> {
        let mut bytes = vec![0u8; len]; 
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(Blob(bytes))
    }

    fn deserialize_hash128(&mut self) -> Result<Hash128, BinaryCodecError> {
        let mut bytes = [0u8; 16];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(Hash128(bytes))
    }

    fn deserialize_hash160(&mut self) -> Result<Hash160, BinaryCodecError> {
        let mut bytes = [0u8; 20];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(Hash160(bytes))
    }

    fn deserialize_hash256(&mut self) -> Result<Hash256, BinaryCodecError> {
        let mut bytes = [0u8; 32];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(Hash256(bytes))
    }

    fn deserialize_uint8(&mut self) -> Result<UInt8, BinaryCodecError> {
        let mut bytes = [0u8; 1];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(UInt8::from_be_bytes(bytes))
    }

    fn deserialize_uint16(&mut self) -> Result<UInt16, BinaryCodecError> {
        let mut bytes = [0u8; 2];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(UInt16::from_be_bytes(bytes))
    }

    fn deserialize_uint32(&mut self) -> Result<UInt32, BinaryCodecError> {
        let mut bytes = [0u8; 4];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(UInt32::from_be_bytes(bytes))
    }

    fn deserialize_uint64(&mut self) -> Result<Uint64, BinaryCodecError> {
        let mut bytes = [0u8; 8];
        self.cursor.read_exact(&mut bytes).map_err(|e| BinaryCodecError::InsufficientBytes(e.to_string()))?;
        Ok(Uint64::from_be_bytes(bytes))
    }

    fn deserialize_array(&mut self) -> Result<Vec<u8>, BinaryCodecError> {
        let mut bytes = Vec::new();
        while !self.end() {
            let field = self.read_field()?;
            if field.name == constants::ARRAY_END_MARKER_NAME {
                break;
            }
            let header: Vec<u8> = FieldId::from(field.info.clone()).into();
            bytes.extend_from_slice(&header);
            let data = self.read_field_value(&field.info)?;
            bytes.extend_from_slice(data.as_ref());
            bytes.extend_from_slice(constants::OBJECT_END_MARKER_ARRAY);
        }
        bytes.extend_from_slice(constants::ARRAY_END_MARKER);
        Ok(bytes)
    }

    fn deserialize_object(&mut self) -> Result<Vec<u8>, BinaryCodecError> {
        let mut sink: Vec<Vec<u8>> = Vec::new();
        while !self.end() {
            let field = self.read_field()?;
            if field.name == constants::OBJECT_END_MARKER_NAME {
                break;
            }
            let data = self.read_field_value(&field.info)?;
            sink.push(FieldId::from(field.info.clone()).into()); // push header
            if field.info.is_vl_encoded {
                let vl = encode_variable_length(data.len())?;
                sink.push(vl);
            }
            sink.push(data);
            if field.name == constants::OBJECT_NAME {
                sink.push(constants::OBJECT_END_MARKER_BYTE.to_vec());
            }
        }
        let concatenated_sink: Vec<u8> = sink.into_iter().flatten().collect();
        Ok(concatenated_sink)
    }
}

fn encode_variable_length(length: usize) -> Result<Vec<u8>, BinaryCodecError> {
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
        Err(BinaryCodecError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;
    use crate::serializer::field_info::field_info_lookup;

    #[test]
    fn test_decode_account_txn_id() {
        // AccountTxnID encoded
        let encoded_account_id = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C25";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_account_id).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_hash256().unwrap();
        let hex_val = hex::encode_upper(&data.0);
        assert_eq!(encoded_account_id, hex_val);
    }

    #[test]
    fn test_decode_account_txn_id_obj() {
        let encoded_account_id_obj = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_account_id_obj).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let hex_val = hex::encode_upper(&data);
        assert_eq!(encoded_account_id_obj, hex_val);

        let deserializer_2 = &mut Deserializer::new(data.clone(), field_info_lookup());
        let json_val = deserializer_2.to_json(&TypeCode::Object, &data).unwrap();
        let expected_val = r#"{
            "AccountTxnID": "16969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580"
        }"#;
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
    }

    #[test]
    fn test_decode_account_id() {
        // AccountID encoded
        let encoded_account_id = "811424A53BB5CAAD40A961836FEF648E8424846E";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_account_id).unwrap(), field_info_lookup());
        let account_id = deserializer.deserialize_account_id().unwrap();
        let want_acc_id = "rUmWJKM2b87GsKeTzSw14NeuubPQc8meTL";
        assert_eq!(want_acc_id, account_id.to_address());
    }

    #[test]
    fn test_decode_account_id_obj() {
        let encoded_account_id_obj = "811424A53BB5CAAD40A961836FEF648E8424846EC75A";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_account_id_obj).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let hex_val = hex::encode_upper(&data);
        assert_eq!(encoded_account_id_obj, hex_val);

        let deserializer = &mut Deserializer::new(hex::decode(encoded_account_id_obj).unwrap(), field_info_lookup());
        let json_val = deserializer.to_json(&TypeCode::Object, &data).unwrap();
        let expected_val = r#"{
            "Account": "rhLmGWkHr59h9ffYgPEAqZnqiQZMGb71yo"
        }"#;
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
    }

    #[test]
    fn test_decode_txn_signature() {
        // TxnSignature encoded
        let encoded_tx_sig = "74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CD";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_sig).unwrap(), field_info_lookup());
        let blob = deserializer.deserialize_blob(encoded_tx_sig.len() / 2).unwrap();
        let hex_val = hex::encode_upper(&blob.0);
        assert_eq!(encoded_tx_sig, hex_val);
    }

    #[test]
    fn test_decode_txn_signature_obj() {
        let encoded_tx_sig_obj = "74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_sig_obj).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let str_val = hex::encode_upper(&data);
        assert_eq!(encoded_tx_sig_obj, str_val);

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_sig_obj).unwrap(), field_info_lookup());
        let json_val = deserializer.to_json(&TypeCode::Object, &data).unwrap();
        let expected_val = r#"{
            "TxnSignature": "3045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC"
        }"#;
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
    }

    #[test]
    fn test_decode_memos_array() {
        let encoded_tx_memos_arr = "7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74F1E1F1F1E1F1";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_memos_arr).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let hex_val = hex::encode_upper(&data);
        assert_eq!(encoded_tx_memos_arr, hex_val);
    }

    #[test]
    fn test_decode_memos_txn_obj() {
        let encoded_tx_obj = "F9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_obj).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let hex_val = hex::encode_upper(&data);
        assert_eq!(encoded_tx_obj, hex_val);

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_obj).unwrap(), field_info_lookup());
        let json_val = deserializer.to_json(&TypeCode::Object, &data).unwrap();
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
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
        // assert!(false, "✅ testing failure; this is successful!");
    }

    #[test]
    fn test_custom_tx() {
        let encoded_obj = "EA7D0472656E74E1";

        let deserializer = &mut Deserializer::new(hex::decode(encoded_obj).unwrap(), field_info_lookup());
        let json_val = deserializer.to_json(&TypeCode::Object, &hex::decode(encoded_obj).unwrap()).unwrap();
        let expected_val = r#"{
            "Memo": {
                "MemoData": "72656E74"
            }
        }"#;
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
    }

    #[test]
    fn test_decode_txn_obj() {
        let encoded_tx_obj = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC811424A53BB5CAAD40A961836FEF648E8424846EC75AF9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";
        
        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_obj).unwrap(), field_info_lookup());
        let data = deserializer.deserialize_object().unwrap();
        let str_val = hex::encode_upper(&data);
        assert_eq!(encoded_tx_obj, str_val);

        let deserializer = &mut Deserializer::new(hex::decode(encoded_tx_obj).unwrap(), field_info_lookup());
        let json_val = deserializer.to_json(&TypeCode::Object, &data).unwrap();
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
        assert_eq!(from_str::<Value>(expected_val).unwrap(), json_val);
    }
}
