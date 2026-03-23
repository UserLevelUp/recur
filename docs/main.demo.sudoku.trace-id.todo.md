# Demo: Sudoku + trace-id (Persistent Tracking)

Status: `todo` (active planning, see `.current` for live context)
Date: 2026-03-08

## Goal

Build a human-driven Sudoku demo with clean separation of concerns:
- **recur** does only what it's good at: hierarchical trace + file filtering
- **Game engine** (Julia CLI + JS/HTML5) owns all Sudoku rules and UI
- **Eventness files** are the protocol between the game and recur
- Human places a value → game engine writes event → recur traces cascade → engine renders

recur does not know it is playing Sudoku.

## Architecture

```
Julia (native recur user)
  ├── Calls recur trace-id, files, merge directly
  ├── Enforces Sudoku rules + constraint propagation
  ├── Writes eventness files (sudoku.solution, masks, flow events)
  ├── Runs as CLI game (live, interactive)
  └── Generates pre-baked puzzle packages for HTML5

HTML5 / JavaScript (recur-free at runtime)
  ├── Loads pre-generated JSON files produced by Julia
  ├── Runs full game in browser — no recur dependency
  ├── JSON structure follows recur's hierarchical conventions
  └── Optional: Julia local server mode for live recur queries
```

### Julia generates the puzzle package
```
puzzles/easy-001/
  sudoku.solution.json      ← recur trace-id output: full grid
  sudoku.mask.easy.json     ← recur files --stdin output: revealed cells
  sudoku.mask.medium.json
  sudoku.mask.hard.json
  sudoku.cascades.json      ← pre-computed trace-id for every cell placement
  sudoku.merge.json         ← recur merge output: unified row/col/box view
```
Everything HTML5 needs is in these files. Julia ran recur. JavaScript just reads JSON.

### Two runtime modes
1. **Static mode** — Julia pre-generates puzzle package → HTML5 loads it → fully offline
2. **Live mode** — Julia runs as local server → HTML5 calls Julia → Julia calls recur live

Static mode is the default. Live mode enables on-the-fly trace-id for novel placements.

## Why This Demo Matters

This is a **full recur capability showcase** using Sudoku as the narrative thread.
Every major command family gets a dedicated scene:

| Scene | Command | What it shows |
|---|---|---|
| 1 | `tree` + `--sep` | Same grid, three separator views (./_ /-) |
| 2 | `files --stdin` | Difficulty mask as Unix pipe — no special API |
| 3 | `children` + `related` | Constraint peer groups as hierarchy scopes |
| 4 | `stats` | Puzzle complexity as hierarchy depth/width |
| 5 | `find` | Candidate and trigger pattern search |
| 6 | `merge` | Multi-separator constraint unification |
| 7 | `trace-id` | **Full showcase** — format, json, scope, glob, depth, guardrails, trait config |

trace-id is the centrepiece. The demo shows every flag: `--format`, `--json`,
`--scope`, glob patterns, `--depth`, `--depth-guard`, `--force`, and `recur trait set`
for live keyword tuning — all in a Sudoku context that makes the output intuitive.

- recur stays pure — domain-agnostic, no Sudoku logic baked in
- Julia is the recur bridge — calls recur, generates puzzle packages for HTML5
- HTML5 loads pre-baked JSON — full browser game, zero recur dependency at runtime
- Any language that reads JSON can play a recur-powered Sudoku game

## Phases

1. File protocol spec + keyword vocabulary (`publish`, `subscribe`, `trigger solve`)
2. Julia CLI prototype — hardcoded puzzle, call recur, verify cascade JSON
3. Julia puzzle package generator — produce all JSON artifacts for one puzzle
4. Julia CLI game loop — playable terminal game driven by recur
5. HTML5 static game — load puzzle package, full browser game, no recur at runtime
6. Optional: Julia local server mode for live recur queries from HTML5
7. Demo script (`demos/sudoku/demo.ps1` — scripted walkthrough)

## Deferred Until

- trace-stats phase 3 complete (improvement 7)
- trace-id tests stabilized (improvement 8)

## Related

- `docs/main.demo.sudoku.trace-id.todo.current.md` — active planning
- `demos/ascii-drinks/` — beer demo reference
