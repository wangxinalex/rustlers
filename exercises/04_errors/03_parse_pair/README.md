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

`split_once(',')` returns `Option<(&str, &str)>`: `Some` contains the two
parts, and `None` means the comma is missing. Before using `?` in this
`Result`-returning function, turn that `Option` into a `Result` with
`ok_or(String::from("..."))`, choosing a message for the missing separator.
This turns `Some(parts)` into `Ok(parts)` and `None` into `Err(message)`.

Trim each part and parse it as `i32`. As in `parse_quantity`, convert each
`ParseIntError` to a readable `String` with `map_err` before using `?`.
Once both numbers are available, return the pair inside `Ok`.
