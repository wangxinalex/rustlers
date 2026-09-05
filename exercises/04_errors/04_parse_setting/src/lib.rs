// Concept: validating a small `key=value` configuration line.
// Task: implement `parse_setting(line)` for the required `port` key and numeric value.
// Expected behavior/output: `parse_setting("port=8080")` returns `Ok(("port", 8080))`; wrong keys or missing values return `Err`.
// Hint: convert `split_once('=')` from `Option` to `Result` with `ok_or`
// and a `String` error before `?`, then check for `"port"`. Parse as `u32`
// and convert `ParseIntError` to `String` with `map_err` before the next `?`.
pub fn parse_setting(_line: &str) -> Result<(&str, u32), String> {
    // TODO: parse and validate a port setting.
    Err(String::from("setting is not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_setting_accepts_the_port_setting() {
        assert_eq!(parse_setting("port=8080"), Ok(("port", 8080)));
    }

    #[test]
    fn parse_setting_rejects_a_wrong_key() {
        assert!(parse_setting("host=8080").is_err());
    }

    #[test]
    fn parse_setting_rejects_a_missing_value() {
        assert!(parse_setting("port=").is_err());
    }
}
