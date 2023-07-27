use serde::{Serialize, Serializer};

/// An XRP Ledger currency. Can be either an Issued Currency (IOU) or the native
/// XRP digital asset. See <https://xrpl.org/currency-formats.html#specifying-without-amounts>
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Currency {
    Issued {
        /// Currency code, see <https://xrpl.org/currency-formats.html#currency-codes>
        currency: String,
        /// Issuer of token, see <https://xrpl.org/currency-formats.html#specifying-without-amounts>
        issuer: String,
    },
    Xrp,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::Xrp
    }
}

impl Currency {
    pub fn xrp() -> Self {
        Self::Xrp
    }

    pub fn issued(currency: impl Into<String>, issuer: impl Into<String>) -> Self {
        Self::Issued {
            currency: currency.into(),
            issuer: issuer.into(),
        }
    }

    pub fn is_xrp(&self) -> bool {
        match self {
            Self::Xrp => true,
            Self::Issued { .. } => false,
        }
    }

    pub fn is_issued(&self) -> bool {
        !self.is_xrp()
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Debug, Clone, Serialize)]
        struct CurrencyRaw<'a> {
            currency: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            issuer: Option<&'a str>,
        }

        impl<'a> CurrencyRaw<'a> {
            fn from(currency: &'a Currency) -> Self {
                match currency {
                    Currency::Xrp => CurrencyRaw {
                        currency: "XRP",
                        issuer: None,
                    },
                    Currency::Issued { currency, issuer } => CurrencyRaw {
                        currency: currency.as_str(),
                        issuer: Some(issuer.as_str()),
                    },
                }
            }
        }

        let currency_raw = CurrencyRaw::from(self);
        currency_raw.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::Currency;
    use serde::Serialize;

    #[test]
    fn test_serialize_xrp() {
        let currency = Currency::xrp();

        let mut v = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut v);
        currency.serialize(&mut serializer).unwrap();
        assert_eq!(r#"{"currency":"XRP"}"#, String::from_utf8(v).unwrap());
    }

    #[test]
    fn test_serialize_issued() {
        let currency = Currency::issued("USD", "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");

        let mut v = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut v);
        currency.serialize(&mut serializer).unwrap();
        assert_eq!(
            r#"{"currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}"#,
            String::from_utf8(v).unwrap()
        );
    }
}
