// Concept: looking up a value when the requested item may be absent.
// Task: implement `find_price(prices, name)` so it returns the matching price or `None`.
// Expected behavior/output: `find_price(&[("tea", 12)], "tea")` returns `Some(12)`; unknown names return `None`.
// Hint: use `.iter().find` to search the borrowed slice and map the tuple to its price.
pub fn find_price(_prices: &[(&str, u32)], _name: &str) -> Option<u32> {
    // TODO: find the requested item's price.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_price_returns_matching_price() {
        let prices = [("tea", 12), ("coffee", 20)];

        assert_eq!(find_price(&prices, "tea"), Some(12));
    }

    #[test]
    fn find_price_returns_none_for_unknown_name() {
        let prices = [("tea", 12), ("coffee", 20)];

        assert_eq!(find_price(&prices, "juice"), None);
    }
}
