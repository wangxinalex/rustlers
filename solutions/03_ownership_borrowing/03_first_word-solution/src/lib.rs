pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
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
