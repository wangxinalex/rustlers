# Task 5 implementation report

Date: 2026-09-02

## Scope

Task 5 expands `03_ownership_borrowing` with eight exercise packages and
eight matching `-solution` packages. The workspace manifest now includes all
16 new members. No source files from earlier chapters were modified.

Every package uses edition 2024, has no dependencies, and contains
`Cargo.toml`, `src/lib.rs`, `src/main.rs`, and a package README. Each
exercise has exactly one TODO in its starter library; each solution has no
TODO. Corresponding exercise and solution libraries contain the same
colocated unit tests.

## Ownership contracts implemented

| Package | Public function | Solution behavior |
| --- | --- | --- |
| `01_borrow_string` | `length_after_borrow(text: &String) -> usize` | Reads `text.len()` through a shared borrow, leaving the caller's `String` usable. |
| `02_return_string` | `build_greeting(name: &str) -> String` | Formats and returns an owned `"Hello, {name}!"` string. |
| `03_first_word` | `first_word(text: &str) -> &str` | Uses whitespace splitting to skip leading whitespace and return a borrowed first word. |
| `04_mutable_string` | `add_suffix(text: &mut String, suffix: &str)` | Appends through an exclusive mutable borrow with `push_str`. |
| `05_retain_numbers` | `retain_non_negative(numbers: &mut Vec<i32>)` | Filters the caller's vector in place with `retain`. |
| `06_copy_and_move` | `copy_and_move() -> (i32, usize)` | Assigns an `i32` while its original remains usable, then moves a four-letter `String` and uses the moved binding; returns `(14, 4)`. |
| `07_clone_when_needed` | `duplicate_for_two_places(text: &str) -> (String, String)` | Creates one owned string and clones it only because both returned places need ownership. |
| `08_unicode_lengths` | `text_lengths(text: &str) -> (usize, usize)` | Returns `(text.chars().count(), text.len())` as `(Unicode scalar values, UTF-8 bytes)`. |

The exercise READMEs name the ownership rule for their package and tell
learners to read compiler errors before reaching for `clone`. The Unicode
README explicitly distinguishes `String::len()` byte counts from
`chars().count()` scalar-value counts and warns against slicing by byte index.
No explicit lifetime annotations or extra abstractions were added.

## TDD evidence

The focused exercise tests were run against the typed placeholders before the
corresponding solution implementations:

- The first four exercise tests failed on the expected placeholder values for
  borrowing, returning ownership, first-word extraction, and mutable suffix
  appending.
- The next three exercise tests failed on the expected placeholder values for
  vector retention, Copy versus move, and duplicate owned strings.
- The Unicode exercise test failed for both `"你好" -> (2, 6)` and the ASCII
  guard `"rust" -> (4, 4)`.

These failures are intentional: exercise packages remain starter material.
The solution packages contain the working implementations and identical test
cases.

## Verification

- `sh check.sh solutions/03_ownership_borrowing --run-all` — 8/8 solution
  packages passed.
- `cargo check --workspace` — passed; all exercise starters compile.
- `cargo test --manifest-path solutions/03_ownership_borrowing/03_first_word-solution/Cargo.toml` — passed; 2 tests passed.
- `cargo fmt --all -- --check` — passed.

The direct test against the exercise `03_first_word` package remains expected
to fail because its starter implementation returns the placeholder empty
slice; the matching solution direct test above passes.
