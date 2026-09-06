# Slice 3: Keep hints and pencil annotations consistent after changes

Status: todo
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-3:v1

## Acceptance and implementation scope

Increment board identity/revision on relevant state changes. Invalidate or revalidate
cached deductions and overlays after correct placement, tentative entry/correction,
new puzzle and difficulty change. A hint from an older board must not be applied as
current. Read-only hint requests must not place values.

Refresh the pencil panel and counts when a cell is filled. Remove solved-cell rows.
Distinguish player-authored notes from auto-computed candidates: preserve manual intent
unless an explicit operation changes it; recompute the automatic layer as appropriate.
Define rather than silently change existing Auto-fill replacement behavior.
Test rapid selection/hint/move sequences, reset, zero remaining cells, and no stale
highlight or annotation count. Record browser proof of the former 20-annotated versus
19-remaining discrepancy being resolved in a controlled fixture.

## Required evidence gates

- state-invalidation-tests
- pencil-ui-regression

## Observed state and next action

Pending. These are target criteria, not completed implementation or test receipts.
Start by reading main.demo.sudoku.teaching.readme.md and the accepted predecessor, if any.
Retain failing/reproduced behavior, implementation evidence and verified results before
writing an acceptance layer. Keep one current marker within this bubble.

defines: sudoku.teaching.slice.3 Keep hints and pencil annotations consistent after changes
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims
