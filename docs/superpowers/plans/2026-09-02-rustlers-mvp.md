# Rustlers MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a standard-library-only Rust learning repository with 27 progressively harder exercises, runnable reference solutions, and a `gostlings`-style shell checker.

**Architecture:** Use one root Cargo workspace with one independent package per exercise and one matching package per solution. Each package exposes a tiny library function in `src/lib.rs`, keeps tests next to that function, and uses a thin `src/main.rs` for the learner-facing example. A POSIX `check.sh` discovers package manifests, runs one package at a time in numeric order, and reports each result.

**Tech Stack:** Rust stable 1.97.1, Cargo workspace, edition 2024, POSIX `sh`, Rust standard library only.

**Spec:** `docs/superpowers/specs/2026-09-02-rustlers-mvp-design.md`

## Global Constraints

- The workspace sets `edition = "2024"` and requires no nightly features.
- No third-party Rust crates, package managers, network access, scoring, accounts, or custom interactive runner.
- Exercise starters remain behavior-incomplete; `cargo check --workspace` must compile them, while the matching solutions must pass all tests.
- Every exercise has a README and source header containing Concept, Task, Expected behavior or output, and Hint.
- Every exercise has one clearly marked `TODO` that the learner edits.
- Tests assert observable behavior or public function results and remain deterministic and offline.
- The checker is POSIX `sh`, quotes paths, returns non-zero on failure, supports a direct package path and `--run-all`, and never mutates exercise files.
- The current directory is not a Git repository, so implementation must not add fake commits or depend on Git metadata.

## Package inventory and exact contracts

All listed packages are created under both `exercises/` and `solutions/` with the same package name, public function signatures, and tests. Exercise implementations intentionally return a simple incorrect value or incomplete behavior while remaining compilable; solution implementations satisfy the listed examples.

| Path suffix | Public contract | Required examples |
|---|---|---|
| `00_hello/01_print_line` | `pub fn greeting() -> &'static str` | `"Hello, Rustlers!"` |
| `00_hello/02_variables` | `pub fn welcome_message(name: &str) -> String` | `"Welcome, Ada!"` |
| `00_hello/03_mutability` | `pub fn add_tax(price: i32) -> i32` | `100 -> 108` |
| `00_hello/04_types` | `pub fn receipt_total(price: u32, quantity: u32) -> u32` | `14, 3 -> 42` |
| `01_control_flow/01_if_expression` | `pub fn classify_temperature(celsius: i32) -> &'static str` | `18 -> "mild"`, `30 -> "hot"`, `-2 -> "cold"` |
| `01_control_flow/02_for_sum` | `pub fn sum_up_to(n: u32) -> u32` | `5 -> 15` |
| `01_control_flow/03_while_countdown` | `pub fn countdown(start: u32) -> Vec<u32>` | `3 -> [3, 2, 1]` |
| `01_control_flow/04_match_command` | `pub fn command_label(command: &str) -> &'static str` | `"start" -> "starting"`, unknown -> `"unknown command"` |
| `02_functions_collections/01_parameters` | `pub fn greet(name: &str) -> String` | `"Lin" -> "Hello, Lin!"` |
| `02_functions_collections/02_push_vec` | `pub fn add_item(items: &mut Vec<String>, item: &str)` | `[] + "tea" -> ["tea"]` |
| `02_functions_collections/03_slice_sum` | `pub fn total(items: &[i32]) -> i32` | `[3, 4, 5] -> 12` |
| `02_functions_collections/04_word_count` | `pub fn word_count(text: &str) -> usize` | `"one two two" -> 3` |
| `02_functions_collections/05_shopping_total` | `pub fn shopping_total(prices: &[u32], discount_percent: u32) -> u32` | `[100, 50], 10 -> 135` |
| `03_ownership_borrowing/01_borrow_string` | `pub fn length_after_borrow(text: &String) -> usize` | `"rust" -> 4` |
| `03_ownership_borrowing/02_return_string` | `pub fn build_greeting(name: &str) -> String` | `"Mia" -> "Hello, Mia!"` |
| `03_ownership_borrowing/03_first_word` | `pub fn first_word(text: &str) -> &str` | `"hello world" -> "hello"`, `"  rust" -> "rust"` |
| `03_ownership_borrowing/04_mutable_string` | `pub fn add_suffix(text: &mut String, suffix: &str)` | `"learn" + "ing" -> "learning"` |
| `03_ownership_borrowing/05_retain_numbers` | `pub fn retain_non_negative(numbers: &mut Vec<i32>)` | `[3, -1, 0, -5] -> [3, 0]` |
| `03_ownership_borrowing/06_copy_and_move` | `pub fn copy_and_move() -> (i32, usize)` | returns `(14, 4)` and demonstrates `Copy` vs moved `String` |
| `03_ownership_borrowing/07_clone_when_needed` | `pub fn duplicate_for_two_places(text: &str) -> (String, String)` | `"notes" -> ("notes", "notes")` |
| `03_ownership_borrowing/08_unicode_lengths` | `pub fn text_lengths(text: &str) -> (usize, usize)` | `"你好" -> (2, 6)` as `(chars, bytes)` |
| `04_errors/01_find_price` | `pub fn find_price(prices: &[(&str, u32)], name: &str) -> Option<u32>` | finds `Some(12)` or returns `None` |
| `04_errors/02_parse_quantity` | `pub fn parse_quantity(input: &str) -> Result<u32, String>` | `"7" -> Ok(7)`, invalid/zero -> friendly `Err` |
| `04_errors/03_parse_pair` | `pub fn parse_pair(input: &str) -> Result<(i32, i32), String>` | `"3,4" -> Ok((3,4))`, malformed -> `Err` |
| `04_errors/04_parse_setting` | `pub fn parse_setting(line: &str) -> Result<(&str, u32), String>` | `"port=8080" -> Ok(("port",8080))` |
| `05_mini_cli/01_text_stats` | `TextStats { lines: usize, words: usize, bytes: usize }`; `pub fn stats(text: &str) -> TextStats` | `"one two\nthree" -> 2 lines, 3 words, 13 bytes` |
| `05_mini_cli/02_file_report` | `pub fn report_file(path: &Path) -> Result<String, String>` | renders `lines=... words=... bytes=...` and reports missing files |

