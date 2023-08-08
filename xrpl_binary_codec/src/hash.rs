use sha2::Sha512;
use sha2::Digest;
use xrpl_types::Hash256;

/// Calculate hash <https://xrpl.org/basic-data-types.html#hashes>
pub fn hash(prefix: [u8; 4], data: &[u8]) -> Hash256 {
    // INSIGHT: Sha512Trunc245 does not give same result as Sha512[0..32]
    let mut hasher = Sha512::new_with_prefix(&prefix);
    hasher.update(data);
    let hash: [u8; 64] = hasher.finalize().into();
    Hash256(hash[0..32].try_into().expect("length 64"))
}
