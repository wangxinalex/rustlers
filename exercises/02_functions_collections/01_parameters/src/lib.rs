pub fn greet(_name: &str) -> String {
    // TODO: include name in the greeting.
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_name() {
        assert_eq!(greet("Lin"), "Hello, Lin!");
    }
}
