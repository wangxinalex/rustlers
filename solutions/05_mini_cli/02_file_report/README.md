# File report — solution

## Concept

Reading a UTF-8 text file, reusing text statistics, and handling I/O failures
with `Result<String, String>`.

## Task

This package contains the reference answer for `report_file(path)`. Its CLI
accepts one path argument and prints the returned report.

## Expected behavior/output

For a file containing `buy milk\nread book\n`, return
`lines=2 words=4 bytes=19`. A missing path returns an error that includes the
path. With no path argument, print a usage message and exit with status 2.

## Hint

Compare the implementation with the exercise README and tests. The solution
uses `std::fs::read_to_string`, `map_err`, and the standard string counters.