For the starter/solution split, write the tests once in the exercise package, then copy the exact test module into the matching solution package before changing only the implementation. The implementation plan below names that copy explicitly so the behavior contract cannot drift.

## Task 1: Scaffold the workspace and course documentation

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `README.md`
- Create: `exercises/README.md`
- Create: `solutions/README.md`
- Create: `exercises/00_hello/README.md`, `exercises/01_control_flow/README.md`, `exercises/02_functions_collections/README.md`, `exercises/03_ownership_borrowing/README.md`, `exercises/04_errors/README.md`, `exercises/05_mini_cli/README.md`

**Interfaces:** The root workspace owns all package manifests created by later tasks; the README commands must match the final `check.sh` interface.

- [ ] **Step 1: Write the workspace manifest and ignore file**

  Put `resolver = "3"` and `edition = "2024"` in `[workspace.package]`. Leave `members = []` initially so the root can compile before packages are added; later tasks replace it with the explicit package list. Ignore `target/` and editor files only.

- [ ] **Step 2: Write the learner workflow documentation**

  Explain the prerequisite (`rustc --version`, `cargo --version`), order of six chapters, this exact single-exercise command, and the solution command:

  ```sh
  cargo test --manifest-path exercises/03_ownership_borrowing/03_first_word/Cargo.toml
  sh check.sh solutions --run-all
  ```

  State that starters are intentionally incomplete and that `solutions/` is a reference, not the first stop. Include the chapter table from the spec and the ownership chapter's eight-item emphasis.

- [ ] **Step 3: Write chapter READMEs**

  Each chapter README lists its learning goal, numbered exercise names in order, one prerequisite sentence, and the next chapter. Use Chinese explanations with Rust identifiers/commands left in code formatting. Keep each README under 80 lines.

- [ ] **Step 4: Verify the scaffold**

  Run `cargo metadata --no-deps --format-version 1` and confirm it exits 0 with zero members before later package tasks populate the list. Run `sh -n` only after `check.sh` exists.

## Task 2: Define checker behavior with failing shell tests, then implement `check.sh`

**Files:**
- Create: `scripts/test_check.sh`
- Create: `check.sh`
- Test data: use the real starter/solution paths; do not add fake Cargo projects.

**Interfaces:** `sh check.sh [target] [--run-all]`; default target is `exercises`; a target may be `solutions` or a direct package directory containing `Cargo.toml`; unknown flags and missing targets return status 2.

**Dependency:** The positive assertions use the solution and starter package paths created in Task 3. Write this test file now, then execute the focused test cycle after Task 3 has created those packages and before implementing `check.sh`.

