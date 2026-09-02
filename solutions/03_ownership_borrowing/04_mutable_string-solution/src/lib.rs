pub fn add_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
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
