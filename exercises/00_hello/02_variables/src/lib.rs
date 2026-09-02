pub fn welcome_message(_name: &str) -> String {
    String::from("Welcome!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_message_includes_name() {
        assert_eq!(welcome_message("Ada"), "Welcome, Ada!");
    }
}
