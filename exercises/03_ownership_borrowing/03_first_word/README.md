# First word

## Concept

Finding a word by borrowing a string slice.

## Ownership rule

A returned `&str` is a borrowed slice of the input, so it does not allocate or
take ownership of the original text. The compiler can infer the relationship
between the input and output lifetimes here.

## Task

Implement `first_word(text)` so it skips leading whitespace and returns the
first remaining word.

## Expected behavior/output

`first_word("hello world")` returns `"hello"`, while `first_word("  rust")`
returns `"rust"`.

## Hint

Split the borrowed text on whitespace and take the first item.
Read compiler errors before reaching for `clone`; cloning is not the first fix
for an ownership problem.
