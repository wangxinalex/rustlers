pub fn parse_setting(line: &str) -> Result<(&str, u32), String> {
    let (key, value) = line
        .split_once('=')
        .ok_or_else(|| String::from("setting must use key=value format"))?;

    if key != "port" {
        return Err(String::from("setting key must be port"));
    }

    let value = value
        .parse::<u32>()
        .map_err(|_| String::from("port value must be a number"))?;

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_setting_accepts_the_port_setting() {
        assert_eq!(parse_setting("port=8080"), Ok(("port", 8080)));
    }

    #[test]
    fn parse_setting_rejects_a_wrong_key() {
        assert!(parse_setting("host=8080").is_err());
    }

    #[test]
    fn parse_setting_rejects_a_missing_value() {
        assert!(parse_setting("port=").is_err());
    }
}
