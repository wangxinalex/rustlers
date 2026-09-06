# Ownership Curriculum Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebuild the functions/collections and ownership chapters into a dependency-ordered, learning-first curriculum without changing the total exercise count.

**Architecture:** Move `push_vec` into ownership and move `unicode_lengths` into functions/collections. Renumber the package pairs, then revise every ownership starter, solution, README, binary import, workspace member, and inventory entry so each lesson has one primary learning outcome and starts incomplete.

**Tech Stack:** Rust 2024, Cargo workspace, shell verification, Rust standard library only.

**Spec:** `docs/superpowers/specs/2026-09-06-ownership-curriculum-redesign.md`

## Global Constraints

- Work only on `curriculum/ownership-exercises`, based on `origin/main`.
- Do not read, stage, copy, commit, or push any `working-progress` content.
- Keep every starter package compilable and intentionally incomplete.
- Add no dependencies, source-inspection tests, or test framework.
- A learner-facing README introduces the prerequisite syntax before relying on it.
- Direct pushes to `main` are forbidden; submit the completed branch as a PR.

---

## Final Package Map

| Source package | Destination package | Primary lesson |
| --- | --- | --- |
| `02/01_parameters` | `02/01_parameters` | parameters and returned values |
| `02/03_slice_sum` | `02/02_slice_sum` | read-only slices |
| `02/04_word_count` | `02/03_word_count` | splitting and iteration |
| `02/05_shopping_total` | `02/04_shopping_total` | collection calculation |
| `03/08_unicode_lengths` | `02/05_unicode_lengths` | Unicode scalars versus UTF-8 bytes |
| `03/06_copy_and_move` | `03/01_copy_and_move` | `Copy` versus move |
| `03/01_borrow_string` | `03/02_borrow_string` | shared borrow as `&str` |
| `03/04_mutable_string` | `03/03_mutable_string` | mutable borrow of `String` |
| `02/02_push_vec` | `03/04_push_vec` | mutable borrow of `Vec<String>` |
| `03/05_retain_numbers` | `03/05_retain_numbers` | `retain`, closure, and `&i32` |
| `03/07_clone_when_needed` | `03/06_clone_when_needed` | clone for two owned outputs |
| `03/02_return_string` | `03/07_return_string` | returning a new owned `String` |
| `03/03_first_word` | `03/08_first_word` | returned borrowed slice |

Apply the same map under both `exercises/` and `solutions/`.

### Task 1: Move and renumber package pairs

**Files:**
- Move: all package directories in the Final Package Map, including their `Cargo.toml`, `README.md`, `src/lib.rs`, and `src/main.rs`.
- Modify: `Cargo.toml`
- Modify: every moved package `Cargo.toml` and `src/main.rs`

**Interfaces:**
- Consumes: the existing public exercise function signatures.
- Produces: the final chapter paths and crate names used by chapter documentation and validation commands.

- [ ] **Step 1: Move each package pair with Git-aware renames**

Use a temporary in-repository staging directory to avoid destination-name
collisions, move both exercise and solution directories through it with
`git mv`, then place them at exactly the destinations in the Final Package
Map. Remove the empty staging directories before continuing.

- [ ] **Step 2: Update every moved manifest package name**

Set manifests to the destination naming convention. For example:

```toml
[package]
name = "rustlers-03-ownership-borrowing-01-copy-and-move"
version = "0.1.0"
edition = "2024"
```

The matching solution manifest uses:

```toml
name = "rustlers-03-ownership-borrowing-01-copy-and-move-solution"
```

Apply the analogous chapter/number/slug update to every renamed package.

- [ ] **Step 3: Update binary imports to the renamed library crates**

For the copy/move exercise binary, the final call is:

```rust
println!(
    "{:?}",
    rustlers_03_ownership_borrowing_01_copy_and_move::copy_and_move()
);
```

