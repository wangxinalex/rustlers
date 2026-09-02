# Match a command

## Concept

Matching a string against known patterns with `match`.

## Task

Return `"starting"` for `"start"` and `"unknown command"` for anything else.

## Expected behavior/output

`command_label("start")` returns `"starting"`; unknown commands return `"unknown command"`.

## Hint

Use one match arm for `"start"` and `_` for the fallback.
