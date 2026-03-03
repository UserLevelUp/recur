# Command: trace-id JSON Pipeline Composition

Status: `todo`
Date: 2026-03-03

## Goal

Track cross-command JSON composition contracts that feed recur analysis outputs into `merge --stdin`.

## Contract Targets

1. `trace --json` piped to `merge --stdin`.
2. `callers --json` piped to `merge --stdin`.
3. `callees --json` piped to `merge --stdin`.
4. `trace-id --json` piped to `merge --stdin`.

## Current State

- Placeholder tests exist in `julia-tests/runtests.trace-id.jl` under Phase 5.
- Contracts are intentionally `@test_broken` until semantic merge metadata lands.

## Completion Criteria

1. Replace placeholder expectations with active assertions.
2. Validate stable composed output schema for pipeline workflows.
3. Add command docs examples once behavior is stable.

## Related

- `docs/main.command.trace-id.test.todo.current.md`
- `docs/main.improvement.8.trace-id.todo.current.md`
- `docs/main.improvement.9.trace-id.todo.future-plan.md`
