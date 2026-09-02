// Concept: inspecting a `String` through a shared reference.
// Task: implement `length_after_borrow(text)` so it returns the borrowed string's length.
// Expected behavior/output: `length_after_borrow(&"rust".to_string())` returns `4`.
// Hint: use the borrowed string's `len` method.
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
