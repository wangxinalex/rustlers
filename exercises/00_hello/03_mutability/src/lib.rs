// Concept: mutable bindings with `let mut` and integer arithmetic.
// Task: implement `add_tax(price)` by adding eight percent tax to the price.
// Expected behavior/output: `add_tax(100)` returns `108`.
// Hint: add `price * 8 / 100` to a mutable price variable.
pub fn add_tax(price: i32) -> i32 {
    // TODO: calculate and return the price with eight percent tax.
    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tax_adds_eight_percent() {
        assert_eq!(add_tax(100), 108);
        assert_eq!(add_tax(200), 216);
    }
}
