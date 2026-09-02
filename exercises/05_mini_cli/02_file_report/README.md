# File report

## Concept

Reading a UTF-8 text file, reusing text statistics, and handling I/O failures
with `Result<String, String>`.

## Task

Implement `report_file(path)` so it reads the file and returns one line in the
form `lines=... words=... bytes=...`. The CLI accepts one path argument.

## Expected behavior/output

For a file containing `buy milk\nread book\n`, return
`lines=2 words=4 bytes=19`. A missing path returns an error that includes the
path. With no path argument, print a usage message and exit with status 2.

## Hint

Use `std::fs::read_to_string`, propagate its error through `map_err`, and
apply `lines().count()`, `split_whitespace().count()`, and `len()` to the
loaded text.
