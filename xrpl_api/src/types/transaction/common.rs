use crate::types::Meta;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use xrpl_types::LedgerTimestamp;

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionCommon {
    pub account: String,
    pub fee: String,
    pub sequence: u32,
    #[serde(rename = "AccountTxnID", skip_serializing_if = "Option::is_none")]
    pub account_txn_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ledger_sequence: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memos: Option<Vec<Memo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tag: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_pub_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_sequence: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_signature: Option<String>,

    /// Close time of the ledger in which the transaction is included
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<LedgerTimestamp>,

    /// Transaction hash
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    /// The ledger index of the ledger that includes this transaction.
    #[serde(rename = "ledger_index", skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<u32>,
    /// If true, this data comes from a validated ledger version; if omitted or
    /// set to false, this data is not final.
    #[serde(rename = "validated", skip_serializing_if = "Option::is_none")]
    pub validated: Option<bool>,

    /// Meta is present in transactions returned by https://xrpl.org/ledger.html and
    /// also <https://xrpl.org/tx.html>. In other API
    /// methods it is found outside (next to) the transaction field.
    #[serde(
        rename = "meta",
        alias = "metaData",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<Meta>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Memo {
    pub memo_data: Option<String>,
    pub memo_format: Option<String>,
    pub memo_type: Option<String>,
}

impl<'de> Deserialize<'de> for Memo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct MemoObjRaw {
            memo_data: Option<String>,
            memo_format: Option<String>,
            memo_type: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct MemoElmRaw {
            memo: MemoObjRaw,
        }

        let elm = MemoElmRaw::deserialize(deserializer)?;

        Ok(Memo {
            memo_data: elm.memo.memo_data,
            memo_format: elm.memo.memo_format,
            memo_type: elm.memo.memo_type,
        })
    }
}

impl Serialize for Memo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct MemoObjRaw<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            memo_data: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            memo_format: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            memo_type: Option<&'a str>,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct MemoElmRaw<'a> {
            memo: MemoObjRaw<'a>,
        }

        let elm = MemoElmRaw {
            memo: MemoObjRaw {
                memo_data: self.memo_data.as_deref(),
                memo_format: self.memo_format.as_deref(),
                memo_type: self.memo_type.as_deref(),
            },
        };

        elm.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::{Memo, TransactionCommon};

    fn remove_whitespace(s: &str) -> String {
        let mut s = s.to_string();
        s.retain(|c| !c.is_whitespace());
        s
    }

    #[test]
    fn test_deserialize_transaction_common() {
        let json = r#"
{
    "Account": "rMmTCjGFRWPz8S2zAUUoNVSQHxtRQD4eCx",
    "Sequence": 2,
    "Fee": "12",
    "Memos": [
        {
            "Memo": {
                "MemoData": "72656e74",
                "MemoType": "687474703a2f2f6578616d706c652e636f6d2f6d656d6f2f67656e65726963"
            }
        }
    ]
}
        "#;

        let tx: TransactionCommon = serde_json::from_str(json).unwrap();
        assert!(tx.memos.is_some());
        assert_eq!(tx.memos.as_ref().unwrap().len(), 1);
        assert_eq!(
            tx.memos.as_ref().unwrap()[0].memo_type.as_deref(),
            Some("687474703a2f2f6578616d706c652e636f6d2f6d656d6f2f67656e65726963")
        );
        assert_eq!(
            tx.memos.as_ref().unwrap()[0].memo_data.as_deref(),
            Some("72656e74")
        );
    }

    #[test]
    fn test_serialize_transaction_common() {
        let tx = TransactionCommon {
            account: "rMmTCjGFRWPz8S2zAUUoNVSQHxtRQD4eCx".to_string(),
            fee: "12".to_string(),
            sequence: 2,
            account_txn_id: None,
            last_ledger_sequence: None,
            memos: Some(vec![Memo {
                memo_data: Some("72656e74".to_string()),
                memo_format: None,
                memo_type: Some(
                    "687474703a2f2f6578616d706c652e636f6d2f6d656d6f2f67656e65726963".to_string(),
                ),
            }]),
            network_id: None,
            source_tag: None,
            signing_pub_key: None,
            ticket_sequence: None,
            txn_signature: None,
            date: None,
            hash: None,
            ledger_index: None,
            validated: None,
            meta: None,
        };

        let json = r#"
{
    "Account": "rMmTCjGFRWPz8S2zAUUoNVSQHxtRQD4eCx",
    "Fee": "12",
    "Sequence": 2,
    "Memos": [
        {
            "Memo": {
                "MemoData": "72656e74",
                "MemoType": "687474703a2f2f6578616d706c652e636f6d2f6d656d6f2f67656e65726963"
            }
        }
    ]
}
        "#;

        assert_eq!(serde_json::to_string(&tx).unwrap(), remove_whitespace(json));
    }
}