- [ ] **Step 1: Write the failing checker tests**

  In `scripts/test_check.sh`, use POSIX shell helpers to capture status and output. Assert these behaviors:

  ```sh
  sh check.sh solutions/00_hello/01_print_line
  # status 0 and output contains PASS: solutions/00_hello/01_print_line

  sh check.sh solutions/00_hello
  # status 0 and output contains 4/4 passed

  sh check.sh exercises/00_hello/01_print_line
  # status non-zero and output contains FAIL: exercises/00_hello/01_print_line

  sh check.sh exercises/00_hello/01_print_line --run-all
  # status non-zero and output contains the final N/N summary without stopping at the first package

  sh check.sh does-not-exist
  # status 2 and output contains target directory does not exist
  ```

  Keep assertions in shell, so the checker has no dependency on another test framework.

- [ ] **Step 2: Run the tests before implementing the checker**

  Run `sh scripts/test_check.sh`. It must fail because `check.sh` does not exist yet; confirm the failure is from the missing checker, not a malformed test.

- [ ] **Step 3: Implement package discovery and argument parsing**

  Use `set -eu`, default `target=exercises`, and parse `--run-all` plus one target. Reject duplicate targets, unknown options, and `--run-all` without a target only if the shell parser cannot safely handle them. If `target/Cargo.toml` exists, check only that directory; otherwise find `*/ */Cargo.toml` below the target, sort paths lexically, and derive a display path relative to the repository root.

- [ ] **Step 4: Implement one-package execution and summary**

  Run exactly `cargo test --quiet --manifest-path "$dir/Cargo.toml"`. Print `PASS: <display>` on zero status and `FAIL: <display>` otherwise. Stop and print command output on the first failure by default; with `--run-all`, retain the failure count, continue, and print `<passed>/<total> passed` plus `All <target> pass` only when failures are zero.

- [ ] **Step 5: Run the checker tests**

  Run `sh scripts/test_check.sh`, then `sh -n check.sh`. The expected result is all shell assertions passing and syntax validation exiting 0.

## Task 3: Build the `00_hello` and `01_control_flow` exercise/solution pairs

**Files:**
- Create 8 exercise packages under `exercises/00_hello` and `exercises/01_control_flow`.
- Create the matching 8 solution packages under `solutions/00_hello` and `solutions/01_control_flow`.
- Modify root `Cargo.toml` to list all 16 manifests as workspace members.

**Interfaces:** Use the exact contracts in the inventory table. Each package has this manifest shape:

```toml
[package]
name = "rustlers-00-hello-01-print-line"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Use `src/lib.rs` for the function, `src/main.rs` for the fixed demonstration, and unit tests in `src/lib.rs` under `#[cfg(test)]`.

- [ ] **Step 1: Write tests for the first exercise before its implementation**

  For `00_hello/01_print_line`, add a test asserting `greeting() == "Hello, Rustlers!"`, then run `cargo test --manifest-path exercises/00_hello/01_print_line/Cargo.toml`; it must fail because the starter returns a wrong placeholder.

- [ ] **Step 2: Implement the minimum first exercise pair**

  Make the exercise return a clearly wrong but typed placeholder and include one `TODO`. Make the solution return `"Hello, Rustlers!"`. `main` prints `greeting()` with `println!` in both packages. Run the exercise test (expected FAIL) and solution test (expected PASS).

- [ ] **Step 3: Add the remaining `00_hello` tests before implementations**

  Add exact tests for `welcome_message("Ada")`, `add_tax(100)`, and `receipt_total(14, 3)`. For each, run its focused test once and confirm it fails against the starter placeholder.

- [ ] **Step 4: Implement the remaining `00_hello` pairs**

  Use `let` for bindings, `let mut` for the tax calculation, explicit `u32` values for the receipt, and no helper abstraction. Copy the tests unchanged to each solution and make the solution functions pass. Run all eight focused package tests, expecting four exercise failures and four solution passes.

- [ ] **Step 5: Write and verify the four `01_control_flow` pairs**

  Add tests for the three temperature branches, `sum_up_to(5)`, `countdown(3)`, and known/unknown commands. Exercise implementations leave one branch or accumulator wrong; solutions use an `if` expression, `for`, `while`, and `match` respectively. Run each test red before implementation, then green for each solution.

- [ ] **Step 6: Verify chapter behavior and compilation**

  Run `sh check.sh solutions/00_hello --run-all`, `sh check.sh solutions/01_control_flow --run-all`, and `cargo check --workspace`. Confirm both solution summaries are fully passing and all starters compile.

## Task 4: Build `02_functions_collections`

**Files:**
- Create 5 exercise packages under `exercises/02_functions_collections`.
- Create 5 matching solution packages under `solutions/02_functions_collections`.
- Modify: `Cargo.toml` to add these 10 members.

