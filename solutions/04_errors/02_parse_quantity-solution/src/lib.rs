pub fn parse_quantity(input: &str) -> Result<u32, String> {
    let quantity = input
        .parse::<u32>()
        .map_err(|_| String::from("quantity must be a positive number"))?;

    if quantity == 0 {
        Err(String::from("quantity must be greater than zero"))
    } else {
        Ok(quantity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quantity_accepts_a_positive_number() {
        assert_eq!(parse_quantity("7"), Ok(7));
    }

    #[test]
    fn parse_quantity_rejects_invalid_input() {
        assert!(parse_quantity("many").is_err());
    }

    #[test]
    fn parse_quantity_rejects_zero() {
        assert!(parse_quantity("0").is_err());
    }
}
