pub fn duplicate_for_two_places(text: &str) -> (String, String) {
    let first = text.to_owned();
    let second = first.clone();

    (first, second)
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
