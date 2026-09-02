# Unicode lengths

## Concept

Counting Unicode text without confusing characters and UTF-8 storage bytes.

## Ownership rule

Borrowing `&str` lets the function inspect the original text without taking
ownership. Its representation still has UTF-8 length rules that are separate
from ownership.

## Task

Implement `text_lengths(text)` so it returns `(chars, bytes)`.

## Expected behavior/output

`text_lengths("你好")` returns `(2, 6)`, while `text_lengths("rust")` returns
`(4, 4)`.

## Hint

`String::len()` counts UTF-8 bytes and `chars().count()` counts Unicode scalar
values. Do not slice by byte index. Read compiler errors before reaching for
`clone`; cloning is not the first fix for an ownership problem.
