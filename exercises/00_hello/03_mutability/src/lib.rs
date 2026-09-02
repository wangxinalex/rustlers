pub fn add_tax(price: i32) -> i32 {
    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tax_adds_eight_percent() {
        assert_eq!(add_tax(100), 108);
    }
}
