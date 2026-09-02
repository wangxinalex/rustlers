pub fn text_lengths(text: &str) -> (usize, usize) {
    (text.chars().count(), text.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_text_reports_chars_and_bytes() {
        assert_eq!(text_lengths("你好"), (2, 6));
    }

    #[test]
    fn ascii_text_keeps_the_two_counts_equal() {
        assert_eq!(text_lengths("rust"), (4, 4));
    }
}
