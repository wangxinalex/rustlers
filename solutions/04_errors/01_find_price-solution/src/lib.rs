pub fn find_price(prices: &[(&str, u32)], name: &str) -> Option<u32> {
    prices.iter().find(|item| item.0 == name).map(|item| item.1)
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
