pub fn add_tax(price: i32) -> i32 {
    let mut total = price;
    total += total * 8 / 100;
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tax_adds_eight_percent() {
        assert_eq!(add_tax(100), 108);
    }
}
