# Warp: Sudoku teaching

Created: 2026-09-05
State: planned; Slice 0 current; no slices accepted.
Warp identity: main.demo.sudoku.teaching

## Desired result

The player learns one justified deduction at a time through a clickable,
spoiler-controlled explanation grounded in the actual board. Hints and pencil
annotations stay current, and playable puzzles have validated uniqueness and
honestly described difficulty.

## Slice trajectory

- Slice 0: Capture current playable and test baseline.
- Slice 1: Define and verify board-bound deduction records.
- Slice 2: Present a clickable progressive teaching hint.
- Slice 3: Keep hints and pencil annotations consistent after changes.
- Slice 4: Generate and validate playable puzzles in Julia.
- Slice final: Verify the complete teaching loop and close out.

## Architecture and scope

Julia owns generation/validation of playable Sudoku packages and authoring relationship
files. Recur classifies and exposes those generic relationships. Browser JavaScript
computes live board-bound deductions and renders the progressive interaction.
Pre-generated cascades are dependency context, not proofs of current-board deductions.

Production scope: demos/sudoku/julia/{Generator,Engine,Recur}.jl as required,
demos/sudoku/html5/{generate,serve}.jl, relevant HTML/CSS/JS, package fixtures,
focused browser/Julia tests and demo docs. Inspect Game.jl for compatibility but
do not rewrite the terminal game unless a demonstrated shared-contract change requires it.
Core Recur behavior, unrelated project files and the user's existing live puzzle are
outside implementation scope.

Existing docs/main.demo.sudoku.eyeball-order.todo.current.md is an input to the
baseline audit, not an additional instruction to execute a competing plan. Reconcile
its status only when evidence warrants it; preserve its historical intent.

## Explicit non-goals

- No live deduction persistence/export service, watcher orchestration rewrite or new
  core Recur language is required in this Warp.
- No new advanced Sudoku strategy catalog, AI-generated explanation service or
  automatic player is required.
- No automatic Git operations, installed-binary replacement or release publication.
- No claim that trace classification validates a Sudoku deduction.
- No acceptance based solely on existing green Julia tests for a browser change.

## Verification and compatibility

Use synthetic/reproducible fixtures and temporary generation outputs. Preserve existing
test expectations; add targeted cases for the actual changed contracts. An intentional
compatibility change must be identified and agreed, not silently introduced.
Browser interaction and console evidence are required in addition to mathematical
and regression tests. Missing browser tooling is an explicit gate limitation, not a pass.

The final map requires six slices. Contract values are opaque versioned identifiers,
not cryptographic hashes. Gate references record reviewed evidence; do not label
them machine-checked external evidence unless that mode is explicitly adopted.
Record source fingerprints and actual outcomes before accepting each slice.

## Rehydrate

```powershell
recur reveal main.demo.sudoku.teaching
recur warp map main.demo.sudoku.teaching -d docs --json
recur warp merge main.demo.sudoku.teaching -d docs --json
recur files "main.demo.sudoku.teaching.**.current" -d docs
recur trace-id "sudoku.teaching.**" --scope "main.demo.sudoku.teaching.**" -d docs --ext .md --format full
```

Use target/release-safe/recur.exe if the installed executable differs.
Creating this Warp does not implement the teaching feature or accept any slice.

defines: sudoku.teaching.contract board-bound progressive teaching with unique playable puzzles and preserved query/execution boundaries
