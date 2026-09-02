pub fn text_lengths(_text: &str) -> (usize, usize) {
    // TODO: return Unicode scalar-value count first and UTF-8 byte count second.
    (0, 0)
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
