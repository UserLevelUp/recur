# Improvement Status Index

This note keeps improvement posture visible in the `main.improvement.*` tree.

## Naming

- `main.improvement.<n>.complete.md` = completed improvement record
- `main.improvement.<n>.todo.future-plan.md` = future-plan / parked improvement
- `main.improvement.<n>.<topic>.todo.current.md` = active focused cursor
- `main.improvement.<n>.<topic>.complete.md` = completed sub-lane under an improvement
- `main.improvement.<n>.<topic>.readme.core.md` = concept or core reference without by-itself implying active state

## Current Snapshot (2026-05-23)

### Complete

- `1`
- `2`
- `3`
- `4`
- `5`
- `6`

### Mostly Complete, With Residual Backlog Or Phase History

- `7`
  phase 1, phase 2, and phase 3 all have `complete` records; a parked
  `todo.future-plan` residue still exists
- `9`
  the `trace-id` sub-lane is complete, while broader future-plan residue remains
- `28`
  seed capability-card query surface is complete; future card authoring remains optional

### Future Plan / Parked

- `8`
- `14`
- `15`
- `17`
- `18`
- `19`
- `20`
- `21`
- `22`
- `25`
- `26`
- `27`
- `29`

### Root-Doc Future Vision, Not Yet Fully Mirrored Into `docs/main.improvement.*`

- `12`
- `13`
- `16`

## Active Cursors Right Now

The improvement tree itself is mostly calm right now.
The active work is showing up more in command and demo lanes than in broad
improvement-number `todo.current` files.

Useful current adjacent lanes:

- `docs/main.command.tests.progress.current.md`
- `docs/main.command.trace-id.run.todo.current.md`
- `docs/main.improvement.pre27and28.todo.current.md`
- `docs/main.demo.skippy.trace-id.todo.current.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
- `docs/main.demo.sudoku.eyeball-order.todo.current.md`

## Why This Exists

We already do use eventness to show whether an improvement is complete or not.

The gap was that some newer proposal-style improvements lived mainly as
`README.CORE.IMPROVEMENT*.md` files at repo root, which made the `docs/`
improvement tree under-report them.

This index keeps the high-level truth easy to scan, while future-plan bridge
notes make the proposal improvements visible in `recur files "main.improvement.**" -d docs/`.

## Discovery

```powershell
recur files "main.improvement.**" -d docs/
recur tree "main.improvement" -d docs/
recur files "README.CORE.IMPROVEMENT**" -d ./
recur files "**.current" -d docs/
```

## Related

- `README.CORE.IMPROVEMENT19.md`
- `README.CORE.IMPROVEMENT20.md`
- `README.CORE.IMPROVEMENT21.md`
- `README.CORE.IMPROVEMENT22.md`
- `README.CORE.IMPROVEMENT25.md`
- `README.CORE.IMPROVEMENT26.md`
- `README.CORE.IMPROVEMENT27.md`
- `README.CORE.IMPROVEMENT28.md`
- `README.CORE.IMPROVEMENT29.md`
- `docs/main.improvement.delivery-loop.recurring.md`
