# Text statistics — solution

## Concept

Borrowing text and measuring lines, whitespace-separated words, and UTF-8
bytes with the standard library.

## Task

This package contains the reference answer for `stats(text)` and its public
`TextStats` fields: `lines`, `words`, and `bytes`.

## Expected behavior/output

`stats("one two\nthree")` returns `lines=2`, `words=3`, and `bytes=13`.
`stats("")` returns zero for every field. The demo prints the three counters
for the sample text.

## Hint

Compare the implementation with the exercise README and tests. The solution
uses `lines().count()`, `split_whitespace().count()`, and `len()`.
