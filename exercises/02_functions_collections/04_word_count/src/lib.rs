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
