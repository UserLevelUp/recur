# Slice 0: Capture current playable and test baseline

Status: todo.current
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-0:v1

## Acceptance and implementation scope

Inspect the current Julia generator, Recur wrapper, event files, browser mask/solver,
game state, overlays and pencil UI. Record source revision, dirty state, exact binaries,
browser version and commands. Run the existing Sudoku suites and isolate watcher
checks because their script clears the demo table.

Create reproducible synthetic board fixtures rather than relying on the user's mutable
easy-001 data. Capture the 19-blank screenshot scenario where practical, with provenance
clearly marked as transcribed. Verify board validity before using it as test evidence.
Document stale pencil rows, candidate spoilers, unproved contradiction messages and
the difference between generated relationships and live deductions. Existing reported
test results are historical until rerun. Do not regenerate or overwrite the user's puzzle.

## Required evidence gates

- source-and-runtime-baseline
- reproducible-fixtures

## Observed state and next action

Pending. These are target criteria, not completed implementation or test receipts.
Start by reading main.demo.sudoku.teaching.readme.md and the accepted predecessor, if any.
Retain failing/reproduced behavior, implementation evidence and verified results before
writing an acceptance layer. Keep one current marker within this bubble.

defines: sudoku.teaching.slice.0 Capture current playable and test baseline
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims
