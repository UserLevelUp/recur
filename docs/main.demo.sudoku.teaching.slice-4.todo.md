# Slice 4: Generate and validate playable puzzles in Julia

Status: todo
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

## Observed state and next action

Pending. These are target criteria, not completed implementation or test receipts.
Start by reading main.demo.sudoku.teaching.readme.md and the accepted predecessor, if any.
Retain failing/reproduced behavior, implementation evidence and verified results before
writing an acceptance layer. Keep one current marker within this bubble.

defines: sudoku.teaching.slice.4 Generate and validate playable puzzles in Julia
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims
