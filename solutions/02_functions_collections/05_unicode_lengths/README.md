# Unicode lengths — solution

## Concept

This pre-ownership lesson accepts `&str`, a view of UTF-8 text. It counts
Unicode text without confusing scalar values and storage bytes.

## Text lengths

`text.len()` counts UTF-8 bytes. `text.chars().count()` counts Unicode scalar
values. These counts differ for text such as `"你好"`, whose two scalar values
use six UTF-8 bytes.

## Task

This package contains the reference answer for `text_lengths(text)`.

## Expected behavior/output

`text_lengths("你好")` returns `(2, 6)`, while `text_lengths("rust")` returns
`(4, 4)`.

## Hint

Return `(text.chars().count(), text.len())`. Do not slice text by byte index.
