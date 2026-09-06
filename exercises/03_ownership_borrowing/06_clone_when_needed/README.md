# Clone when needed

## Concept

Creating two owned strings from one borrowed string slice.

## Ownership rule

Moving an owned `String` gives it to one binding. When two places genuinely
need owned values, clone the first owned value for the second place.

## Task

Implement `duplicate_for_two_places(text)` so it returns two equal owned
`String` values.

## Expected behavior/output

`duplicate_for_two_places("notes")` returns `("notes", "notes")` as two owned
strings.

## Hint

Create one owned string, then clone it only because both returned values are
needed. Read compiler errors before reaching for `clone`; first understand
which binding owns the value and why both places need it.
