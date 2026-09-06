// Concept: comparing a small integer with an owned `String`.
// Task: implement `copy_and_move()` so it returns `(14, 4)` and shows both assignment behaviors.
// Expected behavior/output: `copy_and_move()` returns `(14, 4)`.
// Hint: copy `14`, then move a four-letter `String` and use the new binding.
pub fn copy_and_move() -> (i32, usize) {
    // TODO: repair this return line to show an i32 copy and a moved String.
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_move_returns_number_and_string_length() {
        assert_eq!(copy_and_move(), (14, 4));
    }
}