**Interfaces:** Use the exact five inventory signatures and deterministic examples. Keep the functions small enough that a learner can understand them without a helper module.

- [ ] **Step 1: Write the five focused tests**

  Assert `greet("Lin")`, mutation of an empty `Vec<String>` by `add_item`, `total(&[3,4,5])`, `word_count("one two two")`, and `shopping_total(&[100,50],10)`. Run each focused exercise test before implementing and record the expected assertion failure.

- [ ] **Step 2: Implement the exercise starters**

  Add one `TODO` per package. Keep signatures correct and use typed placeholders so `cargo check` remains green. Do not add any collection type beyond `Vec`, slices, and `String` needed for the contract.

- [ ] **Step 3: Implement the solutions**

  Use a direct function body for each solution: format the greeting, `push` the item, iterate a slice, use `split_whitespace().count()`, and sum then apply the integer discount. Copy tests unchanged from exercises.

- [ ] **Step 4: Verify the collection chapter**

  Run `sh check.sh solutions/02_functions_collections --run-all` and `cargo check --workspace`. Confirm five solution packages pass and no new dependency appears in any manifest.

## Task 5: Build the expanded `03_ownership_borrowing` chapter

**Files:**
- Create 8 exercise packages under `exercises/03_ownership_borrowing`.
- Create 8 matching solution packages under `solutions/03_ownership_borrowing`.
- Modify: `Cargo.toml` to add these 16 members.

**Interfaces:** Use the exact eight signatures from the inventory. Every README must name the ownership rule being practiced and include a warning that the learner should read compiler errors rather than immediately clone values.

- [ ] **Step 1: Write focused tests for borrowing and returning ownership**

  Assert `length_after_borrow(&"rust".to_string()) == 4`, `build_greeting("Mia") == "Hello, Mia!"`, `first_word("hello world") == "hello"`, and `first_word("  rust") == "rust"`. Run each exercise test before adding implementations and confirm the placeholder fails.

- [ ] **Step 2: Implement the first four starter/solution pairs**

  Starters retain typed placeholders with one TODO. Solutions pass references without moving the caller's `String`, construct and return an owned greeting, skip leading whitespace before finding the first word, and use `push_str` through `&mut String`. Avoid explicit lifetime annotations because elision is sufficient at this level.

- [ ] **Step 3: Write tests for mutation, `Copy`/move, and `clone`**

  Assert that `retain_non_negative` changes `[3,-1,0,-5]` to `[3,0]`, `copy_and_move() == (14,4)`, and `duplicate_for_two_places("notes")` returns two equal owned strings. Run these tests against starters and confirm failure.

- [ ] **Step 4: Implement the middle four starter/solution pairs**

  Use `retain` for in-place vector filtering, make the solution's `copy_and_move` visibly show an `i32` copied after assignment and a `String` used through its moved binding, and use `clone` only to produce two owned strings when both are needed. Keep the exercise's comments explicit about which line the learner should repair.

- [ ] **Step 5: Write the UTF-8 test before implementation**

  Assert `text_lengths("你好") == (2, 6)`, and add an ASCII assertion to prevent an accidental hard-coded answer. Run the exercise test and confirm it fails.

- [ ] **Step 6: Implement and document Unicode lengths**

  Make the solution return `(text.chars().count(), text.len())`. Explain in the README that `String::len()` counts UTF-8 bytes and `chars().count()` counts Unicode scalar values; do not slice by byte index.

- [ ] **Step 7: Verify the ownership chapter**

  Run `sh check.sh solutions/03_ownership_borrowing --run-all`, `cargo check --workspace`, and a direct command for `03_first_word`. Confirm all eight solution packages pass and all starter packages compile.

## Task 6: Build `04_errors`

**Files:**
- Create 4 exercise packages under `exercises/04_errors`.
- Create 4 matching solution packages under `solutions/04_errors`.
- Modify: `Cargo.toml` to add these 8 members.

**Interfaces:** Use `Option` for absence, `Result<_, String>` for learner-readable errors, and no custom error type in the MVP.

- [ ] **Step 1: Write tests for all four error contracts**

  Assert both success and failure cases: `find_price` returns `Some(12)`/`None`; `parse_quantity("7")` is `Ok(7)` and invalid/zero inputs are `Err`; `parse_pair("3,4")` is `Ok((3,4))` and malformed input is `Err`; `parse_setting("port=8080")` is `Ok(("port",8080))` while wrong key/missing value are `Err`. Run each exercise test and confirm the starter fails.

