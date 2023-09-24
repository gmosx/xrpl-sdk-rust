use crate::TransactionCommon;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::AccountSetTransactionFlags;

/// An `AccountSet` transaction <https://xrpl.org/accountset.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AccountSetTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<AccountSetTransactionFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_flag: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    #[serde(rename = "NFTokenMinter", skip_serializing_if = "Option::is_none")]
    pub nf_token_minter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_flag: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_size: Option<u32>,
}

#[cfg(test)]
mod test {
    use crate::AccountSetTransaction;

    #[test]
    fn test_account_set_deserialize() {
        let json = r#"
{
    "TransactionType": "AccountSet",
    "Account" : "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
    "Fee": "12",
    "Sequence": 5,
    "Domain": "6578616D706C652E636F6D",
    "SetFlag": 5,
    "MessageKey": "03AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB"
}
        "#;

        let _: AccountSetTransaction = serde_json::from_str(json).unwrap();
    }
}
