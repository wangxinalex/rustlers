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

Use `str::parse` with `?` for the conversion error, then check that the parsed
quantity is greater than zero.
