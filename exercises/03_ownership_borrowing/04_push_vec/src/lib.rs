// Concept: mutable references and adding owned `String` values to a `Vec`.
// Task: implement `add_item(items, item)` so it appends the item to the vector.
// Expected behavior/output: adding `"tea"` to an empty vector leaves `["tea"]`.
// Hint: convert the borrowed `&str` to a `String`, then call `push`.
pub fn add_item(_items: &mut Vec<String>, _item: &str) {
    unimplemented!("append an owned item to the vector")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_item_appends_to_empty_vector() {
        let mut items = Vec::new();
        add_item(&mut items, "tea");
        assert_eq!(items, vec![String::from("tea")]);
    }
}
