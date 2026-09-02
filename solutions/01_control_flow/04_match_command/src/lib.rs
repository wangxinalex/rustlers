pub fn command_label(command: &str) -> &'static str {
    match command {
        "start" => "starting",
        _ => "unknown command",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_label_handles_known_and_unknown_commands() {
        assert_eq!(command_label("start"), "starting");
        assert_eq!(command_label("pause"), "unknown command");
    }
}
