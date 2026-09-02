pub fn receipt_total(_price: u32, _quantity: u32) -> u32 {
    // TODO: multiply price by quantity.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_total_multiplies_price_and_quantity() {
        assert_eq!(receipt_total(14, 3), 42);
    }
}
