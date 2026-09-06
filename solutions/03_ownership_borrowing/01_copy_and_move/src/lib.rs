pub fn copy_and_move() -> (i32, usize) {
    let number = 14;
    let _copied_number = number;
    let number_still_usable = number;

    let text = String::from("move");
    let moved_text = text;

    (number_still_usable, moved_text.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_move_returns_number_and_string_length() {
        assert_eq!(copy_and_move(), (14, 4));
    }
}
