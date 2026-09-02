# Parse a quantity — solution

## Concept

Parsing text into a positive unsigned integer.

## Error-handling rule

`Result<u32, String>` distinguishes a valid quantity from an input that can
be explained to the learner as an error.

## Task

This package contains the reference answer for `parse_quantity(input)`.

## Expected behavior/output

`parse_quantity("7")` returns `Ok(7)`. Invalid text such as `"many"` and the
zero input `"0"` return `Err`; the demo prints `Ok(7)`.

## Hint

Compare the implementation with the exercise README and its tests. The
solution uses `str::parse`, `?`, and a separate zero check.
