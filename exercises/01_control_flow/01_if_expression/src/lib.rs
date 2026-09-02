pub fn classify_temperature(celsius: i32) -> &'static str {
    // TODO: make the middle branch return "mild".
    if celsius < 10 {
        "cold"
    } else if celsius < 25 {
        "cold"
    } else {
        "hot"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_temperature_has_cold_mild_and_hot_branches() {
        assert_eq!(classify_temperature(-2), "cold");
        assert_eq!(classify_temperature(18), "mild");
        assert_eq!(classify_temperature(30), "hot");
    }
}
