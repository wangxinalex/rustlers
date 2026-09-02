pub fn parse_pair(_input: &str) -> Result<(i32, i32), String> {
    // TODO: parse the two comma-separated integers.
    Err(String::from("pair is not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pair_accepts_two_integers() {
        assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    }

    #[test]
    fn parse_pair_rejects_a_missing_separator() {
        assert!(parse_pair("3").is_err());
    }

    #[test]
    fn parse_pair_rejects_a_non_numeric_value() {
        assert!(parse_pair("3,nope").is_err());
    }
}
