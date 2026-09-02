# Text statistics

## Concept

Borrowing text and measuring lines, whitespace-separated words, and UTF-8
bytes with the standard library.

## Task

Implement `stats(text)` so it returns a `TextStats` value with public
`lines`, `words`, and `bytes` fields.

## Expected behavior/output

`stats("one two\nthree")` returns `lines=2`, `words=3`, and `bytes=13`.
`stats("")` returns zero for every field. The demo prints the three counters
for the sample text.

## Hint

Use `lines().count()`, `split_whitespace().count()`, and `len()`; the empty
string naturally has zero lines.
