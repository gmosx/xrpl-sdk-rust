use crate::Error;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct AccountId(pub [u8; 20]);

impl AccountId {
    /// Decodes account id from address, see <https://xrpl.org/accounts.html#address-encoding>
    pub fn from_address(address: &str) -> Result<Self, Error> {
        let decoded = bs58::decode(address)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .with_check(Some(0u8))
            .into_vec()
            .map_err(|err| Error::InvalidData(format!("invalid address: {}", err)))?;

        // Skip the 0x00 ('r') version prefix
        let decoded = &decoded[1..];

        let bytes: [u8; 20] = decoded.try_into().map_err(|_| {
            Error::InvalidData("address does not encode exactly 20 bytes".to_string())
        })?;

        Ok(Self(bytes))
    }

    /// Encodes account id to address, see <https://xrpl.org/accounts.html#address-encoding>
    pub fn to_address(&self) -> String {
        bs58::encode(&self.0)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .with_check_version(0u8) // Add the 0x00 ('r') version prefix
            .into_string()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Blob(pub Vec<u8>);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hash128(pub [u8; 16]);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hash160(pub [u8; 20]);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hash256(pub [u8; 32]);

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Uint64 = u64;

impl Hash128 {
    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        let decoded =
            hex::decode(hex).map_err(|err| Error::InvalidData(format!("invalid hex: {}", err)))?;

        let bytes: [u8; 16] = decoded.try_into().map_err(|_| {
            Error::InvalidData("address does not encode exactly 16 bytes".to_string())
        })?;

        Ok(Hash128(bytes))
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(self.0)
    }
}

impl Hash160 {
    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        let decoded =
            hex::decode(hex).map_err(|err| Error::InvalidData(format!("invalid hex: {}", err)))?;

        let bytes: [u8; 20] = decoded.try_into().map_err(|_| {
            Error::InvalidData("address does not encode exactly 20 bytes".to_string())
        })?;

        Ok(Hash160(bytes))
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(self.0)
    }
}

impl Hash256 {
    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        let decoded =
            hex::decode(hex).map_err(|err| Error::InvalidData(format!("invalid hex: {}", err)))?;

        let bytes: [u8; 32] = decoded.try_into().map_err(|_| {
            Error::InvalidData("address does not encode exactly 32 bytes".to_string())
        })?;

        Ok(Hash256(bytes))
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(self.0)
    }
}

impl Blob {
    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        let decoded =
            hex::decode(hex).map_err(|err| Error::InvalidData(format!("invalid hex: {}", err)))?;

        Ok(Blob(decoded))
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(&self.0)
    }
}

impl Debug for Hash128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0 {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

impl Debug for Hash160 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0 {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

impl Debug for Hash256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0 {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0 {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0 {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_account_id_from_address() {
        let account_id = AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap();
        assert_eq!(
            hex::encode(account_id.0),
            "4b4e9c06f24296074f7bc48f92a97916c6dc5ea9"
        );
    }

    #[test]
    fn test_account_id_from_address_invalid_checksum() {
        let result = AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpm");
        assert_matches!(result, Err(Error::InvalidData(message)) => {
           assert!(message.contains("invalid checksum"), "message: {message}")
        });
    }

    #[test]
    fn test_account_id_from_address_invalid_type_prefix() {
        let result = AccountId::from_address("XU8q4Ao1L1ggD6CAn9iA4oDoQZ7mXntZy");
        assert_matches!(result, Err(Error::InvalidData(message)) => {
           assert!(message.contains("invalid version"), "message: {message}")
        });
    }

    #[test]
    fn test_account_id_from_address_invalid_length() {
        let result = AccountId::from_address("r3wVnsK");
        assert_matches!(result, Err(Error::InvalidData(message)) => {
           assert!(message.contains("address does not encode exactly 20 bytes"), "message: {message}")
        });
    }