- [ ] **Step 2: Implement the starters and solutions**

  Add typed placeholders with one TODO in each exercise. Solutions use `.iter().find`, `str::parse`, `split_once`, and `?` where each concept is named. Validate zero quantities, missing separators, non-numeric values, and the required `port` key with deterministic messages.

- [ ] **Step 3: Verify error propagation**

  Run `sh check.sh solutions/04_errors --run-all`, `cargo check --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` if Clippy is installed. If the local toolchain lacks Clippy, record that fact and still run the required cargo checks; do not add a dependency to compensate.

## Task 7: Build the practical `05_mini_cli` capstone

**Files:**
- Create 2 exercise packages under `exercises/05_mini_cli`.
- Create 2 matching solution packages under `solutions/05_mini_cli`.
- Modify: `Cargo.toml` to add these 4 members.

**Interfaces:** `TextStats` has public `lines`, `words`, and `bytes` fields. `stats` treats an empty string as zero lines, counts lines with `text.lines().count()`, words with `split_whitespace`, and bytes with `len()`. `report_file` reads a `Path`, returns a single-line summary on success, and maps I/O errors to a useful `String`.

- [ ] **Step 1: Write the pure statistics tests**

  Assert `stats("")` is all zero, `stats("one two\nthree")` is `(2,3,13)`, and a Chinese string has byte count greater than character count. Run before implementation and confirm failure.

- [ ] **Step 2: Implement the statistics starter and solution**

  Put the struct and function in `src/lib.rs`, add one TODO to the exercise, and make `src/main.rs` print a deterministic sample. The solution uses only `lines`, `split_whitespace`, and `len`.

- [ ] **Step 3: Write file-report tests before implementation**

  Create a temporary path under `std::env::temp_dir()` with a process-unique filename, write `"buy milk\nread book\n"`, call `report_file`, assert `lines=2 words=4 bytes=19`, then remove the file. Add a missing-path assertion that checks the error mentions the path. Run the exercise test and confirm failure.

- [ ] **Step 4: Implement the file-report starter and solution**

  Use `std::fs::read_to_string`, propagate its error through `map_err`, and format the stats from the first capstone function. `main` accepts exactly one path argument, prints a usage message and exits with status 2 when missing, and prints the report otherwise. Do not add a CLI parser crate.

- [ ] **Step 5: Verify the capstone end to end**

  Run the solution package test, then create a temporary text file from the shell and run `cargo run --quiet --manifest-path solutions/05_mini_cli/02_file_report/Cargo.toml -- <temp-file>`. Confirm the output has the expected three counters, then remove only that exact temp file.

## Task 8: Finish documentation, package metadata, and full verification

**Files:**
- Modify: `README.md`, all exercise READMEs as needed, `solutions/README.md`, `Cargo.toml`
- Create: `scripts/verify_solutions.sh`

**Interfaces:** `scripts/verify_solutions.sh` is a convenience wrapper around `sh check.sh solutions --run-all`; it must return the checker status and print a short purpose line.

- [ ] **Step 1: Complete the root README checklist**

  Add the full 27-exercise table, the direct command template, checker options, the distinction between `cargo check --workspace` and solving exercises, and the ownership/borrowing recommendation to spend extra time on chapters 03.

- [ ] **Step 2: Add the solution verification wrapper**

  Write a POSIX script with `set -eu` that changes to the repository directory based on the script's own location and executes `sh check.sh solutions --run-all`. Run it from both the repository root and a different current directory.

- [ ] **Step 3: Check manifests and formatting**

  Run `cargo metadata --no-deps --format-version 1`, verify the member count is 54 packages (27 exercises plus 27 solutions), run `cargo fmt --all -- --check`, and run `sh -n check.sh scripts/test_check.sh scripts/verify_solutions.sh`.

- [ ] **Step 4: Run the complete verification suite**

  Run these commands fresh and inspect exit codes/output:

  ```sh
  sh scripts/test_check.sh
  sh check.sh solutions --run-all
  sh scripts/verify_solutions.sh
  cargo check --workspace
  cargo fmt --all -- --check
  sh -n check.sh scripts/test_check.sh scripts/verify_solutions.sh
  ```

  Do not claim the exercise tree is green; its intended fresh state is incomplete. Confirm the solutions summary is fully passing and all workspace members compile.

- [ ] **Step 5: Review for unnecessary complexity**

  Run `rg -n "TODO|unwrap\(|expect\(|unsafe|extern crate|dependencies" exercises solutions README.md check.sh scripts` and inspect every hit. Keep TODOs only in starters and keep `unwrap`/`expect` out of learner-facing error paths; remove any dependency or helper abstraction not required by the contracts.
