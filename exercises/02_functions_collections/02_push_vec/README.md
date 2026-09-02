# Push into a vector

## Concept

Mutable references and adding owned `String` values to a `Vec`.

## Task

Implement `add_item(items, item)` so it appends the item to the vector.

## Expected behavior/output

Adding `"tea"` to an empty vector leaves it as `["tea"]`, and the demo prints the vector.

## Hint

Convert the borrowed `&str` to a `String`, then call `push`.
