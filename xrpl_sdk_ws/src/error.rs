use serde::{Deserialize, Serialize};
use thiserror::Error;

// #TODO: Connection

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("malformed JSON payload: {0}")]
    MalformedJSON(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::MalformedJSON(e.to_string())
    }
}
