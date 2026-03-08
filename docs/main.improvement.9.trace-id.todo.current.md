# Improvement 9: trace-id Pipeline + Sudoku Demo

Status: `todo.current` (active)
Date: 2026-03-08

## Focus

Two lanes, both trace-id ecosystem:

1. **merge edge-type pipeline** — `trace-id --json | merge --stdin` retains `edge_type`
2. **Sudoku demo** — full showcase of recur as discoverability engine

## Lane 1: merge edge-type

### What's needed

`merge --stdin` currently extracts file paths from JSON and builds a file-path tree.
It doesn't understand the trace-id JSON shape (`define`/`produce`/`consume`/`trigger` arrays)
and discards `edge_type` metadata entirely.

To make `trace-id --json | merge --stdin --json` output include `edge_type`:

1. Teach `extract_paths_from_json` to parse trace-id JSON shape
2. Track `edge_type` per path through merge
3. Include `edge_type` in merge JSON output

### Test

`julia-tests/runtests.trace-id.jl` — Phase 5: "trace-id -> merge (full composition placeholder)"

```julia
@test_broken success && contains(output, "\"edge_type\"")
```

Flip to `@test` when implemented.

### Descoped

`trace -> merge`, `callers -> merge`, `callees -> merge` edge_type tests —
those commands don't produce `edge_type`. Marked `@test_skip` permanently.

## Lane 2: Sudoku Demo

Full planning in `docs/main.demo.sudoku.trace-id.todo.current.md`.

7 phases:
1. File protocol spec + keyword vocabulary
2. Julia CLI prototype — hardcoded puzzle, call recur, verify cascade JSON
3. Julia puzzle package generator — produce all JSON artifacts for one puzzle
4. Julia CLI game loop — playable terminal game
5. HTML5 static game — load puzzle package, full browser game, no recur at runtime
6. Optional: Julia local server mode for live recur queries from HTML5
7. Demo script (`demos/sudoku/demo.ps1`)

## Close-out Criteria

1. Phase 5 trace-id → merge test flips from `@test_broken` → `@test`
2. Sudoku demo Phase 1-5 complete (Phase 6-7 optional)
3. Create `docs/main.improvement.9.trace-id.complete.md`
4. Delete this `.current.md`

## References

- `julia-tests/runtests.trace-id.jl` — Phase 5 (Lane 1)
- `src/main_command_merge_impl.rs` — merge stdin implementation
- `docs/main.demo.sudoku.trace-id.todo.current.md` — Sudoku demo (Lane 2)
- `docs/main.improvement.9.trace-id.todo.future-plan.md` — original future-plan
