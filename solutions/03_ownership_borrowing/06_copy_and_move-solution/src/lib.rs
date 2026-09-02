pub fn copy_and_move() -> (i32, usize) {
    let number = 14;
    let copied_number = number;
    let _original_number = number;

    let text = String::from("move");
    let moved_text = text;

    (copied_number, moved_text.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_move_returns_number_and_string_length() {
        assert_eq!(copy_and_move(), (14, 4));
    }
}
