// Concept: finding a word by borrowing a string slice.
// Task: implement `first_word(text)` so it skips leading whitespace and returns the first word.
// Expected behavior/output: `first_word("hello world")` returns `"hello"`; `first_word("  rust")` returns `"rust"`.
// Hint: split the borrowed text on whitespace and take the first item.
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
