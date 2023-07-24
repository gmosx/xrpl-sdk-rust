use serde::Deserialize;

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
#[derive(Debug, Clone, Deserialize)]
pub struct AccountRoot {
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<String>,

    #[serde(rename = "Balance")]
    pub balance: Option<String>,

    #[serde(rename = "BurnedNFTokens")]
    pub burned_nf_tokens: Option<u32>,

    #[serde(rename = "Domain")]
    pub domain: Option<String>,

    #[serde(rename = "EmailHash")]
    pub email_hash: Option<String>,

    #[serde(rename = "FirstNFTokenSequence")]
    pub first_nf_token_sequence: Option<u32>,

    #[serde(rename = "Flags")]
    pub flags: u32,

    #[serde(rename = "MessageKey")]
    pub message_key: Option<String>,

    #[serde(rename = "MintedNFTokens")]
    pub minted_nf_tokens: Option<u32>,

    #[serde(rename = "NFTokenMinter")]
    pub nf_token_minter: Option<String>,

    #[serde(rename = "OwnerCount")]
    pub owner_count: u32,

    /// This field is mandatory on `AccountRoot` object, but we leave it optional, such
    /// that we can parse the object from `ModifiedNode` fields. See <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Option<String>,

/// This field is mandatory on `AccountRoot` object, but we leave it optional, such
    /// that we can parse the object from `ModifiedNode` fields. See <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: Option<u32>,

    #[serde(rename = "RegularKey")]
    pub regular_key: Option<String>,

    #[serde(rename = "Sequence")]
    pub sequence: u32,

    #[serde(rename = "TicketCount")]
    pub ticket_count: Option<u32>,

    #[serde(rename = "TickSize")]
    pub tick_size: Option<u8>,

    #[serde(rename = "TransferRate")]
    pub transfer_rate: Option<u32>,

    #[serde(rename = "WalletLocator")]
    pub wallet_locator: Option<String>,

    #[serde(rename = "WalletSize")]
    pub wallet_size: Option<u32>,

    pub index: Option<String>,
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

        let _account_root: AccoutRoot = serde_json::from_str(json).unwrap();
    }
}
