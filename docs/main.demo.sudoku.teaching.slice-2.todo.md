# Slice 2: Present a clickable progressive teaching hint

Status: todo
Warp: main.demo.sudoku.teaching
Contract: contract:main.demo.sudoku.teaching.slice-2:v1

## Acceptance and implementation scope

Drive one teaching sequence from the accepted deduction record:
look at a group -> notice missing digits -> compare decisive peers -> reveal conclusion.
Provide visible Help with this cell and Find an easier move actions. Do not silently
replace the selected cell when suggesting another. Keep H as a compatible shortcut.

Make the recommendation clickable and highlight the exact group, target and decisive
peers with readable labels such as row 2, column 9; reserve trace IDs for technical detail.
In teaching mode, do not reveal computed candidates or the answer before the user
requests that level. Preserve an explicit way to view candidates.

Label stored-answer disclosure Show solution, not a full logical proof. Remove or
qualify deeper-contradiction claims unless backed by an actual derivation.
When no supported deduction exists, state that honestly and offer another cell or
an explicit solution reveal. Verify keyboard access, focus and spoiler boundaries in
a real browser; do not substitute a screenshot of markup for interaction evidence.

## Required evidence gates

- progressive-hint-tests
- browser-interaction-evidence

## Observed state and next action

Pending. These are target criteria, not completed implementation or test receipts.
Start by reading main.demo.sudoku.teaching.readme.md and the accepted predecessor, if any.
Retain failing/reproduced behavior, implementation evidence and verified results before
writing an acceptance layer. Keep one current marker within this bubble.

defines: sudoku.teaching.slice.2 Present a clickable progressive teaching hint
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims
