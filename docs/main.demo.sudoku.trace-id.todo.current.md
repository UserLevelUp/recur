# Demo: Sudoku + trace-id (Pure Separation Architecture)

Status: `todo.current` (active analysis / planning)
Date: 2026-03-08

---

## Core Principle: recur is a Discoverability Engine

**recur does not know it is playing Sudoku.**

recur's purpose is discoverability of interesting things through eventness.
In a Sudoku game, recur helps a human, LLM, or game engine discover:

| Question | recur command |
|---|---|
| What cells are in row 3? | `recur children "sudoku.row.3"` |
| What cells share constraints with r3.c5? | `recur related "sudoku.row.3.c5"` |
| What happened when I placed 7 here? | `recur trace-id "sudoku.r3.c5" --json` |
| What's still unsolved? | `recur stats "sudoku.**"` |
| Which cells triggered cascades? | `recur find "trigger solve" --scope "sudoku.**"` |
| How complex is this propagation? | `recur trace-stats --scope "sudoku.**"` |
| What does the easy mask reveal? | `cat mask.easy \| recur files "sudoku.**" --stdin` |

recur operates at three layers simultaneously — all just hierarchical text:
1. **Game state** — the eventness files (`sudoku.solution`, `sudoku.flow.*`)
2. **Engine code** — the Julia source (`Engine.jl`, `Recur.jl`)
3. **Tool docs** — recur's own command surface (`docs/main.command.**`)

Everything else — puzzle rules, grid display, difficulty, human interaction,
game loop — lives in the **game engine**, written in whatever language fits.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│  Game Engine (Julia or JavaScript/HTML5)         │
│                                                  │
│  - Enforces Sudoku rules                         │
│  - Manages human input                           │
│  - Writes eventness files (hierarchical format)  │
│  - Calls: recur trace-id --json                  │
│  - Calls: recur merge --json                     │
│  - Calls: recur files --stdin (mask filtering)   │
│  - Interprets JSON output → renders UI           │
└──────────────────┬──────────────────────────────┘
                   │  writes/reads eventness files
                   ▼
┌─────────────────────────────────────────────────┐
│  Eventness Files (hierarchical identifiers)      │
│                                                  │
│  sudoku.solution          ← full 81-cell grid    │
│  sudoku.mask.easy         ← revealed cell IDs   │
│  sudoku.mask.medium                              │
│  sudoku.mask.hard                                │
│  sudoku.flow.r3c5         ← move event file     │
└──────────────────┬──────────────────────────────┘
                   │  recur reads these
                   ▼
┌─────────────────────────────────────────────────┐
│  recur (pure hierarchical tool)                  │
│                                                  │
│  recur trace-id "sudoku.r3.c5" --json           │
│  recur files "sudoku.**" --stdin                 │
│  recur merge --sep . --sep _ --sep -             │
│                                                  │
│  recur does not know what Sudoku is.             │
│  It sees: identifiers, keywords, hierarchies.    │
└─────────────────────────────────────────────────┘
```

---

## Eventness File Format

The game engine writes these. recur reads them. No Sudoku logic in recur.

### `sudoku.solution` — canonical solution
```
sudoku.r1.c1 = 5
sudoku.r1.c2 = 3
...
sudoku.r9.c9 = 9
```
Plain hierarchical identifiers. recur trace-id finds `define` here.

### `sudoku.mask.easy` — easy difficulty reveal list
```
sudoku.r1.c1
sudoku.r1.c4
sudoku.r2.c3
...
```
A plain list of cell identifiers. Piped to `recur files --stdin` to filter display.

### `sudoku.flow.r3c5` — move event file (written per move)
```
sudoku.r3.c5 = 7
sudoku.r3.c1 publish 7
sudoku.r3.c2 publish 7
sudoku.r1.c5 publish 7
sudoku.r7.c5 subscribe sudoku.r3.c5
sudoku.r7.c5 trigger solve
```
The game engine writes this when the human (or solver) places a value.
recur trace-id classifies these lines into produce/consume/trigger roles.
`publish` and `subscribe` map to the trait keywords in `[traits.trace_id]`.

### `sudoku.mask.easy/medium/hard` — difficulty masks
Static files generated once when the puzzle is initialized.
The game engine picks which mask to apply. recur doesn't pick.

---

## Game Session Management

**The `-d` flag is the game scope selector.** recur is always pointed at one
game directory at a time. The game engine manages which directory to use.

### Directory Layout (Pattern A: copy-everything)

```
puzzles/
  easy-001/                     ← master package, never modified during play
    sudoku.solution
    sudoku.mask.easy
    sudoku.mask.medium
    sudoku.mask.hard
    sudoku.cascades.json
    sudoku.merge.json

