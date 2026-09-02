pub fn first_word(_text: &str) -> &str {
    // TODO: return the first word while borrowing from text.
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_stops_at_whitespace() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn first_word_skips_leading_whitespace() {
        assert_eq!(first_word("  rust"), "rust");
    }
}
