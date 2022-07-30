use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    // #[error("failed request: {err}")]
    // FailedRequest { err: String, status: Option<u16> },
    // #[error("not authorized: missing api_credentials")]
    // Unauthorized,
    #[error("format error: {0}")]
    Format(String),
    #[error("api error: {0}")]
    Api(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Format(e.to_string())
    }
}
