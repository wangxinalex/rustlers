pub fn greeting() -> &'static str {
    // TODO: return the greeting required by the exercise.
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_the_rustlers_message() {
        assert_eq!(greeting(), "Hello, Rustlers!");
    }
}