games/
  game-2026030801-easy/         ← Julia creates this at game start
    sudoku.solution             ← copied from puzzles/easy-001/
    sudoku.mask.easy            ← copied
    sudoku.flow.r3c5            ← session: player's move
    sudoku.flow.r5c7            ← session: player's move
  game-2026030802-hard/         ← next game, fully independent
    sudoku.solution             ← different puzzle, different solution
    sudoku.flow.r1c1
  archive/
    game-2026030701-medium/     ← completed games moved here
```

### Game Lifecycle

| Event | What Julia does | What recur sees |
|---|---|---|
| Start game | Create `games/TIMESTAMP-DIFFICULTY/`, copy puzzle files | Fresh `-d` target |
| Make a move | Write `sudoku.flow.rXcY` to game dir | New file in same `-d` |
| Abandon | Move game dir to `archive/` | Different `-d` target |
| Resume | Point `-d` at existing game dir | All eventness still there |
| Replay | `recur find "trigger" --scope "sudoku.**" -d games/archive/GAME/` | Full move history |

Each game is cleanly separated because each game IS a directory.
recur never sees two games at once. No session concept needed in recur.

---

## Game Engine Responsibilities

The game engine (Julia or JS) owns all Sudoku logic:

1. **Initialize** — generate solution, write `sudoku.solution`, generate masks
2. **Display** — apply mask via `recur files --stdin`, render grid in UI
3. **Accept input** — human picks cell + value
4. **Validate** — check against solution (engine knows the rules)
5. **Write move event** — write `sudoku.flow.r{R}c{C}` with proper keywords
6. **Call recur** — `recur trace-id "sudoku.r{R}.c{C}" --scope "sudoku.**" --json`
7. **Render cascade** — parse JSON, display define/produce/consume/trigger in UI
8. **Propagate** — update candidate lists, detect naked singles, write next events
9. **Hint** — switch to easier mask, call `recur files --stdin` with new mask

---

## recur Showcase Map

The demo is structured so each scene highlights a different recur capability.
Sudoku is the narrative thread — recur's features are the actual subject.

### Scene 1: `tree` + `--sep` — Grid as Hierarchy
```bash
# Show the full puzzle as a hierarchy tree (row-centric, dot separator)
recur tree "sudoku.row" --sep .

# Show box-centric view (underscore separator)
recur tree "sudoku_box" --sep _

# Show column-centric view (hyphen separator)
recur tree "sudoku-col" --sep -
```
**Showcase:** same grid data, three separator conventions, three hierarchy views.
The `--sep` flag is the lens that changes what you see.

### Scene 2: `files` + `--stdin` — Difficulty as Filter
```bash
# Hard puzzle: only 25 cells visible
cat sudoku.mask.hard | recur files "sudoku.**" --stdin --count

# Easy puzzle: 45+ cells visible
cat sudoku.mask.easy | recur files "sudoku.**" --stdin --count

# Gap: how many cells still unresolved?
recur files "sudoku.**" --count
```
**Showcase:** `--stdin` as a difficulty overlay. The mask IS a recur file list.
No special API — just Unix composability.

### Scene 3: `children` + `related` — Constraint Scope Discovery
```bash
# All cells in row 3 (row scope = siblings)
recur children "sudoku.row.3" --sep .

# All cells in box 2 (box scope)
recur children "sudoku_box_2" --sep _

# Peers of a specific cell (related = siblings in the hierarchy)
recur related "sudoku.row.3.col.5" --sep .
```
**Showcase:** `children` and `related` naturally express Sudoku peer groups.
Constraint scopes are just hierarchy scopes.

### Scene 4: `stats` — Puzzle Complexity at a Glance
```bash
# How many cells defined vs total
recur stats "sudoku.**" -l 1

# Depth breakdown (row → col → value hierarchy)
recur stats "sudoku.**"
```
**Showcase:** `stats` gives instant structural complexity — how deep, how wide,
how many nodes. Maps directly to puzzle difficulty intuition.

### Scene 5: `find` — Search for Patterns in Puzzle State
```bash
# Find all cells that still have 7 as a candidate
recur find "7" --scope "sudoku.candidates.**"

