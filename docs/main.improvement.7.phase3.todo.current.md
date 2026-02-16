# Improvement 7 Phase 3: trace-stats

Status: `todo.current` (active)

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

## Active Patch Lane (Guardrails)

Parallel to `trace-stats`, we are tracking a trace safety override patch:

- `docs/main.command.trace.force.guardrails.todo.current.md`

This lane keeps default safety behavior, but adds explicit `--force` bypass for requested deep traces.

## Code-First Rule

Follow code-first eventness:
1. implement and validate code changes first
2. mirror implementation state in docs/eventness artifacts

## Related Files

- `docs/main.improvement.7.phase3.todo.current.reference.md`
- `docs/main.improvement.7.phase3.todo.trigger.event.md`
- `docs/main.improvement.7.todo.future-plan.md`
- `docs/main.command.trace.force.guardrails.todo.current.md`
