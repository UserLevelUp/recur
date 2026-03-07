# Improvement 7 Phase 3: trace-stats

Status: `todo.current` (active)
Date: 2026-03-06

## Goal

Implement `trace-stats` to provide call graph complexity statistics across scoped functions.

## Source Spec

- `README.CORE.IMPROVEMENT7.md`

## Phase 3 Scope

1. Add `trace-stats` CLI command surface and validation.
2. Add stats collection pipeline (direct, transitive, circular patterns, depth, risk).
3. Add sorting/filtering/top-N behavior.
4. Add table/JSON/CSV output support.
5. Add tests for core metrics and filters.

## Phase 3 State

- Step 1 complete: CLI surface + validation (`docs/main.command.trace-stats.cli-surface.complete.md`)
- Core metrics implementation active: `docs/main.command.trace-stats.metrics.todo.current.md`
- Current focus: metric accuracy hardening (distinct cycle patterns), stdin coverage, and performance checks

## Eventness Snapshot (2026-03-01)

- Validation run: `julia julia-tests/main.command.trace-stats.test.jl`
- Current suite status: `74 pass`, `6 broken` (intentional placeholder lanes via `@test_skip`)
- Active execution lane remains metrics hardening, not command-surface work.
- Ordering lanes active and passing: `sort-by depth`, `sort-by risk`.
- Remaining work is concentrated in six placeholder tests across stdin, circular accuracy, risk fixtures, and performance.

Phase 3 work left is concentrated in:

1. Add stdin-focused trace-stats integration assertions.
2. Upgrade circular metric to distinct cycle-pattern counting and add no-false-positive coverage.
3. Add medium/high risk fixture assertions.
4. Add larger-scope performance regression fixture.

## Phase 3 Exit Criteria

1. Remove remaining `trace-stats` `@test_skip` placeholders by implementing those lanes.
2. Keep `recur trace-stats` behavior stable for scope/filter/sort/top/guardrail options.
3. Validate `julia julia-tests/main.command.trace-stats.test.jl` with no broken placeholders.
4. Record completion eventness and promote to `docs/main.improvement.7.phase3.complete.md`.

## Recent Completions

- `docs/main.command.trace.force.guardrails.complete.md`
- `docs/main.command.trace-stats.cli-surface.complete.md`

## Active Patch Lane

- `docs/main.command.trace-stats.metrics.todo.current.md`

## Code-First Rule

Follow code-first eventness:
1. implement and validate code changes first
2. mirror implementation state in docs/eventness artifacts

## Related Files

- `docs/main.improvement.7.phase3.todo.current.reference.md`
- `docs/main.improvement.7.phase3.todo.trigger.event.md`
- `docs/main.improvement.7.todo.future-plan.md`
- `docs/main.command.trace-stats.metrics.todo.current.md`
