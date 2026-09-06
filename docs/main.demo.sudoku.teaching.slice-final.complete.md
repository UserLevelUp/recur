# Slice final: Verify the complete teaching loop and close out

Status: complete
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-final:v1

## Acceptance and implementation scope

Exercise the full browser loop on a reproducible unique puzzle: request an easier move,
follow the progressive explanation, choose a value, observe updated hints and pencil
state, correct a tentative entry, change/reset the puzzle and finish a board.
Verify no unsolicited placement, premature spoiler, stale deduction, false proof claim,
lost focus or new browser-console error. Record actual environment and observations.

Run existing Cargo/Julia regressions plus added deduction/UI/generation tests. Report
ignored or expected-broken cases separately; do not rewrite existing expectations merely
to make new behavior pass. Any intentional schema or behavior change needs a documented
compatibility decision and regression coverage. Do not claim browser verification when
only source or unit tests were inspected.

Retain exact commands, source binding, fixtures, results, screenshots/trace references
and remaining limitations. Final acceptance depends on every preceding slice. Only
after gates pass, collapse this bubble's active markers and update reveal. The separate
Improvement 27 docs-reconciliation Warp is not completed by this work.

## Required evidence gates

- end-to-end-browser-receipt
- full-regression
- evidence-and-eventness-closeout

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.final Verify the complete teaching loop and close out
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

All preceding slices passed their targeted checks. End-to-end browser and real Julia API tests passed; Cargo 179 passed / 7 ignored; full Julia 2723 passed / 73 expected broken / zero failed. Exact commands, environment, compatibility decisions, local screenshot references, limitations and source-byte hashes are recorded in main.demo.sudoku.teaching.verification.md and .sources.json. No core Recur changes; no live user puzzle regeneration. Reveal and readme now point to completed evidence. Layer references represent reviewed declared evidence, not a claim that Recur itself ran the tests.
