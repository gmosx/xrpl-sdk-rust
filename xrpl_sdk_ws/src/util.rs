use crate::error::Error;

// #TODO move to `util`?
pub type Result<T> = std::result::Result<T, Error>;

// #TODO optimize this.
// pub fn format_joined_keys(keys: &[&str]) -> String {
//     keys.iter()
//         .map(|s| format!("\"{}\"", s))
//         .collect::<Vec<String>>()
//         .join(",")
// }
