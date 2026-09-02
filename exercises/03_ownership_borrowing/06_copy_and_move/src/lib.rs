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
