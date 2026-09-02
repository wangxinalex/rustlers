# Clone when needed — solution

## Concept

Creating two owned strings from one borrowed string slice.

## Ownership rule

Moving an owned `String` gives it to one binding. When two places genuinely
need owned values, clone the first owned value for the second place.

## Task

This package contains the reference answer for
`duplicate_for_two_places(text)`.

## Expected behavior/output

`duplicate_for_two_places("notes")` returns `("notes", "notes")` as two owned
strings.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; first understand which binding
owns the value and why both places need it.
