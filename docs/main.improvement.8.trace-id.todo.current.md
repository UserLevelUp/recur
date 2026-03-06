# Improvement 8: trace-id MVP

Status: `todo.current` (active)
Date: 2026-03-03

## Goal

Implement a build-now `trace-id` MVP that can trace hierarchical identifier flow using built-in heuristics and existing recur infrastructure.

## Active Lane Decision

- Flatten-specific follow-up is explicitly paused while trace-id MVP is active:
  - `docs/main.command.flatten.todo.future-plan.md`
- Trace-id is now the top active implementation lane for Improvement 8.

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

## Action Plan (Crunchable)

### Phase A: Command Surface

1. Add `TraceId` command in `src/main.rs`.
2. Add parser-validated flags:
   - identifier pattern
   - `--scope`
   - `--ext`
   - `--format`
   - `--stdin`
   - `--depth`
   - `--depth-guard`
   - `--force`
3. Wire dispatch to new `main_command_trace_id_impl::execute`.

### Phase B: Heuristic Engine

1. Add built-in heuristic role detection in search layer:
   - define (`const/static readonly` style)
   - produce (`Publish*`, `Send*`, `Emit*`)
   - consume (`Subscribe`, `QueueBind`, `routingKey`)
   - trigger (pattern registration references)
2. Start with deterministic regex/substring matchers.
3. Keep MVP in-code only; no external heuristic config files yet.

### Phase C: Output + Validation

1. Add terminal and JSON output shape for trace-id results.
2. Add unit tests for role classification and parser behavior.
3. Add Julia integration tests with phased `@test_broken` contracts.
4. Keep command out of full-suite include until minimum contract lands.

## Implemented (Previously Listed as Deferred)

The following was originally listed as "Improvement 9" scope but landed in Improvement 8:

1. Project-custom heuristic config — implemented as `[traits.trace_id]` in `.recur/config.toml`
   (not a separate `.recur/trace-id.toml`; uses the unified trait system).
   - `recur trait set trace_id.producer_keywords "..."` tunes classification at runtime.
   - `src/trait/trace_id.rs` — `resolve_trace_id_policy()` reads this section.
2. `recur trait` command — first-class `list`/`get`/`set` CLI for managing all trait config.
   See `docs/main.command.trait.readme.md` and `docs/main.command.trait.todo.current.md`.

## Still Deferred (Improvement 9)

1. `merge --edge-type` semantic lane stitching.
2. Full multi-lane composition across call/route/config graphs.

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
- `docs/main.command.trace-id.test.todo.current.md`
- `docs/main.command.trace-id.test.todo.current.reference.md`
- `docs/main.command.trace-id.test.todo.trigger.event.md`
- `docs/main.command.trace-id.pipeline.todo.md`
- `docs/main.command.flatten.todo.future-plan.md`
- `docs/main.improvement.9.trace-id.todo.future-plan.md`
