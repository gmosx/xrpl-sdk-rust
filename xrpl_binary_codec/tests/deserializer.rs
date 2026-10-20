//! Tests for the public `Deserializer` API, used the way an external crate would use it.
//!
//! The test vectors are taken from the unit tests in `src/deserializer.rs`.

use xrpl_binary_codec::deserializer::Deserializer;
use xrpl_binary_codec::serializer::field_info::field_info_lookup;
use xrpl_binary_codec::BinaryCodecError;

/// `SigningPubKey` field: header `0x73`, length prefix `0x21` (33), then the public key.
const SIGNING_PUB_KEY_FIELD: &str =
    "732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A";

/// `TxnSignature` value (71 bytes), without the field header and length prefix.
const TXN_SIGNATURE: &str = "3045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC";

/// `Account` field: header `0x81`, length prefix `0x14` (20), then the account ID.
const ACCOUNT_FIELD: &str = "811424A53BB5CAAD40A961836FEF648E8424846EC75A";

/// Transaction fields `AccountTxnID`, `SigningPubKey`, `TxnSignature`, `Account` and `Memos`.
const TXN_FIELDS: &str = "5916969036626990000000000000000000F236FD752B5E4C84810AB3D41A3C2580732102A6934E87988466B98B51F2EB09E5BC4C09E46EB5F1FE08723DF8AD23D5BB9C6A74473045022100FB7583772B8F348F4789620C5571146B6517887AC231B38E29D7688D73F9D2510220615DC87698A2BA64DF2CA83BD9A214002F74C2D615CA20E328AC4AB5E4CDE8BC811424A53BB5CAAD40A961836FEF648E8424846EC75AF9EA7C1F687474703A2F2F6578616D706C652E636F6D2F6D656D6F2F67656E657269637D0472656E74E1F1";

fn deserializer(hex_bytes: &str) -> Deserializer {
    Deserializer::new(hex::decode(hex_bytes).unwrap(), field_info_lookup())
}

#[test]
fn deserialize_blob_reads_the_requested_number_of_bytes() {
    let mut deserializer = deserializer(&format!("7447{TXN_SIGNATURE}"));

    let header = deserializer.deserialize_blob(2).unwrap();
    assert_eq!(header.0, [0x74, 0x47]);

    let signature = deserializer.deserialize_blob(71).unwrap();
    assert_eq!(hex::encode_upper(signature.0), TXN_SIGNATURE);
    assert!(deserializer.end());
}

#[test]
fn deserialize_blob_fails_on_insufficient_bytes() {
    let mut deserializer = deserializer("0102");

    let result = deserializer.deserialize_blob(3);
    assert!(
        matches!(result, Err(BinaryCodecError::InsufficientBytes(_))),
        "{result:?}"
    );
}

#[test]
fn deserialize_object_returns_the_serialized_fields() {
    let mut deserializer = deserializer(TXN_FIELDS);

    let bytes = deserializer.deserialize_object().unwrap();
    assert_eq!(hex::encode_upper(bytes), TXN_FIELDS);
    assert!(deserializer.end());
}

#[test]
fn deserialize_object_stops_after_the_object_end_marker() {
    // `SigningPubKey`, then `ObjectEndMarker` (0xE1), then two more bytes.
    let mut deserializer = deserializer(&format!("{SIGNING_PUB_KEY_FIELD}E1ABCD"));

    let bytes = deserializer.deserialize_object().unwrap();
    assert_eq!(hex::encode_upper(bytes), SIGNING_PUB_KEY_FIELD);

    let rest = deserializer.deserialize_blob(2).unwrap();
    assert_eq!(rest.0, [0xAB, 0xCD]);
    assert!(deserializer.end());
}

#[test]
fn deserialize_object_of_empty_input_is_empty() {
    let mut deserializer = Deserializer::new(Vec::new(), field_info_lookup());

    let bytes = deserializer.deserialize_object().unwrap();
    assert!(bytes.is_empty());
}

#[test]
fn deserialize_object_fails_on_unknown_field() {
    let mut fields = field_info_lookup().clone();
    fields.remove("Account");
    let mut deserializer = Deserializer::new(hex::decode(ACCOUNT_FIELD).unwrap(), &fields);

    let result = deserializer.deserialize_object();
    assert!(
        matches!(result, Err(BinaryCodecError::FieldNotFound(_))),
        "{result:?}"
    );
}

#[test]
fn deserialize_object_fails_on_truncated_input() {
    // The length prefix announces 71 bytes, but only 70 follow.
    let field = format!("7447{TXN_SIGNATURE}");
    let mut deserializer = deserializer(&field[..field.len() - 2]);

    let result = deserializer.deserialize_object();
    assert!(
        matches!(result, Err(BinaryCodecError::InsufficientBytes(_))),
        "{result:?}"
    );
}

#[cfg(feature = "json")]
#[test]
fn deserialize_object_output_converts_to_json() {
    use xrpl_binary_codec::serializer::field_id::TypeCode;

    let mut deserializer = deserializer(TXN_FIELDS);
    let bytes = deserializer.deserialize_object().unwrap();

    let json = deserializer.to_json(&TypeCode::Object, &bytes).unwrap();
    assert_eq!(json["Account"], "rhLmGWkHr59h9ffYgPEAqZnqiQZMGb71yo");
    assert_eq!(json["TxnSignature"], TXN_SIGNATURE);
    assert_eq!(json["Memos"][0]["Memo"]["MemoData"], "72656E74");
}
