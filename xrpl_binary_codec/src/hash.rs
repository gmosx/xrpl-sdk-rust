use sha2::Digest;
use sha2::Sha512;
use xrpl_types::Hash256;

/// Unsigned single signer transactions prefix <https://xrpl.org/basic-data-types.html#hash-prefixes>
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

/// Signed transactions prefix <https://xrpl.org/basic-data-types.html#hash-prefixes>
pub const HASH_PREFIX_SIGNED_TRANSACTION: [u8; 4] = [0x54, 0x58, 0x4E, 0x00];

/// Calculate hash <https://xrpl.org/basic-data-types.html#hashes>
pub fn hash(prefix: [u8; 4], data: &[u8]) -> Hash256 {
    // INSIGHT: Sha512Trunc245 does not give same result as Sha512[0..32]
    let mut hasher = Sha512::new_with_prefix(prefix);
    hasher.update(data);
    let hash: [u8; 64] = hasher.finalize().into();
    Hash256(hash[0..32].try_into().expect("length 64"))
}
