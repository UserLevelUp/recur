# Improvement 9: merge edge-type Pipeline

Status: `todo.current` (active — focused on merge edge-type only)
Date: 2026-03-24

## Focus

Single lane: teach `merge --stdin` to retain `edge_type` from trace-id JSON.

The Sudoku demo (formerly Lane 2) is now its own standalone lane:
`docs/main.demo.sudoku.trace-id.todo.current.md`

## What's Needed

`merge --stdin` currently extracts file paths from JSON and builds a file-path tree.
It doesn't understand the trace-id JSON shape (`define`/`produce`/`consume`/`trigger` arrays)
and discards `edge_type` metadata entirely.

To make `trace-id --json | merge --stdin --json` output include `edge_type`:

1. Teach `extract_paths_from_json` to parse trace-id JSON shape
2. Track `edge_type` per path through merge
3. Include `edge_type` in merge JSON output

## Test Gate

`julia-tests/runtests.trace-id.jl` — Phase 5: "trace-id -> merge (full composition placeholder)"

```julia
@test_broken success && contains(output, "\"edge_type\"")
```

Flip to `@test` when implemented.

Also absorbs the trace-id test lane Phase 5 (cross-command JSON pipeline contracts).
That lane's Phases 1-4 are complete and wired into the suite — see
`docs/main.command.trace-id.test.complete.md` for the record.

### Descoped

`trace -> merge`, `callers -> merge`, `callees -> merge` edge_type tests —
those commands don't produce `edge_type`. Marked `@test_skip` permanently.

## Close-out Criteria

1. Phase 5 trace-id → merge test flips from `@test_broken` → `@test`
2. Create `docs/main.improvement.9.trace-id.complete.md`
3. Delete this `.current.md`

## References

- `julia-tests/runtests.trace-id.jl` — Phase 5 tests
- `src/main_command_merge_impl.rs` — merge stdin implementation
- `docs/main.command.trace-id.test.complete.md` — test lane record (Phases 1-4 done)
- `docs/main.demo.sudoku.trace-id.todo.current.md` — Sudoku demo (standalone)
- `docs/main.improvement.9.trace-id.todo.future-plan.md` — original future-plan
