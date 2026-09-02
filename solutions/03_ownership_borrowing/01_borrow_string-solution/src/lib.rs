pub fn length_after_borrow(text: &String) -> usize {
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
}
