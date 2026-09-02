pub fn add_suffix(_text: &mut String, _suffix: &str) {
    // TODO: mutably borrow text and append suffix in place.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_borrow_appends_suffix() {
        let mut text = String::from("learn");

        add_suffix(&mut text, "ing");

        assert_eq!(text, "learning");
    }
}
