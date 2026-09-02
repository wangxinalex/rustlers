# Rustlers MVP final fix wave report

Date: 2026-09-02
Workspace: `/Users/wangxinalex/SelfStudy/Rust/rustlers`

## Outcome

Both Important findings from the final review package are fixed. The change
does not alter any public API, solution implementation, exercise
implementation, or test.

## Finding 1: solution paths

The eight ownership/borrowing solution directories were renamed so their
paths exactly mirror the exercise-relative paths:

- `03_ownership_borrowing/01_borrow_string`
- `03_ownership_borrowing/02_return_string`
- `03_ownership_borrowing/03_first_word`
- `03_ownership_borrowing/04_mutable_string`
- `03_ownership_borrowing/05_retain_numbers`
- `03_ownership_borrowing/06_copy_and_move`
- `03_ownership_borrowing/07_clone_when_needed`
- `03_ownership_borrowing/08_unicode_lengths`

The four error-handling solution directories were renamed to:

- `04_errors/01_find_price`
- `04_errors/02_parse_quantity`
- `04_errors/03_parse_pair`
- `04_errors/04_parse_setting`

`Cargo.toml` now lists all 12 new paths. Each moved package retains its
`-solution` Cargo package name. Git-object checks confirmed that every file in
the 12 moved directories is byte-identical to its pre-rename content. A
repository search found no old `-solution` directory or path references.

## Finding 2: starter source headers

Four concise comments were added at the top of each of the 25 non-capstone
starter `src/lib.rs` files in chapters `00_hello`, `01_control_flow`,
`02_functions_collections`, `03_ownership_borrowing`, and `04_errors`:

```text
Concept
Task
Expected behavior/output
Hint
```

The wording comes from the existing per-exercise README files, with expected
values matching the current tests, including `"Hello, Rustlers!"`, `42`,
`[3, 2, 1]`, `[3, 0]`, `Some(12)`, `Ok((3, 4))`, and `(2, 6)`. The two
`05_mini_cli` capstone starter headers were already present and were not
duplicated. Every one of the 27 starter libraries still has exactly one TODO,
and no solution library contains a TODO.

The diff audit confirms that each affected starter file has exactly four added
comment lines and no implementation or test changes.

## Verification

All requested checks were run after the edits:

| Check | Result |
| --- | --- |
| Structural solution-path check | PASS — 12/12 paths mirror exercises; no old directories or path references |
| Starter-header/TODO check | PASS — 27/27 starters have all four headers and exactly one TODO; no solution TODOs |
| `sh check.sh solutions --run-all` | PASS — `27/27 passed`; `All solutions pass` |
| `cargo check --workspace` | PASS — exit status 0 |
| `cargo fmt --all -- --check` | PASS — exit status 0 |
| `sh scripts/test_check.sh` | PASS — `all shell assertions passed` |
| `sh -n check.sh scripts/test_check.sh scripts/verify_solutions.sh` | PASS — exit status 0 |

For test-first evidence, the structural checks were also run before the fix:
the path check reported all 12 mirrored paths missing and all 12 old
directories present, while the header check reported the four missing header
labels on the 25 affected starters. Both checks passed after the fix.

## Concerns

None.
