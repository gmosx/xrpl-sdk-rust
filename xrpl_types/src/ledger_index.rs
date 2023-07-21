use serde::Serialize;

/// Specification of ledger.
/// See <<https://xrpl.org/basic-data-types.html#specifying-ledgers>
#[derive(Debug, Clone)]
pub enum LedgerSpec {
    LedgerIndex(LedgerIndex),
    LedgerHash(String),
}

/// Specification of ledger by the `ledger_index` property.
/// See <<https://xrpl.org/basic-data-types.html#specifying-ledgers>
#[derive(Debug, Clone, Copy)]
pub enum LedgerIndex {
    Validated,
    Closed,
    Current,
    Index(u32),
}

impl Serialize for LedgerIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Encodes the specification as `ledger_index` property value,
        // see <https://xrpl.org/basic-data-types.html#specifying-ledgers>.
        match self {
            LedgerIndex::Validated => serializer.serialize_str("validated"),
            LedgerIndex::Closed => serializer.serialize_str("closed"),
            LedgerIndex::Current => serializer.serialize_str("current"),
            LedgerIndex::Index(index) => serializer.serialize_u32(*index),
        }
    }
}
