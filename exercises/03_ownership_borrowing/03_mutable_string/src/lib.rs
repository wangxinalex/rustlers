// Concept: changing a `String` through a mutable reference.
// Task: implement `add_suffix(text, suffix)` so it appends `suffix` to `text`.
// Expected behavior/output: starting with `"learn"`, the result is `"learning"`.
// Hint: use `push_str` through the mutable reference.
pub fn add_suffix(_text: &mut String, _suffix: &str) {
    unimplemented!("mutably borrow text and append suffix in place")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_borrow_appends_suffix() {
        let mut text = String::from("learn");

        add_suffix(&mut text, "ing");

        assert_eq!(text, "learning");
    }
}