Use the corresponding `_solution` crate names under `solutions/`; update all
other moved `src/main.rs` imports in the same way.

- [ ] **Step 4: Rewrite workspace member ordering**

Replace the affected `Cargo.toml` member lists with the Final Package Map in
chapter and numeric order for both `exercises/` and `solutions/`. Retain all
unaffected members unchanged.

- [ ] **Step 5: Verify the move plumbing**

Run: `cargo check --workspace`

Expected: all renamed packages compile; no unresolved library-crate imports
or missing workspace paths occur.

- [ ] **Step 6: Commit the package move**

```bash
git add Cargo.toml exercises solutions
git commit -m "refactor(curriculum): reorder ownership prerequisites"
```

### Task 2: Establish the revised functions/collections chapter

**Files:**
- Modify: `exercises/02_functions_collections/README.md`
- Modify: `exercises/02_functions_collections/05_unicode_lengths/README.md`
- Modify: `exercises/02_functions_collections/05_unicode_lengths/src/lib.rs`
- Modify: `solutions/02_functions_collections/05_unicode_lengths/src/lib.rs`
- Modify: the matching relocated `src/main.rs` files

**Interfaces:**
- Consumes: `pub fn text_lengths(text: &str) -> (usize, usize)`.
- Produces: a five-exercise prerequisite chapter that no longer asks learners
  to perform a mutable borrow before the ownership chapter.

- [ ] **Step 1: Rewrite the chapter sequence**

List exactly `01_parameters`, `02_slice_sum`, `03_word_count`,
`04_shopping_total`, and `05_unicode_lengths`. Describe the chapter as
functions, collections, strings, slices, and iteration; do not present
mutable ownership as a prerequisite.

- [ ] **Step 2: Make the Unicode task self-contained**

State that `text.chars().count()` counts Unicode scalar values and
`text.len()` counts UTF-8 bytes. Keep `"你好" -> (2, 6)` and
`"rust" -> (4, 4)` as the two learner-visible examples. The starter is:

```rust
pub fn text_lengths(_text: &str) -> (usize, usize) {
    unimplemented!("count characters and UTF-8 bytes")
}
```

The reference implementation is:

```rust
pub fn text_lengths(text: &str) -> (usize, usize) {
    (text.chars().count(), text.len())
}
```

- [ ] **Step 3: Verify the relocated reference package**

Run: `cargo test --manifest-path solutions/02_functions_collections/05_unicode_lengths/Cargo.toml`

Expected: two tests pass for Unicode and ASCII input.

- [ ] **Step 4: Commit the chapter revision**

```bash
git add exercises/02_functions_collections solutions/02_functions_collections
git commit -m "docs(curriculum): place Unicode with string collections"
```

### Task 3: Build the move-to-mutable-borrow learning sequence

**Files:**
- Modify: `exercises/03_ownership_borrowing/01_copy_and_move/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/01_copy_and_move/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/02_borrow_string/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/02_borrow_string/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/03_mutable_string/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/03_mutable_string/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/04_push_vec/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/04_push_vec/src/lib.rs`

**Interfaces:**
- Produces `copy_and_move() -> (i32, usize)`;
  `length_after_borrow(&str) -> usize`;
  `add_suffix(&mut String, &str)`; and
  `add_item(&mut Vec<String>, &str)`.

- [ ] **Step 1: Make the copy/move starter fail for the lesson**

Use this starter and keep the existing expected result test:

