# Find a price — solution

## Concept

Looking up a value when the requested item may be absent.

## Error-handling rule

`Option<u32>` represents a found price with `Some` or an absent price with
`None`; no error string is needed for a normal missing lookup.

## Task

This package contains the reference answer for `find_price(prices, name)`.

## Expected behavior/output

`find_price(&[("tea", 12)], "tea")` returns `Some(12)`, while an unknown name
returns `None`. The demo prints `Some(12)`.

## Hint

Compare the implementation with the exercise README and its tests. The
solution uses `.iter().find` on the borrowed slice.
