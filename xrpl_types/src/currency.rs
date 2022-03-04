use serde::Serialize;

/// An XRP Ledger currency. Can be either an Issued Currency (IOU) or the native
/// XRP digital asset.
#[derive(Clone, Serialize)]
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
