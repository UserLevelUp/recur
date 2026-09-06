# Slice 4: Generate and validate playable puzzles in Julia

Status: complete
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-4:v1

## Acceptance and implementation scope

Have Julia produce the playable givens/mask as well as the completed solution.
Validate row/column/box correctness, consistency of givens and exactly one solution;
bound solution counting at two to reject ambiguous puzzles.

Replace candidate-count-only difficulty promises with a documented, reproducible
technique-based rubric. Do not label a puzzle as requiring advanced techniques without
evidence. If the implemented grader cannot justify a difficulty, report that limitation
or retry within a bounded generation budget; do not weaken uniqueness.

Version or add to the puzzle package/API without breaking existing solution/cascade
consumers. The browser should consume the validated givens, not independently remove
new cells and invalidate the guarantee. Specify legacy-package handling explicitly.
Keep Julia as the domain generator and Recur as generic relationship discovery.

Test valid, invalid, ambiguous and bounded-generation-failure cases, all advertised
difficulty paths, legacy behavior and API/browser loading. Publish a consistent package
without exposing partially updated solution/givens/cascades during generation; preserve
the old puzzle on failure. Test in temporary output directories, not the user's live data.

## Required evidence gates

- puzzle-uniqueness-tests
- difficulty-and-package-contract
- generation-integration

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.4 Generate and validate playable puzzles in Julia
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

Added Julia Playable.jl: legal-board validation, capped-at-two MRV solution counting with a node budget, bounded uniqueness-preserving removals, and a replayable naked-single grade. Presets now describe 25/35/45 gaps; ungraded means unsupported by this grader, never an asserted hard puzzle. Additive sudoku-playable-v1 contains all givens/solution/cascades in one atomic publication. Legacy files remain untouched and the fallback UI explicitly reports unverified status.

139 focused Julia assertions pass for legality, ambiguity, budgets, every preset, proof replay, legacy preservation, failed trace/removal preservation, replacement and staging cleanup. Real generation invoked Recur 81 times successfully; it exposed and fixed unconditional --run-name when saved runs were disabled. Real API/browser test passed with all three gap counts, 81 nonempty produce/consume cascades, independent Python uniqueness checks and zero page errors. Browser package tests reject eight malformed cases. Commands and contracts are in html5/TEACHING.md. Temporary cascade source paths are historical generation metadata, not durable live deductions.
