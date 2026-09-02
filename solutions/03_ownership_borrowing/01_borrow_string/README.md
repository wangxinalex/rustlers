# Borrow a string — solution

## Concept

Inspecting a `String` through a shared reference.

## Ownership rule

Shared borrowing with `&String` lets a function read the value without taking
ownership, so the caller can keep using its `String`.

## Task

This package contains the reference answer for `length_after_borrow(text)`.

## Expected behavior/output

`length_after_borrow(&"rust".to_string())` returns `4`, and the demo prints
`4`.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; cloning is not the first fix for
an ownership problem.
