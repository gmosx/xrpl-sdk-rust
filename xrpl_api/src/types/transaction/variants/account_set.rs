use crate::TransactionCommon;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::AccountSetTransactionFlags;

/// An `AccountSet` transaction <https://xrpl.org/accountset.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountSetTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<AccountSetTransactionFlags>,
    pub clear_flag: Option<u32>,
    pub domain: Option<String>,
    pub email_hash: Option<String>,
    pub message_key: Option<String>,
    #[serde(rename = "NFTokenMinter")]
    pub nf_token_minter: Option<String>,
    pub set_flag: Option<u32>,
    pub transfer_rate: Option<u32>,
    pub tick_size: Option<u8>,
    pub wallet_locator: Option<String>,
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
