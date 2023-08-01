use crate::AccountId;

/// Amount of XRP or issued token. See <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone)] // todo allan Copy
pub enum Amount {
    Issued(IssuedAmount),
    Drops(u64),
}

impl Amount {
    pub fn issued(
        value: impl Into<String>,
        currency: impl Into<String>,
        issuer: impl Into<String>,
    ) -> Self {
        Self::Issued(IssuedAmount::new(value, currency, issuer))
    }

    pub fn drops(value: u64) -> Self {
        Self::Drops(value)
    }
}

/// Amount of issued token. See <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IssuedAmount {
    /// Decimal representation of token amount, see <https://xrpl.org/serialization.html#amount-fields>
    pub value: String,
    /// Currency code, see <https://xrpl.org/serialization.html#amount-fields>
    pub currency: String,
    /// Issuer of token, see <https://xrpl.org/serialization.html#amount-fields>
    pub issuer: AccountId,
}

impl IssuedAmount {
    pub fn new(
        value: impl Into<String>,
        currency: impl Into<String>,
        issuer: impl Into<String>,
    ) -> Self {
        todo!()
        // Self {
        //     value: value.into(),
        //     currency: currency.into(),
        //     issuer: issuer.into(),
        // }// todo allan
    }
}
