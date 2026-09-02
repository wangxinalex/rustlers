# Parse a setting — solution

## Concept

Validating a small `key=value` configuration line.

## Error-handling rule

`Result<(&str, u32), String>` returns borrowed key text and a parsed value on
success, while invalid format, keys, and values produce readable errors.

## Task

This package contains the reference answer for `parse_setting(line)`.

## Expected behavior/output

`parse_setting("port=8080")` returns `Ok(("port", 8080))`. A wrong key or a
missing value returns `Err`; the demo prints `Ok(("port", 8080))`.

## Hint

Compare the implementation with the exercise README and its tests. The
solution uses `split_once('=')`, validates `"port"`, and parses with `?`.
