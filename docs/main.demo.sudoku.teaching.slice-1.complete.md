# Slice 1: Define and verify board-bound deduction records

Status: complete
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

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.1 Define and verify board-bound deduction records
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

Added js/teaching.js and tests/teaching.test.js. Records bind puzzle ID, revision, full board fingerprint, canonical cell IDs, technique, all occupied peer premises, conclusion and highlights. Only naked singles are supported. Tentative attempts block teaching; manual notes are not logical premises. Invalid boards, stale/tampered records and unsupported techniques are rejected. Row-major ordering is deterministic, not a claimed difficulty ranking. Existing cascade JSON is unchanged.

`python demos/sudoku/html5/tests/browser_test.py --baseline` executed ES modules in actual Edge 152.0.4191.62: 44 assertions passed, including 12 single proofs, candidate spoilers staged explicitly, mismatched identity/revision/fingerprint, tampering, duplicate board and no board mutation. Tests use only the accepted board, no stored solution. Implementation is currently uncommitted; evidence is source-bound by subsequent final receipt.
