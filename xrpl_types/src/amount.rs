use crate::Currency;
use serde::{Deserialize, Serialize};

/// Amount of XRP or issued token. See <https://xrpl.org/currency-formats.html#specifying-currency-amounts>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Amount {
    Issued(IssuedTokenAmount),
    Drops(String),
}

impl Default for Amount {
    fn default() -> Self {
        Amount::drops(0)
    }
}

impl Amount {
    pub fn issued(
        value: impl Into<String>,
        currency: impl Into<String>,
        issuer: impl Into<String>,
    ) -> Self {
        Self::Issued(IssuedTokenAmount::new(value, currency, issuer))
    }

    pub fn xrp(value: &str) -> Self {
        let value: f64 = value.parse().unwrap();
        Self::Drops(((value * 1_000_000.0) as u64).to_string())
    }

    pub fn drops(value: u64) -> Self {
        Self::Drops(value.to_string())
    }

    pub fn with_currency(value: &str, currency: &Currency) -> Self {
        match currency {
            Currency::Issued { name, issuer } => Self::issued(value, name, issuer),
            Currency::Xrp => Self::xrp(value),
        }
    }

    pub fn size(&self) -> f64 {
        match self {
            Amount::Drops(value) => value.parse::<f64>().unwrap() / 1_000_000.0,
            Amount::Issued(IssuedTokenAmount { value, .. }) => value.parse::<f64>().unwrap(),
        }
    }
}

/// Amount of issued token. See <https://xrpl.org/currency-formats.html#token-amounts>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct IssuedTokenAmount {
    pub value: String,
    pub currency: String,
    pub issuer: String,
}

impl IssuedTokenAmount {
    pub fn new(
        value: impl Into<String>,
        currency: impl Into<String>,
        issuer: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            currency: currency.into(),
            issuer: issuer.into(),
        }
    }
}
