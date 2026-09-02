// Concept: string slices and iterators over whitespace-separated words.
// Task: implement `word_count(text)` so it counts the words in a string.
// Expected behavior/output: `word_count("one two two")` returns `3`.
// Hint: split the text on whitespace and count the resulting pieces.
pub fn word_count(_text: &str) -> usize {
    // TODO: count the whitespace-separated words.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_counts_whitespace_separated_words() {
        assert_eq!(word_count("one two two"), 3);
    }
}
