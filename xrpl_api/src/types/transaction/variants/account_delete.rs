use crate::TransactionCommon;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::AccountDeleteFlags;

/// An `AccountDelete` transaction <https://xrpl.org/accountdelete.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AccountDeleteTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<AccountDeleteFlags>,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<u32>,
}

#[cfg(test)]
mod test {
    use crate::AccountDeleteTransaction;

    #[test]
    fn test_account_delete_deserialize() {
        let json = r#"
{
    "TransactionType": "AccountDelete",
    "Account": "rWYkbWkCeg8dP6rXALnjgZSjjLyih5NXm",
    "Destination": "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe",
    "DestinationTag": 13,
    "Fee": "2000000",
    "Sequence": 2470665,
    "Flags": 2147483648
}
        "#;

        let _: AccountDeleteTransaction = serde_json::from_str(json).unwrap();
    }
}
