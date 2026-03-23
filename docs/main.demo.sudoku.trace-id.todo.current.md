# Demo: Sudoku + trace-id (Pure Separation Architecture)

Status: `todo.current` (active — Phase 4 complete, Phase 5 next)
Date: 2026-03-13

## Phase 1 Complete (2026-03-13)

`julia-tests/runtests.demo.sudoku.jl` — 32 pass, 0 fail, 0 broken. Wired into full suite.

**Proven:**
- trace-id works on plain `.txt` eventness files (not just code)
- `publish` / `subscribe` classify with DEFAULT vocabulary (no config needed)
- `trigger` requires project-local `.recur/config.toml` — proves recur is data-driven
- Full cascade (define/produce/consume/trigger) fires with project config
- JSON `edge_type` field present per site
- Project config in `-d` directory is auto-discovered (portable, self-contained)

## Phase 2 Complete (2026-03-13)

`Recur.jl` wrapper + `demos/sudoku/puzzles/easy-001/` — 20 tests green.
`Recur.trace_id()` → subprocess → JSON → all 4 roles proven.

## Phase 3 Complete (2026-03-13)

`julia-tests/runtests.demo.sudoku.phase3.jl` — 46 pass, 0 fail. Wired into full suite.
`demos/sudoku/julia/Generator.jl` — Sudoku geometry + flow file authoring + cascade generation.
Suite total: 644 passed.

**Proven:**
- `box_of(row, col)` computes correct box numbers (fixed handwritten fixture: box.5 → box.2)
- `all_peers(3, 5)` returns exactly 20 unique peers (row 8 + col 8 + box-only 4)
- `write_flow_event` writes correctly-formed file — all 20 subscribe lines reference identifier
- Flow file produced by Generator classifies correctly: 20 consume sites, all 4 roles
- `generate_cascade` returns `{cell, value, cascade}` dict
- `generate_cascades` writes `sudoku.cascades.json` for a subset solution

**Key insight captured:** Traditional solvers recurse in memory. This externalizes the
stack into files — the cascade JSON is the stack trace, queryable by any consumer.
The reasoning recurs in the file hierarchy. The tool is called `recur`.

## Phase 4 Complete (2026-03-13)

`julia-tests/runtests.demo.sudoku.phase4.jl` — 39 pass, 0 fail. Wired into full suite.
Suite total: 683 passed.

**Proven:**
- `Engine.load_solution` parses 81 cells with correct values
- `Engine.make_grid` builds masked 9x9 grid (nothing = player must fill)
- `Engine.is_valid_placement` validates against solution
- `Engine.get_candidates` returns only the solution value for empty cells
- `Engine.is_solved` returns true only when all 81 cells match solution
- `Engine.apply_placement!` mutates grid; subsequent Generator+Recur integration classifies 20 consume sites
- `Display.render_grid` outputs multi-line ASCII box-drawing grid
- `Display.render_cascade` formats define/produce/consume/trigger counts

**Architecture proven:**
- Engine.jl: rules + grid state + validity
- Generator.jl: flow event file authoring (Phase 3)
- Recur.jl: subprocess wrapper (Phase 2)
- Display.jl: terminal renderer
- Game.jl: game loop orchestrator (ties all four)

## Phase 5: HTML5 Static Game — In Progress

Status: `in-progress` (core game working, polishing)

### What's Working (2026-03-15)

- **Grid + cascade panel** — 9x9 grid with cell selection, digit input, cascade display
- **Difficulty switching** — Easy/Medium/Hard buttons change mask (symmetric 180° rotation)
- **Wrong answer handling** — incorrect digits shown in red with conflict explanations
  - Row/col/box violations explained with specific conflicting cell
  - Logic-only conflicts show candidates and elimination guidance
- **Progressive hint system** — 5 levels via H key:
  - Level 0: Nudge (how many candidates)
  - Level 1: Elimination (which values ruled out and why)
  - Level 2: Candidates (exact list of remaining values)
  - Level 3: Strategy (Hidden Single, Pointing Pair, Naked Pair, X-Wing detection)
  - Level 4: Answer (reveals the solution value)
- **Hint overlay toggle** — highlights constraint peers on the grid
- **Cascade panel** — shows define/produce/consume/trigger from pre-generated JSON
  - Expandable "How was this generated?" section explains the 3-step recur pipeline
- **New Puzzle button** — calls `/api/generate` which runs the same pipeline as `generate.jl`:
  1. `Generator.generate_solution()` — random valid 9x9 grid via backtracking
  2. `Generator.write_solution_file()` + `write_flow_event()` for all 81 cells
  3. `Recur.trace_id()` × 81 — subprocess calls to `recur trace-id --json`
  4. Writes `sudoku.cascades.json` — browser reloads with new puzzle
- **Win detection** — banner + panel message when all cells correctly placed
- **serve.jl** — local dev server with static files + `/api/generate` endpoint
  - Loads Generator.jl + Recur.jl at startup (one-time cost)
  - Julia is sandboxed: only predefined API endpoints exposed, no user code execution

