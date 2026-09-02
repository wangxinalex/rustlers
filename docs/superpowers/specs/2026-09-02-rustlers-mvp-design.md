# Rustlers MVP Design

## Goal

Build a beginner-friendly Rust practice repository inspired by `gostlings`: learners fix one small, practical exercise at a time, run a focused check, and progress from syntax to a useful command-line tool.

## Scope

The MVP contains six ordered chapters with intentionally uneven exercise counts. Foundational concepts receive more repetition; later topics use fewer, integrated exercises.

| Chapter | Focus | Exercises |
|---|---|---:|
| `00_hello` | program shape, printing, variables, mutability, types | 4 |
| `01_control_flow` | expressions, conditions, loops, `match` | 4 |
| `02_functions_collections` | functions, `Vec`, `String`, iteration | 5 |
| `03_ownership_borrowing` | moves, `Copy`, `clone`, references, slices, mutable borrowing, UTF-8 strings | 8 |
| `04_errors` | `Option`, `Result`, parsing, `?` | 4 |
| `05_mini_cli` | file input and text statistics as an applied capstone | 2 |

The MVP deliberately excludes external crates, async, traits, lifetimes as a named topic, macros, and a custom interactive runner. Those can be added after the exercise contract is proven useful.

## User experience

The root README is the course guide. It explains prerequisites, the recommended order, the smallest useful command for running one exercise, how to run all checks, and when to inspect `solutions/`. Each exercise README and source header contains:

- Concept
- Task
- Expected behavior or exact output
- Hint
- A single clearly marked `TODO`

The learner workflow is:

1. Read the topic and exercise README.
2. Open the matching file under `exercises/`.
3. Change the marked `TODO`.
4. Run `cargo test --manifest-path exercises/<topic>/<exercise>/Cargo.toml` or the command shown by the exercise.
5. Compare with the same path under `solutions/` only when stuck.

## Repository layout

```text
rustlers/
├── Cargo.toml                  # workspace, no shared runtime code
├── README.md
├── check.sh                    # ordered exercise/solution checker
├── exercises/
│   ├── 00_hello/
│   │   ├── README.md
│   │   └── 01_print_line/
│   │       ├── Cargo.toml
│   │       ├── README.md
│   │       └── src/{lib.rs,main.rs}
│   └── ...
├── solutions/                  # runnable reference crates, same paths
│   └── 00_hello/01_print_line/...
└── docs/superpowers/
    ├── specs/...
    └── plans/...
```

Each exercise and solution is an independent package in the root workspace. A package has a small library API in `src/lib.rs`, a thin `src/main.rs` for runnable examples, and unit tests colocated with the library. This lets the checker use `cargo test` without adding a test dependency and keeps the behavior contract close to the code. Exercises that demonstrate a compile-time concept may use comments and a runtime equivalent so the starter remains checkable.

The root workspace lists all exercise and solution packages explicitly. No package shares source files with another package; copying a solved exercise into the solution path remains understandable to a beginner.

## Exercise contract

Every exercise follows these rules:

- Starter code compiles before the learner changes it, but its test fails whenever practical; this makes the first feedback actionable. A fresh checkout is therefore expected to have failing exercise behavior until the learner solves the TODOs.
- The learner edits a small code region marked `TODO`; surrounding setup is intentionally complete.
- Tests assert observable behavior or a small public function result, never implementation details.
- `src/main.rs` demonstrates the exercise with fixed, deterministic input and output.
- The solution has the same package name and public function signatures as the starter.
- Tests are deterministic, offline, and use only the Rust standard library.

The checker treats a package with tests as a test target and reports `PASS` or `FAIL` per path. It runs in lexical numeric order, stops on the first failure by default, supports `--run-all`, accepts a direct exercise path, and can check `solutions` separately. It uses `cargo test --quiet --manifest-path <path>/Cargo.toml`; it does not mutate exercise files or copy tests between trees.

## Curriculum design

### `00_hello` — 4 exercises

1. Print a greeting and learn `fn main`/`println!`.
2. Bind values with `let` and infer basic types.
3. Repair a mutation example with `let mut`.
4. Use an explicit type annotation and arithmetic in a small bill calculation.

### `01_control_flow` — 4 exercises

1. Return a value from an `if` expression.
2. Use a `for` loop to sum a range.
3. Use `while` to count down safely.
4. Map a command word with `match` and a wildcard branch.

### `02_functions_collections` — 5 exercises

1. Define a function with parameters and a return value.
2. Add an item to a `Vec` and report its length.
3. Iterate over a slice without taking ownership.
4. Count words in a string using whitespace splitting.
5. Build a small “shopping total” function from collection data.

### `03_ownership_borrowing` — 8 exercises

1. Pass a `String` by reference so the caller can still use it.
2. Return ownership from a function that builds a greeting.
3. Borrow a string slice and return its first word.
4. Mutably borrow a `String` to append a suffix.
5. Mutably borrow a `Vec<i32>` to remove negative values in place.
6. Compare a copied integer with a moved `String` and identify which value remains usable.
7. Use `clone` only when an owned duplicate is genuinely needed by two callers.
8. Count Unicode characters separately from UTF-8 bytes without slicing at an invalid byte boundary.

### `04_errors` — 4 exercises

1. Return `Option` when a requested item is absent.
2. Parse a number into `Result<i32, _>` and provide a friendly fallback.
3. Propagate a parse error with `?` through a small function.
4. Validate a simple configuration line and distinguish invalid input from success.

### `05_mini_cli` — 2 exercises

1. Implement pure text statistics (`lines`, `words`, `bytes`) for a string.
2. Wire the same logic into a file-reading CLI with a clear usage/error message.

The text-statistics capstone is deliberately standard-library-only and small enough to understand end to end. It demonstrates how earlier ownership, borrowing, strings, collections, and `Result` concepts combine in a useful program.

## Testing strategy

The first implementation cycle uses TDD for checker behavior and any non-trivial helper logic: write a focused failing test, observe the failure, add the minimum implementation, then run the full suite. Course tests themselves are part of the teaching artifact and should stay small.

Required verification commands before delivery:

```sh
sh check.sh solutions --run-all
cargo check --workspace
sh -n check.sh
```

After solving an exercise, the learner runs `sh check.sh exercises --run-all` (or the direct exercise command). The exercise tree is intentionally not required to be behavior-green in a fresh checkout.

The checker also gets shell-level tests for ordering, direct-target handling, first-failure behavior, `--run-all`, and missing-target errors. The final capstone gets an end-to-end smoke check using a temporary text file without committing generated files.

## Error handling and portability

The shell script uses POSIX `sh`, quotes paths, returns non-zero on failure, and avoids external package managers. Rust code uses `Result` for file and argument failures, does not assume a locale, and keeps expected output deterministic. The minimum toolchain is the installed stable Rust toolchain; the workspace sets `edition = "2024"` and requires no nightly features.

## Explicit non-goals

- No web UI or custom TUI.
- No solution-hiding mechanism.
- No third-party dependencies or network access during checks.
- No scoring, progress database, or user accounts.
- No attempt to cover every Rust feature in the MVP.
