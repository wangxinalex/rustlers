// Concept: inspecting text through a shared `&str` reference.
// Task: implement `length_after_borrow(text)` so it returns the borrowed text length.
// Expected behavior/output: both a `String` and a string literal produce their lengths.
// Hint: this function needs text data, not a specific owned allocation.
pub fn length_after_borrow(_text: &str) -> usize {
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

    #[test]
    fn borrowed_literal_has_a_length() {
        assert_eq!(length_after_borrow("rust"), 4);
    }
}