### Files Implemented

```
demos/sudoku/html5/
  index.html               # Single page — grid left, cascade panel right
  serve.jl                 # Local dev server + /api/generate API
  generate.jl              # Offline cascade generation (same pipeline, no server)
  css/game.css             # Grid + panel + difficulty + hint styling
  js/
    puzzle.js              # Load solution.txt + cascades.json, build mask, index by cell
    grid.js                # DOM-based 9x9 grid, cell selection, digit input, highlights
    cascade.js             # Cascade panel renderer (roles, conflicts, hints, pipeline)
    solver.js              # Conflict detection, candidates, progressive hints, strategies
    game.js                # Game loop orchestrator — wires grid ↔ solver ↔ cascade
  data/easy-001/           # Pre-generated by generate.jl or /api/generate
    sudoku.solution.txt
    sudoku.cascades.json
```

### Strategy Overlays — DONE (2026-03-15)

All 7 strategies wired with `overlayCells` in solver.js. `highlightStrategy` (amber)
and `highlightEliminations` (red dashed) both implemented and styled.

| # | Strategy | Status |
|---|----------|--------|
| 1 | Naked Single | done + overlay |
| 2 | Hidden Single | done + overlay |
| 3 | Pointing Pair | done + overlay |
| 4 | Naked Pair | done + overlay |
| 5 | X-Wing | done + overlay |
| 6 | Box/Line Reduction | done + overlay |
| 7 | Swordfish | done + overlay |

### Crosshatch + Pencil Marks + Pencil Mode — DONE (2026-03-15)

**Crosshatch (3D constraint intersection):**
- Click cell → row band (warm), col band (cool), box band (green) appear
- Click same cell → everything clears (toggle select/deselect)
- H key → crosshatch clears, strategy overlay replaces it

**Pencil marks (MANUAL — not auto-populated):**
- Every empty cell has a 3×3 mini-grid for candidates, starts EMPTY
- Player adds marks via pencil mode (P key) — this is where they LEARN elimination
- Auto-fill is opt-in cheat via button in Pencil Ins panel, never automatic
- Pencil marks in crosshatch bands get tinted to match the band color

**Pencil mode (P key toggle):**
- Normal mode: digit key = place value
- Pencil mode: digit key = toggle that candidate on/off in selected cell
- Manual marks appear bright (`.pm-manual`)
- Player annotates their own reasoning before committing

**Pencil Ins panel (right side, persistent):**
- Shows all cells with manual pencil marks and their values
- "Auto-fill all candidates" button (cheat, clearly labeled)
- Updates live as player adds/removes marks

**Eventness mapping:**
```
crosshatch.row        → warm band     (row constraint plane)
crosshatch.col        → cool band     (col constraint plane)
crosshatch.box        → green band    (box constraint plane)
strategy-highlight    → amber         (produce — pattern source)
elimination-highlight → red dashed    (consume — affected peers)
selected cell         → bright        (trigger — where player acts)
pencil marks          → manual only   (player's own elimination work)
```

### Next: Server-Side Strategy APIs (serve.jl)

| Endpoint | What it enables |
|---|---|
| `POST /api/candidates` | Server-authoritative live candidates |
| `POST /api/solve-step` | "What should I do next?" — easiest cell + strategy |
| `POST /api/elimination-wave` | Animate ripple after placement |
| `POST /api/walkthrough` | Full solve sequence — step-by-step replay |

### Why This Phase Matters

Phase 5 proves recur's deepest design principle: **recur makes itself unnecessary.**

The Julia game (Phase 4) calls recur live — subprocess per move. The HTML5 game
calls recur zero times. It loads pre-generated JSON that recur produced, and
navigates it with plain JavaScript. The player sees the same cascades, the same
define/produce/consume/trigger structure, the same peer relationships — but recur
isn't running.

This is the point. recur is a **discoverability engine that teaches structure.**
Once the structure is externalized into JSON, any tool — JavaScript, Python, a
spreadsheet, a human reading the file — can navigate it independently.

### The Independence Principle

recur exists to help users, tools, and AI find patterns in hierarchical data.
But the patterns are *in the data*, not in recur. recur is a lens, not a crutch.

The Sudoku demo teaches this in three steps:

1. **Phase 2-4 (Julia + recur live):** recur classifies flow events in real time.
   The user sees: "recur finds define/produce/consume/trigger for me."

2. **Phase 5 (HTML5 + pre-generated JSON):** Same game, same cascades, no recur.
   The user sees: "The structure recur found is just JSON. I can read it myself."

3. **The takeaway:** You don't need recur to *use* hierarchical structure.
   You need recur to *discover* it. Once discovered, it's yours.

This applies to every recur use case:
- Code navigation: recur shows you the call graph. Once you see it, you navigate it.
- Event tracing: recur finds the cascade. Once traced, it's a JSON file.
- Project structure: recur maps the hierarchy. Once mapped, it's a tree you know.

