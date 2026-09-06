# Copy and move

## Concept

Comparing a small integer with an owned `String`.

## Ownership rule

An `i32` implements `Copy`, so assignment leaves the original usable. A
`String` does not implement `Copy`, so assignment moves ownership to the new
binding and the old binding cannot be used.

## Task

Before coding, predict why a second use of an `i32` binding after assignment
is valid, while a second use of the moved `String` binding is rejected.
Then implement `copy_and_move()` so it returns `(14, 4)` and makes the two
assignment behaviors visible.

## Expected behavior/output

`copy_and_move()` returns `(14, 4)`, where `4` is the length of the moved
`String`.

## Hint

Assign `14` to another integer binding and use the original afterward. Then
assign a four-letter `String` and use its new binding. Read compiler errors
before reaching for `clone`; cloning is not the first fix for an ownership
problem.