    #[test]
    fn test_account_id_debug() {
        let account_id = AccountId([
            0x4b, 0x4e, 0x9c, 0x06, 0xf2, 0x42, 0x96, 0x07, 0x4f, 0x7b, 0xc4, 0x8f, 0x92, 0xa9,
            0x79, 0x16, 0xc6, 0xdc, 0x5e, 0xa9,
        ]);
        assert_eq!(
            format!("{:?}", account_id),
            "4B4E9C06F24296074F7BC48F92A97916C6DC5EA9"
        );
        assert_eq!(
            format!("{:#?}", account_id),
            "0x4B4E9C06F24296074F7BC48F92A97916C6DC5EA9"
        );
    }

    #[test]
    fn test_account_id_to_address() {
        let account_id = AccountId(
            hex::decode("4b4e9c06f24296074f7bc48f92a97916c6dc5ea9")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        assert_eq!(
            account_id.to_address(),
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"
        );
    }

    #[test]
    fn test_hash128_from_hex() {
        let hash = Hash128::from_hex("A00000000000000000000000000000A1").unwrap();
        assert_eq!(
            hash.0,
            [0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1]
        );
    }

    #[test]
    fn test_hash128_to_hex() {
        let hex = Hash128([0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1]).to_hex();
        assert_eq!(hex, "A00000000000000000000000000000A1");
    }

    #[test]
    fn test_hash128_debug() {
        let hash = Hash128([0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1]);
        assert_eq!(format!("{:?}", hash), "A00000000000000000000000000000A1");
        assert_eq!(format!("{:#?}", hash), "0xA00000000000000000000000000000A1");
    }

    #[test]
    fn test_hash160_from_hex() {
        let hash = Hash160::from_hex("A0000000000000000000000000000000000000A1").unwrap();
        assert_eq!(
            hash.0,
            [0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1]
        );
    }

    #[test]
    fn test_hash160_to_hex() {
        let hex = Hash160([
            0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1,
        ])
        .to_hex();
        assert_eq!(hex, "A0000000000000000000000000000000000000A1");
    }

    #[test]
    fn test_hash160_debug() {
        let hash = Hash160([
            0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1,
        ]);
        assert_eq!(
            format!("{:?}", hash),
            "A0000000000000000000000000000000000000A1"
        );
        assert_eq!(
            format!("{:#?}", hash),
            "0xA0000000000000000000000000000000000000A1"
        );
    }

    #[test]
    fn test_hash256_from_hex() {
        let hash =
            Hash256::from_hex("A0000000000000000000000000000000000000000000000000000000000000A1")
                .unwrap();
        assert_eq!(
            hash.0,
            [
                0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0xA1
            ]
        );
    }

    #[test]
    fn test_hash256_to_hex() {
        let hex = Hash256([
            0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0xA1,
        ])
        .to_hex();
        assert_eq!(
            hex,
            "A0000000000000000000000000000000000000000000000000000000000000A1"
        );
    }

    #[test]
    fn test_hash256_debug() {
        let hash = Hash256([
            0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0xA1,
        ]);
        assert_eq!(
            format!("{:?}", hash),
            "A0000000000000000000000000000000000000000000000000000000000000A1"
        );
        assert_eq!(
            format!("{:#?}", hash),
            "0xA0000000000000000000000000000000000000000000000000000000000000A1"
        );
    }

    #[test]
    fn test_blob_from_hex() {
        let hash = Blob::from_hex("A00000A1").unwrap();
        assert_eq!(hash.0, [0xA0, 0, 0, 0xA1]);
    }

    #[test]
    fn test_blob_to_hex() {
        let hex = Blob(vec![0xA0, 0, 0, 0xA1]).to_hex();
        assert_eq!(hex, "A00000A1");
    }

    #[test]
    fn test_blob_debug() {
        let blob = Blob(vec![0xA0, 0, 0, 0xA1]);
        assert_eq!(format!("{:?}", blob), "A00000A1");
        assert_eq!(format!("{:#?}", blob), "0xA00000A1");
    }
}
