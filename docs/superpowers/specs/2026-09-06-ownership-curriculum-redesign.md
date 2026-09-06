# Ownership Curriculum Redesign

## Goal

Make the Rustlers ownership chapter teach one dependency-ordered idea at a
time. Every starter must remain intentionally incomplete, compile as a
workspace member, and have a corresponding solution that passes its tests.

## Learning-First Acceptance Criteria

Passing a test is evidence, not the learning goal. A learner completing this
chapter must be able to predict who owns a value after an assignment, choose a
shared or mutable borrow for its purpose, explain when a clone is necessary,
and recognize that a returned `&str` borrows rather than allocates.

Each exercise therefore has one primary concept and introduces at most one
new piece of supporting syntax. Its README explains the prerequisite in plain
language before asking the learner to use it; its starter fails for the exact
behavior the learner must supply; and its tests check an observable result or
that a caller can still use a borrowed value. For compile-time-only rules such
as use-after-move, a `compile_fail` example makes the compiler feedback part
of the lesson. Hints explain unfamiliar syntax and the reason it is needed,
but do not provide the final expression.

No exercise is considered well-designed merely because its expected output
can be hard-coded. The exercise body, README, and test together must make the
intended ownership decision the shortest and clearest route to a solution.

## Problem

The current chapter begins with borrowing before it establishes move and
`Copy`; it introduces mutable `Vec` operations, closures, and dereferencing
before those prerequisites are explained; and its copy/move exercise can be
completed with a hard-coded tuple. `02_functions_collections` also introduces
a mutable borrow in `push_vec` before ownership is taught.

## Curriculum Boundaries

`02_functions_collections` remains a five-exercise chapter about functions,
collections, strings, and iteration:

1. `01_parameters`
2. `02_slice_sum`
3. `03_word_count`
4. `04_shopping_total`
5. `05_unicode_lengths`

`03_ownership_borrowing` remains an eight-exercise chapter. Its progression
is now:

1. `01_copy_and_move`: assignment copies `i32` but moves `String`.
2. `02_borrow_string`: a shared `&str` reads text while its owner remains
   usable.
3. `03_mutable_string`: a `&mut String` changes its caller's value in place.
4. `04_push_vec`: a `&mut Vec<String>` changes a collection in place.
5. `05_retain_numbers`: `Vec::retain` uses a small closure over `&i32`.
6. `06_clone_when_needed`: clone only because two owned output values are
   both required.
7. `07_return_string`: construct and return a newly owned `String` from
   borrowed input.
8. `08_first_word`: return a borrowed `&str` slice without allocating.

The move is physical for both `exercises/` and `solutions/`: `push_vec` moves
from chapter 02 to chapter 03, and `unicode_lengths` moves from chapter 03 to
chapter 02. Existing exercise directories are renumbered to match the new
order. Cargo package names, binary imports, workspace members, chapter
READMEs, and the root inventory will all use the new paths.

## Teaching Contracts

Each exercise README will state one concept, the smallest ownership rule that
explains it, a task, observable expected behavior, and a hint limited to the
concepts already introduced.

The `copy_and_move` starter will begin red with a failing behavioral test,
not a pre-filled tuple. Its explanation will include a small `compile_fail`
example showing that a moved `String` binding cannot be used, while a copied
`i32` binding remains usable. This keeps every workspace package compilable
while still making the compiler diagnostic part of the lesson.

`retain_numbers` will explicitly introduce its only new syntax: `retain`
keeps elements when its closure returns `true`; the closure parameter is an
`&i32`; and `*number` reads the integer. No source-inspection test or new
dependency will be added.

`clone_when_needed` will accept an owned `String`, so the exercise isolates
the real choice: retain one owned binding and clone it only for the second
owned result. It will not combine `&str` conversion with cloning.

## Validation

- `cargo check --workspace` proves every incomplete starter and every
  solution still compiles.
- `sh check.sh solutions --run-all` proves all reference solutions pass.
- Each relocated solution package is tested directly at its new path.
- Targeted starter tests are run to confirm each starter begins red rather
  than returning the expected answer unchanged.
- The staged diff is checked for whitespace, renamed package/import
  consistency, and absence of any `working-progress` path.

## Constraints

- Standard library only; no new dependencies or test framework.
- Keep all starter implementations incomplete.
- Preserve one exercise/solution package pair per lesson.
- Work only on branch `curriculum/ownership-exercises`, based on `origin/main`.
- Open a PR to protected `main`; do not push directly to `main`.
