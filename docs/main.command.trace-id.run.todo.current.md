# Command: trace-id Saved Runs

Status: `todo.current`
Date: 2026-04-05

## Purpose

Track the saved-run persistence layer for `recur trace-id` so the next polish work
stays visible in eventness instead of living only in code or chat memory.

Current behavior is still `latest`-oriented. The open design question is whether
saved runs stay lightweight reusable evidence or become part of a richer retained
eventness history under `.recur/`.

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
- `Phase 4c: transition audit stays traceable across rollback`

Covered behaviors:

1. saving a named run creates `manifest.toml` and `latest.json`
2. `--check-run` reports `fresh` when request/config/files still match
3. `--reuse-if-fresh` replays the saved JSON instead of recomputing
4. changing the scoped file set marks the run `stale`
5. forward and rollback eventness transitions can be re-traced after a stale run
   is refreshed

Repro:

```powershell
julia julia-tests/main.command.trace-id.test.jl
```

## Remaining Todo

1. Decide whether saved runs should keep timestamped history under
   `.recur/trace-id/runs/<name>/history/` or remain `latest` only
2. Decide whether freshness should stay metadata-based (`path` + `size` + `mtime`) or
   grow an optional content-hash mode
3. Add one durable example showing how saved runs fit with eventness workflows in
   `.recur/`
4. Decide whether future `.recur/eventness` snapshots should count `fresh` vs `stale`
   trace-id runs
5. Decide whether saved runs remain reusable evidence only, or whether future
   mirror/in-file eventness treats them as derived audit artifacts tied to one
   canonical source of truth

## Not This Lane

- `trace-id --json | merge --stdin` retaining `edge_type` is complete in
  `docs/main.improvement.9.trace-id.complete.md`
- role keyword configuration remains in `.recur/config.toml` under `[traits.trace_id]`

## Close-out Criteria

1. Persistence policy is explicit (`latest` only or `history/` added)
2. Freshness policy is explicit enough to document as stable behavior
3. Saved-run artifacts have an explicit relationship to future canonical
   eventness or mirror state
4. This lane collapses into either `recurring` or `future-plan` once the workflow settles

## References

- `src/main_command_trace_id_impl.rs`
- `julia-tests/runtests.trace-id.jl`
- `docs/main.command.trace-id.readme.md`
- `docs/main.improvement.9.trace-id.complete.md`
- `docs/main.improvement.14.todo.future-plan.md`
