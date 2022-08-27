use crate::Currency;
use serde::{Deserialize, Serialize};

/// <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Amount {
    Issued {
        value: String,
        currency: String,
        issuer: String,
    },
    Drops(String),
}

impl Default for Amount {
    fn default() -> Self {
        Amount::drops(0)
    }
}

impl Amount {
    pub fn issued(value: &str, currency: &str, issuer: &str) -> Self {
        Self::Issued {
            value: value.to_string(),
            currency: currency.to_string(),
            issuer: issuer.to_string(),
        }
    }

    pub fn iou(value: &str, currency: &str, issuer: &str) -> Self {
        Self::issued(value, currency, issuer)
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
            Amount::Issued { value, .. } => value.parse::<f64>().unwrap(),
        }
    }
}
