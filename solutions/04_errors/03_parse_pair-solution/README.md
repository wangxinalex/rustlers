# Parse a pair — solution

## Concept

Splitting and parsing two signed integers while propagating errors.

## Error-handling rule

`Result<(i32, i32), String>` returns the pair only when both values are valid;
malformed input becomes a learner-readable error.

## Task

This package contains the reference answer for `parse_pair(input)`.

## Expected behavior/output

`parse_pair("3,4")` returns `Ok((3, 4))`. Missing separators and non-numeric
values return `Err`; the demo prints `Ok((3, 4))`.

## Hint

Compare the implementation with the exercise README and its tests. The
solution uses `split_once(',')`, `parse`, and `?` for propagation.
