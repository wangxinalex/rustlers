// Concept: combine file input, borrowed text, statistics, and `Result`.
// Task: implement `report_file(path)` with a single-line text summary.
// Expected behavior/output: `buy milk\nread book\n` reports 2, 4, and 19.
// Hint: use `std::fs::read_to_string` and map its error into a useful string.
use std::path::Path;

pub fn report_file(_path: &Path) -> Result<String, String> {
    // TODO: read the file and format its text statistics.
    Err(String::from("file report is not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, path::PathBuf};

    fn temp_path(label: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "rustlers-05-mini-cli-02-file-report-{label}-{}.txt",
            std::process::id()
        ))
    }

    #[test]
    fn report_file_counts_lines_words_and_bytes() {
        let path = temp_path("counts");
        std::fs::write(&path, "buy milk\nread book\n").unwrap();

        let report = report_file(&path);
        std::fs::remove_file(&path).unwrap();

        assert_eq!(report, Ok(String::from("lines=2 words=4 bytes=19")));
    }

    #[test]
    fn report_file_mentions_a_missing_path() {
        let path = temp_path("missing");
        let _ = std::fs::remove_file(&path);

        let error = report_file(&path).expect_err("a missing file should return an error");
        let displayed_path = path.display().to_string();

        assert!(error.contains(displayed_path.as_str()));
    }
}
