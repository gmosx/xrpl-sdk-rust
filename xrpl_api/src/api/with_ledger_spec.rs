use crate::{Request};
use serde::{Deserialize, Serialize};
use xrpl_types::{LedgerIndex, LedgerSpec};

/// Request that allows specifying ledger index to execute
/// request on. See <https://xrpl.org/basic-data-types.html#specifying-ledgers>.
pub trait RequestWithLedgerSpec: Request {
    fn as_ledger_index(&self) -> &LedgerSpecRequestFragment;
    fn as_ledger_index_mut(&mut self) -> &mut LedgerSpecRequestFragment;

    fn ledger_index(mut self, ledger_index: LedgerIndex) -> Self
    where
        Self: Sized,
    {
        self.as_ledger_index_mut().ledger_index = Some(ledger_index);
        self
    }

    fn ledger_hash(mut self, ledger_hash: String) -> Self
    where
        Self: Sized,
    {
        self.as_ledger_index_mut().ledger_hash = Some(ledger_hash);
        self
    }

    fn ledger(self, ledger: LedgerSpec) -> Self
    where
        Self: Sized,
    {
        match ledger {
            LedgerSpec::LedgerIndex(ledger_index) => self.ledger_index(ledger_index),
            LedgerSpec::LedgerHash(ledger_hash) => self.ledger_hash(ledger_hash),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerSpecRequestFragment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<LedgerIndex>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LedgerSpecResponseFragment {
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<u32>,
    pub ledger_current_index: Option<u32>,
    pub validated: Option<bool>,
}
