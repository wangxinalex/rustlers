// Concept: immutable bindings and using a function parameter.
// Task: implement `welcome_message(name)` so the supplied name appears in the message.
// Expected behavior/output: `welcome_message("Ada")` returns `"Welcome, Ada!"`.
// Hint: build the string with `format!` and include `name` in the message.
pub fn welcome_message(_name: &str) -> String {
    // TODO: use name in the welcome message.
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
