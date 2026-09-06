pub fn build_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_owns_its_returned_string() {
        assert_eq!(build_greeting("Mia"), "Hello, Mia!");
        assert_eq!(build_greeting("Noa"), "Hello, Noa!");
    }
}