```rust
/// ```compile_fail
/// let text = String::from("move");
/// let moved_text = text;
/// println!("{text}");
/// println!("{moved_text}");
/// ```
pub fn copy_and_move() -> (i32, usize) {
    unimplemented!("show an i32 copy and a moved String")
}
```

Its README must ask learners to predict why a second use of an `i32` binding
is valid while a second use of the moved `String` binding is rejected.

- [ ] **Step 2: Verify copy/move starts red, then write the solution**

Run: `cargo test --manifest-path exercises/03_ownership_borrowing/01_copy_and_move/Cargo.toml`

Expected: the starter test fails because `unimplemented!` panics.

Write the reference implementation:

```rust
pub fn copy_and_move() -> (i32, usize) {
    let number = 14;
    let _copied_number = number;
    let number_still_usable = number;

    let text = String::from("move");
    let moved_text = text;

    (number_still_usable, moved_text.len())
}
```

- [ ] **Step 3: Make shared borrowing use the idiomatic boundary**

Change the interface to `length_after_borrow(text: &str) -> usize`. Keep a
test that passes `&String` and then uses that `String`, and add a second test
that passes a string literal. The solution is `text.len()`. Explain that
`&String` can coerce to `&str`, but the function only needs text data.

- [ ] **Step 4: Keep mutable string mutation focused**

Use an `unimplemented!` starter for `add_suffix`; retain the test that calls
`add_suffix(&mut text, "ing")` and observes `"learning"`. The solution is
`text.push_str(suffix)` and the README explains exclusive, temporary access.

- [ ] **Step 5: Move mutable vector mutation after mutable strings**

Use an `unimplemented!` starter for `add_item`. Preserve the public
signature `add_item(items: &mut Vec<String>, item: &str)`, the caller-owned
vector test, and the reference implementation:

```rust
pub fn add_item(items: &mut Vec<String>, item: &str) {
    items.push(item.to_owned());
}
```

Its README should state that `to_owned()` creates the owned `String` required
by the vector; the vector itself stays owned by the caller.

- [ ] **Step 6: Verify the four reference packages**

Run:

```bash
sh check.sh solutions/03_ownership_borrowing/01_copy_and_move
sh check.sh solutions/03_ownership_borrowing/02_borrow_string
sh check.sh solutions/03_ownership_borrowing/03_mutable_string
sh check.sh solutions/03_ownership_borrowing/04_push_vec
```

Expected: each solution package passes all unit and documentation tests.

- [ ] **Step 7: Commit the foundation sequence**

```bash
git add exercises/03_ownership_borrowing solutions/03_ownership_borrowing
git commit -m "feat(curriculum): teach ownership from moves to mutable borrows"
```

### Task 4: Complete the ownership decisions and borrowed-slice sequence

**Files:**
- Modify: `exercises/03_ownership_borrowing/05_retain_numbers/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/05_retain_numbers/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/06_clone_when_needed/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/06_clone_when_needed/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/07_return_string/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/07_return_string/src/lib.rs`
- Modify: `exercises/03_ownership_borrowing/08_first_word/{README.md,src/lib.rs,src/main.rs}`
- Modify: `solutions/03_ownership_borrowing/08_first_word/src/lib.rs`

**Interfaces:**
- Produces `retain_non_negative(&mut Vec<i32>)`,
  `duplicate_for_two_places(String) -> (String, String)`,
  `build_greeting(&str) -> String`, and `first_word(&str) -> &str`.

- [ ] **Step 1: Explain every element of `retain` before using it**

The README must state: `retain` mutates the same vector; its closure has the
form `|number| condition`; it keeps elements when the condition is `true`;
`number` is an `&i32`; and `*number >= 0` reads and keeps non-negative
values. Use this starter and solution:

```rust
pub fn retain_non_negative(_numbers: &mut Vec<i32>) {
    unimplemented!("retain only non-negative values")
}

pub fn retain_non_negative(numbers: &mut Vec<i32>) {
    numbers.retain(|number| *number >= 0);
}
```

- [ ] **Step 2: Make clone demonstrate a genuine ownership need**

Change the starter, binary, tests, and solution to accept an owned `String`:

```rust
pub fn duplicate_for_two_places(_text: String) -> (String, String) {
    unimplemented!("return two owned copies")
}

