# Task 6 implementation report

Date: 2026-09-02

## Scope

Task 6 adds the `04_errors` chapter with four exercise packages and four
matching `-solution` packages. The workspace manifest now includes all eight
new members. No source files from earlier chapters were modified.

Every package uses edition 2024, has no dependencies, and contains
`Cargo.toml`, `src/lib.rs`, `src/main.rs`, and a package README. Each README
contains the required `Concept`, `Task`, `Expected behavior/output`, and
`Hint` sections. Each exercise starter has exactly one TODO, each solution has
none, and each exercise/solution pair contains identical colocated unit tests.

## Error-handling contracts implemented

| Package | Public function | Solution behavior |
| --- | --- | --- |
| `01_find_price` | `find_price(prices: &[(&str, u32)], name: &str) -> Option<u32>` | Searches with `.iter().find`, returning `Some(price)` for a match and `None` for an unknown item. |
| `02_parse_quantity` | `parse_quantity(input: &str) -> Result<u32, String>` | Parses with `str::parse`, propagates conversion errors with `?`, and rejects zero with a deterministic readable error. |
| `03_parse_pair` | `parse_pair(input: &str) -> Result<(i32, i32), String>` | Uses `split_once(',')`, parses both trimmed values, and propagates missing-separator or parse errors with `?`. |
| `04_parse_setting` | `parse_setting(line: &str) -> Result<(&str, u32), String>` | Uses `split_once('=')`, requires the `port` key, parses the value, and returns readable `String` errors for invalid input. |

The starter functions retain typed placeholders so the packages compile while
the learner-facing tests expose the missing behavior. The mains are thin demos
of the required examples.

## TDD evidence

All success and failure tests were written in both exercise and solution
libraries before the corresponding implementations.

- The first test run against test-only libraries failed at unresolved public
  function names, confirming the tests were not accidentally passing.
- After adding the typed starter placeholders, every exercise test failed on
  its required success assertion while its absence/error assertions passed:
  `Some(12)`, `Ok(7)`, `Ok((3, 4))`, and `Ok(("port", 8080))` were the expected
  missing behaviors.
- After implementing the solutions, all copied success and failure tests
  passed for all four solution packages.

## Verification

- `sh check.sh solutions/04_errors --run-all` — passed, 4/4 solution packages.
- `cargo check --workspace` — passed; all eight Task 6 packages and earlier
  workspace members compile.
- `cargo fmt --all -- --check` — passed.
- Clippy on each Task 6 manifest with
  `--all-targets -- -D warnings` — passed for all eight packages.
- A structural read-only check — passed for required package files, exact
  signatures, README sections, matching test modules, starter/solution TODO
  counts, no-dependency manifests, and required implementation patterns.

## Concern

The requested workspace-wide command
`cargo clippy --workspace --all-targets -- -D warnings` exits 101 because of
two pre-existing warnings outside Task 6: `clippy::ptr_arg` for the required
`&String` contract in
`solutions/03_ownership_borrowing/01_borrow_string-solution/src/lib.rs`, and
`clippy::if_same_then_else` in the intentionally incomplete starter
`exercises/01_control_flow/01_if_expression/src/lib.rs`. Task 6 packages pass
the same Clippy settings individually, and earlier package source was left
unchanged as required.