# Find all cells where a naked single was triggered
recur find "trigger solve" --scope "sudoku.**"
```
**Showcase:** `find` as constraint pattern search. Not just code — any
hierarchical text with meaningful keywords.

### Scene 6: `merge` — Unified Constraint View
```bash
# Unify row/col/box views of the same placement
recur merge \
  --pattern "sudoku.row.3" --sep . \
  --pattern "sudoku_box_2" --sep _ \
  --pattern "sudoku-col-5" --sep -
```
**Showcase:** `merge` as the multi-convention unifier. Three separator views
of the same grid cell, merged into one tree. The beer demo pattern applied to constraints.

### Scene 7: `trace-id` — The Full Capability Showcase

This is the centrepiece. Each flag gets its own moment.

```bash
# Basic: what happens when we place 7 at r3.c5?
recur trace-id "sudoku.r3.c5" --scope "sudoku.**"

# Full output: show every site (define/produce/consume/trigger)
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --format full

# JSON output: machine-readable for Julia to parse and visualize
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --json

# Scoped to row only: just the row constraint cascade
recur trace-id "sudoku.r3.c5" --scope "sudoku.row.3.**"

# Scoped to box only: just the box constraint cascade
recur trace-id "sudoku.r3.c5" --scope "sudoku_box_2.**" --sep _

# Glob: trace ALL cells in row 3 simultaneously
recur trace-id "sudoku.row.3.**" --scope "sudoku.**"

# Depth control: how deep does the cascade go?
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --depth 1
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --depth 3

# Depth guardrail: clamp instead of fail on deep puzzles
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --depth 5 --depth-guard clamp

# Force: bypass the cap for a full solution trace
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --depth 9 --force

# Configurable keywords: tune what counts as produce/consume/trigger
recur trait set trace_id.producer_keywords "publish,emit,propagate"
recur trait set trace_id.consumer_keywords "subscribe,bind,consume"
recur trait set trace_id.trigger_keywords "trigger,register,solve"
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --format full
```

**trace-id showcase moments:**
1. Summary vs full format — two ways to see the same cascade
2. JSON output — the bridge between recur and Julia/HTML5
3. Scope narrowing — row vs box vs full grid constraint view
4. Glob patterns — trace an entire row at once
5. Depth control — how far does the cascade ripple?
6. Guardrail system — safety + override (`--force`) for deep analysis
7. Trait config — customize vocabulary without touching Rust source

recur returns JSON. The game engine decides what to show.

---

## Two Implementations

### Julia App — native recur user (CLI game + package generator)
```
demos/sudoku/julia/
  run.jl           # Entry point — CLI game or package generator
  Game.jl          # Main game loop (interactive CLI)
  Engine.jl        # Sudoku rules + candidate propagation
  Recur.jl         # recur subprocess wrapper (trace-id, files, merge)
  Generator.jl     # Puzzle package generator → writes JSON for HTML5
  Display.jl       # Terminal grid renderer
```

Modes:
```bash
julia run.jl --play hard          # Interactive CLI game (calls recur live)
julia run.jl --generate easy-001  # Generate puzzle package for HTML5
```

### HTML5 / JavaScript App — recur-free at runtime (browser game)
```
demos/sudoku/html5/
  index.html             # Game UI
  game.js                # Game loop (reads pre-generated JSON)
  grid.js                # Canvas/SVG grid renderer + cascade visualization
  puzzles/
    easy-001/
      sudoku.solution.json    # Full grid (from recur trace-id)
      sudoku.mask.easy.json   # Revealed cells (from recur files --stdin)
      sudoku.mask.medium.json
      sudoku.mask.hard.json
      sudoku.cascades.json    # Pre-computed cascades for every placement
      sudoku.merge.json       # Unified row/col/box view (from recur merge)
