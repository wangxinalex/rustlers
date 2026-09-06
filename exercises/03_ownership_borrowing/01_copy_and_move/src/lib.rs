/// ```compile_fail
/// let text = String::from("move");
/// let moved_text = text;
/// println!("{text}");
/// println!("{moved_text}");
/// ```
pub fn copy_and_move() -> (i32, usize) {
    unimplemented!("show an i32 copy and a moved String")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_move_returns_number_and_string_length() {
        assert_eq!(copy_and_move(), (14, 4));
    }
}
