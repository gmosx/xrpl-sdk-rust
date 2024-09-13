use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

/// An account root in the ledger.
///
/// <https://xrpl.org/accountroot.html>
///
///{
///    "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
///    "AccountTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
///    "Balance": "148446663",
///    "Domain": "6D64756F31332E636F6D",
///    "EmailHash": "98B4375E1D753E5B91627516F6D70977",
///    "Flags": 8388608,
///    "LedgerEntryType": "AccountRoot",
///    "MessageKey": "0000000000000000000000070000000300",
///    "OwnerCount": 3,
///    "PreviousTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
///    "PreviousTxnLgrSeq": 14091160,
///    "Sequence": 336,
///    "TransferRate": 1004999999,
///    "index": "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
///}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRoot {
    pub account: String,
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<String>,
    #[serde(rename = "AMMID")]
    pub amm_id: Option<String>,
    pub balance: Option<String>,
    #[serde(rename = "BurnedNFTokens")]
    pub burned_nf_tokens: Option<u32>,
    pub domain: Option<String>,
    pub email_hash: Option<String>,
    #[serde(rename = "FirstNFTokenSequence")]
    pub first_nf_token_sequence: Option<u32>,
    pub flags: BitFlags<AccountRootFlags>,
    pub message_key: Option<String>,
    #[serde(rename = "MintedNFTokens")]
    pub minted_nf_tokens: Option<u32>,
    #[serde(rename = "NFTokenMinter")]
    pub nf_token_minter: Option<String>,
    pub owner_count: u32,
    pub regular_key: Option<String>,
    pub sequence: u32,
    pub ticket_count: Option<u32>,
    pub tick_size: Option<u8>,
    pub transfer_rate: Option<u32>,
    pub wallet_locator: Option<String>,
    pub wallet_size: Option<u32>,
    /// Declared optional since it is not part of transaction metadata fields <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Option<String>,
    /// Declared optional since it is not part of transaction metadata fields <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    pub previous_txn_lgr_seq: Option<u32>,
    #[serde(rename = "index")]
    pub index: Option<String>,
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountRootFlags {
    AllowTrustLineClawback = 0x80000000,
    DefaultRipple = 0x00800000,
    DepositAuth = 0x01000000,
    DisableMaster = 0x00100000,
    DisallowIncomingCheck = 0x08000000,
    DisallowIncomingNFTokenOffer = 0x04000000,
    DisallowIncomingPayChan = 0x10000000,
    DisallowIncomingTrustline = 0x20000000,
    DisallowXRP = 0x00080000,
    GlobalFreeze = 0x00400000,
    NoFreeze = 0x00200000,
    PasswordSpent = 0x00010000,
    RequireAuth = 0x00040000,
    RequireDestTag = 0x00020000,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_account_root() {
        let json = r#"
{
    "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
    "AccountTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
    "Balance": "148446663",
    "Domain": "6D64756F31332E636F6D",
    "EmailHash": "98B4375E1D753E5B91627516F6D70977",
    "Flags": 8388608,
    "LedgerEntryType": "AccountRoot",
    "MessageKey": "0000000000000000000000070000000300",
    "OwnerCount": 3,
    "PreviousTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
    "PreviousTxnLgrSeq": 14091160,
    "Sequence": 336,
    "TransferRate": 1004999999,
    "index": "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
}
"#;

        let _account_root: AccountRoot = serde_json::from_str(json).unwrap();
    }
}
