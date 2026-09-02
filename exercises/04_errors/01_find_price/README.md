# Find a price

## Concept

Looking up a value when the requested item may be absent.

## Error-handling rule

`Option<u32>` represents a found price with `Some` or an absent price with
`None`; no error string is needed for a normal missing lookup.

## Task

Implement `find_price(prices, name)` so it returns the matching price or
`None` when the item is not present.

## Expected behavior/output

`find_price(&[("tea", 12)], "tea")` returns `Some(12)`, while an unknown name
returns `None`. The demo prints `Some(12)`.

## Hint

Use `.iter().find` to search the borrowed slice and map the matching tuple to
its price.