```

**No recur installed in the browser.** Julia generated the puzzle package.
JavaScript reads JSON. The hierarchical structure from recur is preserved in the files.
HTML5 navigates `sudoku.cascades.json` hierarchically — same structure recur produced.

### Optional: Live mode (Julia as local server)
Julia can also serve as a local HTTP server for on-the-fly trace-id queries:
```bash
julia run.jl --serve          # Julia listens on localhost:8765
# HTML5 POSTs { cell: "r3c5", value: 7 }
# Julia calls: recur trace-id "sudoku.r3.c5" --json
# Julia returns the live cascade JSON to the browser
```
Live mode enables novel placements not in the pre-generated package.
Static mode (pre-generated JSON) is the default and works fully offline.

---

## What Makes This Architecture Sound

1. **recur is agnostic** — it sees identifiers and keywords, not Sudoku
2. **The game engine is swappable** — Julia and JS share the same file protocol
3. **Eventness is the contract** — the format of the files is the API between engine and tool
4. **trace-id is configurable** — `recur trait set trace_id.producer_keywords "publish,emit,send"`
   tunes keyword recognition without changing recur source
5. **Masks = `--stdin` lists** — no new recur feature needed for difficulty filtering
6. **Extensible** — any constraint propagation problem (not just Sudoku) can use this pattern

---

## Implementation Phases

### Phase 1: File Protocol + Keyword Vocabulary
- Finalize identifier naming convention for `sudoku.solution`
- Finalize `sudoku.flow.*` keyword vocabulary (`publish`, `subscribe`, `trigger solve`)
- Confirm `recur trait set trace_id.producer_keywords "publish"` works project-scoped
- Confirm `recur trace-id` works on a hand-written non-code `.solution` file

### Phase 2: Julia CLI Prototype
- Hardcoded puzzle, hand-write solution + one flow event
- Call `recur trace-id` from Julia subprocess, inspect raw JSON
- Verify define/produce/consume/trigger output is correct
- No game loop yet — just prove the recur integration

### Phase 3: Julia Puzzle Package Generator
- `Generator.jl` — runs recur for every possible cell placement
- Writes `sudoku.cascades.json`, `sudoku.solution.json`, masks, merge view
- Output: self-contained puzzle package ready for HTML5

### Phase 4: Julia CLI Game Loop
- Interactive terminal game calling recur live
- Human input → write flow event → trace-id → display cascade
- Mask switching for hints, naked single auto-propagation

### Phase 5: HTML5 Static Game
- Load Julia-generated puzzle package (no recur at runtime)
- Full browser game with cascade visualization from `sudoku.cascades.json`
- Hierarchical JSON structure mirrors recur output — JavaScript navigates it natively

### Phase 6: Optional — Julia Local Server (Live Mode)
- Julia HTTP server proxies recur calls for the browser
- Enables on-the-fly trace-id for placements not in the pre-generated package

### Phase 7: Demo Script
- `demos/sudoku/demo.ps1` — scripted walkthrough (like beer demo)
- Shows: generate → play → place → trace → cascade → hint → solve

---

## Open Questions

1. Does `recur trait set trace_id.producer_keywords "publish,emit"` scope to the
   project `.recur/config.toml`, or is it global? (Needs to be project-scoped for portability)
2. Can the Julia app ship a `.recur/config.toml` alongside the demo files?
3. Does `recur trace-id` work on non-source files (plain `.txt` identifier files)?

---

## Files To Create

```
demos/sudoku/
  demo.ps1                     # Scripted walkthrough (like beer demo)
  README.md                    # Protocol spec: file format + recur commands
  .recur/config.toml           # trace_id keyword config (publish, subscribe, trigger)

  julia/
    run.jl                     # Entry point (--play, --generate, --serve)
    Game.jl                    # Interactive CLI game loop
    Engine.jl                  # Sudoku rules + constraint propagation
    Recur.jl                   # recur subprocess wrapper
    Generator.jl               # Puzzle package generator for HTML5
    Display.jl                 # Terminal grid renderer

  html5/
    index.html                 # Browser game UI
    game.js                    # Game loop (reads pre-generated JSON)
    grid.js                    # Canvas/SVG grid + cascade visualization

  puzzles/
    easy-001/
      sudoku.solution.json     # Full grid (from recur trace-id)
      sudoku.mask.easy.json    # Revealed cells (from recur files --stdin)
      sudoku.mask.medium.json
      sudoku.mask.hard.json
      sudoku.cascades.json     # Pre-computed cascades for all placements
      sudoku.merge.json        # Unified row/col/box view (from recur merge)

docs/
  main.demo.sudoku.trace-id.todo.md          # Persistent tracking
  main.demo.sudoku.trace-id.todo.current.md  # (this file)
```

---

## References

- `demos/ascii-drinks/demo.ps1` — beer demo (multi-separator merge pattern)
- `src/main_command_trace_id_impl.rs` — trace-id implementation
- `src/main_command_trait_impl.rs` — trait config (producer_keywords tuning)
- `docs/main.improvement.8.trace-id.todo.current.md` — trace-id MVP lane
- `docs/main.improvement.9.trace-id.todo.future-plan.md` — merge edge-type (future)
