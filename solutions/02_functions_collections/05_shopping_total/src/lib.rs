pub fn shopping_total(prices: &[u32], discount_percent: u32) -> u32 {
    let subtotal: u32 = prices.iter().sum();
    subtotal * (100 - discount_percent) / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shopping_total_applies_discount() {
        assert_eq!(shopping_total(&[100, 50], 10), 135);
    }
}
