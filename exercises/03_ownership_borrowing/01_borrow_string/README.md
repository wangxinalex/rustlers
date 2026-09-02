# Borrow a string

## Concept

Inspecting a `String` through a shared reference.

## Ownership rule

Shared borrowing with `&String` lets a function read the value without taking
ownership, so the caller can keep using its `String`.

## Task

Implement `length_after_borrow(text)` so it returns the length of the borrowed
string.

## Expected behavior/output

`length_after_borrow(&"rust".to_string())` returns `4`, and the demo prints
`4`.

## Hint

Use the borrowed string's `len` method. Read compiler errors before reaching
for `clone`; cloning is not the first fix for an ownership problem.
