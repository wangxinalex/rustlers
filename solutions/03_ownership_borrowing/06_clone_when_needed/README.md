# Clone when needed — solution

## Concept

Creating two owned strings from one owned `String` input.

## Ownership rule

`text` is an owned `String`. Moving it into `first` transfers ownership. Both
tuple positions need an owned `String`, so clone `first` to create the second
independent value.

## Task

This package contains the reference answer for
`duplicate_for_two_places(text)`.

## Expected behavior/output

`duplicate_for_two_places(String::from("notes"))` returns `("notes", "notes")`
as two owned strings.

## Hint

Move the input into `first`, then clone `first` because both returned values
need ownership.
