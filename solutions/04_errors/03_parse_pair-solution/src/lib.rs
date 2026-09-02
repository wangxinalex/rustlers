pub fn parse_pair(input: &str) -> Result<(i32, i32), String> {
    let (left, right) = input
        .split_once(',')
        .ok_or_else(|| String::from("pair must contain a comma"))?;
    let left = left
        .trim()
        .parse::<i32>()
        .map_err(|_| String::from("pair values must be integers"))?;
    let right = right
        .trim()
        .parse::<i32>()
        .map_err(|_| String::from("pair values must be integers"))?;

    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pair_accepts_two_integers() {
        assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    }

    #[test]
    fn parse_pair_rejects_a_missing_separator() {
        assert!(parse_pair("3").is_err());
    }

    #[test]
    fn parse_pair_rejects_a_non_numeric_value() {
        assert!(parse_pair("3,nope").is_err());
    }
}
