pub mod api;
pub mod client;
pub mod error;

pub use client::{Client, Result};
// pub use xrpl_api::api as api2;
pub use xrpl_api::api::*;
