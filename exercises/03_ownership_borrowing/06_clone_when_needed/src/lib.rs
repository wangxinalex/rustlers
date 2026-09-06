// Concept: creating two owned strings from one owned `String`.
// Task: implement `duplicate_for_two_places(text)` so it returns two equal owned `String` values.
// Expected behavior/output: `duplicate_for_two_places(String::from("notes"))` returns `("notes", "notes")` as owned strings.
// Hint: move the input into one binding, then clone it because both returned values are needed.
pub fn duplicate_for_two_places(_text: String) -> (String, String) {
    unimplemented!("return two owned copies")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_provides_two_owned_strings() {
        assert_eq!(
            duplicate_for_two_places(String::from("notes")),
            ("notes".into(), "notes".into())
        );
        assert_eq!(
            duplicate_for_two_places(String::from("draft")),
            ("draft".into(), "draft".into())
        );
    }
}
