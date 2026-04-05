# Improvement 9: merge edge-type Pipeline

Status: `complete`
Date: 2026-04-05
Original lane: `docs/main.improvement.9.trace-id.todo.future-plan.md`

## What Landed

`recur merge` now preserves `trace-id` edge metadata when consuming JSON from files or
stdin.

Completed behavior:

1. merge JSON intake recognizes trace-id site objects and extracts both `path` and
   `edge_type`
2. merge unions repeated edge roles per file path (`produce` + `trigger`, etc.)
3. `recur merge --json` emits `edge_type` on leaf nodes
4. when incoming file names do not already share the requested `--base`, merge roots
   them under that synthetic base while preserving the emitted leaf `path`

## Test Gate

Phase 5 in `julia-tests/runtests.trace-id.jl` is now active and passing:

```julia
@test success
@test parse_ok
@test contains(output, "\"edge_type\"")
```

Current trace-id suite snapshot:

- `recur trace-id command (IMPROVEMENT8) |   63       3     66  0.7s`

The remaining `3` broken items are the intentionally skipped edge-metadata placeholders
for `trace -> merge`, `callers -> merge`, and `callees -> merge`.

## Verification

Verified with:

- `cargo test --bin recur main_command_merge_impl -- --nocapture`
- `cargo build --profile release-safe --bin recur`
- `julia julia-tests/main.command.trace-id.test.jl`
- `include("julia-tests/runtests.setup.jl"); setup_test_environment(); include("julia-tests/runtests.merge.jl")`

## Notes

- `README.CORE.IMPROVEMENTS*.md` does not currently contain a higher-level persisted
  trace-id flow spec to reconcile against.
- The saved-run persistence lane remains tracked in
  `docs/main.command.trace-id.run.todo.current.md`.
- Layer 1 (`trace-id --json` emitting `edge_type`) remains recorded in
  `docs/main.command.trace-id.edge-type.complete.md`.

## References

- `src/main_command_merge_impl.rs`
- `julia-tests/runtests.trace-id.jl`
- `julia-tests/runtests.merge.jl`
- `docs/main.command.trace-id.edge-type.complete.md`
- `docs/main.command.trace-id.run.todo.current.md`
