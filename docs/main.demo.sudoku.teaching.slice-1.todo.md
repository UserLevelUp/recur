# Slice 1: Define and verify board-bound deduction records

Status: todo
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-1:v1

## Acceptance and implementation scope

Define a versioned browser-side deduction record containing puzzle identity, board
revision or fingerprint, stable target/source cell IDs, technique, explicit premises,
conclusion and highlight references. Start with naked singles; do not require a new
advanced-strategy engine.

Compute premises from the visible logical board, not the stored solution. Separate
accepted values, tentative wrong entries and manual notes in the input contract.
Define rejection of invalid boards, mismatched puzzles, stale records and unsupported
techniques. Test that each accepted deduction follows from its premises, including
negative fixtures. Establish a deterministic ordering without claiming that a heuristic
rank is an objective measure of human difficulty.

Keep existing cascade JSON compatible. Recur classifies authored relationships; it
does not certify mathematical validity. No new generic Recur schema is required.

## Required evidence gates

- deduction-contract
- deduction-soundness-tests

## Observed state and next action

Pending. These are target criteria, not completed implementation or test receipts.
Start by reading main.demo.sudoku.teaching.readme.md and the accepted predecessor, if any.
Retain failing/reproduced behavior, implementation evidence and verified results before
writing an acceptance layer. Keep one current marker within this bubble.

defines: sudoku.teaching.slice.1 Define and verify board-bound deduction records
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims
