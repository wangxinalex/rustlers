# Return a string

## Concept

Constructing an owned `String` from a borrowed string slice.

## Ownership rule

Returning a `String` transfers ownership of the new value to the caller. A
borrowed `&str` can be used to construct that owned result.

## Task

Implement `build_greeting(name)` so it returns `"Hello, {name}!"`.

## Expected behavior/output

`build_greeting("Mia")` returns `"Hello, Mia!"`, and the demo prints that
greeting.

## Hint

Format a new owned `String` from `name`. Read compiler errors before reaching
for `clone`; cloning is not the first fix for an ownership problem.
