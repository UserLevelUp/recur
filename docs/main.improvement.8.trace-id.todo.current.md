# Improvement 8: trace-id MVP

Status: `todo.current` (active)
Date: 2026-03-03

## Goal

Implement a build-now `trace-id` MVP that can trace hierarchical identifier flow using built-in heuristics and existing recur infrastructure.

## In Scope (Now)

1. Add `recur trace-id` CLI command surface.
2. Implement built-in role classification for identifier matches:
   - `define`
   - `produce`
   - `consume`
   - `trigger`
3. Support scope/ext/stdin filtering consistent with existing content-search commands.
4. Add JSON output and human-readable terminal output for traced identifier flows.
5. Reuse traversal guardrails (`--depth-guard`, `--force`) where applicable.
6. Add unit and Julia integration tests for MVP behavior.

## Out of Scope (Deferred)

Deferred to Improvement 9:

1. Project-custom heuristic config (`.recur/trace-id.toml`).
2. `merge --edge-type` semantic lane stitching.
3. Full multi-lane composition across call/route/config graphs.

## Working Files

- `src/main.rs`
- `src/main_command_trace_id_impl.rs` (new)
- `src/search.rs` (trace-id support)
- `src/output.rs` (trace-id formatting)
- `julia-tests/main.command.trace-id.test.jl` (new)
- `julia-tests/runtests.trace-id.jl` (new)
- `docs/main.command.trace-id.readme.md` (new)

## Success Criteria

1. `recur trace-id --help` documents command usage and examples.
2. `recur trace-id "<id>" --scope "**"` produces stable JSON and terminal output.
3. MVP tests pass for define/produce/consume/trigger detection.
4. No regressions in existing `trace`, `id`, and `merge` command behavior.

## Related

- `docs/main.improvement.8.trace-id.todo.current.reference.md`
- `docs/main.improvement.8.trace-id.todo.trigger.event.md`
- `docs/main.improvement.9.trace-id.todo.future-plan.md`
