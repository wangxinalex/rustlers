pub fn add_item(_items: &mut Vec<String>, _item: &str) {
    // TODO: append item to the vector.
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
