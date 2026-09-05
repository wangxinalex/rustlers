# Parse a setting

## Concept

Validating a small `key=value` configuration line.

## Error-handling rule

`Result<(&str, u32), String>` returns borrowed key text and a parsed value on
success, while invalid format, keys, and values produce readable errors.

## Task

Implement `parse_setting(line)` so it accepts the required `port` key and a
numeric value, rejecting wrong keys and missing values.

## Expected behavior/output

`parse_setting("port=8080")` returns `Ok(("port", 8080))`. A wrong key or a
missing value returns `Err`; the demo prints `Ok(("port", 8080))`.

## Hint

As in `parse_pair`, `split_once('=')` returns an `Option`, not a `Result`.
Use `ok_or` with a `String` error message for a missing separator before
using `?` to obtain the key and value.

Check that the borrowed key is exactly `"port"`. Parse the value as `u32`
and use `map_err` to turn `ParseIntError` into a readable `String` before
using `?`. An empty value fails to parse too. Return the borrowed key and
parsed number inside `Ok`; the key does not need to be cloned.
