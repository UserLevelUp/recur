# Slice 0: Capture current playable and test baseline

Status: complete
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

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.0 Capture current playable and test baseline
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

Baseline HEAD 4b9d8af805a3e35040bf3a1a2fdfc211cbdc0ac3; clean before implementation. Recur Git checkpoint ck-sudoku-teaching-plan-20260905 at unix:1788675756 captured preceding plan state, not file contents. Runtime: Julia 1.12.0; Edge 152.0.4191.62; repository target/release-safe Recur binaries. Cargo test --locked passed (179 tests; 7 ignored doctests). Sudoku phases 1-4 rerun: 32 + 20 + 53 + 39 assertions passed. Isolated watcher mirror at TEMP/recur-sudoku-check-1b95cf43f3cc4c80ac23100b13cc7027: 3 moves / 3 results in 0.38s; live table untouched.

Browser: disposable Playwright 1.62.0 venv; `python demos/sudoku/html5/tests/browser_test.py --baseline` reproduced immediate candidate disclosure with zero page errors. Screenshot retained locally at TEMP/sudoku-teaching-baseline.png. Synthetic screenshot board is in tests/teaching.test.js (19 blanks, 12 naked singles); 44 deduction assertions pass. Source inspection confirms missing pencil refresh, stored-answer-based contradiction claim, and static relationship cascades distinct from live proofs. Neither live easy-001 package was regenerated.
