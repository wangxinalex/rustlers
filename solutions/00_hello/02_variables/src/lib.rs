pub fn welcome_message(name: &str) -> String {
    let message = format!("Welcome, {name}!");
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_message_includes_name() {
        assert_eq!(welcome_message("Ada"), "Welcome, Ada!");
    }
}
