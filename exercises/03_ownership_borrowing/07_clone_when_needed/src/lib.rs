// Concept: creating two owned strings from one borrowed string slice.
// Task: implement `duplicate_for_two_places(text)` so it returns two equal owned `String` values.
// Expected behavior/output: `duplicate_for_two_places("notes")` returns `("notes", "notes")` as owned strings.
// Hint: create one owned string, then clone it because both returned values are needed.
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
