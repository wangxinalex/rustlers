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
