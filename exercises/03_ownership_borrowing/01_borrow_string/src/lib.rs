pub fn length_after_borrow(_text: &String) -> usize {
    // TODO: borrow text and return its length without taking ownership.
    0
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
