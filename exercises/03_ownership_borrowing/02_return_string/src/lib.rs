pub fn build_greeting(_name: &str) -> String {
    // TODO: build and return an owned greeting.
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_owns_its_returned_string() {
        assert_eq!(build_greeting("Mia"), "Hello, Mia!");
    }
}
