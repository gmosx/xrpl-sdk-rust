use serde::{Deserialize, Serialize};
use thiserror::Error;

// #TODO: Connection

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    // #[error("failed request: {err}")]
    // FailedRequest { err: String, status: Option<u16> },
    // #[error("not authorized: missing api_credentials")]
    // Unauthorized,
    // #[error("api error: {0}")]
    // Api(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Internal(e.to_string())
        // Self::Internal {
        //     err: e.to_string(),
        //     status: e.status().map(|c| c.as_u16()),
        // }
    }
}
