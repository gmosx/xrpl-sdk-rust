//! The random command provides a random number to be used as a source of
//! entropy for random number generation by clients.
//!
//! <https://xrpl.org/random.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct RandomRequest {}

impl Request for RandomRequest {
    type Response = RandomResponse;

    fn method(&self) -> String {
        "random".to_owned()
    }
}

impl RandomRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Deserialize)]
pub struct RandomResponse {
    /// Random 256-bit hex value.
    pub random: String,
}

// #TODO add client.random().await? helper
