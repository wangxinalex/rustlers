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

Use `split_once('=')`, check that the key is exactly `"port"`, then parse the
value as `u32` and propagate conversion errors with `?`.
