# Copy and move — solution

## Concept

Comparing a small integer with an owned `String`.

## Ownership rule

An `i32` implements `Copy`, so assignment leaves the original usable. A
`String` does not implement `Copy`, so assignment moves ownership to the new
binding and the old binding cannot be used.

## Task

This package contains the reference answer for `copy_and_move()`.

## Expected behavior/output

`copy_and_move()` returns `(14, 4)`, where `4` is the length of the moved
`String`.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; cloning is not the first fix for
an ownership problem.
