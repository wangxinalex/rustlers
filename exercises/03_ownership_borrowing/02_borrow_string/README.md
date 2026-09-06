# Borrow a string

## Concept

Inspecting text through a shared `&str` reference.

## Ownership rule

Shared borrowing lets a function read text without taking ownership, so the
caller can keep using its `String`. A `&String` can coerce to `&str`, but this
function only needs text data, not a particular owned allocation.

## Task

Implement `length_after_borrow(text)` so it returns the length of borrowed
text.

## Expected behavior/output

`length_after_borrow(&"rust".to_string())` and
`length_after_borrow("rust")` both return `4`, and the caller can still use
its `String` afterward.

## Hint

Choose the shared reference type that represents text data rather than a
specific allocation. Read compiler errors before reaching for `clone`; cloning
is not the first fix for an ownership problem.
