// Concept: constructing an owned `String` from a borrowed string slice.
// Task: implement `build_greeting(name)` so it returns `"Hello, {name}!"`.
// Expected behavior/output: `build_greeting("Mia")` returns `"Hello, Mia!"`.
// Hint: format a new owned `String` from `name`.
pub fn build_greeting(_name: &str) -> String {
    unimplemented!("return an owned greeting")
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
