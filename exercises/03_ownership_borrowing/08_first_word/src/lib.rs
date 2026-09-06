// Concept: finding a word by borrowing a string slice.
// Task: skip leading whitespace and return the first word, or `""` if there is none.
// Expected behavior/output: `first_word("hello world")` returns `"hello"`; `first_word("  rust")` returns `"rust"`.
// Hint: `split_whitespace().next()` returns `Option<&str>`: `Some(word)`
// or `None`. Use `unwrap_or` with `""` for the missing word, not `unwrap()`.
// The word is already borrowed; no allocation or clone is needed.
pub fn first_word(_text: &str) -> &str {
    unimplemented!("return the first borrowed word")
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

    #[test]
    fn first_word_returns_empty_when_no_word_exists() {
        assert_eq!(first_word(""), "");
        assert_eq!(first_word(" \t\n"), "");
    }
}
