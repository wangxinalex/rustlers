# First word

## Concept

Finding a word by borrowing a string slice.

## Ownership rule

A returned `&str` is a borrowed slice of the input, so it does not allocate or
take ownership of the original text. The compiler can infer the relationship
between the input and output lifetimes here.

## Task

Implement `first_word(text)` so it skips leading whitespace and returns the
first remaining word. Return `""` if there is no word.

## Expected behavior/output

`first_word("hello world")` returns `"hello"`, while `first_word("  rust")`
returns `"rust"`. Empty or whitespace-only input returns `""`.

## Hint

`split_whitespace()` produces borrowed words and skips leading whitespace.
`split_whitespace().next()` yields `Option<&str>`: `Some(word)` when a word
exists, or `None` when there is no word. `Option` will be covered in more
detail in the next chapter; here it lets you handle an empty result safely.

Use `unwrap_or("")` to extract the word with a safe fallback. Unlike
`unwrap()`, this handles empty and whitespace-only inputs without panicking or
allocating. The word already borrows from the input, so no `String` allocation
or `clone` is needed.
