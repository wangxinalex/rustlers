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
