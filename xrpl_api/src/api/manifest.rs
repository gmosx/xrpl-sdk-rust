use crate::Request;
use serde::{Deserialize, Serialize};

/// The manifest method reports the current "manifest" information for a given
/// validator public key. The "manifest" is the public portion of that
/// validator's configured token. Updated in: rippled 1.7.0
///
/// -https://xrpl.org/manifest.html
#[derive(Default, Clone, Serialize)]
pub struct ManifestRequest {
    public_key: String,
}

impl Request for ManifestRequest {
    type Response = ManifestResponse;

    fn method(&self) -> String {
        "manifest".to_owned()
    }
}

impl ManifestRequest {
    pub fn new(public_key: &str) -> Self {
        Self {
            public_key: public_key.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Details {
    /// The domain name this validator claims to be associated with. If the
    /// manifest does not contain a domain, this is an empty string.
    pub domain: String,
    /// The ephemeral public key for this validator, in base58.
    pub ephemeral_key: String,
    /// The master public key for this validator, in base58.
    pub master_key: String,
    /// The sequence number of this manifest. This number increases whenever
    /// the validator operator updates the validator's token to rotate ephemeral
    /// keys or change settings.
    pub seq: u32,
}

#[derive(Debug, Deserialize)]
pub struct ManifestResponse {
    /// The data contained in this manifest. Omitted if the server does not have a manifest for the public_key from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
    /// The full manifest data in base64 format. This data is serialized to
    /// binary before being base64-encoded. Omitted if the server does not have
    /// a manifest for the public_key from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    /// The public_key from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
}
