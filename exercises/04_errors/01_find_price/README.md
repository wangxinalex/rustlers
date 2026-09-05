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

Start with a `for` loop over the borrowed slice. A tuple's `.0` field is
the name and `.1` is the price. Return `Some(price)` when a name matches;
return `None` only after checking all entries.

Alternatively, `.iter().find(...)` takes a closure shaped like
`|item| condition` and returns `Some` containing the first matching reference,
or `None`. Calling `map` on that `Option` applies another closure only to the
found entry, extracting its price; `None` stays `None`. This is an optional
shorter form of the same lookup, not a requirement for this exercise.
