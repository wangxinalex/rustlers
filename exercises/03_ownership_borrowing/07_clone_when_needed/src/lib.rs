pub fn duplicate_for_two_places(_text: &str) -> (String, String) {
    // TODO: repair this return line so both places receive owned strings.
    (String::new(), String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_provides_two_owned_strings() {
        assert_eq!(
            duplicate_for_two_places("notes"),
            ("notes".into(), "notes".into())
        );
    }
}
