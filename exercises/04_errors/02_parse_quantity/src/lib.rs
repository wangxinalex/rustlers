// Concept: parsing text into a positive unsigned integer.
// Task: implement `parse_quantity(input)` so it accepts positive numbers and rejects invalid text or zero.
// Expected behavior/output: `parse_quantity("7")` returns `Ok(7)`; `"many"` and `"0"` return `Err`.
// Hint: use `str::parse` with `?`, then check that the quantity is greater than zero.
pub fn parse_quantity(_input: &str) -> Result<u32, String> {
    // TODO: parse a positive quantity and explain invalid input.
    Err(String::from("quantity is not implemented"))
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
