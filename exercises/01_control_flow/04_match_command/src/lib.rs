// Concept: matching a string against known patterns with `match`.
// Task: return `"starting"` for `"start"` and `"unknown command"` otherwise.
// Expected behavior/output: `command_label("start")` returns `"starting"`; unknown commands return `"unknown command"`.
// Hint: use one match arm for `"start"` and `_` for the fallback.
pub fn command_label(command: &str) -> &'static str {
    // TODO: return "starting" for the "start" command.
    match command {
        "start" => "started",
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