pub fn duplicate_for_two_places(text: String) -> (String, String) {
    let first = text;
    let second = first.clone();
    (first, second)
}
```

Call it with `String::from("notes")`. The README must explain that moving
`first` would leave no owned value for the other return position, so cloning
is the required and deliberate operation.

- [ ] **Step 3: Separate constructing ownership from cloning**

Keep `build_greeting(name: &str) -> String`, but make its starter
`unimplemented!` and explain that `format!` allocates a new owned result for
the caller. The reference implementation remains:

```rust
pub fn build_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}
```

- [ ] **Step 4: End with a returned borrowed slice**

Keep the complete empty-input test set for `first_word`. Its README explains
that `split_whitespace().next()` yields `Option<&str>` and that
`unwrap_or("")` chooses a safe fallback without allocating. Use an
`unimplemented!` starter and this solution:

```rust
pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

- [ ] **Step 5: Verify the four reference packages**

Run:

```bash
sh check.sh solutions/03_ownership_borrowing/05_retain_numbers
sh check.sh solutions/03_ownership_borrowing/06_clone_when_needed
sh check.sh solutions/03_ownership_borrowing/07_return_string
sh check.sh solutions/03_ownership_borrowing/08_first_word
```

Expected: each solution package passes all unit tests.

- [ ] **Step 6: Commit the decision and slice sequence**

```bash
git add exercises/03_ownership_borrowing solutions/03_ownership_borrowing
git commit -m "feat(curriculum): clarify ownership decisions and slices"
```

### Task 5: Update learner navigation and prove the curriculum contract

**Files:**
- Modify: `README.md`
- Modify: `exercises/README.md`
- Modify: `exercises/02_functions_collections/README.md`
- Modify: `exercises/03_ownership_borrowing/README.md`

**Interfaces:**
- Consumes: the Final Package Map and final public function signatures.
- Produces: accurate learner-facing paths, totals, concepts, and commands.

- [ ] **Step 1: Rewrite the root inventory**

Keep the total at 27. List chapter 02 as five functions/collections/string
exercises ending in `05_unicode_lengths`; list chapter 03 as the eight
ownership lessons in their final order. Replace the ownership description so
it promises moves, `Copy`, shared and mutable borrows, `clone`, owned return
values, and slices, without claiming Unicode is part of this chapter.

- [ ] **Step 2: Rewrite the two chapter READMEs and exercises index**

The chapter 02 README must point learners to chapter 03 only after its five
lessons. The chapter 03 README must present the move-to-slice progression and
say that `retain_numbers` introduces its closure syntax locally. The exercises
index must describe chapter 02 as functions/collections/strings and chapter
03 as ownership, borrowing, and slices.

- [ ] **Step 3: Verify starter feedback and solution behavior**

Run these starter commands and confirm each fails because the starter remains
incomplete, not because a package path or import is wrong:

```bash
sh check.sh exercises/03_ownership_borrowing/01_copy_and_move
sh check.sh exercises/03_ownership_borrowing/05_retain_numbers
sh check.sh exercises/03_ownership_borrowing/06_clone_when_needed
```

Then run:

```bash
cargo check --workspace
sh check.sh solutions --run-all
```

Expected: all starters compile; the three targeted starters fail their
behavior tests; all 27 solution packages pass.

- [ ] **Step 4: Run final consistency checks**

Run:

```bash
rg -n "02_push_vec|03_ownership_borrowing/08_unicode_lengths|03_ownership_borrowing/06_copy_and_move" README.md exercises Cargo.toml solutions
git diff --check
git status --short
```

Expected: references use the Final Package Map, whitespace is clean, and the
only untracked build artifact remains `Cargo.lock` outside the staged diff.

- [ ] **Step 5: Commit documentation and validation updates**

```bash
git add README.md exercises/README.md exercises/02_functions_collections exercises/03_ownership_borrowing Cargo.toml solutions
git commit -m "docs: align exercise navigation with ownership progression"
```
