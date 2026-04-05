# Command: trace-id Saved Runs

Status: `todo.current`
Date: 2026-04-05

## Purpose

Track the saved-run persistence layer for `recur trace-id` so the next polish work
stays visible in eventness instead of living only in code or chat memory.

## What Landed

- `--save-run`
- `--reuse-if-fresh`
- `--check-run`
- `--run-name`
- saved artifacts under `.recur/trace-id/runs/<name>/`
- `manifest.toml` for request/config/file fingerprints
- `latest.json` for the last saved `trace-id` payload

## Test Coverage

Saved-run coverage now lives in `julia-tests/runtests.trace-id.jl` as:

- `Phase 4b: saved runs persist and refresh`

Covered behaviors:

1. saving a named run creates `manifest.toml` and `latest.json`
2. `--check-run` reports `fresh` when request/config/files still match
3. `--reuse-if-fresh` replays the saved JSON instead of recomputing
4. changing the scoped file set marks the run `stale`

Repro:

```powershell
julia julia-tests/main.command.trace-id.test.jl
```

## Remaining Todo

1. Decide whether saved runs should keep timestamped history or remain `latest` only
2. Decide whether freshness should stay metadata-based (`path` + `size` + `mtime`) or
   grow an optional content-hash mode
3. Add one durable example showing how saved runs fit with eventness workflows in
   `.recur/`
4. Decide whether future `.recur/eventness` snapshots should count `fresh` vs `stale`
   trace-id runs

## Not This Lane

- `trace-id --json | merge --stdin` retaining `edge_type` is complete in
  `docs/main.improvement.9.trace-id.complete.md`
- role keyword configuration remains in `.recur/config.toml` under `[traits.trace_id]`

## Close-out Criteria

1. Persistence policy is explicit (`latest` only or `history/` added)
2. Freshness policy is explicit enough to document as stable behavior
3. This lane collapses into either `recurring` or `future-plan` once the workflow settles

## References

- `src/main_command_trace_id_impl.rs`
- `julia-tests/runtests.trace-id.jl`
- `docs/main.command.trace-id.readme.md`
- `docs/main.improvement.9.trace-id.complete.md`
