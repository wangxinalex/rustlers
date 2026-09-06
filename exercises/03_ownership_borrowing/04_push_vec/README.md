# Push into a vector

## Concept

Changing a `Vec<String>` through a mutable reference.

## Ownership rule

A `&mut Vec<String>` gives temporary exclusive access to the caller-owned
vector. `to_owned()` creates the owned `String` required by the vector; the
vector itself stays owned by the caller.

## Task

Implement `add_item(items, item)` so it appends the item to the vector.

## Expected behavior/output

Adding `"tea"` to an empty vector leaves it as `["tea"]`, and the demo prints
the vector.

## Hint

The vector stores owned strings, while the item is borrowed text. Consider the
conversion that makes an owned value before adding it.
