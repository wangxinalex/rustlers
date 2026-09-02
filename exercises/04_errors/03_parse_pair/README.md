# Parse a pair

## Concept

Splitting and parsing two signed integers while propagating errors.

## Error-handling rule

`Result<(i32, i32), String>` returns the pair only when both values are valid;
malformed input becomes a learner-readable error.

## Task

Implement `parse_pair(input)` for comma-separated integers, returning an error
when the separator is missing or either value is not numeric.

## Expected behavior/output

`parse_pair("3,4")` returns `Ok((3, 4))`. Missing separators and non-numeric
values return `Err`; the demo prints `Ok((3, 4))`.

## Hint

Use `split_once(',')`, parse each trimmed part, and use `?` to propagate each
failure without defining a custom error type.
