// Concept: function parameters and returned `String` values.
// Task: implement `greet(name)` so it returns a personalized greeting.
// Expected behavior/output: `greet("Lin")` returns `"Hello, Lin!"`.
// Hint: use the borrowed name when formatting the returned string.
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
