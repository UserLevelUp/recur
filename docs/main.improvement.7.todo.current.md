# Improvement 7: Expelliarmus to Embeddings

Status: `todo.current` (active on branch `flatten-init`)

## Vision

Combine `recur flatten` (data disarming), `.recur/config.toml` (project awareness), and `trace-stats` (code analysis) into a unified phased pipeline that transforms complex hierarchical data into a universal `path = value` intermediate format — farmable, greppable, embeddable, and LLM-friendly.

**Core Insight:** Flatten isn't the product. It's the first stage of a pipeline that turns opaque complex data into something useful to humans. The repeating patterns in flat output reveal the schema. The consumer decides the resolution and aspect ratio.

## Phased Approach

### Phase 1: The Foundation (THIS PHASE)
- `.recur/config.toml` and `recur init`
- Makes recur project-aware and portable
- No more `--sep _` flag juggling
- Every subsequent phase benefits from project config

### Phase 2: Expand the On-Ramp
- More flatten formats: TOML, CSV, YAML, HL7
- Every format that gets a flattener joins the ecosystem
- All output is the same `path = value` shape

### Phase 3: Flatten for Code
- `trace-stats` — call graph complexity as flat readable output
- Code structure joins the same format as data
- Code and data live in the same queryable world

### Phase 4: Farmer John's Tools
- Select, remap, merge across flat sources
- The farming layer between flatten and output
- Small sharp composable Unix-style tools

### Phase 5: The Embedding Bridge
- Each flat line becomes a vector
- Semantic search across ALL flattened sources
- Code, docs, data, game state — all searchable by meaning
- A tiny LLM farms the embeddings instead of reading raw text

## Phase 1 Scope: `.recur/config.toml`

### What It Solves
- Lane detection — `root_pattern = "**"` means checkpoint finds any root, not just `main.**`
- Checkpoint file location — `recur-git checkpoint --append-parallel` just knows where to write
- Multi-dir awareness — `docs/` uses `.` separator, `src/` uses `_`, configured once
- `recur status` — knows which suffixes to count without flags
- Portable across projects — any project can `recur init` and go

### Design Principle: Flexible and Growable

Projects are living things. A C# project today has Julia tests next week and Python scripts next month and somebody drops a MongoDB migration folder in there on a Tuesday. The config must adapt as the project grows — no rigid schemas, no predefined section list, no required fields beyond the basics.

**Rules:**
- Any number of user-named sections (not a fixed list)
- Each section needs `dir` and `sep` at minimum — everything else optional
- Unknown keys are ignored, not errors
- No validation that dirs exist (they might not yet)
- `recur init` starts shallow with sensible defaults — refine later
- `recur init --analyze` can re-scan and suggest updates as project grows
- Progressive disclosure: works without understanding the config, gets better as you learn

### Config Shape (Flexible)
```toml
# Start simple — just what you have today
[src]
dir = "src/"
sep = "_"

[docs]
dir = "docs/"
sep = "."

# Add more sections as your project grows — name them whatever makes sense
[tests]
dir = "julia-tests/"
sep = "."

[scripts]
dir = "scripts/"
sep = "_"

[migrations]
dir = "mongo/"
sep = "."

# Optional: workflow config (only if you use recur-git)
[checkpoint]
file = ".recur/checkpoints.md"
root_pattern = "**"

# Optional: eventness suffixes (only if you use the eventness pattern)
[status]
current_suffix = ".current.md"
todo_suffix = ".todo.md"
complete_suffix = ".complete.md"
```

### Risk Mitigation
- Don't over-specify the schema — keep it loose so it works for projects we haven't imagined
- Don't require sections — an empty config with just `[src]` is fine
- Don't validate too aggressively — a wrong config is a TOML edit away from a right config
- Don't assume the project shape is fixed — sections can be added/removed anytime

### Implementation Steps (recur only — recur-git is out of scope)
1. Design minimal config reader (TOML parsing, any-section support)
2. Implement `recur init` command (auto-detect dirs/seps, create `.recur/config.toml`)
3. Implement `recur init --analyze` (re-scan project, suggest config updates)
4. Wire config reading into existing commands (so `--sep` becomes optional when config exists)
5. Tests

### Future (out of scope for Phase 1)
- `recur-git checkpoint` reads `.recur/config.toml` for lane detection (separate task)
- `recur-to-*` reads config for output format preferences (Improvement 12)
- `flatten` extraction to separate binary if format count grows (when it hurts)

## Related Files
- See: `docs/main.improvement.7.todo.current.reference.md`
- See: `README.CORE.IMPROVEMENT7.md` (trace-stats spec)
- See: `README.CORE.IMPROVEMENT7.recur-git.md` (.recur/config.toml vision)
- See: `README.CORE.IMPROVEMENT12.md` (document transformation vision)
- See: `README.CORE.IMPROVEMENT13.md` (config management vision)

## Predictions (check back after each phase)

### After Phase 1: The Foundation
- `recur init` exists and creates `.recur/config.toml` with auto-detected dirs/seps
- Users stop typing `--sep _` for src/ — config handles it
- Any project (C#, Rust, Python, mixed) can `recur init` and go
- The config is loose enough that we haven't blocked any future phase
- recur-git checkpoint can't read the config yet (out of scope) but the file is there waiting
- We'll discover at least one config field we didn't plan for

### After Phase 2: Expand the On-Ramp
- TOML flattening works — recur can flatten its own config (dog food!)
- At least 2 more formats beyond XML/JSON (TOML + one of CSV/YAML/HL7)
- Flatten starts feeling heavy in recur core — extraction conversation begins
- Someone asks "can it flatten HTML?" and we have to think about that
- The `path = value` format proves itself as a universal intermediate

### After Phase 3: Flatten for Code
- `trace-stats` outputs flat `path = value` lines alongside table/JSON/CSV
- Code complexity and flattened data are queryable with the same tools
- We discover that trace-stats and flatten share output formatting logic
- The "everything is a hierarchy" thesis gets proven across code AND data

### After Phase 4: Farmer John's Tools
- Select/remap/merge tools exist as small composable commands
- Someone builds a real pipeline: flatten → farm → deliver
- The farming step is where most of the value lives (not flatten, not unflatten)
- `recur-to-*` (Improvement 12) becomes urgent — people want the loop closed
- We realize some farming operations are better done by an LLM than by CLI tools

### After Phase 5: The Embedding Bridge
- Each flat line is an embeddable semantic unit
- Cross-format semantic search works (find similar things in XML and JSON sources)
- A small LLM + embeddings replaces most manual farming
- The zombie spelling bee becomes possible
- We look back and realize flatten was the most important command we built

## Origin

This phased approach emerged from a prototyping session exploring `recur flatten` on a 4MB AWS SDK XML file. The conversation surfaced the full pipeline: flatten (Expelliarmus) → farm (Farmer John) → synthesize (Station) → deliver (aspect ratio for the consumer). The game/edtech angle revealed that the same flat data can serve a trillion different consumer modes — zombies, crosswords, reports, whatever the moment requires.

Key participants: Marc Noon (playing Joe Bishop), Claude AI (playing Skippy the Magnificent).