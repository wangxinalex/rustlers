# Mutable string

## Concept

Changing a `String` through a mutable reference.

## Ownership rule

A `&mut String` gives temporary exclusive access for in-place mutation while
the caller keeps ownership of the string.

## Task

Implement `add_suffix(text, suffix)` so it appends `suffix` to `text`.

## Expected behavior/output

Starting with `"learn"`, `add_suffix(&mut text, "ing")` changes `text` to
`"learning"`, and the demo prints `learning`.

## Hint

Use `push_str` through the mutable reference. Read compiler errors before
reaching for `clone`; cloning is not the first fix for an ownership problem.
