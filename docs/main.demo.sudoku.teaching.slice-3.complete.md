# Slice 3: Keep hints and pencil annotations consistent after changes

Status: complete
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

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.3 Keep hints and pencil annotations consistent after changes
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

Game revisions increment on accepted placement and tentative entry/clear. New puzzles and presets reload bound identities; cached records are revalidated before display. Hint requests do not clear attempts or place values. Manual notes override the live computed layer per cell; explicit Auto-fill replaces them. Solved notes are removed and counts refresh immediately.

Browser fixture starts with the screenshot's 19 blanks. Auto-fill reports 19 annotated; placing r2c9=4 reports 18 and removes that row (no stale 20-versus-19 discrepancy). Manual notes survive another move. Tests cover tentative wrong entry, blocked teaching, Backspace correction, rapid hint/move transitions, zero remaining/notes, difficulty navigation, failed generation retaining board and successful reset.
