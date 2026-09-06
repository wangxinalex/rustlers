# Unicode lengths

## Concept

`text.chars().count()` counts Unicode scalar values, while `text.len()` counts
UTF-8 bytes.

## Task

Implement `text_lengths(text)` so it returns `(chars, bytes)`.

## Expected behavior/output

`text_lengths("你好")` returns `(2, 6)`, while `text_lengths("rust")` returns
`(4, 4)`.

## Hint

Use `text.chars().count()` for Unicode scalar values and `text.len()` for
UTF-8 bytes. Do not slice by byte index.
