// Concept: count lines, whitespace-separated words, and UTF-8 bytes.
// Task: implement `stats` for borrowed text.
// Expected behavior/output: `one two\nthree` has 2 lines, 3 words, and 13 bytes.
// Hint: use `lines().count()`, `split_whitespace().count()`, and `len()`.
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

pub fn stats(text: &str) -> TextStats {
    TextStats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        bytes: text.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_has_zero_stats() {
        let actual = stats("");

        assert_eq!(actual.lines, 0);
        assert_eq!(actual.words, 0);
        assert_eq!(actual.bytes, 0);
    }

    #[test]
    fn stats_count_lines_words_and_utf8_bytes() {
        let actual = stats("one two\nthree");

        assert_eq!(actual.lines, 2);
        assert_eq!(actual.words, 3);
        assert_eq!(actual.bytes, 13);
    }

    #[test]
    fn chinese_text_has_more_bytes_than_characters() {
        let actual = stats("你好");

        assert_eq!(actual.lines, 1);
        assert_eq!(actual.words, 1);
        assert_eq!(actual.bytes, "你好".len());
        assert!(actual.bytes > "你好".chars().count());
    }
}
