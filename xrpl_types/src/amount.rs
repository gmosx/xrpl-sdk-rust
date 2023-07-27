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
    pub fn issued(
        value: impl Into<String>,
        currency: impl Into<String>,
        issuer: impl Into<String>,
    ) -> Self {
        Self::Issued {
            value: value.into(),
            currency: currency.into(),
            issuer: issuer.into(),
        }
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
            Currency::Issued { currency, issuer } => Self::issued(value, currency, issuer),
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

#[cfg(test)]
mod test {
    use crate::Amount;
    use assert_matches::assert_matches;
    use serde::Serialize;

    #[test]
    fn test_serialize_drops() {
        let amount = Amount::drops(100);

        let mut v = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut v);
        amount.serialize(&mut serializer).unwrap();
        assert_eq!(r#""100""#, String::from_utf8(v).unwrap());
    }

    #[test]
    fn test_serialize_issued_amount() {
        let amount = Amount::issued("12.34", "USD", "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");

        let mut v = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut v);
        amount.serialize(&mut serializer).unwrap();
        assert_eq!(
            r#"{"value":"12.34","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}"#,
            String::from_utf8(v).unwrap()
        );
    }

    #[test]
    fn test_deserialize_drops() {
        let amount = serde_json::from_str(r#""100""#).unwrap();
        assert_matches!(amount, Amount::Drops(drops) => {
            assert_eq!(drops, "100");
        });
    }

    #[test]
    fn test_deserialize_issued_amount() {
        let amount = serde_json::from_str(
            r#"{"value":"12.34","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}"#,
        )
        .unwrap();
        assert_matches!(amount, Amount::Issued {value, currency, issuer} => {
            assert_eq!(value, "12.34");
            assert_eq!(currency, "USD");
            assert_eq!(issuer, "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");
        });
    }
}
