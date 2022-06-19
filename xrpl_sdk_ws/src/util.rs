// #TODO optimize this.
pub fn format_joined_keys(keys: &[&str]) -> String {
    keys.iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join(",")
}
