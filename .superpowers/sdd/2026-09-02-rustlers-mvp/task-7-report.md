# Task 7 report: `05_mini_cli`

## Scope

Implemented the two practical capstone packages under both `exercises/05_mini_cli`
and `solutions/05_mini_cli`, and added all four packages to the root workspace.
No earlier package source or colocated tests were changed.

## Implemented contracts

### `01_text_stats`

- Added public `TextStats` fields: `lines`, `words`, and `bytes`.
- Added `pub fn stats(text: &str) -> TextStats`.
- Empty text returns zero for all fields.
- Lines use `text.lines().count()`, words use `split_whitespace().count()`,
  and bytes use `text.len()`.
- The deterministic main prints `lines=... words=... bytes=...` for
  `one two\nthree`.

### `02_file_report`

- Added `pub fn report_file(path: &Path) -> Result<String, String>`.
- Reads UTF-8 text with `std::fs::read_to_string`.
- Maps read errors to strings containing the displayed path.
- Returns the single-line format `lines=... words=... bytes=...`.
- The CLI accepts one path, reports a usage message, and exits with status 2
  when the path is missing or an extra argument is supplied.
- File-report I/O failures are printed to stderr and exit with status 1.

## TDD evidence

1. Added the `01_text_stats` unit tests before the API implementation. The
   focused exercise test failed because `stats` was unresolved.
2. Added the starter and solution implementations. The starter then failed
   the non-empty and Chinese cases, while the solution passed all three tests.
3. Added the `02_file_report` temporary-file and missing-path tests before the
   API implementation. The focused exercise test failed because
   `report_file` was unresolved.
4. Added the starter and solution implementations. The starter failed both
   assertions with its placeholder error; the solution passed both tests.
5. Copied the unit-test modules unchanged into the corresponding solutions.

## Verification

- `cargo test --manifest-path solutions/05_mini_cli/01_text_stats/Cargo.toml`:
  3 passed, 0 failed.
- `cargo test --manifest-path solutions/05_mini_cli/02_file_report/Cargo.toml`:
  2 passed, 0 failed.
- Temporary-file end-to-end command:
  `cargo run --quiet --manifest-path solutions/05_mini_cli/02_file_report/Cargo.toml -- <temp-file>`
  produced `lines=2 words=4 bytes=19`; the exact temporary file was removed.
- Missing-argument CLI check returned status 2 and printed usage text.
- `cargo check --workspace` passed, including all starter packages.
- `cargo fmt --all -- --check` passed after formatting the two file-report
  mains.
- All four manifests use edition 2024 and have empty `[dependencies]` tables.
- The two exercise starters contain exactly one TODO each; the solution tree
  contains no TODOs for this capstone.

## Files added or modified

- `Cargo.toml`
- `exercises/05_mini_cli/01_text_stats/`
- `exercises/05_mini_cli/02_file_report/`
- `solutions/05_mini_cli/01_text_stats/`
- `solutions/05_mini_cli/02_file_report/`

## Concerns

None. The exercise implementations remain intentionally incomplete, so their
behavior tests are expected to fail until a learner completes the TODOs.
