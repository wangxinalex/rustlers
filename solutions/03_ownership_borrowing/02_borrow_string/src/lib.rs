pub fn length_after_borrow(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_string_keeps_its_owner() {
        let text = "rust".to_string();

        assert_eq!(length_after_borrow(&text), 4);
        assert_eq!(text, "rust");
    }

    #[test]
    fn borrowed_literal_has_a_length() {
        assert_eq!(length_after_borrow("rust"), 4);
    }
}
