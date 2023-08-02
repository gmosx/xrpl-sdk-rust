use crate::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AccountId(pub [u8; 20]);

impl AccountId {
    /// Decodes account id from address, see <https://xrpl.org/accounts.html#address-encoding>
    pub fn from_address(address: &str) -> Result<Self, Error> {
        let decoded = bs58::decode(address)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_vec()
            .unwrap(); // TODO allan

        // TODO check checksum

        // Skip the 0x00 ('r') version prefix, skip the 4-byte checksum postfix.
        let decoded = &decoded[1..21];

        Ok(Self(decoded.try_into().unwrap())) // TODO allan
    }

    /// Encodes account id to address, see <https://xrpl.org/accounts.html#address-encoding>
    pub fn to_address(&self) -> String {
        let encoded = bs58::encode(&self.0)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_string();
        // When serializing we skipped the ('r') version prefix and the 4-byte checksum postfix.
        let account_id = format!("{}{}", String::from("r"), encoded);
        // TODO allan
        let re_decoded = bs58::decode(account_id)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_vec()
            .expect("decoding encoded with prefix from alphabet");
        let re_encoded = bs58::encode(re_decoded)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .with_check()
            .into_string();
        re_encoded
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
    use crate::AccountId;

    #[test]
    fn test_account_id_from_address() {
        let account_id = AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap();
        assert_eq!(
            hex::encode(&account_id.0),
            "4b4e9c06f24296074f7bc48f92a97916c6dc5ea9"
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
}
