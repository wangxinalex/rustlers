// Concept: summing a slice and applying an integer percentage discount.
// Task: implement `shopping_total(prices, discount_percent)` so it returns the discounted total.
// Expected behavior/output: `shopping_total(&[100, 50], 10)` returns `135`.
// Hint: sum the prices first, then multiply by the remaining percentage and divide by `100`.
pub fn shopping_total(_prices: &[u32], _discount_percent: u32) -> u32 {
    // TODO: sum prices and apply the discount.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shopping_total_applies_discount() {
        assert_eq!(shopping_total(&[100, 50], 10), 135);
    }
}
