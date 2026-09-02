pub fn greeting() -> &'static str {
    "Hello, Rustlers!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_the_rustlers_message() {
        assert_eq!(greeting(), "Hello, Rustlers!");
    }
}
