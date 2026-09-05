# First word — solution

## Concept

Finding a word by borrowing a string slice.

## Ownership rule

A returned `&str` is a borrowed slice of the input, so it does not allocate or
take ownership of the original text. The compiler can infer the relationship
between the input and output lifetimes here.

## Task

This package contains the reference answer for `first_word(text)`.

## Expected behavior/output

`first_word("hello world")` returns `"hello"`, while `first_word("  rust")`
returns `"rust"`. Empty or whitespace-only input returns `""`.

## Hint

Compare the implementation with the exercise README and its test. Read
compiler errors before reaching for `clone`; cloning is not the first fix for
an ownership problem.
