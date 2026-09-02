pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_counts_whitespace_separated_words() {
        assert_eq!(word_count("one two two"), 3);
    }
}
