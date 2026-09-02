# Task 8 implementation report

Date: 2026-09-02

## Scope

Task 8 completes the course guide, solution verification instructions, and
workspace metadata without changing exercise or solution APIs, starter TODOs,
solution behavior, or the standard-library-only constraint.

The root README now contains the full 27-exercise checklist, a direct
`cargo test --manifest-path` template, checker usage and option behavior, the
distinction between compilation and behavior verification, and an explicit
recommendation to spend extra time on `03_ownership_borrowing`. The solution
README documents the location-independent verification wrapper. The workspace
member list is ordered as all exercises followed by all solutions, with all 54
packages still present.

The new `scripts/verify_solutions.sh` is a POSIX `set -eu` wrapper. It resolves
the repository from its own location, prints its purpose, delegates to
`sh check.sh solutions --run-all`, and preserves the checker status.

No exercise README changes were needed: all 27 package READMEs already contain
the required concept, task, expected behavior/output, and hint sections. The
ownership package READMEs already identify the relevant ownership rule and
warn learners to read compiler errors before reaching for `clone`.

## Verification

The complete Task 8 verification suite was run fresh after the changes:

- `sh scripts/test_check.sh` — passed; all shell assertions passed.
- `sh check.sh solutions --run-all` — passed; 27/27 solution packages passed.
- `sh scripts/verify_solutions.sh` from the repository root — passed; 27/27.
- `sh scripts/verify_solutions.sh` from a different current directory — passed;
  27/27.
- `cargo metadata --no-deps --format-version 1` — passed; 54 workspace
  packages: 27 exercises and 27 solutions.
- `cargo check --workspace` — passed; all workspace members compile, including
  intentionally incomplete exercise starters.
- `cargo fmt --all -- --check` — passed.
- `sh -n check.sh scripts/test_check.sh scripts/verify_solutions.sh` — passed.

The exercise tree was not expected to be behavior-green and was not treated as
a passing target.

## Complexity and marker review

The required scan was run:

```sh
rg -n "TODO|unwrap\(|expect\(|unsafe|extern crate|dependencies" exercises solutions README.md check.sh scripts
```

All TODO markers are in exercise starters. The only `unwrap` calls are in the
temporary-file test setup and cleanup for the file-report exercise and its
solution; learner-facing file errors remain returned as `Result<String, String>`.
There are no `expect`, `unsafe`, or `extern crate` hits, and all Cargo
dependency tables are empty. No unnecessary dependency or helper abstraction
was introduced or required removal.

## Concerns

None.
