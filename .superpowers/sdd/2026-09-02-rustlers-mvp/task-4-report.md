# Task 4 implementation report

Date: 2026-09-02
Workspace: `/Users/wangxinalex/SelfStudy/Rust/rustlers`

## Outcome

Implemented `02_functions_collections` with five exercise packages and five matching solution packages. The root workspace now includes all ten new package manifests.

## Packages and contracts

| Exercise | Solution | Contract | Solution behavior |
| --- | --- | --- | --- |
| `01_parameters` | `01_parameters-solution` | `pub fn greet(name: &str) -> String` | Formats `Hello, {name}!` |
| `02_push_vec` | `02_push_vec-solution` | `pub fn add_item(items: &mut Vec<String>, item: &str)` | Pushes an owned item into the vector |
| `03_slice_sum` | `03_slice_sum-solution` | `pub fn total(items: &[i32]) -> i32` | Iterates and sums the slice |
| `04_word_count` | `04_word_count-solution` | `pub fn word_count(text: &str) -> usize` | Counts `split_whitespace()` items |
| `05_shopping_total` | `05_shopping_total-solution` | `pub fn shopping_total(prices: &[u32], discount_percent: u32) -> u32` | Sums prices and applies the integer discount |

Each package has edition 2024, no dependencies, a package README, `src/lib.rs`, a thin `src/main.rs`, and an inline unit-test module. Exercise and solution test modules are identical. Each exercise starter contains exactly one TODO; solutions contain no TODOs.

## TDD evidence

The five focused tests were written before the contract implementations. The initial test-first run failed at compilation with `E0425: cannot find function` for the missing contract in each exercise. After adding typed incomplete starters, the focused tests reached the intended assertion failures:

- `greet`: `""` vs. `"Hello, Lin!"`
- `add_item`: `[]` vs. `["tea"]`
- `total`: `0` vs. `12`
- `word_count`: `0` vs. `3`
- `shopping_total`: `0` vs. `135`

The solution implementations were then added with the tests copied unchanged from their exercises.

## Verification

The following required checks completed successfully:

- `sh check.sh solutions/02_functions_collections --run-all` — `5/5 passed`
- `cargo check --workspace` — passed
- `cargo fmt --all -- --check` — passed

Additional audits passed:

- All ten new manifests have empty `[dependencies]` sections.
- All five exercise starters have exactly one TODO.
- No solution package contains TODO.
- All five exercise/solution test modules compare identical.
- The new package files are limited to Task 4 scope plus this report and the root workspace member additions.

## Concerns

None. Exercise tests are intentionally failing until a learner completes each TODO; the reference solution chapter is fully passing.
