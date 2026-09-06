# Clone when needed

## Concept

Creating two owned strings from one owned `String`.

## Ownership rule

Moving an owned `String` gives it to one binding. Moving `first` into one
return position would leave no owned value for the other return position. When
two places genuinely need owned values, cloning is the required and deliberate
operation for the second value.

## Task

Implement `duplicate_for_two_places(text)` so it returns two equal owned
`String` values.

## Expected behavior/output

`duplicate_for_two_places(String::from("notes"))` returns `("notes", "notes")`
as two owned strings.

## Hint

Move the input into `first`, then clone `first` only because both returned
values are needed. Read compiler errors before reaching for `clone`; first
understand which binding owns the value and why both places need it.
