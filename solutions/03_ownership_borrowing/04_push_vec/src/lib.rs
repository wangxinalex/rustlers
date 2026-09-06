pub fn add_item(items: &mut Vec<String>, item: &str) {
    items.push(item.to_owned());
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
