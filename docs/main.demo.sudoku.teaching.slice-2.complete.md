# Slice 2: Present a clickable progressive teaching hint

Status: complete
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

## Observed state

Complete. See verified outcome below and the final source-bound verification receipt.

defines: sudoku.teaching.slice.2 Present a clickable progressive teaching hint
consumes: sudoku.teaching.contract board-bound progressive teaching without false proof claims


## Verified slice outcome

Implemented explicit Help with this cell / Find an easier move, clickable suggestion selection, H shortcut, and four progressive explanation stages. Candidate and stored-solution disclosure are separate actions. No unsupported contradiction claim is made. Focus stays on the next-step control through conclusion; suggesting a cell does not silently select or place it.

`python demos/sudoku/html5/tests/browser_test.py` passes in Edge 152.0.4191.62. Real click/keyboard checks cover spoiler boundaries, row/peer highlights, selection preservation, explicit jump focus, and four steps. See html5/TEACHING.md and final verification receipt. Advanced legacy exports remain available but the teaching UI supports only verified naked singles.
