pub fn receipt_total(price: u32, quantity: u32) -> u32 {
    let total: u32 = price * quantity;
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_total_multiplies_price_and_quantity() {
        assert_eq!(receipt_total(14, 3), 42);
    }
}
