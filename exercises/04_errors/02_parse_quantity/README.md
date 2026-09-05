# Parse a quantity

## Concept

Parsing text into a positive unsigned integer.

## Error-handling rule

`Result<u32, String>` distinguishes a valid quantity from an input that can
be explained to the learner as an error.

## Task

Implement `parse_quantity(input)` so it accepts positive numbers, rejects
invalid text, and rejects zero with a friendly error.

## Expected behavior/output

`parse_quantity("7")` returns `Ok(7)`. Invalid text such as `"many"` and the
zero input `"0"` return `Err`; the demo prints `Ok(7)`.

## Hint

`parse::<u32>()` returns `Result<u32, ParseIntError>`, but this function must
return errors as `String`. Convert the error with `map_err` before using `?`;
`?` does not automatically turn an error into its text representation.

For example, `.map_err(|error| error.to_string())` changes only the error
type and leaves a successful number untouched. `|error| ...` is a closure:
a small function that receives the error and returns its replacement. You
can return your own friendly `String` message instead.

Then `?` extracts the number on success or returns the converted error from
the function. Check zero separately: it parses successfully but is not a
valid quantity. Return `Ok(quantity)` only for a positive number.
