use crate::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AccountId(pub [u8; 20]);

impl AccountId {
    /// Decodes account id from address, see <https://xrpl.org/accounts.html#address-encoding>
    pub fn from_address(address: &str) -> Result<Self, Error> {
        let decoded = bs58::decode(address)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .with_check(Some(0u8))
            .into_vec()
            .map_err(|err| Error::InvalidData(format!("Invalid address: {}", err)))?;

        // Skip the 0x00 ('r') version prefix
        let decoded = &decoded[1..];

        let bytes: [u8; 20] = decoded.try_into().map_err(|_| {
            Error::InvalidData("Address does not encode exactly 20 bytes".to_string())
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

#[derive(Debug, Clone)]
pub struct Blob(pub Vec<u8>);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash128(pub [u8; 16]);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash160(pub [u8; 20]);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash256(pub [u8; 32]);

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Uint64 = u64;

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
           assert!(message.contains("Address does not encode exactly 20 bytes"), "message: {message}")
        });
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
}
