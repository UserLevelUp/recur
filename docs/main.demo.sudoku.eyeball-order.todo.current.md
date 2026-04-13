# Demo: Sudoku Eyeball Order + Teaching Bubble

Status: `todo.current`
Date: 2026-04-09

## Purpose

Track the next HTML5 Sudoku teaching lane:

- do not jump straight to the answer
- guide the player toward the next best area to scan
- explain why that scan order matters
- let the player learn stable pattern recognition instead of consuming spoilers

This is the "help me see it" layer, not the "place the digit for me" layer.

## Current Truth

The live HTML5 demo now has a first landed teaching slice:

- `solver.js` exports `eyeballOrder(...)` and ranks unsolved cells by human-facing scan priority
- `cascade.js` renders an `Eyeball Order` block in both selected-cell view and progressive hints
- pressing `H` now cycles: eyeball overlay -> hint ladder -> clear back to no-hint state
- each ranked step carries a clickable `?` thought bubble with:
  - why this target is next
  - what to notice
  - why not elsewhere
  - how to escalate without jumping to the answer
- `game.js` wires the ordered scan guidance into the live selection flow
- the final answer is still reserved for the last hint escalation, not the first

What remains open:

- remove the temporary `game.js` monkey-patch now that the panel owns `renderCellInfo(...)`
- decide whether advanced hint order should exactly mirror the new eyeball-order priority
- add a proper browser or JS syntax/smoke harness when the repo has one available

## Landed Slice

Date: 2026-04-09

- first teaching slice landed in `solver.js`, `game.js`, `cascade.js`, and `css/game.css`
- `demos.sudoku.html5.js.solver.scan.order.rank = active`
- `demos.sudoku.html5.js.game.eyeball.panel.current = active`
- `demos.sudoku.html5.js.cascade.hint.bubble.why.this.first = active`
- `demos.sudoku.html5.js.cascade.hint.bubble.what.to.notice = active`
- `demos.sudoku.html5.js.cascade.hint.bubble.why.not.elsewhere = active`
- `demos.sudoku.html5.js.cascade.hint.bubble.next.escalation = active`
- `demos.sudoku.html5.css.game.eyeball.order.current = active`

## Goal

Add an explicit eyeball order that trains the player how to search the board in
an organized way.

The experience should answer:

1. where should I look first?
2. what pattern am I trying to notice there?
3. why is this area better than the others right now?
4. what should I do next if I still do not see it?

## Desired Behavior

The ordered hint flow should prefer the easiest stable human-recognizable
patterns first:

1. naked single
2. hidden single
3. pointing pair / box-line reduction
4. naked pair
5. x-wing
6. swordfish
7. fallback: fewest candidates + most constrained group

The important part is not just the ranking itself. It is the explanation of the
ranking.

## Planned UI

- add an `Eyeball Order` panel or sub-panel in the existing hint area
- show one or more ranked scan targets, for example:
  - `1. box 5`
  - `2. row 7`
  - `3. sudoku.r7.c3`
- add a clickable `?` beside each step
- clicking `?` opens a small thought bubble:
  - why this target is next
  - what pattern to look for
  - why other regions are lower priority
  - how to escalate without revealing the answer too early

## Demo-Aligned Teaching Identifiers

The doc lane keeps the `main.demo.sudoku.*` file naming convention, but the
identifiers inside the lane should echo the actual demo implementation surfaces:

- `demos.sudoku.julia.*`
- `demos.sudoku.html5.index.*`
- `demos.sudoku.html5.css.game.*`
- `demos.sudoku.html5.js.solver.*`
- `demos.sudoku.html5.js.game.*`
- `demos.sudoku.html5.js.cascade.*`

Primary teaching root:

`demos.sudoku.html5.js.solver.eyeball.order.current = active`

Supporting identifiers:

- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.solver.scan.order.rank`
- `demos.sudoku.html5.js.solver.scan.order.rank subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.solver.scan.pattern.priority`
- `demos.sudoku.html5.js.solver.scan.pattern.priority subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.game.eyeball.panel.current`
- `demos.sudoku.html5.js.game.eyeball.panel.current subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.cascade.hint.bubble.why.this.first`
- `demos.sudoku.html5.js.cascade.hint.bubble.why.this.first subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.cascade.hint.bubble.what.to.notice`
- `demos.sudoku.html5.js.cascade.hint.bubble.what.to.notice subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.cascade.hint.bubble.why.not.elsewhere`
- `demos.sudoku.html5.js.cascade.hint.bubble.why.not.elsewhere subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.js.cascade.hint.bubble.next.escalation`
- `demos.sudoku.html5.js.cascade.hint.bubble.next.escalation subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.css.game.eyeball.order.current`
- `demos.sudoku.html5.css.game.eyeball.order.current subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current publish demos.sudoku.html5.index.eyeball.order.current`
- `demos.sudoku.html5.index.eyeball.order.current subscribe demos.sudoku.html5.js.solver.eyeball.order.current`
- `demos.sudoku.html5.js.solver.eyeball.order.current trigger demos.sudoku.html5.js.game.hint.keep.answer.hidden`
- `demos.sudoku.html5.js.game.hint.keep.answer.hidden subscribe demos.sudoku.html5.js.solver.eyeball.order.current`

## Implementation Shape

1. `solver.js`
   - add a global `eyeballOrder(...)` or equivalent ranking function
   - score unsolved cells by human-friendly solve priority, not just raw candidate count
   - return ordered targets plus explanation metadata

2. `game.js`
   - render the eyeball-order list
   - keep current selected-cell `H` behavior
   - optionally let the user jump from a ranked step into the existing per-cell hint flow

3. `cascade.js` or adjacent UI helper
   - render the `?` thought bubble content
   - keep the copy explanation-first and answer-last

4. later optional trace-id layer
   - mirror the teaching categories in plain-text protocol files
   - let `trace-id` audit why a given eyeball step was chosen
   - keep the live solver responsible for actual board-state ranking

## Module Echo

The eventness should stay close to the actual demo file layout:

- `demos/sudoku/julia/Recur.jl`
  - saved-run and generation support stay in the Julia lane, not in the eyeball-order lane
- `demos/sudoku/html5/js/solver.js`
  - owns ranking, scan order, and pattern-priority reasoning
- `demos/sudoku/html5/js/game.js`
  - owns state, selection, and wiring the eyeball panel into the live play loop
- `demos/sudoku/html5/js/cascade.js`
  - likely owns thought-bubble rendering unless a new small UI helper is cleaner
- `demos/sudoku/html5/index.html`
  - hosts panel containers, buttons, and bubble anchors
- `demos/sudoku/html5/css/game.css`
  - styles the eyeball-order lane and `?` bubble states

## Guardrails

- train recognition before revealing placement
- do not collapse the whole experience into answer vending
- prefer stable pattern language over solver-internal jargon
- keep the explanation short enough to read during play
- let advanced players ignore the bubble and keep moving

## Exit Criteria

- a player can ask "where should I look next?" without getting the answer immediately
- the UI shows an ordered scan target list
- each target can explain itself with a `?` thought bubble
- explanation copy teaches pattern recognition and scan discipline
- final answer remains the last escalation step, not the first

## Discovery

```powershell
recur files "main.demo.sudoku.**" -d docs/
recur trace-id "demos.sudoku.html5.js.solver.eyeball.order.current" --scope "main.demo.sudoku.**" --ext .md --json -d docs/
recur find "hint" --scope "solver.**" -d demos/sudoku/html5/js/
```

## Related

- `docs/main.demo.sudoku.trace-id.todo.current.md`
- `demos/sudoku/html5/js/solver.js`
- `demos/sudoku/html5/js/game.js`
- `demos/sudoku/html5/js/cascade.js`