**Making tools, people, and AI independent is the goal.** recur helps you find
the strategy. Then you apply the strategy yourself, with whatever tools you prefer.

### Data Contract

`sudoku.cascades.json` is an array of entries, each shaped:

```json
{
  "cell": "sudoku.r3.c5",
  "value": 7,
  "cascade": {
    "identifier": "sudoku.r3.c5",
    "define":  [{ "edge_type":"define",  "line":"...", "line_number":N, "path":"..." }],
    "produce": [{ "edge_type":"produce", "line":"...", "line_number":N, "path":"..." }],
    "consume": [{ "edge_type":"consume", "line":"...", "line_number":N, "path":"..." }],
    "trigger": [{ "edge_type":"trigger", "line":"...", "line_number":N, "path":"..." }],
    "request": { ... }
  }
}
```

JavaScript indexes this by cell id. When the player places a value, look up
the cascade and render it. No recur, no server, no subprocess.

### File Layout

```
demos/sudoku/html5/
  index.html          <- single page, loads everything
  css/
    game.css          <- grid + cascade panel styling
  js/
    puzzle.js         <- load & index cascades.json, solution, mask
    grid.js           <- 9x9 grid renderer (DOM), cell selection, input
    cascade.js        <- cascade panel renderer (define/produce/consume/trigger)
    game.js           <- game loop: init, placement, win check, wire grid<->cascade
  data/
    easy-001/         <- generated by Julia (copy from puzzles/)
      sudoku.cascades.json
      sudoku.solution.txt
```

### Module Responsibilities

| Module | Does | Doesn't |
|---|---|---|
| `puzzle.js` | Parse solution, build mask, index cascades by cell | Render anything |
| `grid.js` | Draw 9×9 grid, pencil marks, crosshatch, cell toggle, pencil mode | Know cascade data |
| `solver.js` | Candidate computation, strategy detection, progressive hints | Know DOM/rendering |
| `cascade.js` | Render cascade panel, conflict explanations, hint display | Know grid state |
| `game.js` | Wire grid ↔ solver ↔ cascade, track state, refresh pencil marks | Know rendering details |

### Interaction Flow

```
User clicks cell → grid.js toggles selection
  → game.js shows crosshatch overlay (row/col/box bands)
  → pencil marks visible in all empty cells (auto-computed candidates)
User clicks same cell → grid.js deselects
  → game.js clears all overlays

User types digit (normal mode) → grid.js fires "digit-placed"
  → game.js validates → correct: refreshPencilMarks() + cascade panel
  → incorrect: conflict explanation + highlights

User types digit (pencil mode) → grid.js toggles pencil mark on/off
  → manual mark appears brighter than auto-computed

User presses H → game.js gets hint from solver.js
  → crosshatch clears, strategy overlay replaces it
  → panel shows strategy explanation + overlay legend

User presses P → game.js toggles pencil mode indicator
```

### Design Decisions

1. **No build step.** Vanilla HTML/CSS/JS, ES modules, no bundler. Open
   `index.html` in a browser and play. This is a demo, not a product.

2. **Cascade panel mirrors Display.jl.** Same role counts, same structure —
   just HTML instead of ASCII. Proves they're the same data, different renderer.

3. **Peer highlighting is the visual payoff.** When you place a number, the
   cascade's `consume` array tells you which cells were notified. Highlight
   those cells on the grid. That's the "aha" moment — recur's output driving UI.

4. **Mask = difficulty.** puzzle.js strips revealed cells from solution to
   create the playable grid. Same concept as `recur files --stdin` but in JS.

5. **The game proves independence.** Every feature of the HTML5 game uses
   data that recur produced but does not require recur to run. The JSON
   *is* the knowledge transfer.

### Implementation Steps

1. Run Generator.jl to produce full `sudoku.cascades.json` for easy-001
2. Create `demos/sudoku/html5/` directory structure
3. `puzzle.js` — parse solution.txt, index cascades.json by cell id
4. `grid.js` — DOM-based 9x9 grid with click selection + number input
5. `cascade.js` — cascade panel rendering (role counts + line details)
6. `game.js` — wire everything, state management, win detection
7. `index.html` + `game.css` — layout with grid left, cascade panel right
8. Test: open in browser, play through easy-001, verify cascades match Julia output

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

### Live mode (Julia local server) — Implemented

`serve.jl` serves static files AND exposes `/api/generate` for on-demand puzzle creation:
```bash
julia demos/sudoku/html5/serve.jl    # Listens on localhost:8787
# Browser clicks "New Puzzle" → POST /api/generate
# Julia: generate_solution() → write flow events → recur trace-id × 81 → cascades.json
# Browser reloads with new puzzle — same pipeline as offline generate.jl
```
Static mode (pre-generated JSON) works fully offline — just open `index.html`.
Live mode adds the "New Puzzle" button for generating fresh puzzles via Julia+recur.

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
