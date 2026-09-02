# Mutable string — solution

## Concept

Changing a `String` through a mutable reference.

## Ownership rule

A `&mut String` gives temporary exclusive access for in-place mutation while
the caller keeps ownership of the string.

## Task

This package contains the reference answer for `add_suffix(text, suffix)`.

## Expected behavior/output

Starting with `"learn"`, `add_suffix(&mut text, "ing")` changes `text` to
`"learning"`, and the demo prints `learning`.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; cloning is not the first fix for
an ownership problem.
