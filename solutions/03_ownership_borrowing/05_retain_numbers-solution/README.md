# Retain non-negative numbers — solution

## Concept

Filtering a vector through a mutable borrow.

## Ownership rule

`&mut Vec<i32>` gives the function exclusive access to change the caller's
vector in place without moving it out of the caller.

## Task

This package contains the reference answer for
`retain_non_negative(numbers)`.

## Expected behavior/output

`[3, -1, 0, -5]` becomes `[3, 0]`, and the demo prints `[3, 0]`.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; cloning is not the first fix for
an ownership problem.
