# Retain non-negative numbers

## Concept

Filtering a vector through a mutable borrow with `retain`.

## Ownership rule

`&mut Vec<i32>` gives the function exclusive access to change the caller's
vector in place without moving it out of the caller. `retain` mutates that
same vector.

## Task

Implement `retain_non_negative(numbers)` so it removes every negative number
from the vector.

## Expected behavior/output

`[3, -1, 0, -5]` becomes `[3, 0]`, and the demo prints `[3, 0]`.

## Hint

`retain` changes the existing vector and takes a predicate: a small function
that returns `true` to keep an element or `false` to remove it. It keeps an
element when the condition is `true`.

You can write the predicate as a closure with the shape `|number| condition`.
The parameter between the bars is the current element, and the expression
after the bars must return a `bool`. Here `number` is an `&i32`, even though
the vector itself is mutably borrowed. `*number >= 0` reads the integer and
keeps non-negative values.

`retain` returns `()`, so call it to mutate the vector in place; no replacement
vector or `clone` is needed.
