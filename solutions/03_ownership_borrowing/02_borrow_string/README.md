# Borrow a string — solution

## Concept

Inspecting text through a shared `&str` reference.

## Ownership rule

`&str` is a shared view of text. Passing `&String` coerces to `&str`, and a
string literal is already an `&str`. The function reads the text without
taking ownership, so the caller can keep using its `String`.

## Task

This package contains the reference answer for `length_after_borrow(text)`.

## Expected behavior/output

Both inputs below return `4`:

```rust
let text = String::from("rust");
assert_eq!(length_after_borrow(&text), 4); // `&String` coerces to `&str`
assert_eq!(length_after_borrow("rust"), 4); // string literal: `&str`
```

## Hint

Use `&str` when a function only needs to read text, rather than requiring a
specific `String` allocation.
