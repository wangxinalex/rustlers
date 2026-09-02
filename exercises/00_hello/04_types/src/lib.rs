// Concept: explicit integer types and arithmetic with `u32` values.
// Task: implement `receipt_total(price, quantity)` by multiplying the two values.
// Expected behavior/output: `receipt_total(14, 3)` returns `42`.
// Hint: keep the receipt total as a `u32`; `price * quantity` is enough.
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
