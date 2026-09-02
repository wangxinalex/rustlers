# Retain non-negative numbers

## Concept

Filtering a vector through a mutable borrow.

## Ownership rule

`&mut Vec<i32>` gives the function exclusive access to change the caller's
vector in place without moving it out of the caller.

## Task

Implement `retain_non_negative(numbers)` so it removes every negative number
from the vector.

## Expected behavior/output

`[3, -1, 0, -5]` becomes `[3, 0]`, and the demo prints `[3, 0]`.

## Hint

Use `retain` with a predicate that keeps values greater than or equal to zero.
Read compiler errors before reaching for `clone`; cloning is not the first fix
for an ownership problem.
