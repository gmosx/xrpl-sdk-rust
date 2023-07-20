use serde::Serialize;

/// An XRP Ledger currency. Can be either an Issued Currency (IOU) or the native
/// XRP digital asset.
#[derive(Clone, Serialize, Debug)]
pub enum Currency {
    Issued { name: String, issuer: String },
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

    pub fn issued(name: &str, issuer: &str) -> Self {
        Self::Issued {
            name: name.to_string(),
            issuer: issuer.to_string(),
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

// #TODO implement from/into.
/// Currency specification for books.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencySpec {
    pub currency: String, // TODO: hm, consider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl Default for CurrencySpec {
    fn default() -> Self {
        Self {
            currency: "XRP".to_string(),
            issuer: None,
        }
    }
}

impl CurrencySpec {
    pub fn from_currency(c: &Currency) -> Self {
        match c {
            Currency::Xrp => CurrencySpec {
                currency: "XRP".to_owned(),
                issuer: None,
            },
            Currency::Issued { name, issuer } => CurrencySpec {
                currency: name.clone(),
                issuer: Some(issuer.clone()),
            },
        }
    }
}
