# main.improvement.delivery-loop.recurring

Recurring rediscovery point for the engineering loop that keeps showing up in
this repo when an improvement or demo lane becomes real work instead of vague
intent.

## Core Pattern

This repo repeatedly follows the same practical loop:

1. open or refresh one `todo.current` lane
2. record the eventness and desired behavior in `docs/`
3. add or activate a focused Julia test lane that proves the intended contract
4. run the focused test and verify the real failure mode
5. update Rust or Julia implementation to satisfy the failing contract
6. rerun the focused tests until they pass cleanly
7. run the broader verification battery (`cargo test`, focused Julia lanes,
   then full Julia suite when appropriate)
8. update the lane docs to match the new truth
9. collapse the lane into `complete` or `recurring` when the active work is over

## Why It Repeats

- `docs/` holds the live eventness and lane cursor
- Julia tests are the fastest contract surface for CLI behavior and demo flows
- Rust changes usually implement the command behavior that the Julia tests are
  exercising
- final verification proves both the narrow fix and the wider repo still hold

This is not just a `trace-id` habit. `trace-id` makes it more visible because
the identifiers, docs, tests, and code names line up cleanly.

## Code-Side Echo

The same pattern appears in source naming and comments:

- Rust modules often declare their hierarchical name directly, for example
  `main.command.trace-id.impl`
- docs use dot-path names such as `main.command.trace-id.run.todo.current`
- Julia tests mirror the command lane, for example `main.command.trace-id.test`

That means eventness can live in:

- doc file names
- test file names
- module comments
- stable identifiers inside docs or protocol files

## Durable Identifier

`workflow.pattern.docs.tests.rust.verify.complete = recurring`

Supporting lines:

- `workflow.pattern.docs.tests.rust.verify.complete publish docs.current.lane`
- `docs.current.lane subscribe workflow.pattern.docs.tests.rust.verify.complete`
- `workflow.pattern.docs.tests.rust.verify.complete publish tests.contract.first`
- `tests.contract.first subscribe workflow.pattern.docs.tests.rust.verify.complete`
- `workflow.pattern.docs.tests.rust.verify.complete publish rust.impl.align`
- `rust.impl.align subscribe workflow.pattern.docs.tests.rust.verify.complete`
- `workflow.pattern.docs.tests.rust.verify.complete publish verify.targeted.and.full`
- `verify.targeted.and.full subscribe workflow.pattern.docs.tests.rust.verify.complete`
- `workflow.pattern.docs.tests.rust.verify.complete trigger collapse.to.complete`
- `collapse.to.complete subscribe workflow.pattern.docs.tests.rust.verify.complete`

## Use This When

Use this note when we notice ourselves repeating the same move sequence:

- eventness first
- test first
- fix the real implementation
- verify narrowly
- verify broadly
- update the lane truth

If the loop starts to drift, compare the active work against this recurring note
before inventing a new process.

## Discovery

```powershell
recur files "main.improvement.**" -d docs/
recur trace-id "workflow.pattern.docs.tests.rust.verify.complete" --scope "main.improvement.**" --ext .md --json -d docs/
recur find "maps to hierarchical name" --scope "src.main.**" -d src/
```

## Related

- `docs/main.improvement.readme.md`
- `docs/main.command.tests.progress.current.md`
- `docs/main.command.tests.expectation-drift.recurring.md`
- `docs/main.command.trace-id.run.todo.current.md`
